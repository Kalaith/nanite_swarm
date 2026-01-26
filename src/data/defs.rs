//! Game data definitions loaded from JSON.

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use crate::engine::{ResearchNode, ResearchTree};
use crate::data::load_json;

#[derive(Debug, Clone, Deserialize)]
pub struct Cost {
    pub minerals: f32,
    pub energy: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BuildingDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: Cost,
    pub power_generation: f32,
    pub power_consumption: f32,
    pub hotkey: Option<String>,
    pub texture: String,
    pub build_menu_order: i32,
    pub show_in_build_menu: bool,
    pub start_unlocked: bool,
    pub unlocked_by: Option<String>,
    pub transmits_power: bool,
    pub generates_power: bool,
    pub consumes_power: bool,
    pub uses_efficiency: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HarvestRewards {
    pub minerals: f32,
    pub biomass: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TerrainDef {
    pub id: String,
    pub name: String,
    pub buildable: bool,
    pub harvestable: bool,
    pub harvest_rewards: HarvestRewards,
    pub harvested_to: String,
    pub preservation_bonus: Option<String>,
    pub texture: String,
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResearchNodeDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub data_cost: f32,
    pub prerequisites: Vec<String>,
    pub position: (f32, f32),
}

impl ResearchNodeDef {
    pub fn to_node(&self) -> ResearchNode {
        ResearchNode {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            data_cost: self.data_cost,
            prerequisites: self.prerequisites.clone(),
            position: self.position,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResearchData {
    pub starting_unlocked: Vec<String>,
    pub nodes: Vec<ResearchNodeDef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BuildingDataFile {
    pub buildings: Vec<BuildingDef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TerrainDataFile {
    pub terrain: Vec<TerrainDef>,
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub buildings: Vec<BuildingDef>,
    pub buildings_by_id: HashMap<String, BuildingDef>,
    pub terrain: Vec<TerrainDef>,
    pub terrain_by_id: HashMap<String, TerrainDef>,
    pub research: ResearchData,
}

impl GameData {
    pub fn load() -> Self {
        let buildings_json = fs::read_to_string("assets/buildings.json").unwrap_or_default();
        let terrain_json = fs::read_to_string("assets/terrain.json").unwrap_or_default();
        let research_json = fs::read_to_string("assets/research.json").unwrap_or_default();

        let building_file: BuildingDataFile = load_json(&buildings_json).unwrap_or_else(|_| BuildingDataFile { buildings: vec![] });
        let terrain_file: TerrainDataFile = load_json(&terrain_json).unwrap_or_else(|_| TerrainDataFile { terrain: vec![] });
        let research: ResearchData = load_json(&research_json).unwrap_or_else(|_| ResearchData {
            starting_unlocked: vec!["core".to_string(), "basic_mining".to_string()],
            nodes: vec![],
        });

        let mut buildings_by_id = HashMap::new();
        for def in &building_file.buildings {
            buildings_by_id.insert(def.id.clone(), def.clone());
        }

        let mut terrain_by_id = HashMap::new();
        for def in &terrain_file.terrain {
            terrain_by_id.insert(def.id.clone(), def.clone());
        }

        Self {
            buildings: building_file.buildings,
            buildings_by_id,
            terrain: terrain_file.terrain,
            terrain_by_id,
            research,
        }
    }

    pub fn building(&self, id: &str) -> &BuildingDef {
        self.buildings_by_id.get(id).unwrap_or_else(|| panic!("Missing building def for id: {}", id))
    }

    pub fn terrain(&self, id: &str) -> &TerrainDef {
        self.terrain_by_id.get(id).unwrap_or_else(|| panic!("Missing terrain def for id: {}", id))
    }

    pub fn research_tree(&self) -> ResearchTree {
        ResearchTree::from_nodes(self.research.nodes.iter().map(|node| node.to_node()).collect())
    }
}
