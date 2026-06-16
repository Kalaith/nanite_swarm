//! Nanite Swarm - A self-replicating AI simulation
//!
//! Entry point, game loop, and phase transitions.

#![allow(clippy::too_many_arguments, clippy::wrong_self_convention)]

use macroquad::prelude::*;

mod assets;
mod data;
mod directives;
mod engine;
mod screens;
mod state;
mod ui;

use assets::GameTextures;
use data::{load_game_config, load_game_data, load_ui_theme, set_game_data};
use directives::{pick_directive, Directive};
use engine::{ResearchState, ResearchTree};
use screens::{
    render_interplanetary_view, render_main_menu, render_planetary_view, render_research_view,
    render_settings_menu, InterplanetaryAction, MenuAction, PlanetaryAction, ResearchAction,
    Settings, SettingsAction,
};
use state::{load_from_file, save_to_file, PlanetState};

/// Game phases/screens
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamePhase {
    MainMenu,
    Playing,
    Research,
    Interplanetary,
    Settings,
}

/// Main game state container
pub struct Game {
    phase: GamePhase,
    planet_state: PlanetState,
    research_tree: ResearchTree,
    research_state: ResearchState,
    settings: Settings,
    has_save: bool,
    current_planet_index: usize,
    colonized_planets: [bool; 5],
    textures: GameTextures,
    directive: Directive,
    directive_timer: f32,
    directive_tier: i32,
    config: data::GameConfig,
    ui_theme: data::UiTheme,
}

const SAVE_PATH: &str = "save.json";
const RESEARCH_RATE: f32 = 5.0; // data per second

impl Game {
    pub async fn new() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let config = load_game_config();
        #[cfg(target_arch = "wasm32")]
        let config = load_game_config().await;

        #[cfg(not(target_arch = "wasm32"))]
        let ui_theme = load_ui_theme();
        #[cfg(target_arch = "wasm32")]
        let ui_theme = load_ui_theme().await;

        #[cfg(not(target_arch = "wasm32"))]
        let game_data = load_game_data();
        #[cfg(target_arch = "wasm32")]
        let game_data = load_game_data().await;

        set_game_data(game_data);
        let directive = pick_directive(0);
        Self {
            phase: GamePhase::MainMenu,
            planet_state: PlanetState::new("Mars", 24, 24, 42, config.clone()),
            research_tree: ResearchTree::default(),
            research_state: ResearchState::default(),
            settings: Settings::default(),
            has_save: false,
            current_planet_index: 2, // Mars is starting planet
            colonized_planets: [false, false, true, false, false], // Mars colonized
            textures: GameTextures::load().await,
            directive,
            directive_timer: 0.0,
            directive_tier: 0,
            config,
            ui_theme,
        }
    }

    /// Check if mass driver technology is researched
    fn has_mass_driver(&self) -> bool {
        self.research_state.is_unlocked("mass_driver")
    }

    pub fn update(&mut self) {
        match self.phase {
            GamePhase::MainMenu => match render_main_menu(self.has_save) {
                MenuAction::NewGame => {
                    self.planet_state = PlanetState::new(
                        "Mars",
                        24,
                        24,
                        macroquad_toolkit::rng::random_u64(),
                        self.config.clone(),
                    );
                    self.research_state = ResearchState::default();
                    self.sync_research_to_planet();
                    self.sync_building_unlocks();
                    self.phase = GamePhase::Playing;
                }
                MenuAction::Continue => {
                    self.phase = GamePhase::Playing;
                }
                MenuAction::Load => {
                    if let Ok(state) = load_from_file(SAVE_PATH) {
                        self.planet_state = state;
                        self.sync_research_from_planet();
                        self.phase = GamePhase::Playing;
                        self.has_save = true;
                        self.sync_building_unlocks();
                    }
                }
                MenuAction::Save => {
                    let _ = save_to_file(&mut self.planet_state, SAVE_PATH);
                    self.has_save = true;
                }
                MenuAction::Settings => {
                    self.phase = GamePhase::Settings;
                }
                MenuAction::Quit => {}
                MenuAction::None => {}
            },
            GamePhase::Settings => match render_settings_menu(&mut self.settings) {
                SettingsAction::Back => {
                    self.phase = GamePhase::MainMenu;
                }
                SettingsAction::None => {}
            },
            GamePhase::Playing => {
                let delta = get_frame_time();
                self.planet_state.update(delta);
                self.update_research(delta);
                self.update_directives(delta);

                match render_planetary_view(
                    &mut self.planet_state,
                    self.settings.show_fps,
                    &self.textures,
                    &self.directive,
                    &self.ui_theme,
                ) {
                    PlanetaryAction::OpenResearch => {
                        self.phase = GamePhase::Research;
                    }
                    PlanetaryAction::OpenInterplanetary => {
                        self.phase = GamePhase::Interplanetary;
                    }
                    PlanetaryAction::OpenMenu => {
                        self.phase = GamePhase::MainMenu;
                        self.has_save = true;
                    }
                    PlanetaryAction::None => {}
                }
            }
            GamePhase::Research => {
                let delta = get_frame_time();
                self.planet_state.update(delta);
                self.update_research(delta);
                match render_research_view(
                    &self.research_state,
                    &self.research_tree,
                    self.planet_state.resources.data,
                    self.planet_state.research_lock_timer > 0.0,
                ) {
                    ResearchAction::Close => {
                        self.phase = GamePhase::Playing;
                    }
                    ResearchAction::StartResearch(tech_id) => {
                        let _ = self.research_state.start_research(
                            &tech_id,
                            &self.research_tree,
                            self.planet_state.resources.data,
                        );
                    }
                    ResearchAction::None => {}
                }
            }
            GamePhase::Interplanetary => {
                match render_interplanetary_view(
                    self.current_planet_index,
                    self.has_mass_driver(),
                    &self.colonized_planets,
                ) {
                    InterplanetaryAction::Close => {
                        self.phase = GamePhase::Playing;
                    }
                    InterplanetaryAction::SelectPlanet(index) => {
                        // Travel to colonized planet
                        if self.colonized_planets[index] {
                            self.current_planet_index = index;
                            // Load planet state here (for now, create new)
                            let planet_names = ["Mercury", "Venus", "Mars", "Jupiter", "Saturn"];
                            self.planet_state = PlanetState::new(
                                planet_names[index],
                                24,
                                24,
                                macroquad_toolkit::rng::random_u64(),
                                self.config.clone(),
                            );
                            self.phase = GamePhase::Playing;
                        }
                    }
                    InterplanetaryAction::LaunchMassDriver(index) => {
                        // Launch colonization probe (costs resources)
                        if self.planet_state.resources.minerals >= 100.0 {
                            self.planet_state.resources.minerals -= 100.0;
                            self.colonized_planets[index] = true;
                        }
                    }
                    InterplanetaryAction::None => {}
                }
            }
        }
    }

    fn update_research(&mut self, delta_time: f32) {
        let Some(current_id) = self.research_state.current_research.clone() else {
            self.planet_state.self_cleaning_unlocked =
                self.research_state.is_unlocked("self_cleaning_servos");
            self.sync_building_unlocks();
            self.sync_research_to_planet();
            return;
        };
        if self.planet_state.research_lock_timer > 0.0 {
            return;
        }
        let Some(node) = self.research_tree.get_node(&current_id) else {
            self.research_state.current_research = None;
            self.research_state.research_progress = 0.0;
            self.sync_research_to_planet();
            return;
        };

        let remaining = (node.data_cost - self.research_state.research_progress).max(0.0);
        if remaining <= 0.0 {
            self.research_state.complete_research();
            self.planet_state.self_cleaning_unlocked =
                self.research_state.is_unlocked("self_cleaning_servos");
            self.sync_building_unlocks();
            self.sync_research_to_planet();
            return;
        }

        let available = self.planet_state.resources.data;
        if available <= 0.0 {
            return;
        }

        let spend = (RESEARCH_RATE * delta_time).min(available).min(remaining);
        self.planet_state.resources.data -= spend;
        self.research_state.research_progress += spend;

        if self.research_state.research_progress >= node.data_cost {
            self.research_state.complete_research();
        }

        self.planet_state.self_cleaning_unlocked =
            self.research_state.is_unlocked("self_cleaning_servos");
        self.sync_building_unlocks();
        self.sync_research_to_planet();
    }

    fn sync_research_from_planet(&mut self) {
        self.research_state.unlocked = self.planet_state.research.unlocked_techs.clone();
        for tech in &data::game_data().research.starting_unlocked {
            if !self.research_state.unlocked.contains(tech) {
                self.research_state.unlocked.push(tech.clone());
            }
        }
        self.research_state.current_research = self.planet_state.research.current_research.clone();
        self.research_state.research_progress = self.planet_state.research.research_progress;
    }

    fn sync_research_to_planet(&mut self) {
        self.planet_state.research.unlocked_techs = self.research_state.unlocked.clone();
        self.planet_state.research.current_research = self.research_state.current_research.clone();
        self.planet_state.research.research_progress = self.research_state.research_progress;
    }

    fn sync_building_unlocks(&mut self) {
        for def in &data::game_data().buildings {
            let Some(building_type) = engine::BuildingType::from_id(&def.id) else {
                continue;
            };
            let unlocked = def.start_unlocked
                || def
                    .unlocked_by
                    .as_deref()
                    .map(|tech| self.research_state.is_unlocked(tech))
                    .unwrap_or(false);
            if unlocked {
                self.planet_state.unlock_building(building_type);
            }
        }
    }

    fn update_directives(&mut self, delta_time: f32) {
        self.directive_timer += delta_time;
        if self.directive_timer >= 600.0
            || self.directive.duration <= 0.0
            || self.directive.completed
        {
            if self.directive.completed {
                self.planet_state.resources.data += self.directive.reward_data;
            }
            self.directive_timer = 0.0;
            self.directive_tier += 1;
            self.directive = pick_directive(self.directive_tier);
        }
        self.directive.update(&self.planet_state, delta_time);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Nanite Swarm".to_string(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        game.update();
        next_frame().await;
    }
}
