//! Nanite Swarm - A self-replicating AI simulation
//!
//! Entry point, game loop, and phase transitions.

use macroquad::prelude::*;

mod data;
mod engine;
mod state;
mod ui;
mod screens;
mod assets;
mod directives;

use engine::{ResearchTree, ResearchState};
use state::{PlanetState, save_to_file, load_from_file};
use assets::GameTextures;
use screens::{
    render_main_menu, MenuAction,
    render_planetary_view, PlanetaryAction,
    render_research_view, ResearchAction,
    render_interplanetary_view, InterplanetaryAction,
    render_settings_menu, SettingsAction, Settings,
};
use directives::{Directive, pick_directive};

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
}

const SAVE_PATH: &str = "save.json";
const RESEARCH_RATE: f32 = 5.0; // data per second

impl Game {
    pub async fn new() -> Self {
        let directive = pick_directive(0);
        Self {
            phase: GamePhase::MainMenu,
            planet_state: PlanetState::default(),
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
        }
    }

    /// Check if mass driver technology is researched
    fn has_mass_driver(&self) -> bool {
        self.research_state.is_unlocked("mass_driver")
    }

    pub fn update(&mut self) {
        match self.phase {
            GamePhase::MainMenu => {
                match render_main_menu(self.has_save) {
                    MenuAction::NewGame => {
                        self.planet_state = PlanetState::new("Mars", 24, 24, rand::gen_range(0u64, u64::MAX));
                        self.research_state = ResearchState::default();
                        self.sync_building_unlocks();
                        self.phase = GamePhase::Playing;
                    }
                    MenuAction::Continue => {
                        self.phase = GamePhase::Playing;
                    }
                    MenuAction::Load => {
                        if let Ok(state) = load_from_file(SAVE_PATH) {
                            self.planet_state = state;
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
                }
            }
            GamePhase::Settings => {
                match render_settings_menu(&mut self.settings) {
                    SettingsAction::Back => {
                        self.phase = GamePhase::MainMenu;
                    }
                    SettingsAction::None => {}
                }
            }
            GamePhase::Playing => {
                let delta = get_frame_time();
                self.planet_state.update(delta);
                self.update_research(delta);
                self.update_directives(delta);

                match render_planetary_view(&mut self.planet_state, self.settings.show_fps, &self.textures, &self.directive) {
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
                                24, 24,
                                rand::gen_range(0u64, u64::MAX),
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
            self.planet_state.self_cleaning_unlocked = self.research_state.is_unlocked("self_cleaning_servos");
            self.sync_building_unlocks();
            return;
        };
        if self.planet_state.research_lock_timer > 0.0 {
            return;
        }
        let Some(node) = self.research_tree.get_node(&current_id) else {
            self.research_state.current_research = None;
            self.research_state.research_progress = 0.0;
            return;
        };

        let remaining = (node.data_cost - self.research_state.research_progress).max(0.0);
        if remaining <= 0.0 {
            self.research_state.complete_research();
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

        self.planet_state.self_cleaning_unlocked = self.research_state.is_unlocked("self_cleaning_servos");
        self.sync_building_unlocks();
    }

    fn sync_building_unlocks(&mut self) {
        if self.research_state.is_unlocked("power_grid") {
            self.planet_state.unlock_building(engine::BuildingType::Conduit);
            self.planet_state.unlock_building(engine::BuildingType::PowerNode);
            self.planet_state.unlock_building(engine::BuildingType::Bridge);
        }
        if self.research_state.is_unlocked("data_processing") {
            self.planet_state.unlock_building(engine::BuildingType::ServerBank);
        }
        if self.research_state.is_unlocked("wind_power") {
            self.planet_state.unlock_building(engine::BuildingType::WindTurbine);
        }
        if self.research_state.is_unlocked("self_cleaning_servos") {
            self.planet_state.unlock_building(engine::BuildingType::Sweeper);
        }
    }

    fn update_directives(&mut self, delta_time: f32) {
        self.directive_timer += delta_time;
        if self.directive_timer >= 600.0 || self.directive.duration <= 0.0 || self.directive.completed {
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
