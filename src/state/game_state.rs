//! Current planetary state

use serde::{Deserialize, Serialize};
use macroquad::prelude::Color;
use macroquad::rand::gen_range;
use crate::engine::{Grid, GridPos, BuildingType, DroneManager, find_path, DroneState, TerrainType};
use std::time::{SystemTime, UNIX_EPOCH};

fn unix_seconds_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
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
    pub time_played: f64,
    pub selected_building: Option<BuildingType>,
    pub power_balance: f32,
    pub battery_seconds: f32,
    pub last_saved_unix: i64,
    pub achievements: Vec<Achievement>,
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
    pub particles: Vec<Particle>,
    #[serde(skip, default)]
    pub particle_timer: f32,
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
            last_offline_seconds: 0.0,
            last_offline_simulated: 0.0,
            offline_notice_timer: 0.0,
            drag_last_pos: None,
            selected_tile: None,
            show_help: false,
            particles: Vec::new(),
            particle_timer: 0.0,
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
                self.update_achievements();

                return true;
            }
        }
        false
    }

    pub fn try_place_conduit_path(&mut self, from: GridPos, to: GridPos) -> bool {
        let Some(path) = self.grid.find_conduit_path(from, to) else {
            return false;
        };

        let mut placed_any = false;
        for pos in path {
            let Some(tile) = self.grid.get(pos) else { continue; };
            if tile.building.as_ref().map(|b| b.building_type == BuildingType::Conduit).unwrap_or(false) {
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
        let Some(tile) = self.grid.get(pos) else { return false; };
        let Some(building) = tile.building.as_ref() else { return false; };
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

        // Update drones
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

        // Process drills and server banks
        self.update_drills(sim_delta);
        self.update_servers(sim_delta);

        // Particles for drone motion
        if allow_visuals {
            self.spawn_drone_trails(sim_delta);
            self.update_particles(sim_delta);
        }

        // Power-based energy generation
        let net_power = self.grid.net_power();
        self.power_balance = net_power;
        self.resources.energy += net_power * sim_delta;

        // Battery drain for offline mechanics
        self.battery_seconds = (self.battery_seconds - delta_time).max(0.0);

        // Cap resources
        self.resources.energy = self.resources.energy.clamp(0.0, 1000.0);
        self.resources.minerals = self.resources.minerals.min(1000.0);
        self.resources.data = self.resources.data.min(1000.0);
        self.resources.biomass = self.resources.biomass.min(1000.0);

        self.update_achievements();

        if self.offline_notice_timer > 0.0 {
            self.offline_notice_timer = (self.offline_notice_timer - delta_time).max(0.0);
        }
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
                        let path = find_path(&self.grid, drill_pos, core);
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

    pub fn battery_time_left(&self) -> (i32, i32) {
        let total = self.battery_seconds.max(0.0) as i32;
        let hours = total / 3600;
        let minutes = (total % 3600) / 60;
        (hours, minutes)
    }

    pub fn apply_offline_progress(&mut self, offline_seconds: f32) {
        if offline_seconds <= 0.0 {
            self.last_offline_seconds = 0.0;
            self.last_offline_simulated = 0.0;
            return;
        }

        let mut remaining = offline_seconds;
        while remaining > 0.0 {
            let step = remaining.min(60.0);
            self.update_simulation(step, false);
            remaining -= step;
        }

        self.last_offline_seconds = offline_seconds;
        let full_speed = offline_seconds.min(4.0 * 60.0 * 60.0);
        let hibernation = (offline_seconds - full_speed).max(0.0) * 0.1;
        self.last_offline_simulated = full_speed + hibernation;
        self.offline_notice_timer = 8.0;
    }

    pub fn achievements_progress(&self) -> (usize, usize) {
        let total = self.achievements.len();
        let unlocked = self.achievements.iter().filter(|a| a.achieved).count();
        (unlocked, total)
    }

    fn update_achievements(&mut self) {
        let has_drill = !self.grid.find_buildings(BuildingType::Drill).is_empty();
        if has_drill {
            self.unlock_achievement("first_drill");
        }

        if self.power_balance > 0.0 {
            self.unlock_achievement("power_surplus");
        }

        if self.resources.data >= 25.0 {
            self.unlock_achievement("data_miner");
        }

        if self.grid.total_buildings() >= 10 {
            self.unlock_achievement("builder");
        }
    }

    fn unlock_achievement(&mut self, id: &str) {
        if let Some(ach) = self.achievements.iter_mut().find(|a| a.id == id) {
            ach.achieved = true;
        }
    }

    fn spawn_particle(&mut self, position: (f32, f32), velocity: (f32, f32), life: f32, color: Color, size: f32) {
        self.particles.push(Particle {
            position,
            velocity,
            life,
            max_life: life,
            color,
            size,
        });
    }

    fn spawn_resource_burst(&mut self) {
        let core_pos = match self.grid.find_core() {
            Some(pos) => pos,
            None => return,
        };
        let origin = (core_pos.x as f32, core_pos.y as f32);
        let count = 8;
        for i in 0..count {
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            let speed = gen_range(0.6, 1.2);
            let velocity = (angle.cos() * speed, angle.sin() * speed);
            let life = gen_range(0.35, 0.6);
            self.spawn_particle(origin, velocity, life, Color::new(1.0, 0.42, 0.21, 1.0), 3.0);
        }
    }

    fn spawn_drone_trails(&mut self, delta_time: f32) {
        self.particle_timer += delta_time;
        if self.particle_timer < 0.08 {
            return;
        }
        self.particle_timer = 0.0;

        let drone_positions: Vec<(f32, f32)> = self.drones.drones()
            .iter()
            .filter(|drone| drone.state == DroneState::MovingToCore || drone.state == DroneState::MovingToDrill)
            .map(|drone| drone.visual_position())
            .collect();

        for (x, y) in drone_positions {
            let jitter = (gen_range(-0.2, 0.2), gen_range(-0.2, 0.2));
            let velocity = (gen_range(-0.4, 0.4), gen_range(-0.4, 0.4));
            let life = gen_range(0.25, 0.5);
            let color = Color::new(0.0, 0.85, 1.0, 1.0);
            self.spawn_particle((x + jitter.0, y + jitter.1), velocity, life, color, 2.0);
        }
    }

    fn update_particles(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.position.0 += particle.velocity.0 * delta_time;
            particle.position.1 += particle.velocity.1 * delta_time;
            particle.life -= delta_time;
        }
        self.particles.retain(|p| p.life > 0.0);
    }
}

impl Default for PlanetState {
    fn default() -> Self {
        Self::new("Mars", 24, 24, 42)
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
        let mut state = PlanetState::default();
        state.battery_seconds = 4.0 * 60.0 * 60.0;

        let offline = 6.0 * 60.0 * 60.0;
        state.apply_offline_progress(offline);

        let expected_sim = (4.0 * 60.0 * 60.0) + (2.0 * 60.0 * 60.0) * 0.1;
        assert!(approx_eq(state.last_offline_simulated, expected_sim, 0.5));
        assert!(approx_eq(state.last_offline_seconds, offline, 0.5));
        assert!(state.battery_seconds <= 0.0);
        assert!(state.offline_notice_timer > 0.0);
    }
}
