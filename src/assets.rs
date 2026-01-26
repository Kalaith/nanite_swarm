//! Texture loading and access helpers.

use macroquad::prelude::*;
use std::collections::HashMap;
use crate::data;

pub struct TerrainTextures {
    pub by_id: HashMap<String, Texture2D>,
}

pub struct BuildingTextures {
    pub by_id: HashMap<String, Texture2D>,
    pub core_stage_1a: Texture2D,
    pub core_stage_1b: Texture2D,
    pub core_stage_1c: Texture2D,
    pub core_stage_2a: Texture2D,
    pub core_stage_2b: Texture2D,
    pub conduit_straight_h: Texture2D,
    pub conduit_straight_v: Texture2D,
    pub conduit_corner_ne: Texture2D,
    pub conduit_corner_nw: Texture2D,
    pub conduit_corner_se: Texture2D,
    pub conduit_corner_sw: Texture2D,
    pub conduit_tee_n: Texture2D,
    pub conduit_tee_e: Texture2D,
    pub conduit_tee_s: Texture2D,
    pub conduit_tee_w: Texture2D,
    pub conduit_cross: Texture2D,
}

pub struct BuildingIconTextures {
    pub by_id: HashMap<String, Texture2D>,
}

pub struct GameTextures {
    pub terrain: TerrainTextures,
    pub buildings: BuildingTextures,
    pub building_icons: BuildingIconTextures,
}

impl GameTextures {
    pub async fn load() -> Self {
        let mut terrain_textures = HashMap::new();
        for def in &data::game_data().terrain {
            let texture = load_texture(&def.texture).await.expect("terrain_texture");
            terrain_textures.insert(def.id.clone(), texture);
        }
        let terrain = TerrainTextures { by_id: terrain_textures };

        let mut building_textures = HashMap::new();
        for def in &data::game_data().buildings {
            let texture = load_texture(&def.texture).await.expect("building_texture");
            building_textures.insert(def.id.clone(), texture);
        }

        let mut building_icon_textures = HashMap::new();
        for def in &data::game_data().buildings {
            let texture = load_texture(&def.icon).await.expect("building_icon");
            building_icon_textures.insert(def.id.clone(), texture);
        }

        let buildings = BuildingTextures {
            by_id: building_textures,
            core_stage_1a: load_texture("assets/tiles/buildings/building_core_stage_1a.png").await.expect("building_core_stage_1a"),
            core_stage_1b: load_texture("assets/tiles/buildings/building_core_stage_1b.png").await.expect("building_core_stage_1b"),
            core_stage_1c: load_texture("assets/tiles/buildings/building_core_stage_1c.png").await.expect("building_core_stage_1c"),
            core_stage_2a: load_texture("assets/tiles/buildings/building_core_stage_2a.png").await.expect("building_core_stage_2a"),
            core_stage_2b: load_texture("assets/tiles/buildings/building_core_stage_2b.png").await.expect("building_core_stage_2b"),
            conduit_straight_h: load_texture("assets/tiles/buildings/building_conduit_straight_h.png").await.expect("conduit_straight_h"),
            conduit_straight_v: load_texture("assets/tiles/buildings/building_conduit_straight_v.png").await.expect("conduit_straight_v"),
            conduit_corner_ne: load_texture("assets/tiles/buildings/building_conduit_corner_ne.png").await.expect("conduit_corner_ne"),
            conduit_corner_nw: load_texture("assets/tiles/buildings/building_conduit_corner_nw.png").await.expect("conduit_corner_nw"),
            conduit_corner_se: load_texture("assets/tiles/buildings/building_conduit_corner_se.png").await.expect("conduit_corner_se"),
            conduit_corner_sw: load_texture("assets/tiles/buildings/building_conduit_corner_sw.png").await.expect("conduit_corner_sw"),
            conduit_tee_n: load_texture("assets/tiles/buildings/building_conduit_tee_n.png").await.expect("conduit_tee_n"),
            conduit_tee_e: load_texture("assets/tiles/buildings/building_conduit_tee_e.png").await.expect("conduit_tee_e"),
            conduit_tee_s: load_texture("assets/tiles/buildings/building_conduit_tee_s.png").await.expect("conduit_tee_s"),
            conduit_tee_w: load_texture("assets/tiles/buildings/building_conduit_tee_w.png").await.expect("conduit_tee_w"),
            conduit_cross: load_texture("assets/tiles/buildings/building_conduit_cross.png").await.expect("conduit_cross"),
        };

        let building_icons = BuildingIconTextures { by_id: building_icon_textures };

        set_filter_nearest(&terrain);
        set_filter_nearest_buildings(&buildings);
        set_filter_nearest_icons(&building_icons);

        Self { terrain, buildings, building_icons }
    }
}

fn set_filter_nearest(terrain: &TerrainTextures) {
    for texture in terrain.by_id.values() {
        texture.set_filter(FilterMode::Nearest);
    }
}

fn set_filter_nearest_buildings(buildings: &BuildingTextures) {
    for texture in buildings.by_id.values() {
        texture.set_filter(FilterMode::Nearest);
    }
    buildings.core_stage_1a.set_filter(FilterMode::Nearest);
    buildings.core_stage_1b.set_filter(FilterMode::Nearest);
    buildings.core_stage_1c.set_filter(FilterMode::Nearest);
    buildings.core_stage_2a.set_filter(FilterMode::Nearest);
    buildings.core_stage_2b.set_filter(FilterMode::Nearest);
    buildings.conduit_straight_h.set_filter(FilterMode::Nearest);
    buildings.conduit_straight_v.set_filter(FilterMode::Nearest);
    buildings.conduit_corner_ne.set_filter(FilterMode::Nearest);
    buildings.conduit_corner_nw.set_filter(FilterMode::Nearest);
    buildings.conduit_corner_se.set_filter(FilterMode::Nearest);
    buildings.conduit_corner_sw.set_filter(FilterMode::Nearest);
    buildings.conduit_tee_n.set_filter(FilterMode::Nearest);
    buildings.conduit_tee_e.set_filter(FilterMode::Nearest);
    buildings.conduit_tee_s.set_filter(FilterMode::Nearest);
    buildings.conduit_tee_w.set_filter(FilterMode::Nearest);
    buildings.conduit_cross.set_filter(FilterMode::Nearest);
}

fn set_filter_nearest_icons(icons: &BuildingIconTextures) {
    for texture in icons.by_id.values() {
        texture.set_filter(FilterMode::Nearest);
    }
}
