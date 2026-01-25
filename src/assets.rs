//! Texture loading and access helpers.

use macroquad::prelude::*;

pub struct TerrainTextures {
    pub ground: Texture2D,
    pub mountain: Texture2D,
    pub forest: Texture2D,
    pub water: Texture2D,
    pub rough: Texture2D,
    pub void: Texture2D,
}

pub struct BuildingTextures {
    pub core_stage_1a: Texture2D,
    pub core_stage_1b: Texture2D,
    pub core_stage_1c: Texture2D,
    pub core_stage_2a: Texture2D,
    pub core_stage_2b: Texture2D,
    pub drill: Texture2D,
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
    pub bridge: Texture2D,
    pub power_node: Texture2D,
    pub wind_turbine: Texture2D,
    pub server_bank: Texture2D,
    pub sweeper: Texture2D,
}

pub struct GameTextures {
    pub terrain: TerrainTextures,
    pub buildings: BuildingTextures,
}

impl GameTextures {
    pub async fn load() -> Self {
        let terrain = TerrainTextures {
            ground: load_texture("assets/tiles/terrain_ground.png").await.expect("terrain_ground"),
            mountain: load_texture("assets/tiles/terrain_mountain.png").await.expect("terrain_mountain"),
            forest: load_texture("assets/tiles/terrain_forest.png").await.expect("terrain_forest"),
            water: load_texture("assets/tiles/terrain_water.png").await.expect("terrain_water"),
            rough: load_texture("assets/tiles/terrain_rough.png").await.expect("terrain_rough"),
            void: load_texture("assets/tiles/terrain_void.png").await.expect("terrain_void"),
        };

        let buildings = BuildingTextures {
            core_stage_1a: load_texture("assets/tiles/buildings/building_core_stage_1a.png").await.expect("building_core_stage_1a"),
            core_stage_1b: load_texture("assets/tiles/buildings/building_core_stage_1b.png").await.expect("building_core_stage_1b"),
            core_stage_1c: load_texture("assets/tiles/buildings/building_core_stage_1c.png").await.expect("building_core_stage_1c"),
            core_stage_2a: load_texture("assets/tiles/buildings/building_core_stage_2a.png").await.expect("building_core_stage_2a"),
            core_stage_2b: load_texture("assets/tiles/buildings/building_core_stage_2b.png").await.expect("building_core_stage_2b"),
            drill: load_texture("assets/tiles/buildings/building_drill.png").await.expect("building_drill"),
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
            bridge: load_texture("assets/tiles/buildings/building_bridge.png").await.expect("building_bridge"),
            power_node: load_texture("assets/tiles/buildings/building_power_node.png").await.expect("building_power_node"),
            wind_turbine: load_texture("assets/tiles/buildings/building_wind_turbine.png").await.expect("building_wind_turbine"),
            server_bank: load_texture("assets/tiles/buildings/building_server_bank.png").await.expect("building_server_bank"),
            sweeper: load_texture("assets/tiles/buildings/building_sweeper.png").await.expect("building_sweeper"),
        };

        set_filter_nearest(&terrain);
        set_filter_nearest_buildings(&buildings);

        Self { terrain, buildings }
    }
}

fn set_filter_nearest(terrain: &TerrainTextures) {
    terrain.ground.set_filter(FilterMode::Nearest);
    terrain.mountain.set_filter(FilterMode::Nearest);
    terrain.forest.set_filter(FilterMode::Nearest);
    terrain.water.set_filter(FilterMode::Nearest);
    terrain.rough.set_filter(FilterMode::Nearest);
    terrain.void.set_filter(FilterMode::Nearest);
}

fn set_filter_nearest_buildings(buildings: &BuildingTextures) {
    buildings.core_stage_1a.set_filter(FilterMode::Nearest);
    buildings.core_stage_1b.set_filter(FilterMode::Nearest);
    buildings.core_stage_1c.set_filter(FilterMode::Nearest);
    buildings.core_stage_2a.set_filter(FilterMode::Nearest);
    buildings.core_stage_2b.set_filter(FilterMode::Nearest);
    buildings.drill.set_filter(FilterMode::Nearest);
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
    buildings.bridge.set_filter(FilterMode::Nearest);
    buildings.power_node.set_filter(FilterMode::Nearest);
    buildings.wind_turbine.set_filter(FilterMode::Nearest);
    buildings.server_bank.set_filter(FilterMode::Nearest);
    buildings.sweeper.set_filter(FilterMode::Nearest);
}
