//! Terrain types affecting buildability and harvesting

use crate::data;
use serde::{Deserialize, Serialize};

/// Terrain types that affect gameplay
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TerrainType {
    #[default]
    Empty, // Buildable ground
    Mountain, // Can harvest for iron or place wind turbine
    Forest,   // Can harvest for biomass or keep as pollution buffer
    Water,    // Cannot build, may provide cooling
    Rough,    // Difficult to build on (result of harvesting)
    Void,     // Unbuildable gap (volcanic terrain)
}

impl TerrainType {
    pub fn id(&self) -> &'static str {
        match self {
            TerrainType::Empty => "empty",
            TerrainType::Mountain => "mountain",
            TerrainType::Forest => "forest",
            TerrainType::Water => "water",
            TerrainType::Rough => "rough",
            TerrainType::Void => "void",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "empty" => Some(TerrainType::Empty),
            "mountain" => Some(TerrainType::Mountain),
            "forest" => Some(TerrainType::Forest),
            "water" => Some(TerrainType::Water),
            "rough" => Some(TerrainType::Rough),
            "void" => Some(TerrainType::Void),
            _ => None,
        }
    }

    /// Whether buildings can be placed on this terrain
    pub fn is_buildable(&self) -> bool {
        data::game_data().terrain(self.id()).buildable
    }

    /// Whether this terrain can be harvested
    pub fn is_harvestable(&self) -> bool {
        data::game_data().terrain(self.id()).harvestable
    }

    /// Get harvest rewards (minerals, biomass)
    pub fn harvest_rewards(&self) -> (f32, f32) {
        let def = data::game_data().terrain(self.id());
        (def.harvest_rewards.minerals, def.harvest_rewards.biomass)
    }

    /// Get terrain after harvesting
    pub fn harvested(&self) -> TerrainType {
        let def = data::game_data().terrain(self.id());
        TerrainType::from_id(&def.harvested_to).unwrap_or(*self)
    }

    /// Get preservation bonus description
    pub fn preservation_bonus(&self) -> Option<&'static str> {
        data::game_data()
            .terrain(self.id())
            .preservation_bonus
            .as_deref()
    }

    /// Display name
    pub fn name(&self) -> &'static str {
        data::game_data().terrain(self.id()).name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::TerrainType;

    #[test]
    fn id_roundtrips_through_from_id() {
        let all = [
            TerrainType::Empty,
            TerrainType::Mountain,
            TerrainType::Forest,
            TerrainType::Water,
            TerrainType::Rough,
            TerrainType::Void,
        ];
        for terrain in all {
            assert_eq!(TerrainType::from_id(terrain.id()), Some(terrain));
        }
    }

    #[test]
    fn from_id_rejects_unknown_strings() {
        assert_eq!(TerrainType::from_id("lava"), None);
    }

    #[test]
    fn mountain_harvests_into_rough_with_mineral_reward() {
        let mountain = TerrainType::Mountain;
        assert!(mountain.is_harvestable());
        assert!(!mountain.is_buildable());
        assert_eq!(mountain.harvested(), TerrainType::Rough);
        let (minerals, biomass) = mountain.harvest_rewards();
        assert!(minerals > 0.0);
        assert_eq!(biomass, 0.0);
    }

    #[test]
    fn forest_harvests_into_empty_with_biomass_reward() {
        let forest = TerrainType::Forest;
        assert_eq!(forest.harvested(), TerrainType::Empty);
        let (minerals, biomass) = forest.harvest_rewards();
        assert_eq!(minerals, 0.0);
        assert!(biomass > 0.0);
    }

    #[test]
    fn empty_and_rough_are_buildable_but_not_harvestable() {
        assert!(TerrainType::Empty.is_buildable());
        assert!(!TerrainType::Empty.is_harvestable());
        assert!(TerrainType::Rough.is_buildable());
        assert!(!TerrainType::Rough.is_harvestable());
    }

    #[test]
    fn water_and_void_are_unbuildable() {
        assert!(!TerrainType::Water.is_buildable());
        assert!(!TerrainType::Void.is_buildable());
    }
}
