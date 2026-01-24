//! Main grid gameplay screen

use macroquad::prelude::*;
use crate::state::PlanetState;
use crate::engine::{GridPos, TerrainType, BuildingType, DroneState};
use crate::ui::{Colors, Dimensions, draw_panel, draw_resource, draw_button};

const TILE_SIZE: f32 = 28.0;
const GRID_OFFSET_X: f32 = 200.0;
const GRID_OFFSET_Y: f32 = 60.0;

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

/// Get color for terrain type
fn terrain_color(terrain: TerrainType, revealed: bool) -> Color {
    if !revealed {
        return Color::new(0.05, 0.05, 0.05, 1.0);
    }

    match terrain {
        TerrainType::Empty => Color::new(0.12, 0.12, 0.12, 1.0),
        TerrainType::Mountain => Color::new(0.4, 0.35, 0.3, 1.0),
        TerrainType::Forest => Color::new(0.15, 0.3, 0.15, 1.0),
        TerrainType::Water => Color::new(0.1, 0.2, 0.4, 1.0),
        TerrainType::Rough => Color::new(0.2, 0.18, 0.15, 1.0),
        TerrainType::Void => Color::new(0.02, 0.02, 0.02, 1.0),
    }
}

/// Get color for building type (dimmed if unpowered)
fn building_color(building_type: BuildingType, powered: bool) -> Color {
    let base = match building_type {
        BuildingType::Core => Colors::PRIMARY,
        BuildingType::Drill => Colors::ACCENT,
        BuildingType::Conduit => Colors::SECONDARY,
        BuildingType::Bridge => Colors::PRIMARY_SOFT,
        BuildingType::PowerNode => Colors::WARNING,
        BuildingType::WindTurbine => Color::new(0.5, 0.8, 0.5, 1.0),
        BuildingType::ServerBank => Color::new(0.3, 0.5, 0.8, 1.0),
    };

    if powered {
        base
    } else {
        Color::new(base.r * 0.4, base.g * 0.4, base.b * 0.4, 1.0)
    }
}

/// Determine current Core evolution stage based on progress
fn core_stage(state: &PlanetState) -> u8 {
    let growth = state.time_played as f32 + (state.resources.minerals + state.resources.data) * 0.5;

    if growth < 120.0 {
        0
    } else if growth < 300.0 {
        1
    } else if growth < 600.0 {
        2
    } else {
        3
    }
}

/// Draw evolved Core visuals
fn draw_core_visual(px: f32, py: f32, size: f32, state: &PlanetState) {
    let stage = core_stage(state);
    let center_x = px + size * 0.5;
    let center_y = py + size * 0.5;
    let pulse = ((state.time_played as f32) * 2.0).sin().abs();

    let base_margin = 2.0;
    let base_color = Colors::PRIMARY;
    draw_rectangle(
        px + base_margin,
        py + base_margin,
        size - base_margin * 2.0 - 1.0,
        size - base_margin * 2.0 - 1.0,
        base_color,
    );

    let core_radius = 3.5 + stage as f32 * 1.4;
    let core_alpha = 0.55 + pulse * 0.25;
    draw_circle(
        center_x,
        center_y,
        core_radius,
        Color::new(base_color.r, base_color.g, base_color.b, core_alpha),
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
            Color::new(base_color.r, base_color.g, base_color.b, glow_alpha),
        );
    }

    draw_text("C", center_x - 4.0, center_y + 5.0, 14.0, Colors::BACKGROUND);
}

/// Render the planetary grid view
pub fn render_planetary_view(state: &mut PlanetState, show_fps: bool) -> PlanetaryAction {
    clear_background(Colors::BACKGROUND);

    let screen_w = screen_width();
    let screen_h = screen_height();
    let pulse = ((state.time_played as f32) * 2.5).sin().abs();
    let time = state.time_played as f32;

    let (mouse_x, mouse_y) = mouse_position();
    let hovered_pos = screen_to_grid(mouse_x, mouse_y);

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
            let color = terrain_color(tile.terrain, tile.revealed);
            draw_rectangle(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, color);

            // Draw harvestable indicator
            if tile.revealed && tile.terrain.is_harvestable() && tile.building.is_none() {
                let indicator_color = match tile.terrain {
                    TerrainType::Mountain => Color::new(0.6, 0.5, 0.3, 0.5),
                    TerrainType::Forest => Color::new(0.2, 0.5, 0.2, 0.5),
                    _ => Color::new(1.0, 1.0, 1.0, 0.3),
                };
                draw_rectangle_lines(px + 2.0, py + 2.0, TILE_SIZE - 5.0, TILE_SIZE - 5.0, 1.0, indicator_color);
            }

            // Draw building if present
            if let Some(ref building) = tile.building {
                if building.building_type == BuildingType::Core {
                    draw_core_visual(px, py, TILE_SIZE, state);
                } else {
                    let bcolor = building_color(building.building_type, building.powered);
                    let margin = 3.0;
                    draw_rectangle(
                        px + margin,
                        py + margin,
                        TILE_SIZE - margin * 2.0 - 1.0,
                        TILE_SIZE - margin * 2.0 - 1.0,
                        bcolor,
                    );

                    let letter = match building.building_type {
                        BuildingType::Core => "C",
                        BuildingType::Drill => "D",
                        BuildingType::Conduit => "=",
                        BuildingType::Bridge => "#",
                        BuildingType::PowerNode => "P",
                        BuildingType::WindTurbine => "W",
                        BuildingType::ServerBank => "S",
                    };
                    let text_color = if building.powered { Colors::BACKGROUND } else { Colors::TEXT_DIM };
                    draw_text(letter, px + 8.0, py + 18.0, 16.0, text_color);
                }

                // Unpowered indicator
                if !building.powered && building.building_type != BuildingType::Core {
                    draw_text("!", px + 18.0, py + 10.0, 12.0, Colors::ERROR);
                }
            }

            if tile.bridge {
                let cross_color = Color::new(0.8, 0.8, 0.9, 0.8);
                draw_line(px + 4.0, py + 4.0, px + TILE_SIZE - 6.0, py + TILE_SIZE - 6.0, 1.0, cross_color);
                draw_line(px + TILE_SIZE - 6.0, py + 4.0, px + 4.0, py + TILE_SIZE - 6.0, 1.0, cross_color);
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
        let drone_x = dx + frac_x * TILE_SIZE + TILE_SIZE / 2.0 - 4.0;
        let drone_y = dy + frac_y * TILE_SIZE + TILE_SIZE / 2.0 - 4.0;

        let drone_color = match drone.state {
            DroneState::Idle => Colors::SECONDARY,
            DroneState::MovingToCore => Colors::SUCCESS,
            DroneState::MovingToDrill => Colors::WARNING,
            DroneState::Delivering => Colors::PRIMARY,
            DroneState::Error => Colors::ERROR,
        };

        draw_circle(drone_x, drone_y, 4.0, drone_color);

        if drone.carrying > 0.0 {
            draw_circle(drone_x, drone_y - 6.0, 2.0, Colors::ACCENT);
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
    let ui_research = draw_ui_panels(state, screen_w, screen_h, hovered_pos, show_fps);

    // Handle input
    if ui_research {
        PlanetaryAction::OpenResearch
    } else {
        handle_input(state, hovered_pos)
    }
}

/// Draw all UI panels
fn draw_ui_panels(state: &mut PlanetState, screen_w: f32, _screen_h: f32, hovered_pos: Option<GridPos>, show_fps: bool) -> bool {
    let mut research_clicked = false;
    // Top bar
    draw_panel(0.0, 0.0, screen_w, 50.0);
    draw_text(&state.name, 20.0, 35.0, Dimensions::FONT_SIZE_LARGE, Colors::PRIMARY);

    // Power indicator
    let power_color = if state.power_balance >= 0.0 { Colors::SUCCESS } else { Colors::ERROR };
    let power_str = format!("Power: {:+.0}", state.power_balance);
    draw_text(&power_str, 200.0, 35.0, Dimensions::FONT_SIZE_NORMAL, power_color);

    // Time played
    let time_str = format!("Time: {:.0}s", state.time_played);
    draw_text(&time_str, screen_w - 150.0, 35.0, Dimensions::FONT_SIZE_NORMAL, Colors::TEXT_DIM);

    // Research button (visible constraint)
    if draw_button(screen_w - 280.0, 6.0, 110.0, "Research") {
        research_clicked = true;
    }

    // Battery status
    let (hours, minutes) = state.battery_time_left();
    let battery_str = format!("Battery: {}h {}m", hours, minutes);
    draw_text(&battery_str, screen_w - 320.0, 35.0, Dimensions::FONT_SIZE_NORMAL, Colors::PRIMARY_SOFT);
    if state.battery_seconds <= 0.0 {
        draw_text("HIBERNATION", screen_w - 460.0, 35.0, Dimensions::FONT_SIZE_NORMAL, Colors::WARNING);
    }

    if show_fps {
        let fps = get_fps();
        draw_text(&format!("FPS: {}", fps), screen_w - 110.0, 18.0, 14.0, Colors::TEXT_DIM);
    }

    // Offline progress banner
    if state.offline_notice_timer > 0.0 && state.last_offline_seconds > 0.0 {
        let (off_h, off_m) = format_hours_minutes(state.last_offline_seconds);
        let (sim_h, sim_m) = format_hours_minutes(state.last_offline_simulated);
        let banner_w = 420.0;
        let banner_h = 38.0;
        let banner_x = (screen_w - banner_w) * 0.5;
        let banner_y = 55.0;
        draw_panel(banner_x, banner_y, banner_w, banner_h);
        let banner_text = format!(
            "Offline: {}h {}m | Simulated: {}h {}m",
            off_h, off_m, sim_h, sim_m
        );
        draw_text(&banner_text, banner_x + 16.0, banner_y + 25.0, 16.0, Colors::SUCCESS);
    }

    // Resource panel
    draw_panel(10.0, 60.0, 180.0, 170.0);
    draw_text("Resources", 20.0, 85.0, Dimensions::FONT_SIZE_NORMAL, Colors::TEXT);
    draw_resource(20.0, 115.0, "Energy", state.resources.energy, Colors::WARNING);
    draw_resource(20.0, 140.0, "Minerals", state.resources.minerals, Colors::ACCENT);
    draw_resource(20.0, 165.0, "Data", state.resources.data, Colors::PRIMARY);
    draw_resource(20.0, 190.0, "Biomass", state.resources.biomass, Colors::SUCCESS);

    let drone_count = state.drones.total_count();
    draw_text(&format!("Drones: {}", drone_count), 20.0, 215.0, 14.0, Colors::TEXT_DIM);

    // Achievements panel
    let (achieved, total) = state.achievements_progress();
    let ach_panel_w = 240.0;
    let ach_panel_h = 140.0;
    let ach_panel_x = screen_w - ach_panel_w - 20.0;
    let ach_panel_y = 60.0;
    draw_panel(ach_panel_x, ach_panel_y, ach_panel_w, ach_panel_h);
    draw_text(
        &format!("Achievements {}/{}", achieved, total),
        ach_panel_x + 12.0,
        ach_panel_y + 24.0,
        16.0,
        Colors::PRIMARY,
    );
    let mut ay = ach_panel_y + 48.0;
    for ach in state.achievements.iter() {
        let mark = if ach.achieved { "[x]" } else { "[ ]" };
        let text = format!("{} {}", mark, ach.name);
        let color = if ach.achieved { Colors::SUCCESS } else { Colors::TEXT_DIM };
        draw_text(&text, ach_panel_x + 12.0, ay, 13.0, color);
        ay += 18.0;
    }

    // Building toolbar
    draw_panel(10.0, 240.0, 180.0, 200.0);
    draw_text("Buildings", 20.0, 265.0, Dimensions::FONT_SIZE_NORMAL, Colors::TEXT);

    let buildings = [
        BuildingType::Drill,
        BuildingType::Conduit,
        BuildingType::Bridge,
        BuildingType::PowerNode,
        BuildingType::WindTurbine,
        BuildingType::ServerBank,
    ];

    let mut y = 285.0;
    for building in buildings {
        let (mineral_cost, energy_cost) = building.cost();
        let can_afford = state.resources.can_afford(mineral_cost, energy_cost);
        let is_selected = state.selected_building == Some(building);

        let label = format!(
            "[{}] {} ({}/{})",
            building.hotkey().unwrap_or(' '),
            building.name(),
            mineral_cost as i32,
            energy_cost as i32
        );

        let color = if is_selected {
            Colors::PRIMARY
        } else if can_afford {
            Colors::TEXT
        } else {
            Colors::TEXT_DIM
        };

        draw_text(&label, 20.0, y, 14.0, color);
        y += 22.0;
    }

    draw_text("[H] Harvest terrain", 20.0, y + 10.0, 12.0, Colors::TEXT_DIM);
    draw_text("Drag to place | F1=Help | ESC=Menu R=Research", 20.0, y + 25.0, 12.0, Colors::TEXT_DIM);

    // Tooltip for hovered tile
    let display_pos = hovered_pos.or(state.selected_tile);
    if let Some(pos) = display_pos {
        if let Some(tile) = state.grid.get(pos) {
            if tile.revealed {
                let tooltip_y = 450.0;
                draw_panel(10.0, tooltip_y, 180.0, 120.0);

                draw_text(tile.terrain.name(), 20.0, tooltip_y + 25.0, 16.0, Colors::TEXT);

                if let Some(ref building) = tile.building {
                    let status = if building.powered { "Powered" } else { "No Power!" };
                    let status_color = if building.powered { Colors::SUCCESS } else { Colors::ERROR };
                    draw_text(building.building_type.name(), 20.0, tooltip_y + 45.0, 14.0, Colors::TEXT_DIM);
                    draw_text(status, 20.0, tooltip_y + 62.0, 14.0, status_color);
                    if tile.bridge {
                        draw_text("Bridge: crossing enabled", 20.0, tooltip_y + 80.0, 11.0, Colors::PRIMARY_SOFT);
                    }

                    if building.building_type != BuildingType::Core {
                        let refund_ratio = 0.5;
                        let (mineral_cost, energy_cost) = building.building_type.cost();
                        let refund_text = format!(
                            "Sell: +{}M +{}E",
                            (mineral_cost * refund_ratio) as i32,
                            (energy_cost * refund_ratio) as i32
                        );
                        draw_text(&refund_text, 20.0, tooltip_y + 98.0, 12.0, Colors::TEXT_DIM);
                        if draw_button(120.0, tooltip_y + 92.0, 60.0, "Sell") {
                            state.try_sell_building(pos);
                        }
                    }
                } else if tile.terrain.is_harvestable() {
                    let (minerals, biomass) = tile.terrain.harvest_rewards();
                    let reward = if minerals > 0.0 {
                        format!("[H] +{} minerals", minerals as i32)
                    } else {
                        format!("[H] +{} biomass", biomass as i32)
                    };
                    draw_text(&reward, 20.0, tooltip_y + 45.0, 14.0, Colors::ACCENT);
                    if let Some(bonus) = tile.terrain.preservation_bonus() {
                        draw_text(&format!("Keep: {}", bonus), 20.0, tooltip_y + 62.0, 11.0, Colors::SUCCESS);
                    }
                }
            }
        }
    }

    // Help overlay
    if state.show_help {
        let help_w = 360.0;
        let help_h = 170.0;
        let help_x = screen_w - help_w - 20.0;
        let help_y = 70.0;
        draw_panel(help_x, help_y, help_w, help_h);
        draw_text("Help & Controls", help_x + 16.0, help_y + 28.0, 18.0, Colors::PRIMARY);
        draw_text("Left Click / Drag: Place building", help_x + 16.0, help_y + 55.0, 14.0, Colors::TEXT);
        draw_text("Right Click: Cancel selection / Harvest", help_x + 16.0, help_y + 75.0, 14.0, Colors::TEXT);
        draw_text("H: Harvest terrain", help_x + 16.0, help_y + 95.0, 14.0, Colors::TEXT);
        draw_text("R: Research  |  M: Map", help_x + 16.0, help_y + 115.0, 14.0, Colors::TEXT);
        draw_text("1-5: Select buildings", help_x + 16.0, help_y + 135.0, 14.0, Colors::TEXT);
        draw_text("F1: Toggle help", help_x + 16.0, help_y + 155.0, 14.0, Colors::TEXT_DIM);
    }

    research_clicked
}

/// Handle keyboard and mouse input
fn handle_input(state: &mut PlanetState, hovered_pos: Option<GridPos>) -> PlanetaryAction {
    // Building hotkeys
    if is_key_pressed(KeyCode::Key1) {
        state.select_building(BuildingType::Drill);
    }
    if is_key_pressed(KeyCode::Key2) {
        state.select_building(BuildingType::Conduit);
    }
    if is_key_pressed(KeyCode::Key6) {
        state.select_building(BuildingType::Bridge);
    }
    if is_key_pressed(KeyCode::Key3) {
        state.select_building(BuildingType::PowerNode);
    }
    if is_key_pressed(KeyCode::Key4) {
        state.select_building(BuildingType::WindTurbine);
    }
    if is_key_pressed(KeyCode::Key5) {
        state.select_building(BuildingType::ServerBank);
    }

    if is_key_pressed(KeyCode::F1) {
        state.show_help = !state.show_help;
    }

    // Harvest terrain with H key
    if is_key_pressed(KeyCode::H) {
        if let Some(pos) = hovered_pos {
            state.try_harvest_terrain(pos);
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
