//! Nanite Swarm - A self-replicating AI simulation
//!
//! Entry point, game loop, and phase transitions.

use macroquad::prelude::*;

mod data;
mod engine;
mod state;
mod ui;
mod screens;

use engine::{ResearchTree, ResearchState};
use state::PlanetState;
use screens::{
    render_main_menu, MenuAction,
    render_planetary_view, PlanetaryAction,
    render_research_view, ResearchAction,
    render_interplanetary_view, InterplanetaryAction,
};

/// Game phases/screens
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamePhase {
    MainMenu,
    Playing,
    Research,
    Interplanetary,
}

/// Main game state container
pub struct Game {
    phase: GamePhase,
    planet_state: PlanetState,
    research_tree: ResearchTree,
    research_state: ResearchState,
    has_save: bool,
    current_planet_index: usize,
    colonized_planets: [bool; 5],
}

impl Game {
    pub fn new() -> Self {
        Self {
            phase: GamePhase::MainMenu,
            planet_state: PlanetState::default(),
            research_tree: ResearchTree::default(),
            research_state: ResearchState::default(),
            has_save: false,
            current_planet_index: 2, // Mars is starting planet
            colonized_planets: [false, false, true, false, false], // Mars colonized
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
                        self.phase = GamePhase::Playing;
                    }
                    MenuAction::Continue => {
                        self.phase = GamePhase::Playing;
                    }
                    MenuAction::Settings => {}
                    MenuAction::Quit => {}
                    MenuAction::None => {}
                }
            }
            GamePhase::Playing => {
                let delta = get_frame_time();
                self.planet_state.update(delta);

                match render_planetary_view(&mut self.planet_state) {
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
                match render_research_view(
                    &self.research_state,
                    &self.research_tree,
                    self.planet_state.resources.data,
                ) {
                    ResearchAction::Close => {
                        self.phase = GamePhase::Playing;
                    }
                    ResearchAction::StartResearch(tech_id) => {
                        // Deduct cost and start research
                        if let Some(node) = self.research_tree.get_node(&tech_id) {
                            if self.planet_state.resources.data >= node.data_cost {
                                self.planet_state.resources.data -= node.data_cost;
                                self.research_state.unlocked.push(tech_id);
                            }
                        }
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
    let mut game = Game::new();

    loop {
        game.update();
        next_frame().await;
    }
}
