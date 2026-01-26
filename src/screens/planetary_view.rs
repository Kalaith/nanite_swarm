//! Main grid gameplay screen

use macroquad::prelude::*;
use crate::state::PlanetState;
use crate::engine::{GridPos, TerrainType, BuildingType, DroneState};
use crate::ui::{Colors, draw_panel, draw_button_sized};
use crate::assets::GameTextures;
use crate::directives::Directive;
use crate::data;

const TILE_SIZE: f32 = 28.0;
const HUD_HEIGHT: f32 = 72.0;
const SIDEBAR_WIDTH: f32 = 260.0;
const RIGHTBAR_WIDTH: f32 = 320.0;
const BOTTOM_BAR_HEIGHT: f32 = 70.0;
const GRID_OFFSET_X: f32 = SIDEBAR_WIDTH + 20.0;
const GRID_OFFSET_Y: f32 = HUD_HEIGHT + 12.0;

/// Actions from the planetary view
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlanetaryAction {
    None,
    OpenResearch,
    OpenInterplanetary,
    OpenMenu,
}

/// Convert screen position to grid position
fn screen_to_grid(screen_x: f32, screen_y: f32) -> Option<GridPos> {
    let grid_x = ((screen_x - GRID_OFFSET_X) / TILE_SIZE).floor() as i32;
    let grid_y = ((screen_y - GRID_OFFSET_Y) / TILE_SIZE).floor() as i32;

    if grid_x >= 0 && grid_y >= 0 {
        Some(GridPos::new(grid_x, grid_y))
    } else {
        None
    }
}

/// Convert grid position to screen position
fn grid_to_screen(pos: GridPos) -> (f32, f32) {
    (
        GRID_OFFSET_X + pos.x as f32 * TILE_SIZE,
        GRID_OFFSET_Y + pos.y as f32 * TILE_SIZE,
    )
}

fn is_cursor_over_ui(screen_x: f32, screen_y: f32, screen_w: f32, screen_h: f32) -> bool {
    if screen_y <= HUD_HEIGHT || screen_y >= screen_h - BOTTOM_BAR_HEIGHT {
        return true;
    }
    if screen_x <= SIDEBAR_WIDTH {
        return true;
    }
    if screen_x >= screen_w - RIGHTBAR_WIDTH {
        return true;
    }
    false
}

fn hash01(seed: u32) -> f32 {
    let noise = (seed as f32 * 12.9898).sin() * 43758.5453;
    noise.fract().abs()
}

fn format_hours_minutes(seconds: f32) -> (i32, i32) {
    let total = seconds.max(0.0) as i32;
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    (hours, minutes)
}

fn keycode_from_hotkey(hotkey: char) -> Option<KeyCode> {
    match hotkey {
        '1' => Some(KeyCode::Key1),
        '2' => Some(KeyCode::Key2),
        '3' => Some(KeyCode::Key3),
        '4' => Some(KeyCode::Key4),
        '5' => Some(KeyCode::Key5),
        '6' => Some(KeyCode::Key6),
        '7' => Some(KeyCode::Key7),
        '8' => Some(KeyCode::Key8),
        '9' => Some(KeyCode::Key9),
        _ => None,
    }
}

fn draw_progress_bar(x: f32, y: f32, width: f32, height: f32, progress: f32, color: Color) {
    let clamped = progress.clamp(0.0, 1.0);
    draw_rectangle(x, y, width, height, Colors::SURFACE_DARK);
    draw_rectangle(x, y, width * clamped, height, color);
    draw_rectangle_lines(x, y, width, height, 1.0, Colors::PANEL_BORDER);
}

fn draw_resource_chip(x: f32, y: f32, width: f32, label: &str, value: f32, color: Color) {
    let height = 36.0;
    draw_panel(x, y, width, height);
    draw_text(label, x + 10.0, y + 14.0, 12.0, Colors::TEXT_DIM);
    draw_text(&format!("{:.0}", value), x + 10.0, y + 30.0, 16.0, color);
}

fn format_power_delta(delta: f32) -> String {
    if delta > 0.0 {
        format!("+{:.0}/s", delta)
    } else if delta < 0.0 {
        format!("{:.0}/s", delta)
    } else {
        "0/s".to_string()
    }
}

fn dust_status(dust: f32) -> (&'static str, Color) {
    if dust >= 100.0 {
        ("Stalled", Colors::ERROR)
    } else if dust >= 75.0 {
        ("Power leakage", Colors::WARNING)
    } else if dust >= 50.0 {
        ("Drones slowed", Colors::WARNING)
    } else if dust >= 25.0 {
        ("Efficiency -10%", Colors::TEXT_DIM)
    } else {
        ("Clean", Colors::SUCCESS)
    }
}

fn draw_build_card(
    state: &mut PlanetState,
    x: f32,
    y: f32,
    width: f32,
    building_type: BuildingType,
) -> f32 {
    let height = 56.0;
    let (mouse_x, mouse_y) = mouse_position();
    let hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
    let selected = state.selected_building == Some(building_type);
    let (mineral_cost, energy_cost) = building_type.cost();
    let can_afford = state.resources.can_afford(mineral_cost, energy_cost);
    let unlocked = state.is_building_unlocked(building_type);

    let base_color = if !unlocked {
        Colors::SURFACE_DARK
    } else if selected {
        Colors::PRIMARY_SOFT
    } else if hovered {
        Colors::SURFACE
    } else {
        Colors::SURFACE_DARK
    };
    let border_color = if unlocked && can_afford { Colors::PANEL_BORDER } else { Colors::SECONDARY };
    let text_color = if unlocked && can_afford { Colors::TEXT } else { Colors::TEXT_DIM };

    draw_rectangle(x + 2.0, y + 3.0, width, height, Color::new(0.0, 0.0, 0.0, 0.3));
    draw_rectangle(x, y, width, height, base_color);
    draw_rectangle_lines(x, y, width, height, 1.0, border_color);

    let hotkey = building_type.hotkey().unwrap_or(' ');
    draw_text(
        &format!("[{}]", hotkey),
        x + 10.0,
        y + 22.0,
        14.0,
        Colors::PRIMARY,
    );
    draw_text(
        building_type.name(),
        x + 48.0,
        y + 22.0,
        14.0,
        text_color,
    );
    draw_text(
        &format!("Cost {}M / {}E", mineral_cost as i32, energy_cost as i32),
        x + 48.0,
        y + 40.0,
        11.0,
        if unlocked { Colors::TEXT_DIM } else { Colors::SECONDARY },
    );
    draw_text(
        &format!("Power {}", format_power_delta(building_type.power_delta())),
        x + width - 110.0,
        y + 40.0,
        11.0,
        Colors::PRIMARY_SOFT,
    );
    if !unlocked {
        draw_text("Locked", x + width - 72.0, y + 22.0, 11.0, Colors::WARNING);
    }

    if unlocked && hovered && is_mouse_button_pressed(MouseButton::Left) {
        state.select_building(building_type);
    }

    height
}

fn draw_planetary_background(screen_w: f32, screen_h: f32, time: f32) {
    // Subtle star field
    for i in 0..80u32 {
        let star_x = hash01(i) * screen_w;
        let star_y = hash01(i + 17) * screen_h;
        let twinkle = 0.5 + (time * 0.5 + i as f32).sin().abs() * 0.5;
        let color = Color::new(0.6, 0.7, 0.8, 0.15 + twinkle * 0.2);
        draw_circle(star_x, star_y, 1.0 + hash01(i + 31), color);
    }

    // Planet glow
    let planet_x = screen_w * 0.82;
    let planet_y = screen_h * 0.85;
    let glow = 0.25 + (time * 0.6).sin().abs() * 0.1;
    draw_circle(planet_x, planet_y, 220.0, Color::new(0.0, 0.2, 0.35, 0.12));
    draw_circle(planet_x, planet_y, 170.0, Color::new(0.0, 0.3, 0.45, glow));
    draw_circle(planet_x, planet_y, 120.0, Color::new(0.0, 0.25, 0.4, 0.25));
}

/// Get color for terrain type with subtle variation
fn terrain_color_at(pos: GridPos, terrain: TerrainType, revealed: bool) -> Color {
    if !revealed {
        return Color::new(0.05, 0.05, 0.05, 1.0);
    }

    let def = data::game_data().terrain(terrain.id());
    let mut color = Color::new(def.color[0], def.color[1], def.color[2], def.color[3]);

    if terrain == TerrainType::Empty {
        let noise = hash01((pos.x as u32).wrapping_mul(73856093) ^ (pos.y as u32).wrapping_mul(19349663));
        let offset = noise * 0.04;
        color = Color::new(
            (color.r + offset).min(1.0),
            (color.g + offset).min(1.0),
            (color.b + offset * 0.8).min(1.0),
            color.a,
        );
    }

    color
}

fn terrain_texture<'a>(terrain: TerrainType, textures: &'a GameTextures) -> &'a Texture2D {
    let id = terrain.id();
    textures
        .terrain
        .by_id
        .get(id)
        .unwrap_or(&textures.terrain.by_id["empty"])
}

fn building_texture<'a>(building_type: BuildingType, textures: &'a GameTextures) -> &'a Texture2D {
    textures
        .buildings
        .by_id
        .get(building_type.id())
        .unwrap_or(&textures.buildings.core_stage_1a)
}

fn conduit_texture<'a>(connections: &[bool; 4], textures: &'a GameTextures) -> &'a Texture2D {
    let n = connections[0];
    let e = connections[1];
    let s = connections[2];
    let w = connections[3];

    let count = connections.iter().filter(|c| **c).count();
    match count {
        4 => &textures.buildings.conduit_cross,
        3 => {
            if !n {
                &textures.buildings.conduit_tee_s
            } else if !e {
                &textures.buildings.conduit_tee_w
            } else if !s {
                &textures.buildings.conduit_tee_n
            } else {
                &textures.buildings.conduit_tee_e
            }
        }
        2 => {
            if (n && s) && !e && !w {
                &textures.buildings.conduit_straight_v
            } else if (e && w) && !n && !s {
                &textures.buildings.conduit_straight_h
            } else if n && e {
                &textures.buildings.conduit_corner_ne
            } else if n && w {
                &textures.buildings.conduit_corner_nw
            } else if s && e {
                &textures.buildings.conduit_corner_se
            } else if s && w {
                &textures.buildings.conduit_corner_sw
            } else if n || s {
                &textures.buildings.conduit_straight_v
            } else {
                &textures.buildings.conduit_straight_h
            }
        }
        1 => {
            if n || s {
                &textures.buildings.conduit_straight_v
            } else {
                &textures.buildings.conduit_straight_h
            }
        }
        _ => &textures.buildings.conduit_straight_h,
    }
}

fn conduit_connections(state: &PlanetState, pos: GridPos) -> [bool; 4] {
    let dirs = [
        GridPos::new(pos.x, pos.y - 1),
        GridPos::new(pos.x + 1, pos.y),
        GridPos::new(pos.x, pos.y + 1),
        GridPos::new(pos.x - 1, pos.y),
    ];

    let mut connections = [false; 4];
    for (index, neighbor) in dirs.iter().enumerate() {
        if let Some(tile) = state.grid.get(*neighbor) {
            if let Some(ref building) = tile.building {
                connections[index] = matches!(
                    building.building_type,
                    BuildingType::Conduit
                        | BuildingType::Bridge
                        | BuildingType::Core
                        | BuildingType::Drill
                        | BuildingType::PowerNode
                        | BuildingType::WindTurbine
                        | BuildingType::ServerBank
                        | BuildingType::Sweeper
                        | BuildingType::Storage
                        | BuildingType::BiomassHarvester
                );
            }
        }
    }
    connections
}

fn draw_conduit_tile(
    px: f32,
    py: f32,
    pos: GridPos,
    state: &PlanetState,
    brightness: f32,
    textures: &GameTextures,
) {
    let connections = conduit_connections(state, pos);
    let tint = Color::new(brightness, brightness, brightness, 1.0);
    let texture = conduit_texture(&connections, textures);
    draw_texture_ex(
        texture,
        px,
        py,
        tint,
        DrawTextureParams {
            dest_size: Some(vec2(TILE_SIZE - 1.0, TILE_SIZE - 1.0)),
            ..Default::default()
        },
    );
}

/// Determine current Core evolution stage based on progress
fn core_stage(state: &PlanetState) -> u8 {
    let growth = state.time_played as f32 + (state.resources.minerals + state.resources.data) * 0.4;

    if growth < 60.0 {
        0
    } else if growth < 120.0 {
        1
    } else if growth < 200.0 {
        2
    } else if growth < 320.0 {
        3
    } else {
        4
    }
}

/// Draw evolved Core visuals
fn draw_core_visual(px: f32, py: f32, size: f32, state: &PlanetState, textures: &GameTextures) {
    let stage = core_stage(state);
    let center_x = px + size * 0.5;
    let center_y = py + size * 0.5;
    let pulse = ((state.time_played as f32) * 2.0).sin().abs();

    let texture = match stage {
        0 => &textures.buildings.core_stage_1a,
        1 => &textures.buildings.core_stage_1b,
        2 => &textures.buildings.core_stage_1c,
        3 => &textures.buildings.core_stage_2a,
        _ => &textures.buildings.core_stage_2b,
    };

    draw_texture_ex(
        texture,
        px,
        py,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(size - 1.0, size - 1.0)),
            ..Default::default()
        },
    );

    if stage >= 1 {
        draw_circle_lines(center_x, center_y, 7.0, 1.0, Colors::ACCENT);
    }
    if stage >= 2 {
        draw_line(center_x - 6.0, center_y, center_x + 6.0, center_y, 1.0, Colors::TEXT);
        draw_line(center_x, center_y - 6.0, center_x, center_y + 6.0, 1.0, Colors::TEXT);
    }
    if stage >= 3 {
        let glow_alpha = 0.2 + pulse * 0.2;
        draw_circle_lines(
            center_x,
            center_y,
            11.0,
            1.0,
            Color::new(Colors::PRIMARY.r, Colors::PRIMARY.g, Colors::PRIMARY.b, glow_alpha),
        );
    }
}

/// Render the planetary grid view
pub fn render_planetary_view(
    state: &mut PlanetState,
    show_fps: bool,
    textures: &GameTextures,
    directive: &Directive,
) -> PlanetaryAction {
    clear_background(Colors::BACKGROUND);

    let screen_w = screen_width();
    let screen_h = screen_height();
    let pulse = ((state.time_played as f32) * 2.5).sin().abs();
    let global_pulse = 0.9 + 0.1 * (state.time_played as f32 * 2.0).sin();
    let time = state.time_played as f32;

    let (mouse_x, mouse_y) = mouse_position();
    let hovered_pos = if is_cursor_over_ui(mouse_x, mouse_y, screen_w, screen_h) {
        None
    } else {
        screen_to_grid(mouse_x, mouse_y)
    };

    draw_planetary_background(screen_w, screen_h, time);

    // Draw grid (only visible range)
    let min_x = ((0.0 - GRID_OFFSET_X) / TILE_SIZE).floor() as i32 - 1;
    let min_y = ((0.0 - GRID_OFFSET_Y) / TILE_SIZE).floor() as i32 - 1;
    let max_x = ((screen_w - GRID_OFFSET_X) / TILE_SIZE).ceil() as i32 + 1;
    let max_y = ((screen_h - GRID_OFFSET_Y) / TILE_SIZE).ceil() as i32 + 1;
    let min_x = min_x.max(0);
    let min_y = min_y.max(0);
    let max_x = max_x.min(state.grid.width as i32 - 1);
    let max_y = max_y.min(state.grid.height as i32 - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = GridPos::new(x, y);
            let Some(tile) = state.grid.get(pos) else { continue; };
            let (px, py) = grid_to_screen(pos);

            // Draw terrain
            if tile.revealed {
                let texture = terrain_texture(tile.terrain, textures);
                draw_texture_ex(
                    texture,
                    px,
                    py,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(TILE_SIZE - 1.0, TILE_SIZE - 1.0)),
                        ..Default::default()
                    },
                );
            } else {
                let color = terrain_color_at(pos, tile.terrain, tile.revealed);
                draw_rectangle(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, color);
            }

            // Draw harvestable indicator
            if tile.revealed && tile.terrain.is_harvestable() && tile.building.is_none() {
                let terrain_def = data::game_data().terrain(tile.terrain.id());
                let indicator_color = Color::new(
                    terrain_def.color[0],
                    terrain_def.color[1],
                    terrain_def.color[2],
                    0.5,
                );
                draw_rectangle_lines(px + 2.0, py + 2.0, TILE_SIZE - 5.0, TILE_SIZE - 5.0, 1.0, indicator_color);
            }

            if tile.filter {
                let filter_color = Color::new(0.2, 0.8, 0.6, 0.7);
                draw_circle_lines(px + TILE_SIZE * 0.5, py + TILE_SIZE * 0.5, 6.0, 1.0, filter_color);
            }
            if tile.forest_cleared {
                let scar_color = Color::new(0.8, 0.4, 0.2, 0.6);
                draw_circle_lines(px + TILE_SIZE * 0.5, py + TILE_SIZE * 0.5, 8.0, 1.0, scar_color);
            }

            // Draw building if present
            if let Some(ref building) = tile.building {
                if building.building_type == BuildingType::Core {
                    draw_core_visual(px, py, TILE_SIZE, state, textures);
                } else {
                    let brightness = if building.powered { global_pulse } else { 0.6 };
                    let tint = Color::new(brightness, brightness, brightness, 1.0);
                    let margin = 2.0;
                    let scale = state.placement_scale(pos);
                    let size = (TILE_SIZE - margin * 2.0 - 1.0) * scale;
                    let offset = (TILE_SIZE - margin * 2.0 - 1.0 - size) * 0.5;
                    let box_x = px + margin + offset;
                    let box_y = py + margin + offset;

                    let center_x = px + TILE_SIZE * 0.5;
                    let center_y = py + TILE_SIZE * 0.5;

                    if building.building_type == BuildingType::Conduit {
                        draw_conduit_tile(px, py, pos, state, brightness, textures);
                    } else {
                        let texture = building_texture(building.building_type, textures);
                        draw_texture_ex(
                            texture,
                            box_x,
                            box_y,
                            tint,
                            DrawTextureParams {
                                dest_size: Some(vec2(size, size)),
                                ..Default::default()
                            },
                        );
                    }

                    if building.building_type == BuildingType::PowerNode && building.powered {
                        let glow_color = Color::new(0.0, 0.85, 1.0, 0.18);
                        draw_circle(center_x, center_y, TILE_SIZE * 2.5, glow_color);
                        draw_circle(center_x, center_y, TILE_SIZE * 1.8, Color::new(0.0, 0.85, 1.0, 0.28));
                    }
                }

                // Unpowered indicator
                if !building.powered && building.building_type != BuildingType::Core {
                    draw_text("!", px + 18.0, py + 10.0, 12.0, Colors::ERROR);
                }
            }

            if tile.bridge {
                let texture = building_texture(BuildingType::Bridge, textures);
                draw_texture_ex(
                    texture,
                    px,
                    py,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(TILE_SIZE - 1.0, TILE_SIZE - 1.0)),
                        ..Default::default()
                    },
                );
            }

            // Draw hover highlight
            if let Some(hover) = hovered_pos {
                if hover == pos && tile.revealed {
                    let line_thickness = 1.5 + pulse * 1.5;
                    draw_rectangle_lines(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, line_thickness, Colors::PRIMARY);

                    // Show placement preview
                    if let Some(building_type) = state.selected_building {
                        let preview_alpha = 0.2 + pulse * 0.15;
                        if state.grid.can_place_building(pos, building_type) {
                            let preview_color = Color::new(0.0, 0.8, 1.0, preview_alpha);
                            draw_rectangle(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, preview_color);
                        } else {
                            let invalid_color = Color::new(1.0, 0.2, 0.2, preview_alpha);
                            draw_rectangle(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, invalid_color);
                        }
                    }

                    // Show harvest preview
                    if state.can_harvest(pos) {
                        let harvest_alpha = 0.2 + pulse * 0.15;
                        let harvest_color = Color::new(1.0, 0.5, 0.0, harvest_alpha);
                        draw_rectangle(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, harvest_color);
                    }
                }
            }
        }
    }

    // Draw drones
    for drone in state.drones.drones() {
        let (vx, vy) = drone.visual_position();
        let (dx, dy) = grid_to_screen(GridPos::new(vx as i32, vy as i32));

        let frac_x = vx - vx.floor();
        let frac_y = vy - vy.floor();
        let mut drone_x = dx + frac_x * TILE_SIZE + TILE_SIZE / 2.0 - 4.0;
        let mut drone_y = dy + frac_y * TILE_SIZE + TILE_SIZE / 2.0 - 4.0;

        let drone_color = match drone.state {
            DroneState::Idle => Colors::SECONDARY,
            DroneState::MovingToCore => Colors::SUCCESS,
            DroneState::MovingToDrill => Colors::WARNING,
            DroneState::Delivering => Colors::PRIMARY,
            DroneState::Error => Colors::ERROR,
        };

        let wobble = (time * 6.0 + drone.id as f32).sin() * 1.2;
        let float = (time * 5.0 + drone.id as f32 * 0.7).cos() * 1.0;

        if drone.state == DroneState::Idle {
            // Idle cluster wobble near drill
            drone_x += wobble * 0.6;
            drone_y += float * 0.6;
            draw_circle(drone_x, drone_y, 3.2, drone_color);
        } else if drone.state == DroneState::Error {
            // Error spin + glyph
            let spin = (time * 8.0 + drone.id as f32).sin();
            draw_circle(drone_x, drone_y, 4.0, drone_color);
            draw_line(drone_x - 4.0, drone_y - 4.0, drone_x + 4.0, drone_y + 4.0, 1.0 + spin.abs(), Colors::ERROR);
            draw_line(drone_x + 4.0, drone_y - 4.0, drone_x - 4.0, drone_y + 4.0, 1.0 + spin.abs(), Colors::ERROR);
        } else {
            draw_circle(drone_x + wobble * 0.2, drone_y + float * 0.2, 4.0, drone_color);
        }

        if drone.state != DroneState::Error && drone.path_index > 0 && drone.path_index < drone.path.len() {
            let prev = drone.path[drone.path_index - 1];
            let next = drone.path[drone.path_index];
            let dir_x = (next.x - prev.x) as f32;
            let dir_y = (next.y - prev.y) as f32;
            let length = (dir_x * dir_x + dir_y * dir_y).sqrt().max(0.01);
            let norm_x = dir_x / length;
            let norm_y = dir_y / length;
            let tail_len = 10.0;
            for segment in 0..3 {
                let segment_ratio = segment as f32 / 3.0;
                let tail_x = drone_x - norm_x * tail_len * segment_ratio;
                let tail_y = drone_y - norm_y * tail_len * segment_ratio;
                let alpha = 0.4 * (1.0 - segment_ratio);
                let tail_color = Color::new(drone_color.r, drone_color.g, drone_color.b, alpha);
                draw_circle(tail_x, tail_y, 3.0 - segment as f32 * 0.6, tail_color);
            }
        }

        if drone.carrying > 0.0 && drone.state != DroneState::Error {
            // Visible cargo packet between drill and core
            let mut cargo_x = drone_x;
            let mut cargo_y = drone_y;
            if drone.path_index > 0 && drone.path_index < drone.path.len() {
                let prev = drone.path[drone.path_index - 1];
                let next = drone.path[drone.path_index];
                let dir_x = (next.x - prev.x) as f32;
                let dir_y = (next.y - prev.y) as f32;
                let length = (dir_x * dir_x + dir_y * dir_y).sqrt().max(0.01);
                let norm_x = dir_x / length;
                let norm_y = dir_y / length;
                cargo_x += norm_x * 6.0;
                cargo_y += norm_y * 6.0;
            } else {
                cargo_y -= 6.0;
            }
            draw_rectangle(cargo_x - 2.0, cargo_y - 2.0, 4.0, 4.0, Colors::ACCENT);
            draw_circle(cargo_x, cargo_y, 2.0, Color::new(1.0, 0.8, 0.4, 0.9));
        }

        if state.power_collapse_shutdown > 0.0 {
            // Power collapse: drones sag/fall
            let fall = (1.0 - (state.power_collapse_shutdown / 20.0)).clamp(0.0, 1.0);
            draw_circle(drone_x, drone_y + fall * 6.0, 2.0, Colors::ERROR);
        }
    }

    // Draw particles
    for particle in &state.particles {
        let screen_x = GRID_OFFSET_X + particle.position.0 * TILE_SIZE + TILE_SIZE * 0.5;
        let screen_y = GRID_OFFSET_Y + particle.position.1 * TILE_SIZE + TILE_SIZE * 0.5;
        let alpha = (particle.life / particle.max_life).clamp(0.0, 1.0);
        let color = Color::new(particle.color.r, particle.color.g, particle.color.b, alpha);
        draw_circle(screen_x, screen_y, particle.size, color);
    }

    // Draw UI panels
    let ui_action = draw_ui_panels(state, screen_w, screen_h, hovered_pos, show_fps, directive);

    // Handle input
    if ui_action != PlanetaryAction::None {
        ui_action
    } else {
        handle_input(state, hovered_pos)
    }
}

/// Draw all UI panels
fn draw_ui_panels(
    state: &mut PlanetState,
    screen_w: f32,
    screen_h: f32,
    hovered_pos: Option<GridPos>,
    show_fps: bool,
    directive: &Directive,
) -> PlanetaryAction {
    let mut ui_action = PlanetaryAction::None;

    // Top bar
    draw_panel(0.0, 0.0, screen_w, HUD_HEIGHT);
    draw_text("NANITE SWARM", 18.0, 30.0, 18.0, Colors::PRIMARY);
    draw_text(&state.name, 18.0, 52.0, 14.0, Colors::TEXT_DIM);
    if !state.tutorial_done && !state.tutorial_hidden {
        let panel_w = 440.0;
        let panel_h = 110.0;
        let map_left = SIDEBAR_WIDTH + 12.0;
        let map_right = screen_w - RIGHTBAR_WIDTH - 12.0;
        let panel_x = map_left + (map_right - map_left - panel_w) * 0.5;
        let panel_y = screen_h - BOTTOM_BAR_HEIGHT - panel_h - 12.0;
        draw_panel(panel_x, panel_y, panel_w, panel_h);
        draw_text("Tutorial", panel_x + 12.0, panel_y + 26.0, 16.0, Colors::PRIMARY);

        let conduits_unlocked = state.is_building_unlocked(BuildingType::Conduit);
        let wind_unlocked = state.is_building_unlocked(BuildingType::WindTurbine);
        let server_unlocked = state.is_building_unlocked(BuildingType::ServerBank);
        let (title, body, hint) = match state.tutorial_step {
            0 => ("Step 1: Start Mining", "Place a Drill near the Core to collect minerals.", "Need more materials? Build another Drill."),
            1 => {
                if !conduits_unlocked {
                    ("Step 2: Unlock Conduits", "Open Research (R) and unlock Power Grid.", "Conduits let you expand beyond the Core area.")
                } else {
                    ("Step 2: Connect to Core", "Select Conduit (2) and drag to the Core.", "Connections power your grid and move resources.")
                }
            }
            2 => ("Step 3: Connect to Core", "Select Conduit (2) and drag to the Core.", "Connections power your grid and move resources."),
            3 => {
                if !server_unlocked {
                    ("Step 4: Faster Research", "Research Data Processing to unlock Server Banks.", "Server Banks generate Data but consume power.")
                } else {
                    ("Step 4: Faster Research", "Build a Server Bank to generate Data.", "More data = faster tech unlocks.")
                }
            }
            4 => {
                if !wind_unlocked {
                    ("Step 5: Power Basics", "Power flows from the Core through Conduits and Power Nodes.", "Research Wind Power to unlock turbines that generate more power.")
                } else {
                    ("Step 5: Build Power", "Place a Wind Turbine on any powered tile or Mountain.", "Connect it with Conduits so power reaches your buildings.")
                }
            }
            _ => ("All set", "You are ready to expand.", ""),
        };
        draw_text(title, panel_x + 12.0, panel_y + 50.0, 13.0, Colors::TEXT);
        draw_text(body, panel_x + 12.0, panel_y + 68.0, 12.0, Colors::TEXT_DIM);
        if !hint.is_empty() {
            draw_text(hint, panel_x + 12.0, panel_y + 86.0, 11.0, Colors::PRIMARY_SOFT);
        }
        draw_text("Press T to hide", panel_x + panel_w - 110.0, panel_y + 86.0, 11.0, Colors::TEXT_DIM);
    }

    // Resource chips
    let chip_width = 120.0;
    let chip_spacing = 10.0;
    let total_width = chip_width * 4.0 + chip_spacing * 3.0;
    let chips_x = (screen_w - total_width) * 0.5;
    let chips_y = 16.0;
    draw_resource_chip(chips_x, chips_y, chip_width, "Energy", state.resources.energy, Colors::WARNING);
    draw_resource_chip(chips_x + (chip_width + chip_spacing), chips_y, chip_width, "Minerals", state.resources.minerals, Colors::ACCENT);
    draw_resource_chip(chips_x + 2.0 * (chip_width + chip_spacing), chips_y, chip_width, "Data", state.resources.data, Colors::PRIMARY);
    draw_resource_chip(chips_x + 3.0 * (chip_width + chip_spacing), chips_y, chip_width, "Biomass", state.resources.biomass, Colors::SUCCESS);
    draw_text(
        &format!("{:.0}/{:.0}", state.resources.minerals, state.mineral_capacity()),
        chips_x + (chip_width + chip_spacing) + 60.0,
        chips_y + 30.0,
        10.0,
        Colors::TEXT_DIM,
    );

    // Status block
    let power_color = if state.power_balance >= 0.0 { Colors::SUCCESS } else { Colors::ERROR };
    let status_x = screen_w - 300.0;
    draw_text(
        &format!("Power {:+.0}/s", state.power_balance),
        status_x,
        56.0,
        11.0,
        power_color,
    );
    let (hours, minutes) = state.battery_time_left();
    draw_text(
        &format!("Battery {}h {}m", hours, minutes),
        status_x,
        68.0,
        11.0,
        Colors::PRIMARY_SOFT,
    );
    if state.battery_seconds <= 0.0 {
        draw_text("HIBERNATION", status_x - 130.0, 68.0, 11.0, Colors::WARNING);
    }

    // Action buttons
    let button_y = 14.0;
    if draw_button_sized(screen_w - 300.0, button_y, 90.0, 34.0, "Research") {
        ui_action = PlanetaryAction::OpenResearch;
    }
    if draw_button_sized(screen_w - 200.0, button_y, 80.0, 34.0, "Map") {
        ui_action = PlanetaryAction::OpenInterplanetary;
    }
    if draw_button_sized(screen_w - 110.0, button_y, 80.0, 34.0, "Menu") {
        ui_action = PlanetaryAction::OpenMenu;
    }

    if show_fps {
        let fps = get_fps();
        draw_text(&format!("FPS {}", fps), screen_w - 80.0, 66.0, 11.0, Colors::TEXT_DIM);
    }

    // Offline progress banner
    if state.offline_notice_timer > 0.0 && state.last_offline_seconds > 0.0 {
        let (off_h, off_m) = format_hours_minutes(state.last_offline_seconds);
        let (sim_h, sim_m) = format_hours_minutes(state.last_offline_simulated);
        let banner_w = 440.0;
        let banner_h = 36.0;
        let banner_x = (screen_w - banner_w) * 0.5;
        let banner_y = HUD_HEIGHT + 6.0;
        draw_panel(banner_x, banner_y, banner_w, banner_h);
        let banner_text = format!(
            "Offline {}h {}m | Simulated {}h {}m",
            off_h, off_m, sim_h, sim_m
        );
        draw_text(&banner_text, banner_x + 16.0, banner_y + 24.0, 14.0, Colors::SUCCESS);
    }

    if state.collapse_notice_timer > 0.0 {
        let banner_w = 520.0;
        let banner_h = 36.0;
        let banner_x = (screen_w - banner_w) * 0.5;
        let banner_y = HUD_HEIGHT + 46.0;
        draw_panel(banner_x, banner_y, banner_w, banner_h);
        draw_text(
            "POWER COLLAPSE: drones offline, data corrupted, research locked",
            banner_x + 16.0,
            banner_y + 24.0,
            13.0,
            Colors::ERROR,
        );
    }

    // Left sidebar: Build palette
    let sidebar_x = 12.0;
    let sidebar_y = HUD_HEIGHT + 12.0;
    let sidebar_w = SIDEBAR_WIDTH - 24.0;
    let sidebar_h = screen_h - sidebar_y - BOTTOM_BAR_HEIGHT - 20.0;
    draw_panel(sidebar_x, sidebar_y, sidebar_w, sidebar_h);
    draw_text("Build Palette", sidebar_x + 12.0, sidebar_y + 26.0, 16.0, Colors::PRIMARY);
    draw_text("Click a card or press hotkeys", sidebar_x + 12.0, sidebar_y + 44.0, 11.0, Colors::TEXT_DIM);

    let mut building_defs: Vec<_> = data::game_data()
        .buildings
        .iter()
        .filter(|def| def.show_in_build_menu)
        .collect();
    building_defs.sort_by_key(|def| def.build_menu_order);

    let mut card_y = sidebar_y + 58.0;
    for def in building_defs {
        let Some(building) = BuildingType::from_id(&def.id) else {
            continue;
        };
        if !state.is_building_unlocked(building) {
            continue;
        }
        let card_height = draw_build_card(state, sidebar_x + 10.0, card_y, sidebar_w - 20.0, building);
        card_y += card_height + 10.0;
    }

    draw_text("Quick Actions", sidebar_x + 12.0, card_y + 16.0, 13.0, Colors::TEXT);
    if draw_button_sized(sidebar_x + 12.0, card_y + 24.0, sidebar_w - 24.0, 30.0, "Clear Selection") {
        state.clear_selection();
    }
    draw_text("[H] Harvest terrain", sidebar_x + 12.0, card_y + 66.0, 11.0, Colors::TEXT_DIM);

    // Right sidebar: Intel
    let right_x = screen_w - RIGHTBAR_WIDTH - 12.0;
    let right_y = HUD_HEIGHT + 12.0;
    let right_w = RIGHTBAR_WIDTH;
    let right_h = screen_h - right_y - BOTTOM_BAR_HEIGHT - 20.0;

    let intel_h = 230.0;
    let power_h = 150.0;
    let intel_y = right_y;
    let power_y = intel_y + intel_h + 10.0;
    let ops_y = power_y + power_h + 10.0;
    let ops_h = (right_h - intel_h - power_h - 20.0).max(120.0);

    draw_panel(right_x, intel_y, right_w, intel_h);
    draw_text("Intel", right_x + 12.0, intel_y + 26.0, 16.0, Colors::PRIMARY);

    let display_pos = hovered_pos.or(state.selected_tile);
    let mut intel_text_y = intel_y + 50.0;

    if let Some(selected_building) = state.selected_building {
        draw_text(&format!("Selected: {}", selected_building.name()), right_x + 12.0, intel_text_y, 13.0, Colors::TEXT);
        intel_text_y += 18.0;
        draw_text(selected_building.description(), right_x + 12.0, intel_text_y, 11.0, Colors::TEXT_DIM);
        intel_text_y += 18.0;
        let (minerals, energy) = selected_building.cost();
        draw_text(
            &format!("Cost {}M / {}E", minerals as i32, energy as i32),
            right_x + 12.0,
            intel_text_y,
            11.0,
            Colors::TEXT_DIM,
        );
        intel_text_y += 16.0;
        draw_text(
            &format!("Power {}", format_power_delta(selected_building.power_delta())),
            right_x + 12.0,
            intel_text_y,
            11.0,
            Colors::PRIMARY_SOFT,
        );
        intel_text_y += 16.0;
    } else {
        draw_text("No build selected", right_x + 12.0, intel_text_y, 13.0, Colors::TEXT);
        intel_text_y += 18.0;
        draw_text("Pick a building to see stats.", right_x + 12.0, intel_text_y, 11.0, Colors::TEXT_DIM);
        intel_text_y += 16.0;
    }

    if let Some(tile_pos) = display_pos {
        if let Some(tile) = state.grid.get(tile_pos) {
            let terrain_name = tile.terrain.name();
            let building_type = tile.building.as_ref().map(|building| building.building_type);
            let building_powered = tile.building.as_ref().map(|building| building.powered).unwrap_or(false);
            let building_dust = tile.building.as_ref().map(|building| building.dust).unwrap_or(0.0);
            let is_harvestable = tile.terrain.is_harvestable();
            let harvest_rewards = tile.terrain.harvest_rewards();
            let preservation_bonus = tile.terrain.preservation_bonus();

            draw_text(&format!("Tile: {}", terrain_name), right_x + 12.0, intel_text_y + 8.0, 11.0, Colors::TEXT_DIM);
            intel_text_y += 26.0;

            if tile.filter {
                draw_text("Forest filter active (dust reduction)", right_x + 12.0, intel_text_y, 10.0, Colors::SUCCESS);
                intel_text_y += 16.0;
            }
            if tile.forest_cleared {
                draw_text("Forest cleared: pollution rises in this sector", right_x + 12.0, intel_text_y, 10.0, Colors::WARNING);
                intel_text_y += 16.0;
            }
            if tile.mountain_harvested {
                draw_text("Mountain scarred: turbine bonus lost forever", right_x + 12.0, intel_text_y, 10.0, Colors::WARNING);
                intel_text_y += 16.0;
            }

            if let Some(building_type) = building_type {
                let status_text = if building_powered { "Powered" } else { "No Power" };
                let status_color = if building_powered { Colors::SUCCESS } else { Colors::ERROR };
                draw_text(building_type.name(), right_x + 12.0, intel_text_y, 12.0, Colors::TEXT);
                draw_text(status_text, right_x + 12.0, intel_text_y + 16.0, 11.0, status_color);
                let (dust_label, dust_color) = dust_status(building_dust);
                draw_text(
                    &format!("Dust {:.0}% ({})", building_dust, dust_label),
                    right_x + 12.0,
                    intel_text_y + 32.0,
                    11.0,
                    dust_color,
                );

                if building_type != BuildingType::Core {
                    if draw_button_sized(right_x + right_w - 96.0, intel_text_y - 2.0, 72.0, 26.0, "Sell") {
                        state.try_sell_building(tile_pos);
                    }
                    let (mineral_cost, energy_cost) = building_type.cost();
                    let refund_ratio = 0.5;
                    draw_text(
                        &format!("Refund +{}M +{}E", (mineral_cost * refund_ratio) as i32, (energy_cost * refund_ratio) as i32),
                        right_x + 12.0,
                        intel_text_y + 50.0,
                        10.0,
                        Colors::TEXT_DIM,
                    );
                }
            } else if is_harvestable {
                let (minerals, biomass) = harvest_rewards;
                let reward_text = if minerals > 0.0 {
                    format!("Harvest +{} minerals", minerals as i32)
                } else {
                    format!("Harvest +{} biomass", biomass as i32)
                };
                draw_text(&reward_text, right_x + 12.0, intel_text_y, 11.0, Colors::ACCENT);
                if let Some(bonus) = preservation_bonus {
                    draw_text(bonus, right_x + 12.0, intel_text_y + 16.0, 10.0, Colors::SUCCESS);
                }
            } else {
                draw_text("Empty tile", right_x + 12.0, intel_text_y, 11.0, Colors::TEXT_DIM);
            }
        }
    }

    draw_panel(right_x, power_y, right_w, power_h);
    draw_text("Power Grid", right_x + 12.0, power_y + 26.0, 16.0, Colors::PRIMARY);
    let generation = state.grid.total_power_generation();
    let consumption = state.grid.total_power_consumption();
    draw_text(&format!("Generation {:.1}/s", generation), right_x + 12.0, power_y + 54.0, 12.0, Colors::SUCCESS);
    draw_text(&format!("Consumption {:.1}/s", consumption), right_x + 12.0, power_y + 72.0, 12.0, Colors::ERROR);
    draw_text(&format!("Net {:+.1}/s", state.power_balance), right_x + 12.0, power_y + 90.0, 12.0, power_color);
    let battery_ratio = (state.battery_seconds / (4.0 * 60.0 * 60.0)).clamp(0.0, 1.0);
    draw_text("Battery", right_x + 12.0, power_y + 112.0, 11.0, Colors::TEXT_DIM);
    draw_progress_bar(right_x + 80.0, power_y + 104.0, right_w - 100.0, 10.0, battery_ratio, Colors::PRIMARY_SOFT);

    draw_panel(right_x, ops_y, right_w, ops_h);
    draw_text("Operations", right_x + 12.0, ops_y + 26.0, 16.0, Colors::PRIMARY);
    draw_text(&format!("Drones {}", state.drones.total_count()), right_x + 12.0, ops_y + 54.0, 12.0, Colors::TEXT);
    draw_text(&format!("Buildings {}", state.grid.total_buildings()), right_x + 12.0, ops_y + 72.0, 12.0, Colors::TEXT_DIM);
    let (achieved, total) = state.achievements_progress();
    draw_text(&format!("Achievements {}/{}", achieved, total), right_x + 12.0, ops_y + 90.0, 12.0, Colors::PRIMARY_SOFT);

    // Bottom command bar
    draw_panel(0.0, screen_h - BOTTOM_BAR_HEIGHT, screen_w, BOTTOM_BAR_HEIGHT);
    draw_text(
        "Left click to place | Drag to paint | Right click clears | H to harvest | F filter | F1 help",
        16.0,
        screen_h - 38.0,
        12.0,
        Colors::TEXT_DIM,
    );
    draw_text(
        &format!("Directive: {} [{}/{}] ({:.0}s)", directive.description, directive.progress, directive.target, directive.duration.max(0.0)),
        screen_w - 520.0,
        screen_h - 38.0,
        11.0,
        if directive.completed { Colors::SUCCESS } else { Colors::PRIMARY_SOFT },
    );
    if let Some(selected) = state.selected_building {
        draw_text(
            &format!("Build mode: {}", selected.name()),
            16.0,
            screen_h - 16.0,
            12.0,
            Colors::PRIMARY,
        );
    } else {
        draw_text("Build mode: None", 16.0, screen_h - 16.0, 12.0, Colors::TEXT_DIM);
    }

    // Help overlay
    if state.show_help {
        let help_w = 360.0;
        let help_h = 200.0;
        let help_x = screen_w - help_w - 20.0;
        let help_y = 90.0;
        draw_panel(help_x, help_y, help_w, help_h);
        draw_text("Help & Controls", help_x + 16.0, help_y + 28.0, 18.0, Colors::PRIMARY);
        draw_text("Left Click / Drag: Place building", help_x + 16.0, help_y + 55.0, 14.0, Colors::TEXT);
        draw_text("Right Click: Cancel selection / Harvest", help_x + 16.0, help_y + 75.0, 14.0, Colors::TEXT);
        draw_text("H: Harvest terrain", help_x + 16.0, help_y + 95.0, 14.0, Colors::TEXT);
        draw_text("R: Research  |  M: Map", help_x + 16.0, help_y + 115.0, 14.0, Colors::TEXT);
        draw_text("1-9: Select buildings", help_x + 16.0, help_y + 135.0, 14.0, Colors::TEXT);
        draw_text("F: Convert forest to filter", help_x + 16.0, help_y + 155.0, 14.0, Colors::TEXT);
        draw_text("F1: Toggle help", help_x + 16.0, help_y + 175.0, 14.0, Colors::TEXT_DIM);
    }

    ui_action
}

/// Handle keyboard and mouse input
fn handle_input(state: &mut PlanetState, hovered_pos: Option<GridPos>) -> PlanetaryAction {
    // Building hotkeys
    for def in &data::game_data().buildings {
        let Some(hotkey) = def.hotkey.as_ref().and_then(|key| key.chars().next()) else {
            continue;
        };
        let Some(keycode) = keycode_from_hotkey(hotkey) else {
            continue;
        };
        if !is_key_pressed(keycode) {
            continue;
        }
        let Some(building_type) = BuildingType::from_id(&def.id) else {
            continue;
        };
        if state.is_building_unlocked(building_type) {
            state.select_building(building_type);
        }
    }

    if is_key_pressed(KeyCode::F1) {
        state.show_help = !state.show_help;
    }
    if is_key_pressed(KeyCode::T) {
        state.tutorial_hidden = !state.tutorial_hidden;
    }

    // Harvest terrain with H key
    if is_key_pressed(KeyCode::H) {
        if let Some(pos) = hovered_pos {
            state.try_harvest_terrain(pos);
        }
    }

    // Convert forest to dust filter
    if is_key_pressed(KeyCode::F) {
        if let Some(pos) = hovered_pos {
            state.try_convert_forest_to_filter(pos);
        }
    }

    // Place building on click
    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(pos) = hovered_pos {
            let mut placed = false;
            if let Some(building_type) = state.selected_building {
                if state.grid.can_place_building(pos, building_type) {
                    placed = state.try_place_building(pos);
                }
            }

            if placed {
                state.drag_last_pos = Some(pos);
            } else {
                state.selected_tile = Some(pos);
            }
        }
    }

    // Drag placement while holding left mouse
    if is_mouse_button_down(MouseButton::Left) {
        if let Some(pos) = hovered_pos {
            if state.drag_last_pos != Some(pos) {
                if state.selected_building == Some(BuildingType::Conduit) {
                    if let Some(start_pos) = state.drag_last_pos {
                        if state.try_place_conduit_path(start_pos, pos) {
                            state.drag_last_pos = Some(pos);
                        }
                    }
                } else {
                    state.drag_last_pos = Some(pos);
                    state.try_place_building(pos);
                }
            }
        }
    }

    if is_mouse_button_released(MouseButton::Left) {
        state.drag_last_pos = None;
    }

    // Right click to cancel selection or harvest
    if is_mouse_button_pressed(MouseButton::Right) {
        if let Some(pos) = hovered_pos {
            if !state.try_harvest_terrain(pos) {
                state.clear_selection();
            }
        } else {
            state.clear_selection();
        }
        state.selected_tile = None;
    }

    // Navigation keys
    if is_key_pressed(KeyCode::Escape) {
        state.selected_tile = None;
        return PlanetaryAction::OpenMenu;
    }
    if is_key_pressed(KeyCode::R) {
        return PlanetaryAction::OpenResearch;
    }
    if is_key_pressed(KeyCode::M) {
        return PlanetaryAction::OpenInterplanetary;
    }

    PlanetaryAction::None
}
