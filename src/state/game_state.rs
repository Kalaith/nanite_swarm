//! Current planetary state

use crate::data::GameConfig;
use crate::engine::{
    find_path, BuildingType, DroneManager, DroneState, Grid, GridPos, TerrainType,
};
use macroquad::miniquad;
use macroquad::prelude::Color;
use serde::{Deserialize, Serialize};

const DUST_RATE: f32 = 0.12; // dust per second
const SWEEPER_RATE: f32 = 0.6; // dust cleared per second
const SWEEPER_RADIUS: i32 = 3;
const FILTER_RADIUS: i32 = 3;
const FILTER_RATE_MULTIPLIER: f32 = 0.6;
const POLLUTION_RADIUS: i32 = 3;
const POLLUTION_RATE_MULTIPLIER: f32 = 1.3;
// Config-driven values are loaded from assets/game_config.json.

fn unix_seconds_now() -> i64 {
    (miniquad::date::now() as i64).max(0)
}

/// Simple particle for visual effects
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: (f32, f32), // grid-space
    pub velocity: (f32, f32), // grid-space per second
    pub life: f32,
    pub max_life: f32,
    pub color: Color,
    pub size: f32,
}

/// Placement animation for newly placed buildings
#[derive(Debug, Clone)]
pub struct PlacementAnim {
    pub position: GridPos,
    pub timer: f32,
}
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
            unlocked_techs: crate::data::game_data().research.starting_unlocked.clone(),
            current_research: None,
            research_progress: 0.0,
        }
    }
}

/// Achievement tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub achieved: bool,
}

/// Current game state for a planet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetState {
    pub name: String,
    pub resources: Resources,
    pub grid: Grid,
    pub drones: DroneManager,
    pub research: ResearchProgress,
    pub config: GameConfig,
    pub time_played: f64,
    pub selected_building: Option<BuildingType>,
    pub power_balance: f32,
    #[serde(skip, default)]
    pub biomass_power_bonus: f32,
    pub battery_seconds: f32,
    pub last_saved_unix: i64,
    pub achievements: Vec<Achievement>,
    pub unlocked_buildings: Vec<BuildingType>,
    #[serde(skip, default)]
    pub self_cleaning_unlocked: bool,
    #[serde(skip, default)]
    pub power_negative_seconds: f32,
    #[serde(skip, default)]
    pub power_collapse_cooldown: f32,
    #[serde(skip, default)]
    pub power_collapse_shutdown: f32,
    #[serde(skip, default)]
    pub research_lock_timer: f32,
    #[serde(skip, default)]
    pub collapse_notice_timer: f32,
    #[serde(skip, default)]
    pub forest_harvested_count: i32,
    #[serde(skip, default)]
    pub tutorial_step: u8,
    #[serde(skip, default)]
    pub tutorial_hidden: bool,
    #[serde(skip, default)]
    pub tutorial_done: bool,
    #[serde(skip, default)]
    pub last_offline_seconds: f32,
    #[serde(skip, default)]
    pub last_offline_simulated: f32,
    #[serde(skip, default)]
    pub offline_notice_timer: f32,
    #[serde(skip, default)]
    pub drag_last_pos: Option<GridPos>,
    #[serde(skip, default)]
    pub selected_tile: Option<GridPos>,
    #[serde(skip, default)]
    pub show_help: bool,
    #[serde(skip, default)]
    pub build_palette_scroll: f32,
    #[serde(skip, default)]
    pub particles: Vec<Particle>,
    #[serde(skip, default)]
    pub particle_timer: f32,
    #[serde(skip, default)]
    pub placement_anims: Vec<PlacementAnim>,
    // Drill production timers (drill position -> accumulated time)
    #[serde(skip)]
    pub drill_timers: std::collections::HashMap<(i32, i32), f32>,
    // Server bank data generation timers
    #[serde(skip)]
    pub server_timers: std::collections::HashMap<(i32, i32), f32>,
}

impl PlanetState {
    pub fn new(name: &str, width: u32, height: u32, seed: u64, config: GameConfig) -> Self {
        let mut grid = Grid::new_with_terrain(width, height, seed);
        grid.initialize_forest_biomass(config.resources.forest_biomass);

        // Place Core at center
        let center = GridPos::new(width as i32 / 2, height as i32 / 2);
        grid.place_building(center, BuildingType::Core);
        grid.update_power_grid();

        let mut unlocked_buildings = Vec::new();
        for def in &crate::data::game_data().buildings {
            if def.start_unlocked {
                if let Some(building_type) = BuildingType::from_id(&def.id) {
                    unlocked_buildings.push(building_type);
                }
            }
        }

        Self {
            name: name.to_string(),
            resources: Resources {
                energy: config.resources.starting_energy,
                minerals: config.resources.starting_minerals,
                data: 0.0,
                biomass: 0.0,
            },
            grid,
            drones: DroneManager::new(
                config.resources.drone_carry_capacity,
                config.resources.drone_speed,
            ),
            research: ResearchProgress::default(),
            config,
            time_played: 0.0,
            selected_building: Some(BuildingType::Drill),
            power_balance: 10.0,
            biomass_power_bonus: 0.0,
            battery_seconds: 4.0 * 60.0 * 60.0,
            last_saved_unix: unix_seconds_now(),
            achievements: vec![
                Achievement {
                    id: "first_drill".to_string(),
                    name: "First Drill".to_string(),
                    description: "Place your first drill.".to_string(),
                    achieved: false,
                },
                Achievement {
                    id: "power_surplus".to_string(),
                    name: "Power Surplus".to_string(),
                    description: "Reach positive net power.".to_string(),
                    achieved: false,
                },
                Achievement {
                    id: "data_miner".to_string(),
                    name: "Data Miner".to_string(),
                    description: "Accumulate 25 data.".to_string(),
                    achieved: false,
                },
                Achievement {
                    id: "builder".to_string(),
                    name: "Builder".to_string(),
                    description: "Place 10 buildings.".to_string(),
                    achieved: false,
                },
            ],
            self_cleaning_unlocked: false,
            power_negative_seconds: 0.0,
            power_collapse_cooldown: 0.0,
            power_collapse_shutdown: 0.0,
            research_lock_timer: 0.0,
            collapse_notice_timer: 0.0,
            forest_harvested_count: 0,
            tutorial_step: 0,
            tutorial_hidden: false,
            tutorial_done: false,
            unlocked_buildings,
            last_offline_seconds: 0.0,
            last_offline_simulated: 0.0,
            offline_notice_timer: 0.0,
            drag_last_pos: None,
            selected_tile: None,
            show_help: false,
            build_palette_scroll: 0.0,
            particles: Vec::new(),
            particle_timer: 0.0,
            placement_anims: Vec::new(),
            drill_timers: std::collections::HashMap::new(),
            server_timers: std::collections::HashMap::new(),
        }
    }

    /// Try to place a building at position
    pub fn try_place_building(&mut self, pos: GridPos) -> bool {
        if let Some(building_type) = self.selected_building {
            if !self.is_building_unlocked(building_type) {
                return false;
            }
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
                self.update_achievements();

                self.placement_anims.push(PlacementAnim {
                    position: pos,
                    timer: 0.3,
                });
                self.spawn_place_burst(pos);

                return true;
            }
        }
        false
    }

    pub fn try_place_conduit_path(&mut self, from: GridPos, to: GridPos) -> bool {
        if !self.is_building_unlocked(BuildingType::Conduit) {
            return false;
        }
        let Some(path) = self.grid.find_conduit_path(from, to) else {
            return false;
        };

        let mut placed_any = false;
        for pos in path {
            let Some(tile) = self.grid.get(pos) else {
                continue;
            };
            if tile
                .building
                .as_ref()
                .map(|b| b.building_type == BuildingType::Conduit)
                .unwrap_or(false)
            {
                continue;
            }

            let (mineral_cost, energy_cost) = BuildingType::Conduit.cost();
            if !self.resources.can_afford(mineral_cost, energy_cost) {
                break;
            }

            if self.grid.place_building(pos, BuildingType::Conduit) {
                self.resources.spend(mineral_cost, energy_cost);
                self.grid.reveal_around(pos, 3);
                placed_any = true;
            }
        }

        if placed_any {
            self.grid.update_power_grid();
            self.power_balance = self.grid.net_power();
        }

        placed_any
    }

    pub fn try_sell_building(&mut self, pos: GridPos) -> bool {
        let Some(tile) = self.grid.get(pos) else {
            return false;
        };
        let Some(building) = tile.building.as_ref() else {
            return false;
        };
        if building.building_type == BuildingType::Core {
            return false;
        }

        let building_type = building.building_type;
        let (mineral_cost, energy_cost) = building_type.cost();
        let refund_ratio = 0.5;

        if let Some(removed) = self.grid.remove_building(pos) {
            match removed.building_type {
                BuildingType::Drill => {
                    self.drill_timers.remove(&(pos.x, pos.y));
                    self.drones.remove_drones_at_drill(pos);
                }
                BuildingType::ServerBank => {
                    self.server_timers.remove(&(pos.x, pos.y));
                }
                _ => {}
            }

            self.resources.minerals += mineral_cost * refund_ratio;
            self.resources.energy += energy_cost * refund_ratio;

            self.grid.update_power_grid();
            self.power_balance = self.grid.net_power();
            return true;
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
                match terrain {
                    TerrainType::Mountain => {
                        tile.mountain_harvested = true;
                    }
                    TerrainType::Forest => {
                        tile.forest_cleared = true;
                        tile.biomass_amount = 0.0;
                        self.forest_harvested_count += 1;
                    }
                    _ => {}
                }
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
        self.update_simulation(delta_time, true);
    }

    pub fn update_simulation(&mut self, delta_time: f32, allow_visuals: bool) {
        let sim_delta = if self.battery_seconds <= 0.0 {
            delta_time * 0.1
        } else {
            delta_time
        };

        self.time_played += sim_delta as f64;

        self.update_dust(sim_delta);
        self.update_drone_speeds();
        self.update_biomass_harvesters(sim_delta);
        self.update_tutorial();

        if self.power_collapse_cooldown > 0.0 {
            self.power_collapse_cooldown = (self.power_collapse_cooldown - delta_time).max(0.0);
        }
        if self.power_collapse_shutdown > 0.0 {
            self.power_collapse_shutdown = (self.power_collapse_shutdown - delta_time).max(0.0);
        }
        if self.research_lock_timer > 0.0 {
            self.research_lock_timer = (self.research_lock_timer - delta_time).max(0.0);
        }
        if self.collapse_notice_timer > 0.0 {
            self.collapse_notice_timer = (self.collapse_notice_timer - delta_time).max(0.0);
        }

        // Update drones
        if self.power_collapse_shutdown <= 0.0 {
            let events = self.drones.update(sim_delta);
            let mut delivered_total = 0.0;
            for event in events {
                match event {
                    crate::engine::DroneEvent::ReachedCore { amount, .. } => {
                        delivered_total += amount;
                    }
                    crate::engine::DroneEvent::ReachedDrill { drone_id } => {
                        if let Some(drone) = self.drones.get_drone_mut(drone_id) {
                            drone.state = DroneState::Idle;
                        }
                    }
                    _ => {}
                }
            }
            if delivered_total > 0.0 {
                self.resources.minerals += delivered_total;
                if allow_visuals {
                    self.spawn_resource_burst();
                }
            }
        }

        // Process drills and server banks
        if self.power_collapse_shutdown <= 0.0 {
            self.update_drills(sim_delta);
            self.update_servers(sim_delta);
        }

        // Particles for drone motion
        if allow_visuals {
            self.spawn_drone_trails(sim_delta);
            self.update_particles(sim_delta);
        }

        // Power-based energy generation
        self.grid.update_power_grid();
        let net_power = self.grid.net_power();
        self.power_balance = net_power + self.biomass_power_bonus;
        self.resources.energy += self.power_balance * sim_delta;

        // Passive data trickle from Core to avoid research deadlock
        if let Some(core_pos) = self.grid.find_core() {
            if let Some(core_tile) = self.grid.get(core_pos) {
                if let Some(core) = core_tile.building.as_ref() {
                    if core.powered && !core.is_dust_stalled() {
                        self.resources.data += self.config.resources.core_data_rate
                            * sim_delta
                            * core.dust_efficiency();
                    }
                }
            }
        }

        if net_power < 0.0 {
            self.power_negative_seconds += delta_time;
            if self.power_negative_seconds >= 60.0 && self.power_collapse_cooldown <= 0.0 {
                self.trigger_power_collapse();
            }
        } else {
            self.power_negative_seconds = 0.0;
        }

        // Battery drain for offline mechanics
        self.battery_seconds = (self.battery_seconds - delta_time).max(0.0);

        // Cap resources
        self.resources.energy = self
            .resources
            .energy
            .clamp(0.0, self.config.resources.max_energy);
        self.resources.minerals = self.resources.minerals.min(self.mineral_capacity());
        self.resources.data = self.resources.data.min(1000.0);
        self.resources.biomass = self.resources.biomass.min(1000.0);

        self.update_achievements();

        if self.offline_notice_timer > 0.0 {
            self.offline_notice_timer = (self.offline_notice_timer - delta_time).max(0.0);
        }

        for anim in &mut self.placement_anims {
            anim.timer = (anim.timer - delta_time).max(0.0);
        }
        self.placement_anims.retain(|anim| anim.timer > 0.0);
    }

    /// Update drill production and drone dispatching
    fn update_drills(&mut self, delta_time: f32) {
        let drill_positions = self.grid.find_buildings(BuildingType::Drill);
        let core_pos = self.grid.find_core();

        if let Some(core) = core_pos {
            for drill_pos in drill_positions {
                // Check if drill is powered
                let Some(building) = self.grid.get(drill_pos).and_then(|t| t.building.as_ref())
                else {
                    continue;
                };
                let is_powered = building.powered;
                if building.is_dust_stalled() {
                    continue;
                }
                let efficiency = building.dust_efficiency();

                if !is_powered {
                    continue;
                }

                let key = (drill_pos.x, drill_pos.y);
                let timer = self.drill_timers.entry(key).or_insert(0.0);
                *timer += delta_time;

                if *timer >= 2.0 {
                    *timer = 0.0;

                    let idle_drone = self
                        .drones
                        .drones()
                        .iter()
                        .find(|d| d.home_drill == drill_pos && d.state == DroneState::Idle)
                        .map(|d| d.id);

                    if let Some(drone_id) = idle_drone {
                        let path = find_path(&self.grid, drill_pos, core);
                        if let Some(drone) = self.drones.get_drone_mut(drone_id) {
                            drone.dispatch_to_core(core, path, 10.0 * efficiency);
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
            let Some(building) = self.grid.get(server_pos).and_then(|t| t.building.as_ref()) else {
                continue;
            };
            let is_powered = building.powered;
            if building.is_dust_stalled() {
                continue;
            }
            let efficiency = building.dust_efficiency();

            if !is_powered {
                continue;
            }

            let key = (server_pos.x, server_pos.y);
            let timer = self.server_timers.entry(key).or_insert(0.0);
            *timer += delta_time;

            // Generate 1 data per second when powered
            if *timer >= 1.0 {
                *timer = 0.0;
                self.resources.data += 1.0 * efficiency;
            }
        }
    }

    fn update_dust(&mut self, delta_time: f32) {
        let sweeper_positions = self.grid.find_buildings(BuildingType::Sweeper);
        let powered_sweepers: Vec<GridPos> = sweeper_positions
            .into_iter()
            .filter(|pos| {
                self.grid
                    .get(*pos)
                    .and_then(|t| t.building.as_ref())
                    .map(|b| b.powered && !b.is_dust_stalled())
                    .unwrap_or(false)
            })
            .collect();
        let filter_positions: Vec<GridPos> = self
            .grid
            .iter_tiles()
            .filter_map(|(pos, tile)| if tile.filter { Some(pos) } else { None })
            .collect();
        let cleared_forest_positions: Vec<GridPos> = self
            .grid
            .iter_tiles()
            .filter_map(|(pos, tile)| if tile.forest_cleared { Some(pos) } else { None })
            .collect();

        for (pos, tile) in self.grid.iter_tiles_mut() {
            let Some(building) = tile.building.as_mut() else {
                continue;
            };
            let mut rate = DUST_RATE;

            if self.self_cleaning_unlocked {
                rate *= 0.6;
            }

            if filter_positions
                .iter()
                .any(|filter_pos| pos.distance(*filter_pos) as i32 <= FILTER_RADIUS)
            {
                rate *= FILTER_RATE_MULTIPLIER;
            }
            if cleared_forest_positions
                .iter()
                .any(|cleared_pos| pos.distance(*cleared_pos) as i32 <= POLLUTION_RADIUS)
            {
                rate *= POLLUTION_RATE_MULTIPLIER;
            }

            // Apply sweeper cleaning if nearby powered sweeper exists
            let mut clean_rate = 0.0;
            if powered_sweepers
                .iter()
                .any(|sweeper_pos| pos.distance(*sweeper_pos) as i32 <= SWEEPER_RADIUS)
            {
                clean_rate = SWEEPER_RATE;
            }

            building.dust =
                (building.dust + rate * delta_time - clean_rate * delta_time).clamp(0.0, 100.0);
        }
    }

    fn update_drone_speeds(&mut self) {
        let base_speed = self.drones.drone_speed;
        for drone in self.drones.drones_mut() {
            let mut speed = base_speed;
            if let Some(tile) = self.grid.get(drone.home_drill) {
                if let Some(ref building) = tile.building {
                    speed *= building.dust_drone_speed_multiplier();
                }
            }
            drone.speed = speed;
        }
    }

    fn update_biomass_harvesters(&mut self, delta_time: f32) {
        let output = self.config.resources.biomass_power_output;
        let rate = self.config.resources.biomass_consumption_rate;
        let mut power_bonus = 0.0;

        for (_, tile) in self.grid.iter_tiles_mut() {
            let Some(building) = tile.building.as_mut() else {
                continue;
            };
            if building.building_type != BuildingType::BiomassHarvester {
                continue;
            }

            if tile.terrain != TerrainType::Forest || tile.biomass_amount <= 0.0 {
                continue;
            }
            if !building.powered || building.is_dust_stalled() {
                continue;
            }

            let available = tile.biomass_amount;
            if available <= 0.0 || rate <= 0.0 {
                continue;
            }

            let consume = (rate * delta_time).min(available);
            tile.biomass_amount = (tile.biomass_amount - consume).max(0.0);
            self.resources.biomass += consume;

            let fraction = if rate * delta_time > 0.0 {
                consume / (rate * delta_time)
            } else {
                0.0
            };
            power_bonus += output * fraction * building.dust_power_generation_multiplier();

            if tile.biomass_amount <= 0.0 {
                tile.terrain = TerrainType::Empty;
                tile.forest_cleared = true;
                tile.filter = false;
            }
        }

        self.biomass_power_bonus = power_bonus;
    }

    fn update_tutorial(&mut self) {
        if self.tutorial_done {
            return;
        }

        let has_drill = !self.grid.find_buildings(BuildingType::Drill).is_empty();
        let drill_connected = self.grid.iter_tiles().any(|(_, tile)| {
            tile.building
                .as_ref()
                .map(|b| b.building_type == BuildingType::Drill && b.connected_to_core)
                .unwrap_or(false)
        });
        let conduits_unlocked = self.is_building_unlocked(BuildingType::Conduit);
        let server_unlocked = self.is_building_unlocked(BuildingType::ServerBank);
        let wind_unlocked = self.is_building_unlocked(BuildingType::WindTurbine);
        let has_wind_turbine = !self
            .grid
            .find_buildings(BuildingType::WindTurbine)
            .is_empty();
        let has_server_bank = !self
            .grid
            .find_buildings(BuildingType::ServerBank)
            .is_empty();

        match self.tutorial_step {
            0 if has_drill => self.tutorial_step = 1,
            1 if conduits_unlocked => self.tutorial_step = 2,
            2 if drill_connected => self.tutorial_step = 3,
            3 if server_unlocked && has_server_bank => self.tutorial_step = 4,
            4 if wind_unlocked && has_wind_turbine => {
                self.tutorial_step = 5;
                self.tutorial_done = true;
            }
            _ => {}
        }
    }

    fn trigger_power_collapse(&mut self) {
        self.power_negative_seconds = 0.0;
        self.power_collapse_cooldown = 120.0;
        self.power_collapse_shutdown = 20.0;
        self.research_lock_timer = 30.0;
        self.collapse_notice_timer = 10.0;

        // Drones drop cargo and shut down
        for drone in self.drones.drones_mut() {
            drone.carrying = 0.0;
            drone.state = DroneState::Error;
            drone.path.clear();
            drone.path_index = 0;
            drone.progress = 0.0;
            drone.target = drone.position;
        }

        // Corrupt data and research progress
        self.resources.data *= 0.7;
        self.research.research_progress *= 0.75;
    }

    pub fn try_convert_forest_to_filter(&mut self, pos: GridPos) -> bool {
        if let Some(tile) = self.grid.get(pos) {
            if !tile.revealed || tile.terrain != TerrainType::Forest || tile.building.is_some() {
                return false;
            }
        } else {
            return false;
        }

        if let Some(tile) = self.grid.get_mut(pos) {
            tile.terrain = TerrainType::Rough;
            tile.filter = true;
            tile.forest_cleared = true;
            tile.biomass_amount = 0.0;
            self.forest_harvested_count += 1;
            return true;
        }

        false
    }
}

impl Default for PlanetState {
    fn default() -> Self {
        Self::new("Mars", 24, 24, 42, GameConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::PlanetState;

    fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn offline_simulation_uses_hibernation_rate() {
        let mut state = PlanetState {
            battery_seconds: 4.0 * 60.0 * 60.0,
            ..Default::default()
        };

        let offline = 6.0 * 60.0 * 60.0;
        state.apply_offline_progress(offline);

        let expected_sim = (4.0 * 60.0 * 60.0) + (2.0 * 60.0 * 60.0) * 0.1;
        assert!(approx_eq(state.last_offline_simulated, expected_sim, 0.5));
        assert!(approx_eq(state.last_offline_seconds, offline, 0.5));
        assert!(state.battery_seconds <= 0.0);
        assert!(state.offline_notice_timer > 0.0);
    }
}
