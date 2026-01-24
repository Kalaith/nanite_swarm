//! Game balance values and settings

use serde::{Deserialize, Serialize};

/// Core game configuration loaded from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub grid: GridConfig,
    pub resources: ResourceConfig,
    pub buildings: BuildingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    pub initial_width: u32,
    pub initial_height: u32,
    pub tile_size: f32,
    pub max_width: u32,
    pub max_height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub starting_energy: f32,
    pub starting_minerals: f32,
    pub drone_carry_capacity: f32,
    pub drone_speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingConfig {
    pub core_power_consumption: f32,
    pub drill_output_rate: f32,
    pub conduit_throughput: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            grid: GridConfig {
                initial_width: 16,
                initial_height: 16,
                tile_size: 32.0,
                max_width: 64,
                max_height: 64,
            },
            resources: ResourceConfig {
                starting_energy: 100.0,
                starting_minerals: 50.0,
                drone_carry_capacity: 10.0,
                drone_speed: 50.0,
            },
            buildings: BuildingConfig {
                core_power_consumption: 5.0,
                drill_output_rate: 2.0,
                conduit_throughput: 10.0,
            },
        }
    }
}
