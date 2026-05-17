//! Short-term directives for pacing and rewards.

use crate::engine::BuildingType;
use crate::state::PlanetState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DirectiveKind {
    PowerSurplus,
    DrillCount,
    ServerBanks,
    HarvestForest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directive {
    pub kind: DirectiveKind,
    pub description: String,
    pub target: i32,
    pub progress: i32,
    pub duration: f32,
    pub reward_data: f32,
    pub completed: bool,
}

impl Directive {
    pub fn new(kind: DirectiveKind, target: i32, duration: f32, reward_data: f32) -> Self {
        let description = match kind {
            DirectiveKind::PowerSurplus => format!("Sustain +{} power for 60s", target),
            DirectiveKind::DrillCount => format!("Operate {} drills", target),
            DirectiveKind::ServerBanks => format!("Run {} server banks", target),
            DirectiveKind::HarvestForest => format!("Harvest {} forests", target),
        };
        Self {
            kind,
            description,
            target,
            progress: 0,
            duration,
            reward_data,
            completed: false,
        }
    }

    pub fn update(&mut self, state: &PlanetState, delta: f32) {
        if self.completed {
            return;
        }
        self.duration = (self.duration - delta).max(0.0);
        match self.kind {
            DirectiveKind::PowerSurplus => {
                if state.power_balance >= self.target as f32 {
                    self.progress = (self.progress + (delta * 1.0) as i32).min(self.target);
                } else {
                    self.progress = self.progress.saturating_sub(1);
                }
            }
            DirectiveKind::DrillCount => {
                let count = state.grid.find_buildings(BuildingType::Drill).len() as i32;
                self.progress = count.min(self.target);
            }
            DirectiveKind::ServerBanks => {
                let count = state.grid.find_buildings(BuildingType::ServerBank).len() as i32;
                self.progress = count.min(self.target);
            }
            DirectiveKind::HarvestForest => {
                self.progress = state.forest_harvested_count.min(self.target);
            }
        }
        if self.progress >= self.target {
            self.completed = true;
        }
    }
}

pub fn pick_directive(tier: i32) -> Directive {
    match tier % 4 {
        0 => Directive::new(
            DirectiveKind::PowerSurplus,
            20 + tier * 5,
            600.0,
            20.0 + tier as f32 * 5.0,
        ),
        1 => Directive::new(
            DirectiveKind::DrillCount,
            3 + tier,
            600.0,
            15.0 + tier as f32 * 4.0,
        ),
        2 => Directive::new(
            DirectiveKind::ServerBanks,
            2 + tier / 2,
            600.0,
            20.0 + tier as f32 * 5.0,
        ),
        _ => Directive::new(
            DirectiveKind::HarvestForest,
            2 + tier / 2,
            600.0,
            10.0 + tier as f32 * 3.0,
        ),
    }
}
