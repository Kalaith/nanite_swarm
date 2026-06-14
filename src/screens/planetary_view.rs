//! Main grid gameplay screen

use crate::assets::GameTextures;
use crate::data::{self, UiTheme};
use crate::directives::Directive;
use crate::engine::{BuildingType, DroneState, GridPos, TerrainType};
use crate::state::PlanetState;
use crate::ui::{
    color_from_rgba, draw_hud_button, draw_hud_panel, draw_hud_progress_bar, draw_metric_card,
    draw_status_row, Colors,
};
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

#[derive(Debug, Clone, Copy)]
struct HudMetrics {
    tile_size: f32,
    top_bar_height: f32,
    left_panel_width: f32,
    right_panel_width: f32,
    bottom_bar_height: f32,
    panel_gap: f32,
    panel_padding: f32,
    build_row_height: f32,
}

impl HudMetrics {
    fn for_screen(theme: &UiTheme, screen_w: f32, screen_h: f32) -> Self {
        let compact_height = screen_h < 760.0;
        let compact_width = screen_w < 1260.0;
        Self {
            tile_size: theme.layout.tile_size,
            top_bar_height: if compact_height {
                76.0
            } else {
                theme.layout.top_bar_height
            },
            left_panel_width: if compact_width {
                260.0
            } else {
                theme.layout.left_panel_width
            },
            right_panel_width: if compact_width {
                292.0
            } else {
                theme.layout.right_panel_width
            },
            bottom_bar_height: if compact_height {
                64.0
            } else {
                theme.layout.bottom_bar_height
            },
            panel_gap: theme.layout.panel_gap,
            panel_padding: theme.layout.panel_padding,
            build_row_height: if compact_height {
                68.0
            } else {
                theme.layout.build_row_height
            },
        }
    }

    fn grid_offset_x(&self) -> f32 {
        self.left_panel_width + self.panel_gap * 2.0
    }

    fn grid_offset_y(&self) -> f32 {
        self.top_bar_height + self.panel_gap
    }
}

/// Actions from the planetary view
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlanetaryAction {
    None,
    OpenResearch,
    OpenInterplanetary,
    OpenMenu,
}

/// Convert screen position to grid position
fn screen_to_grid(screen_x: f32, screen_y: f32, metrics: HudMetrics) -> Option<GridPos> {
    let grid_x = ((screen_x - metrics.grid_offset_x()) / metrics.tile_size).floor() as i32;
    let grid_y = ((screen_y - metrics.grid_offset_y()) / metrics.tile_size).floor() as i32;

    if grid_x >= 0 && grid_y >= 0 {
        Some(GridPos::new(grid_x, grid_y))
    } else {
        None
    }
}

/// Convert grid position to screen position
fn grid_to_screen(pos: GridPos, metrics: HudMetrics) -> (f32, f32) {
    (
        metrics.grid_offset_x() + pos.x as f32 * metrics.tile_size,
        metrics.grid_offset_y() + pos.y as f32 * metrics.tile_size,
    )
}

fn is_cursor_over_ui(
    screen_x: f32,
    screen_y: f32,
    screen_w: f32,
    screen_h: f32,
    metrics: HudMetrics,
) -> bool {
    if screen_y <= metrics.top_bar_height || screen_y >= screen_h - metrics.bottom_bar_height {
        return true;
    }
    if screen_x <= metrics.left_panel_width {
        return true;
    }
    if screen_x >= screen_w - metrics.right_panel_width {
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

fn fit_text_to_width(text: &str, max_width: f32, font_size: f32) -> String {
    let metrics = measure_ui_text(text, None, font_size as u16, 1.0);
    if metrics.width <= max_width {
        return text.to_string();
    }

    let mut trimmed = text.to_string();
    while !trimmed.is_empty() {
        trimmed.pop();
        let candidate = format!("{}...", trimmed);
        let metrics = measure_ui_text(&candidate, None, font_size as u16, 1.0);
        if metrics.width <= max_width {
            return candidate;
        }
    }

    "...".to_string()
}

fn draw_build_row(
    state: &mut PlanetState,
    theme: &UiTheme,
    textures: &GameTextures,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    building_type: BuildingType,
) -> f32 {
    let (mouse_x, mouse_y) = mouse_position();
    let hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
    let selected = state.selected_building == Some(building_type);
    let (mineral_cost, energy_cost) = building_type.cost();
    let can_afford = state.resources.can_afford(mineral_cost, energy_cost);
    let unlocked = state.is_building_unlocked(building_type);
    let text = color_from_rgba(&theme.colors.text);
    let dim = color_from_rgba(&theme.colors.text_dim);
    let accent = color_from_rgba(&theme.colors.primary);
    let minerals_color = color_from_rgba(&theme.colors.minerals);
    let energy_color = color_from_rgba(&theme.colors.energy);

    let base_color = if !unlocked {
        color_from_rgba(&theme.colors.panel_deep)
    } else if selected {
        color_from_rgba(&theme.colors.panel_inner)
    } else if hovered {
        Color::new(0.035, 0.15, 0.19, 0.96)
    } else {
        color_from_rgba(&theme.colors.panel_deep)
    };

    let border_color = if selected {
        color_from_rgba(&theme.colors.border_bright)
    } else if unlocked && can_afford {
        color_from_rgba(&theme.colors.border)
    } else {
        color_from_rgba(&theme.colors.text_dim)
    };
    let border_width = if selected { 2.0 } else { 1.0 };

    draw_rectangle(
        x + 2.0,
        y + 3.0,
        width,
        height,
        color_from_rgba(&theme.colors.shadow),
    );
    draw_rectangle(x, y, width, height, base_color);
    draw_rectangle_lines(x, y, width, height, border_width, border_color);

    if selected {
        draw_rectangle_lines(
            x - 1.0,
            y - 1.0,
            width + 2.0,
            height + 2.0,
            1.0,
            Color::new(accent.r, accent.g, accent.b, 0.65),
        );
    }

    let icon_size = 46.0;
    let icon_x = x + 10.0;
    let icon_y = y + (height - icon_size) * 0.5;
    draw_rectangle(
        icon_x,
        icon_y,
        icon_size,
        icon_size,
        color_from_rgba(&theme.colors.panel_inner),
    );
    draw_rectangle_lines(icon_x, icon_y, icon_size, icon_size, 1.0, border_color);

    if let Some(icon) = textures
        .building_icons
        .by_id
        .get(building_type.id())
        .or_else(|| textures.buildings.by_id.get(building_type.id()))
    {
        let icon_tint = if unlocked && can_afford {
            WHITE
        } else {
            Color::new(0.45, 0.48, 0.5, 1.0)
        };
        draw_texture_ex(
            icon,
            icon_x + 4.0,
            icon_y + 4.0,
            icon_tint,
            DrawTextureParams {
                dest_size: Some(vec2(icon_size - 8.0, icon_size - 8.0)),
                ..Default::default()
            },
        );
    }

    let name_x = icon_x + icon_size + 10.0;
    let name_text = fit_text_to_width(building_type.name(), width - 138.0, theme.typography.body);
    let name_color = if unlocked && can_afford { text } else { dim };
    draw_ui_text(
        &name_text,
        name_x,
        y + 20.0,
        theme.typography.body,
        name_color,
    );

    if let Some(hotkey) = building_type.hotkey() {
        let hotkey_text = format!("[{}]", hotkey);
        let hotkey_width =
            measure_ui_text(&hotkey_text, None, theme.typography.small as u16, 1.0).width;
        draw_ui_text(
            &hotkey_text,
            x + width - hotkey_width - 10.0,
            y + 20.0,
            theme.typography.small,
            accent,
        );
    }

    let description = fit_text_to_width(
        building_type.description(),
        width - 92.0,
        theme.typography.small,
    );
    draw_ui_text(&description, name_x, y + 38.0, theme.typography.small, dim);

    let cost_y = y + 60.0;
    let minerals_text = format!("M {}", mineral_cost as i32);
    let energy_text = format!("E {}", energy_cost as i32);
    let mineral_value_color = if state.resources.minerals >= mineral_cost {
        minerals_color
    } else {
        color_from_rgba(&theme.colors.error)
    };
    let energy_value_color = if state.resources.energy >= energy_cost {
        energy_color
    } else {
        color_from_rgba(&theme.colors.error)
    };

    draw_ui_text(
        &minerals_text,
        name_x,
        cost_y,
        theme.typography.small,
        mineral_value_color,
    );
    let minerals_width =
        measure_ui_text(&minerals_text, None, theme.typography.small as u16, 1.0).width;
    draw_ui_text(
        &energy_text,
        name_x + minerals_width + 12.0,
        cost_y,
        theme.typography.small,
        energy_value_color,
    );

    let power_text = format!("P {}", format_power_delta(building_type.power_delta()));
    let power_width = measure_ui_text(&power_text, None, theme.typography.small as u16, 1.0).width;
    draw_ui_text(
        &power_text,
        x + width - power_width - 8.0,
        cost_y,
        theme.typography.small,
        accent,
    );

    if !unlocked {
        draw_ui_text(
            "LOCKED",
            name_x,
            y + height - 8.0,
            theme.typography.small,
            color_from_rgba(&theme.colors.warning),
        );
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
        let noise =
            hash01((pos.x as u32).wrapping_mul(73856093) ^ (pos.y as u32).wrapping_mul(19349663));
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
    metrics: HudMetrics,
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
            dest_size: Some(vec2(metrics.tile_size - 1.0, metrics.tile_size - 1.0)),
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
        draw_line(
            center_x - 6.0,
            center_y,
            center_x + 6.0,
            center_y,
            1.0,
            Colors::TEXT,
        );
        draw_line(
            center_x,
            center_y - 6.0,
            center_x,
            center_y + 6.0,
            1.0,
            Colors::TEXT,
        );
    }
    if stage >= 3 {
        let glow_alpha = 0.2 + pulse * 0.2;
        draw_circle_lines(
            center_x,
            center_y,
            11.0,
            1.0,
            Color::new(
                Colors::PRIMARY.r,
                Colors::PRIMARY.g,
                Colors::PRIMARY.b,
                glow_alpha,
            ),
        );
    }
}

/// Render the planetary grid view
pub fn render_planetary_view(
    state: &mut PlanetState,
    show_fps: bool,
    textures: &GameTextures,
    directive: &Directive,
    theme: &UiTheme,
) -> PlanetaryAction {
    clear_background(color_from_rgba(&theme.colors.background));

    let screen_w = screen_width();
    let screen_h = screen_height();
    let metrics = HudMetrics::for_screen(theme, screen_w, screen_h);
    let pulse = ((state.time_played as f32) * 2.5).sin().abs();
    let global_pulse = 0.9 + 0.1 * (state.time_played as f32 * 2.0).sin();
    let time = state.time_played as f32;

    let (mouse_x, mouse_y) = mouse_position();
    let hovered_pos = if is_cursor_over_ui(mouse_x, mouse_y, screen_w, screen_h, metrics) {
        None
    } else {
        screen_to_grid(mouse_x, mouse_y, metrics)
    };

    draw_planetary_background(screen_w, screen_h, time);

    // Draw grid (only visible range)
    let min_x = ((0.0 - metrics.grid_offset_x()) / metrics.tile_size).floor() as i32 - 1;
    let min_y = ((0.0 - metrics.grid_offset_y()) / metrics.tile_size).floor() as i32 - 1;
    let max_x = ((screen_w - metrics.grid_offset_x()) / metrics.tile_size).ceil() as i32 + 1;
    let max_y = ((screen_h - metrics.grid_offset_y()) / metrics.tile_size).ceil() as i32 + 1;
    let min_x = min_x.max(0);
    let min_y = min_y.max(0);
    let max_x = max_x.min(state.grid.width as i32 - 1);
    let max_y = max_y.min(state.grid.height as i32 - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = GridPos::new(x, y);
            let Some(tile) = state.grid.get(pos) else {
                continue;
            };
            let (px, py) = grid_to_screen(pos, metrics);

            // Draw terrain
            if tile.revealed {
                let texture = terrain_texture(tile.terrain, textures);
                draw_texture_ex(
                    texture,
                    px,
                    py,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(metrics.tile_size - 1.0, metrics.tile_size - 1.0)),
                        ..Default::default()
                    },
                );
            } else {
                let color = terrain_color_at(pos, tile.terrain, tile.revealed);
                draw_rectangle(
                    px,
                    py,
                    metrics.tile_size - 1.0,
                    metrics.tile_size - 1.0,
                    color,
                );
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
                draw_rectangle_lines(
                    px + 2.0,
                    py + 2.0,
                    metrics.tile_size - 5.0,
                    metrics.tile_size - 5.0,
                    1.0,
                    indicator_color,
                );
            }

            if tile.filter {
                let filter_color = Color::new(0.2, 0.8, 0.6, 0.7);
                draw_circle_lines(
                    px + metrics.tile_size * 0.5,
                    py + metrics.tile_size * 0.5,
                    6.0,
                    1.0,
                    filter_color,
                );
            }
            if tile.forest_cleared {
                let scar_color = Color::new(0.8, 0.4, 0.2, 0.6);
                draw_circle_lines(
                    px + metrics.tile_size * 0.5,
                    py + metrics.tile_size * 0.5,
                    8.0,
                    1.0,
                    scar_color,
                );
            }

            // Draw building if present
            if let Some(ref building) = tile.building {
                if building.building_type == BuildingType::Core {
                    draw_core_visual(px, py, metrics.tile_size, state, textures);
                } else {
                    let brightness = if building.powered { global_pulse } else { 0.6 };
                    let tint = Color::new(brightness, brightness, brightness, 1.0);
                    let margin = 2.0;
                    let scale = state.placement_scale(pos);
                    let size = (metrics.tile_size - margin * 2.0 - 1.0) * scale;
                    let offset = (metrics.tile_size - margin * 2.0 - 1.0 - size) * 0.5;
                    let box_x = px + margin + offset;
                    let box_y = py + margin + offset;

                    let center_x = px + metrics.tile_size * 0.5;
                    let center_y = py + metrics.tile_size * 0.5;

                    if building.building_type == BuildingType::Conduit {
                        draw_conduit_tile(px, py, pos, state, brightness, textures, metrics);
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
                        draw_circle(center_x, center_y, metrics.tile_size * 2.5, glow_color);
                        draw_circle(
                            center_x,
                            center_y,
                            metrics.tile_size * 1.8,
                            Color::new(0.0, 0.85, 1.0, 0.28),
                        );
                    }
                }

                // Unpowered indicator
                if !building.powered && building.building_type != BuildingType::Core {
                    draw_ui_text("!", px + 18.0, py + 10.0, 12.0, Colors::ERROR);
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
                        dest_size: Some(vec2(metrics.tile_size - 1.0, metrics.tile_size - 1.0)),
                        ..Default::default()
                    },
                );
            }

            // Draw hover highlight
            if let Some(hover) = hovered_pos {
                if hover == pos && tile.revealed {
                    let line_thickness = 1.5 + pulse * 1.5;
                    draw_rectangle_lines(
                        px,
                        py,
                        metrics.tile_size - 1.0,
                        metrics.tile_size - 1.0,
                        line_thickness,
                        Colors::PRIMARY,
                    );

                    // Show placement preview
                    if let Some(building_type) = state.selected_building {
                        let preview_alpha = 0.2 + pulse * 0.15;
                        if state.grid.can_place_building(pos, building_type) {
                            let preview_color = Color::new(0.0, 0.8, 1.0, preview_alpha);
                            draw_rectangle(
                                px,
                                py,
                                metrics.tile_size - 1.0,
                                metrics.tile_size - 1.0,
                                preview_color,
                            );
                        } else {
                            let invalid_color = Color::new(1.0, 0.2, 0.2, preview_alpha);
                            draw_rectangle(
                                px,
                                py,
                                metrics.tile_size - 1.0,
                                metrics.tile_size - 1.0,
                                invalid_color,
                            );
                        }
                    }

                    // Show harvest preview
                    if state.can_harvest(pos) {
                        let harvest_alpha = 0.2 + pulse * 0.15;
                        let harvest_color = Color::new(1.0, 0.5, 0.0, harvest_alpha);
                        draw_rectangle(
                            px,
                            py,
                            metrics.tile_size - 1.0,
                            metrics.tile_size - 1.0,
                            harvest_color,
                        );
                    }
                }
            }
        }
    }

    // Draw drones
    for drone in state.drones.drones() {
        let (vx, vy) = drone.visual_position();
        let (dx, dy) = grid_to_screen(GridPos::new(vx as i32, vy as i32), metrics);

        let frac_x = vx - vx.floor();
        let frac_y = vy - vy.floor();
        let mut drone_x = dx + frac_x * metrics.tile_size + metrics.tile_size / 2.0 - 4.0;
        let mut drone_y = dy + frac_y * metrics.tile_size + metrics.tile_size / 2.0 - 4.0;

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
            draw_line(
                drone_x - 4.0,
                drone_y - 4.0,
                drone_x + 4.0,
                drone_y + 4.0,
                1.0 + spin.abs(),
                Colors::ERROR,
            );
            draw_line(
                drone_x + 4.0,
                drone_y - 4.0,
                drone_x - 4.0,
                drone_y + 4.0,
                1.0 + spin.abs(),
                Colors::ERROR,
            );
        } else {
            draw_circle(
                drone_x + wobble * 0.2,
                drone_y + float * 0.2,
                4.0,
                drone_color,
            );
        }

        if drone.state != DroneState::Error
            && drone.path_index > 0
            && drone.path_index < drone.path.len()
        {
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
        let screen_x = metrics.grid_offset_x()
            + particle.position.0 * metrics.tile_size
            + metrics.tile_size * 0.5;
        let screen_y = metrics.grid_offset_y()
            + particle.position.1 * metrics.tile_size
            + metrics.tile_size * 0.5;
        let alpha = (particle.life / particle.max_life).clamp(0.0, 1.0);
        let color = Color::new(particle.color.r, particle.color.g, particle.color.b, alpha);
        draw_circle(screen_x, screen_y, particle.size, color);
    }

    // Draw UI panels
    let ui_action = draw_ui_panels(
        state,
        screen_w,
        screen_h,
        hovered_pos,
        show_fps,
        directive,
        textures,
        theme,
        metrics,
    );

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
    textures: &GameTextures,
    theme: &UiTheme,
    metrics: HudMetrics,
) -> PlanetaryAction {
    let mut ui_action = PlanetaryAction::None;

    let text = color_from_rgba(&theme.colors.text);
    let dim = color_from_rgba(&theme.colors.text_dim);
    let primary = color_from_rgba(&theme.colors.primary);
    let primary_soft = color_from_rgba(&theme.colors.primary_soft);
    let success = color_from_rgba(&theme.colors.success);
    let warning = color_from_rgba(&theme.colors.warning);
    let error = color_from_rgba(&theme.colors.error);

    // Top bar
    draw_hud_panel(
        theme,
        Rect::new(0.0, 0.0, screen_w, metrics.top_bar_height),
        None,
    );
    let compact_top = screen_w < 1180.0 || metrics.top_bar_height < 80.0;
    let title_size = if compact_top {
        theme.typography.title - 3.0
    } else {
        theme.typography.title
    };
    let brand_w = (screen_w * 0.18).clamp(188.0, 318.0);
    draw_ui_text("NANITE SWARM", 66.0, 31.0, title_size, primary);
    draw_ui_text(
        &format!("{} SECTOR 7-B", state.name.to_uppercase()),
        66.0,
        55.0,
        if compact_top {
            theme.typography.small
        } else {
            theme.typography.body
        },
        dim,
    );
    draw_line(24.0, 24.0, 36.0, 14.0, 1.5, primary);
    draw_line(24.0, 24.0, 36.0, 34.0, 1.5, primary);
    draw_line(48.0, 24.0, 36.0, 14.0, 1.5, primary_soft);
    draw_line(48.0, 24.0, 36.0, 34.0, 1.5, primary_soft);
    draw_circle(36.0, 24.0, 4.0, primary);

    let cards_x = brand_w + metrics.panel_gap;
    let cards_y = 9.0;
    let card_gap = if compact_top { 6.0 } else { 10.0 };
    let actions_w = (screen_w * 0.25).clamp(304.0, 448.0);
    let action_x = screen_w - actions_w - 8.0;
    let card_area_w = (action_x - cards_x - metrics.panel_gap).max(0.0);
    let card_w = ((card_area_w - card_gap * 3.0) / 4.0).max(56.0);
    let card_h = if compact_top { 62.0 } else { 68.0 };
    let energy_value = format!("{:.0}", state.resources.energy);
    let energy_rate = format_power_delta(state.power_balance);
    let minerals_value = format!("{:.0}", state.resources.minerals);
    let minerals_rate = format!("+{:.1}/s", state.config.buildings.drill_output_rate);
    let minerals_cap = format!("/ {:.0}", state.mineral_capacity());
    let data_value = format!("{:.0}", state.resources.data);
    let data_rate = format!("+{:.2}/s", state.config.resources.core_data_rate);
    let biomass_value = format!("{:.0}", state.resources.biomass);
    let biomass_rate = format!("+{:.1}/s", state.biomass_power_bonus.max(0.0));

    draw_metric_card(
        theme,
        Rect::new(cards_x, cards_y, card_w, card_h),
        "energy",
        "ENERGY",
        &energy_value,
        &energy_rate,
        None,
        color_from_rgba(&theme.colors.energy),
    );
    draw_metric_card(
        theme,
        Rect::new(cards_x + (card_w + card_gap), cards_y, card_w, card_h),
        "minerals",
        "MINERALS",
        &minerals_value,
        &minerals_rate,
        Some(&minerals_cap),
        color_from_rgba(&theme.colors.minerals),
    );
    draw_metric_card(
        theme,
        Rect::new(cards_x + 2.0 * (card_w + card_gap), cards_y, card_w, card_h),
        "data",
        "DATA",
        &data_value,
        &data_rate,
        None,
        color_from_rgba(&theme.colors.data),
    );
    draw_metric_card(
        theme,
        Rect::new(cards_x + 3.0 * (card_w + card_gap), cards_y, card_w, card_h),
        "biomass",
        "BIOMASS",
        &biomass_value,
        &biomass_rate,
        Some("/ 1000"),
        color_from_rgba(&theme.colors.biomass),
    );

    let power_color = if state.power_balance >= 0.0 {
        success
    } else {
        error
    };
    let button_y = 10.0;
    let button_gap = 8.0;
    let button_w = (actions_w - button_gap * 2.0) / 3.0;
    if draw_hud_button(
        theme,
        Rect::new(action_x, button_y, button_w, 34.0),
        "RESEARCH",
    ) {
        ui_action = PlanetaryAction::OpenResearch;
    }
    if draw_hud_button(
        theme,
        Rect::new(action_x + button_w + button_gap, button_y, button_w, 34.0),
        "MAP",
    ) {
        ui_action = PlanetaryAction::OpenInterplanetary;
    }
    if draw_hud_button(
        theme,
        Rect::new(
            action_x + (button_w + button_gap) * 2.0,
            button_y,
            button_w,
            34.0,
        ),
        "MENU",
    ) {
        ui_action = PlanetaryAction::OpenMenu;
    }

    draw_ui_text(
        &format!("Power {:+.0}/s", state.power_balance),
        action_x + 2.0,
        62.0,
        theme.typography.small,
        power_color,
    );
    let (hours, minutes) = state.battery_time_left();
    draw_ui_text(
        &format!("Battery {}h {}m", hours, minutes),
        action_x + actions_w * 0.36,
        62.0,
        theme.typography.small,
        primary_soft,
    );
    if state.battery_seconds <= 0.0 {
        draw_ui_text(
            "HIBERNATION",
            action_x + actions_w * 0.7,
            62.0,
            theme.typography.small,
            warning,
        );
    }

    if show_fps {
        let fps = get_fps();
        draw_ui_text(
            &format!("FPS {}", fps),
            screen_w - 80.0,
            80.0,
            theme.typography.small,
            dim,
        );
    }

    // Offline progress banner
    if state.offline_notice_timer > 0.0 && state.last_offline_seconds > 0.0 {
        let (off_h, off_m) = format_hours_minutes(state.last_offline_seconds);
        let (sim_h, sim_m) = format_hours_minutes(state.last_offline_simulated);
        let banner_w = 440.0;
        let banner_h = 36.0;
        let banner_x = (screen_w - banner_w) * 0.5;
        let banner_y = metrics.top_bar_height + 6.0;
        draw_hud_panel(
            theme,
            Rect::new(banner_x, banner_y, banner_w, banner_h),
            None,
        );
        let banner_text = format!(
            "Offline {}h {}m | Simulated {}h {}m",
            off_h, off_m, sim_h, sim_m
        );
        draw_ui_text(
            &banner_text,
            banner_x + 16.0,
            banner_y + 24.0,
            14.0,
            success,
        );
    }

    if state.collapse_notice_timer > 0.0 {
        let banner_w = 520.0;
        let banner_h = 36.0;
        let banner_x = (screen_w - banner_w) * 0.5;
        let banner_y = metrics.top_bar_height + 46.0;
        draw_hud_panel(
            theme,
            Rect::new(banner_x, banner_y, banner_w, banner_h),
            None,
        );
        draw_ui_text(
            "POWER COLLAPSE: drones offline, data corrupted, research locked",
            banner_x + 16.0,
            banner_y + 24.0,
            13.0,
            error,
        );
    }

    // Left sidebar: Build palette
    let sidebar_x = 10.0;
    let sidebar_y = metrics.top_bar_height + metrics.panel_gap;
    let sidebar_w = metrics.left_panel_width - metrics.panel_gap * 1.5;
    let sidebar_h = screen_h - sidebar_y - metrics.bottom_bar_height - metrics.panel_gap;
    draw_hud_panel(
        theme,
        Rect::new(sidebar_x, sidebar_y, sidebar_w, sidebar_h),
        Some("BUILD PALETTE"),
    );
    draw_ui_text(
        "Drag or click to build",
        sidebar_x + metrics.panel_padding,
        sidebar_y + 50.0,
        theme.typography.small,
        dim,
    );

    let mut building_defs: Vec<_> = data::game_data()
        .buildings
        .iter()
        .filter(|def| def.show_in_build_menu)
        .collect();
    building_defs.sort_by_key(|def| def.build_menu_order);

    let list_top = sidebar_y + 62.0;
    let quick_actions_h = 64.0;
    let list_bottom = sidebar_y + sidebar_h - quick_actions_h - metrics.panel_gap;
    let list_height = (list_bottom - list_top).max(0.0);
    let content_x = sidebar_x + metrics.panel_padding;
    let content_w = sidebar_w - metrics.panel_padding * 2.0;
    let row_gap = 8.0;
    let card_w = content_w;

    let mut visible_buildings = Vec::new();
    for def in building_defs {
        let Some(building) = BuildingType::from_id(&def.id) else {
            continue;
        };
        if !state.is_building_unlocked(building) {
            continue;
        }
        visible_buildings.push(building);
    }

    let total_rows = visible_buildings.len();
    let total_height = if total_rows == 0 {
        0.0
    } else {
        total_rows as f32 * metrics.build_row_height
            + (total_rows.saturating_sub(1)) as f32 * row_gap
    };
    let max_scroll = (total_height - list_height).max(0.0);

    let (mouse_x, mouse_y) = mouse_position();
    if mouse_x >= content_x
        && mouse_x <= content_x + content_w
        && mouse_y >= list_top
        && mouse_y <= list_bottom
    {
        let (_wheel_x, wheel_y) = mouse_wheel();
        if wheel_y.abs() > 0.0 {
            state.build_palette_scroll =
                (state.build_palette_scroll - wheel_y * 24.0).clamp(0.0, max_scroll);
        }
    }
    state.build_palette_scroll = state.build_palette_scroll.clamp(0.0, max_scroll);

    let start_y = list_top - state.build_palette_scroll;
    for (index, building) in visible_buildings.into_iter().enumerate() {
        let card_y = start_y + index as f32 * (metrics.build_row_height + row_gap);
        if card_y + metrics.build_row_height < list_top || card_y > list_bottom {
            continue;
        }
        draw_build_row(
            state,
            theme,
            textures,
            content_x,
            card_y,
            card_w,
            metrics.build_row_height,
            building,
        );
    }

    if max_scroll > 0.0 && list_height > 0.0 {
        let scrollbar_w = 6.0;
        let scrollbar_x = sidebar_x + sidebar_w - scrollbar_w - 6.0;
        draw_rectangle(
            scrollbar_x,
            list_top,
            scrollbar_w,
            list_height,
            color_from_rgba(&theme.colors.panel_deep),
        );
        let mut handle_h = list_height * (list_height / total_height);
        if handle_h < 18.0 {
            handle_h = 18.0;
        }
        let handle_y =
            list_top + (state.build_palette_scroll / max_scroll) * (list_height - handle_h);
        draw_rectangle(scrollbar_x, handle_y, scrollbar_w, handle_h, primary_soft);
    }

    let quick_actions_y = list_bottom + 12.0;
    if draw_hud_button(
        theme,
        Rect::new(
            sidebar_x + metrics.panel_padding,
            quick_actions_y,
            sidebar_w - metrics.panel_padding * 2.0,
            30.0,
        ),
        "DEMOLISH MODE",
    ) {
        state.clear_selection();
    }
    draw_ui_text(
        "[H] Harvest terrain  [F] Forest filter",
        sidebar_x + metrics.panel_padding,
        quick_actions_y + 48.0,
        theme.typography.small,
        dim,
    );

    // Right sidebar: structure, grid, operations, and objective stack.
    let right_x = screen_w - metrics.right_panel_width - 10.0;
    let right_y = metrics.top_bar_height + metrics.panel_gap;
    let right_w = metrics.right_panel_width;
    let right_h = screen_h - right_y - metrics.bottom_bar_height - metrics.panel_gap;

    let stack_gap = metrics.panel_gap;
    let available_stack = (right_h - stack_gap * 3.0).max(360.0);
    let compact_stack = available_stack < 520.0;
    let (inspector_h, power_h, ops_h) = if compact_stack {
        (
            available_stack * 0.34,
            available_stack * 0.25,
            available_stack * 0.18,
        )
    } else {
        (
            (available_stack * 0.33).clamp(174.0, 228.0),
            (available_stack * 0.23).clamp(118.0, 148.0),
            (available_stack * 0.19).clamp(96.0, 132.0),
        )
    };
    let directive_h = available_stack - inspector_h - power_h - ops_h;
    let inspector_y = right_y;
    let power_y = inspector_y + inspector_h + stack_gap;
    let ops_y = power_y + power_h + stack_gap;
    let directive_y = ops_y + ops_h + stack_gap;

    draw_hud_panel(
        theme,
        Rect::new(right_x, inspector_y, right_w, inspector_h),
        Some("SELECTED STRUCTURE"),
    );

    let display_pos = hovered_pos.or(state.selected_tile);
    let mut tile_building = None;
    let mut tile_pos_with_building = None;
    let mut tile_terrain = None;
    let mut tile_powered = false;
    let mut tile_dust = 0.0;
    let mut tile_harvest = None;
    let mut tile_bonus = None;
    if let Some(tile_pos) = display_pos {
        if let Some(tile) = state.grid.get(tile_pos) {
            tile_terrain = Some(tile.terrain);
            tile_harvest = Some(tile.terrain.harvest_rewards());
            tile_bonus = tile.terrain.preservation_bonus();
            if let Some(building) = &tile.building {
                tile_building = Some(building.building_type);
                tile_pos_with_building = Some(tile_pos);
                tile_powered = building.powered;
                tile_dust = building.dust;
            }
        }
    }

    let inspected_building = tile_building.or(state.selected_building);
    if let Some(building_type) = inspected_building {
        let header_y = inspector_y + 44.0;
        let compact_inspector = inspector_h < 204.0;
        let icon_size = if compact_inspector { 54.0 } else { 72.0 };
        let icon_rect = Rect::new(right_x + 16.0, header_y, icon_size, icon_size);
        draw_rectangle(
            icon_rect.x,
            icon_rect.y,
            icon_rect.w,
            icon_rect.h,
            color_from_rgba(&theme.colors.panel_inner),
        );
        draw_rectangle_lines(
            icon_rect.x,
            icon_rect.y,
            icon_rect.w,
            icon_rect.h,
            1.0,
            color_from_rgba(&theme.colors.border),
        );
        if let Some(icon) = textures
            .building_icons
            .by_id
            .get(building_type.id())
            .or_else(|| textures.buildings.by_id.get(building_type.id()))
        {
            draw_texture_ex(
                icon,
                icon_rect.x + 6.0,
                icon_rect.y + 6.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(icon_size - 12.0, icon_size - 12.0)),
                    ..Default::default()
                },
            );
        }
        let info_x = icon_rect.x + icon_rect.w + 14.0;
        draw_ui_text(building_type.name(), info_x, header_y + 16.0, 13.0, text);
        draw_ui_text(
            "Tier 1",
            right_x + right_w - 56.0,
            header_y + 16.0,
            10.0,
            dim,
        );
        let description = fit_text_to_width(
            building_type.description(),
            right_x + right_w - info_x - 14.0,
            10.0,
        );
        draw_ui_text(&description, info_x, header_y + 40.0, 10.0, dim);
        let output = format_power_delta(building_type.power_delta());
        let row_base = inspector_y + inspector_h - 62.0;
        draw_status_row(
            theme,
            right_x + 16.0,
            row_base,
            right_w - 32.0,
            "Power",
            &output,
            power_color,
        );
        let (dust_label, dust_color) = dust_status(tile_dust);
        draw_status_row(
            theme,
            right_x + 16.0,
            row_base + 20.0,
            right_w - 32.0,
            "Dust",
            &format!("{:.0}% {}", tile_dust, dust_label),
            dust_color,
        );
        let status_text = if tile_building.is_some() {
            if tile_powered {
                "Powered"
            } else {
                "No power"
            }
        } else {
            "Blueprint"
        };
        let status_color = if tile_building.is_some() && !tile_powered {
            error
        } else {
            success
        };
        draw_status_row(
            theme,
            right_x + 16.0,
            row_base + 40.0,
            right_w - 32.0,
            "Status",
            status_text,
            status_color,
        );
        if let Some(tile_pos) = tile_pos_with_building {
            if building_type != BuildingType::Core
                && draw_hud_button(
                    theme,
                    Rect::new(right_x + right_w - 82.0, inspector_y + 40.0, 62.0, 24.0),
                    "SELL",
                )
            {
                state.try_sell_building(tile_pos);
            }
        }
    } else {
        draw_ui_text(
            "NO STRUCTURE",
            right_x + 16.0,
            inspector_y + 56.0,
            13.0,
            text,
        );
        if let Some(terrain) = tile_terrain {
            draw_ui_text(
                &format!("Terrain: {}", terrain.name()),
                right_x + 16.0,
                inspector_y + 82.0,
                11.0,
                dim,
            );
            if terrain.is_harvestable() {
                let (minerals, biomass) = tile_harvest.unwrap_or((0.0, 0.0));
                let reward_text = if minerals > 0.0 {
                    format!("Harvest +{} minerals", minerals as i32)
                } else {
                    format!("Harvest +{} biomass", biomass as i32)
                };
                draw_ui_text(
                    &reward_text,
                    right_x + 16.0,
                    inspector_y + 106.0,
                    11.0,
                    warning,
                );
                if let Some(bonus) = tile_bonus {
                    let bonus_text = fit_text_to_width(bonus, right_w - 32.0, 10.0);
                    draw_ui_text(
                        &bonus_text,
                        right_x + 16.0,
                        inspector_y + 126.0,
                        10.0,
                        success,
                    );
                }
            }
        } else {
            draw_ui_text(
                "Hover a tile or select a build option.",
                right_x + 16.0,
                inspector_y + 82.0,
                11.0,
                dim,
            );
        }
    }

    draw_hud_panel(
        theme,
        Rect::new(right_x, power_y, right_w, power_h),
        Some("POWER GRID"),
    );
    let generation = state.grid.total_power_generation();
    let consumption = state.grid.total_power_consumption();
    let power_content_y = power_y + if power_h < 130.0 { 48.0 } else { 54.0 };
    let power_row_gap = if power_h < 130.0 { 17.0 } else { 18.0 };
    draw_status_row(
        theme,
        right_x + 12.0,
        power_content_y,
        right_w - 24.0,
        "Generation",
        &format!("+{:.1}/s", generation),
        success,
    );
    draw_status_row(
        theme,
        right_x + 12.0,
        power_content_y + power_row_gap,
        right_w - 24.0,
        "Consumption",
        &format!("-{:.1}/s", consumption),
        error,
    );
    draw_status_row(
        theme,
        right_x + 12.0,
        power_content_y + power_row_gap * 2.0,
        right_w - 24.0,
        "Net",
        &format!("{:+.1}/s", state.power_balance),
        power_color,
    );
    let battery_ratio = (state.battery_seconds / (4.0 * 60.0 * 60.0)).clamp(0.0, 1.0);
    let battery_y = power_y + power_h - 22.0;
    draw_ui_text("Battery", right_x + 12.0, battery_y + 10.0, 10.0, dim);
    draw_hud_progress_bar(
        theme,
        Rect::new(right_x + 86.0, battery_y, right_w - 108.0, 10.0),
        battery_ratio,
        primary_soft,
    );

    draw_hud_panel(
        theme,
        Rect::new(right_x, ops_y, right_w, ops_h),
        Some("OPERATIONS"),
    );
    let ops_content_y = ops_y + if ops_h < 110.0 { 48.0 } else { 54.0 };
    let ops_row_gap = if ops_h < 110.0 { 17.0 } else { 20.0 };
    draw_status_row(
        theme,
        right_x + 12.0,
        ops_content_y,
        right_w - 24.0,
        "Drones",
        &format!("{}", state.drones.total_count()),
        text,
    );
    draw_status_row(
        theme,
        right_x + 12.0,
        ops_content_y + ops_row_gap,
        right_w - 24.0,
        "Structures",
        &format!("{}", state.grid.total_buildings()),
        text,
    );
    let (achieved, total) = state.achievements_progress();
    draw_status_row(
        theme,
        right_x + 12.0,
        ops_content_y + ops_row_gap * 2.0,
        right_w - 24.0,
        "Achievements",
        &format!("{}/{}", achieved, total),
        primary_soft,
    );

    draw_hud_panel(
        theme,
        Rect::new(right_x, directive_y, right_w, directive_h),
        Some("TUTORIAL: EXPAND & AUTOMATE"),
    );
    let directive_text = fit_text_to_width(&directive.description, right_w - 28.0, 11.0);
    let directive_text_y = directive_y + if directive_h < 112.0 { 48.0 } else { 56.0 };
    draw_ui_text(
        &directive_text,
        right_x + 12.0,
        directive_text_y,
        11.0,
        text,
    );
    draw_status_row(
        theme,
        right_x + 12.0,
        directive_text_y + 24.0,
        right_w - 24.0,
        "Directive",
        &format!("{}/{}", directive.progress, directive.target),
        if directive.completed {
            success
        } else {
            warning
        },
    );
    draw_status_row(
        theme,
        right_x + 12.0,
        directive_text_y + 44.0,
        right_w - 24.0,
        "Timer",
        &format!("{:.0}s", directive.duration.max(0.0)),
        primary_soft,
    );
    if !state.tutorial_done && !state.tutorial_hidden && directive_h >= 122.0 {
        draw_ui_text(
            &format!(
                "Tutorial step {} / 5",
                state.tutorial_step.saturating_add(1).min(5)
            ),
            right_x + 12.0,
            directive_y + directive_h - 14.0,
            10.0,
            dim,
        );
    }

    // Bottom command bar
    let bottom_y = screen_h - metrics.bottom_bar_height;
    draw_hud_panel(
        theme,
        Rect::new(0.0, bottom_y, screen_w, metrics.bottom_bar_height),
        None,
    );
    let bottom_gap = if screen_w < 1160.0 { 8.0 } else { 12.0 };
    let alert_w = (screen_w * 0.18).clamp(188.0, 294.0);
    let mut status_w = (screen_w * 0.30).clamp(300.0, 452.0);
    if screen_w < 1050.0 {
        status_w = (screen_w * 0.31).clamp(278.0, 330.0);
    }
    draw_hud_panel(
        theme,
        Rect::new(
            10.0,
            bottom_y + 8.0,
            alert_w,
            metrics.bottom_bar_height - 16.0,
        ),
        None,
    );
    let alert_count = i32::from(state.power_balance < 0.0)
        + i32::from(state.battery_seconds <= 0.0)
        + i32::from(state.power_collapse_shutdown > 0.0);
    draw_ui_text("ALERTS", 30.0, bottom_y + 31.0, 12.0, warning);
    draw_ui_text(
        &format!("{}", alert_count),
        52.0,
        bottom_y + metrics.bottom_bar_height - 17.0,
        if metrics.bottom_bar_height < 70.0 {
            18.0
        } else {
            24.0
        },
        warning,
    );
    let alert_text = if state.power_collapse_shutdown > 0.0 {
        "POWER COLLAPSE"
    } else if state.battery_seconds <= 0.0 {
        "LOW BATTERY"
    } else if state.power_balance < 0.0 {
        "NEGATIVE POWER"
    } else {
        "SYSTEM NOMINAL"
    };
    draw_ui_text(
        alert_text,
        106.0,
        bottom_y + 35.0,
        11.0,
        if alert_count > 0 { error } else { success },
    );

    let controls_x = 10.0 + alert_w + bottom_gap;
    let status_x = screen_w - status_w - 10.0;
    let controls_w = (status_x - controls_x - bottom_gap).max(0.0);
    draw_hud_panel(
        theme,
        Rect::new(
            controls_x,
            bottom_y + 8.0,
            controls_w,
            metrics.bottom_bar_height - 16.0,
        ),
        None,
    );
    let control_y = bottom_y + 30.0;
    let controls = if controls_w < 420.0 {
        vec![
            ("SELECT", "Left Click"),
            ("PAN", "Drag"),
            ("ZOOM", "Wheel"),
            ("PAUSE", "Space"),
        ]
    } else if controls_w < 560.0 {
        vec![
            ("SELECT", "Left Click"),
            ("PAN", "Middle Drag"),
            ("ZOOM", "Wheel"),
            ("BUILD", "B"),
            ("PAUSE", "Space"),
        ]
    } else {
        vec![
            ("SELECT", "Left Click"),
            ("PAN", "Middle Drag"),
            ("BOX SELECT", "Shift + Drag"),
            ("ZOOM", "Mouse Wheel"),
            ("BUILD MENU", "B"),
            ("DEMOLISH", "X"),
            ("PAUSE", "Space"),
        ]
    };
    let slot_w = controls_w / controls.len() as f32;
    for (index, (label, hint)) in controls.iter().enumerate() {
        let x = controls_x + index as f32 * slot_w + 12.0;
        draw_ui_text(label, x, control_y, 10.0, text);
        draw_ui_text(hint, x, control_y + 18.0, 9.0, dim);
        if index > 0 {
            let divider_x = controls_x + index as f32 * slot_w;
            draw_line(
                divider_x,
                bottom_y + 16.0,
                divider_x,
                bottom_y + metrics.bottom_bar_height - 16.0,
                1.0,
                color_from_rgba(&theme.colors.border),
            );
        }
    }

    draw_hud_panel(
        theme,
        Rect::new(
            status_x,
            bottom_y + 8.0,
            status_w,
            metrics.bottom_bar_height - 16.0,
        ),
        None,
    );
    let time_seconds = state.time_played.max(0.0) as i32;
    let time_h = time_seconds / 3600;
    let time_m = (time_seconds % 3600) / 60;
    let time_s = time_seconds % 60;
    draw_ui_text("MISSION TIME", status_x + 22.0, bottom_y + 27.0, 9.0, dim);
    draw_ui_text(
        &format!("{:02}:{:02}:{:02}", time_h, time_m, time_s),
        status_x + 22.0,
        bottom_y + 52.0,
        16.0,
        text,
    );
    let speed_x = status_x + status_w * 0.36;
    draw_ui_text("GAME SPEED", speed_x, bottom_y + 27.0, 9.0, dim);
    draw_ui_text("1.0x", speed_x + 10.0, bottom_y + 52.0, 16.0, text);
    draw_hud_button(
        theme,
        Rect::new(speed_x - 30.0, bottom_y + 34.0, 24.0, 24.0),
        "-",
    );
    draw_hud_button(
        theme,
        Rect::new(speed_x + 58.0, bottom_y + 34.0, 24.0, 24.0),
        "+",
    );

    let graph_x = status_x + status_w * 0.66;
    let graph_y = bottom_y + 22.0;
    let graph_w = status_x + status_w - 18.0 - graph_x;
    let graph_h = if metrics.bottom_bar_height < 70.0 {
        28.0
    } else {
        34.0
    };
    draw_rectangle(
        graph_x,
        graph_y,
        graph_w,
        graph_h,
        color_from_rgba(&theme.colors.panel_deep),
    );
    draw_rectangle_lines(
        graph_x,
        graph_y,
        graph_w,
        graph_h,
        1.0,
        color_from_rgba(&theme.colors.border),
    );
    for i in 1..24 {
        let x0 = graph_x + (i - 1) as f32 * graph_w / 23.0;
        let x1 = graph_x + i as f32 * graph_w / 23.0;
        let y0 =
            graph_y + graph_h * (0.55 + (state.time_played as f32 * 0.1 + i as f32).sin() * 0.18);
        let y1 = graph_y
            + graph_h * (0.55 + (state.time_played as f32 * 0.1 + i as f32 + 1.0).sin() * 0.18);
        draw_line(x0, y0, x1, y1, 1.0, primary);
    }

    let mode_y = bottom_y + metrics.bottom_bar_height - 8.0;
    draw_ui_text(
        if state.selected_building.is_some() {
            "BUILD MODE"
        } else {
            "SELECT MODE"
        },
        controls_x + 12.0,
        mode_y,
        9.0,
        primary_soft,
    );
    if let Some(selected) = state.selected_building {
        draw_ui_text(selected.name(), controls_x + 86.0, mode_y, 9.0, text);
    }

    // Help overlay
    if state.show_help {
        let help_w = 360.0;
        let help_h = 200.0;
        let help_x = screen_w - help_w - 20.0;
        let help_y = 90.0;
        draw_hud_panel(
            theme,
            Rect::new(help_x, help_y, help_w, help_h),
            Some("HELP & CONTROLS"),
        );
        draw_ui_text(
            "Left Click / Drag: Place building",
            help_x + 16.0,
            help_y + 55.0,
            14.0,
            text,
        );
        draw_ui_text(
            "Right Click: Cancel selection / Harvest",
            help_x + 16.0,
            help_y + 75.0,
            14.0,
            text,
        );
        draw_ui_text(
            "H: Harvest terrain",
            help_x + 16.0,
            help_y + 95.0,
            14.0,
            text,
        );
        draw_ui_text(
            "R: Research  |  M: Map",
            help_x + 16.0,
            help_y + 115.0,
            14.0,
            text,
        );
        draw_ui_text(
            "1-9: Select buildings",
            help_x + 16.0,
            help_y + 135.0,
            14.0,
            text,
        );
        draw_ui_text(
            "F: Convert forest to filter",
            help_x + 16.0,
            help_y + 155.0,
            14.0,
            text,
        );
        draw_ui_text("F1: Toggle help", help_x + 16.0, help_y + 175.0, 14.0, dim);
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
