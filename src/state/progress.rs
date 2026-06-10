use crate::engine::{BuildingType, GridPos};

use super::game_state::PlanetState;

impl PlanetState {
    /// Select a building type for placement.
    pub fn select_building(&mut self, building_type: BuildingType) {
        if self.is_building_unlocked(building_type) {
            self.selected_building = Some(building_type);
        }
    }

    /// Clear building selection.
    pub fn clear_selection(&mut self) {
        self.selected_building = None;
    }

    pub fn is_building_unlocked(&self, building_type: BuildingType) -> bool {
        matches!(building_type, BuildingType::Core)
            || self.unlocked_buildings.contains(&building_type)
    }

    pub fn unlock_building(&mut self, building_type: BuildingType) {
        if !self.unlocked_buildings.contains(&building_type) {
            self.unlocked_buildings.push(building_type);
        }
    }

    pub fn mineral_capacity(&self) -> f32 {
        let storage_count = self.grid.find_buildings(BuildingType::Storage).len() as f32;
        let mut cap = self.config.resources.base_mineral_cap
            + storage_count * self.config.resources.storage_bonus;
        if self
            .research
            .unlocked_techs
            .contains(&"storage_optimization".to_string())
        {
            cap += self.config.resources.storage_tech_bonus;
        }
        cap
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

    pub fn placement_scale(&self, pos: GridPos) -> f32 {
        let Some(anim) = self
            .placement_anims
            .iter()
            .find(|anim| anim.position == pos)
        else {
            return 1.0;
        };
        let progress = (anim.timer / 0.3).clamp(0.0, 1.0);
        let bounce_phase = (1.0 - progress) * std::f32::consts::PI * 2.0;
        1.0 + (bounce_phase.sin() * 0.12)
    }

    pub(super) fn update_achievements(&mut self) {
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
}
