//! Current planetary state

use serde::{Deserialize, Serialize};
use crate::engine::{Grid, GridPos, BuildingType, DroneManager, find_path, DroneState, TerrainType};

/// Resources held by the player
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Resources {
    pub energy: f32,
    pub minerals: f32,
    pub data: f32,
    pub biomass: f32,
}

impl Resources {
    /// Check if player can afford a cost
    pub fn can_afford(&self, minerals: f32, energy: f32) -> bool {
        self.minerals >= minerals && self.energy >= energy
    }

    /// Deduct cost from resources
    pub fn spend(&mut self, minerals: f32, energy: f32) -> bool {
        if self.can_afford(minerals, energy) {
            self.minerals -= minerals;
            self.energy -= energy;
            true
        } else {
            false
        }
    }
}

/// Research node status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchProgress {
    pub unlocked_techs: Vec<String>,
    pub current_research: Option<String>,
    pub research_progress: f32,
}

impl Default for ResearchProgress {
    fn default() -> Self {
        Self {
            unlocked_techs: vec!["basic_mining".to_string()],
            current_research: None,
            research_progress: 0.0,
        }
    }
}

/// Current game state for a planet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetState {
    pub name: String,
    pub resources: Resources,
    pub grid: Grid,
    pub drones: DroneManager,
    pub research: ResearchProgress,
    pub time_played: f64,
    pub selected_building: Option<BuildingType>,
    pub power_balance: f32,
    // Drill production timers (drill position -> accumulated time)
    #[serde(skip)]
    pub drill_timers: std::collections::HashMap<(i32, i32), f32>,
    // Server bank data generation timers
    #[serde(skip)]
    pub server_timers: std::collections::HashMap<(i32, i32), f32>,
}

impl PlanetState {
    pub fn new(name: &str, width: u32, height: u32, seed: u64) -> Self {
        let mut grid = Grid::new_with_terrain(width, height, seed);

        // Place Core at center
        let center = GridPos::new(width as i32 / 2, height as i32 / 2);
        grid.place_building(center, BuildingType::Core);
        grid.update_power_grid();

        Self {
            name: name.to_string(),
            resources: Resources {
                energy: 100.0,
                minerals: 50.0,
                data: 0.0,
                biomass: 0.0,
            },
            grid,
            drones: DroneManager::new(10.0, 2.0),
            research: ResearchProgress::default(),
            time_played: 0.0,
            selected_building: Some(BuildingType::Drill),
            power_balance: 10.0,
            drill_timers: std::collections::HashMap::new(),
            server_timers: std::collections::HashMap::new(),
        }
    }

    /// Try to place a building at position
    pub fn try_place_building(&mut self, pos: GridPos) -> bool {
        if let Some(building_type) = self.selected_building {
            let (mineral_cost, energy_cost) = building_type.cost();

            if !self.resources.can_afford(mineral_cost, energy_cost) {
                return false;
            }

            if self.grid.place_building(pos, building_type) {
                self.resources.spend(mineral_cost, energy_cost);

                // Spawn initial drone for drills
                if building_type == BuildingType::Drill {
                    self.drones.spawn_drone(pos);
                    self.drill_timers.insert((pos.x, pos.y), 0.0);
                }

                // Track server banks
                if building_type == BuildingType::ServerBank {
                    self.server_timers.insert((pos.x, pos.y), 0.0);
                }

                // Reveal area around new building
                self.grid.reveal_around(pos, 3);

                // Update power grid
                self.grid.update_power_grid();
                self.power_balance = self.grid.net_power();

                return true;
            }
        }
        false
    }

    /// Try to harvest terrain at position
    pub fn try_harvest_terrain(&mut self, pos: GridPos) -> bool {
        if let Some(tile) = self.grid.get(pos) {
            if !tile.revealed || !tile.terrain.is_harvestable() || tile.building.is_some() {
                return false;
            }

            let terrain = tile.terrain;
            let (minerals, biomass) = terrain.harvest_rewards();

            // Apply harvest
            if let Some(tile) = self.grid.get_mut(pos) {
                tile.terrain = terrain.harvested();
            }

            self.resources.minerals += minerals;
            self.resources.biomass += biomass;

            true
        } else {
            false
        }
    }

    /// Check if terrain at position can be harvested
    pub fn can_harvest(&self, pos: GridPos) -> bool {
        if let Some(tile) = self.grid.get(pos) {
            tile.revealed && tile.terrain.is_harvestable() && tile.building.is_none()
        } else {
            false
        }
    }

    /// Update game simulation
    pub fn update(&mut self, delta_time: f32) {
        self.time_played += delta_time as f64;

        // Update drones
        let events = self.drones.update(delta_time);
        for event in events {
            match event {
                crate::engine::DroneEvent::ReachedCore { amount, .. } => {
                    self.resources.minerals += amount;
                }
                crate::engine::DroneEvent::ReachedDrill { drone_id } => {
                    if let Some(drone) = self.drones.get_drone_mut(drone_id) {
                        drone.state = DroneState::Idle;
                    }
                }
                _ => {}
            }
        }

        // Process drills and server banks
        self.update_drills(delta_time);
        self.update_servers(delta_time);

        // Power-based energy generation
        let net_power = self.grid.net_power();
        self.power_balance = net_power;
        self.resources.energy += net_power * delta_time;

        // Cap resources
        self.resources.energy = self.resources.energy.clamp(0.0, 1000.0);
        self.resources.minerals = self.resources.minerals.min(1000.0);
        self.resources.data = self.resources.data.min(1000.0);
        self.resources.biomass = self.resources.biomass.min(1000.0);
    }

    /// Update drill production and drone dispatching
    fn update_drills(&mut self, delta_time: f32) {
        let drill_positions = self.grid.find_buildings(BuildingType::Drill);
        let core_pos = self.grid.find_core();

        if let Some(core) = core_pos {
            for drill_pos in drill_positions {
                // Check if drill is powered
                let is_powered = self.grid.get(drill_pos)
                    .and_then(|t| t.building.as_ref())
                    .map(|b| b.powered)
                    .unwrap_or(false);

                if !is_powered {
                    continue;
                }

                let key = (drill_pos.x, drill_pos.y);
                let timer = self.drill_timers.entry(key).or_insert(0.0);
                *timer += delta_time;

                if *timer >= 2.0 {
                    *timer = 0.0;

                    let idle_drone = self.drones.drones()
                        .iter()
                        .find(|d| d.home_drill == drill_pos && d.state == DroneState::Idle)
                        .map(|d| d.id);

                    if let Some(drone_id) = idle_drone {
                        let path = find_path(drill_pos, core);
                        if let Some(drone) = self.drones.get_drone_mut(drone_id) {
                            drone.dispatch_to_core(core, path, 10.0);
                        }
                    }
                }
            }
        }
    }

    /// Update server bank data generation
    fn update_servers(&mut self, delta_time: f32) {
        let server_positions = self.grid.find_buildings(BuildingType::ServerBank);

        for server_pos in server_positions {
            // Check if server is powered
            let is_powered = self.grid.get(server_pos)
                .and_then(|t| t.building.as_ref())
                .map(|b| b.powered)
                .unwrap_or(false);

            if !is_powered {
                continue;
            }

            let key = (server_pos.x, server_pos.y);
            let timer = self.server_timers.entry(key).or_insert(0.0);
            *timer += delta_time;

            // Generate 1 data per second when powered
            if *timer >= 1.0 {
                *timer = 0.0;
                self.resources.data += 1.0;
            }
        }
    }

    /// Select a building type for placement
    pub fn select_building(&mut self, building_type: BuildingType) {
        self.selected_building = Some(building_type);
    }

    /// Clear building selection
    pub fn clear_selection(&mut self) {
        self.selected_building = None;
    }
}

impl Default for PlanetState {
    fn default() -> Self {
        Self::new("Mars", 24, 24, 42)
    }
}
