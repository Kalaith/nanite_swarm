//! Main grid gameplay screen

use macroquad::prelude::*;
use crate::state::PlanetState;
use crate::engine::{GridPos, TerrainType, BuildingType, DroneState};
use crate::ui::{Colors, Dimensions, draw_panel, draw_resource};

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

/// Render the planetary grid view
pub fn render_planetary_view(state: &mut PlanetState) -> PlanetaryAction {
    clear_background(Colors::BACKGROUND);

    let screen_w = screen_width();
    let screen_h = screen_height();

    let (mouse_x, mouse_y) = mouse_position();
    let hovered_pos = screen_to_grid(mouse_x, mouse_y);

    // Draw grid
    for (pos, tile) in state.grid.iter_tiles() {
        let (px, py) = grid_to_screen(pos);

        if px > screen_w || py > screen_h || px + TILE_SIZE < 0.0 || py + TILE_SIZE < 0.0 {
            continue;
        }

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
                BuildingType::PowerNode => "P",
                BuildingType::WindTurbine => "W",
                BuildingType::ServerBank => "S",
            };
            let text_color = if building.powered { Colors::BACKGROUND } else { Colors::TEXT_DIM };
            draw_text(letter, px + 8.0, py + 18.0, 16.0, text_color);

            // Unpowered indicator
            if !building.powered && building.building_type != BuildingType::Core {
                draw_text("!", px + 18.0, py + 10.0, 12.0, Colors::ERROR);
            }
        }

        // Draw hover highlight
        if let Some(hover) = hovered_pos {
            if hover == pos && tile.revealed {
                draw_rectangle_lines(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, 2.0, Colors::PRIMARY);

                // Show placement preview
                if let Some(building_type) = state.selected_building {
                    if state.grid.can_place_building(pos, building_type) {
                        let preview_color = Color::new(0.0, 0.8, 1.0, 0.3);
                        draw_rectangle(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, preview_color);
                    }
                }

                // Show harvest preview
                if state.can_harvest(pos) {
                    let harvest_color = Color::new(1.0, 0.5, 0.0, 0.3);
                    draw_rectangle(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, harvest_color);
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

    // Draw UI panels
    draw_ui_panels(state, screen_w, screen_h, hovered_pos);

    // Handle input
    handle_input(state, hovered_pos)
}

/// Draw all UI panels
fn draw_ui_panels(state: &PlanetState, screen_w: f32, _screen_h: f32, hovered_pos: Option<GridPos>) {
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

    // Resource panel
    draw_panel(10.0, 60.0, 180.0, 170.0);
    draw_text("Resources", 20.0, 85.0, Dimensions::FONT_SIZE_NORMAL, Colors::TEXT);
    draw_resource(20.0, 115.0, "Energy", state.resources.energy, Colors::WARNING);
    draw_resource(20.0, 140.0, "Minerals", state.resources.minerals, Colors::ACCENT);
    draw_resource(20.0, 165.0, "Data", state.resources.data, Colors::PRIMARY);
    draw_resource(20.0, 190.0, "Biomass", state.resources.biomass, Colors::SUCCESS);

    let drone_count = state.drones.total_count();
    draw_text(&format!("Drones: {}", drone_count), 20.0, 215.0, 14.0, Colors::TEXT_DIM);

    // Building toolbar
    draw_panel(10.0, 240.0, 180.0, 200.0);
    draw_text("Buildings", 20.0, 265.0, Dimensions::FONT_SIZE_NORMAL, Colors::TEXT);

    let buildings = [
        BuildingType::Drill,
        BuildingType::Conduit,
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
    draw_text("ESC=Menu R=Research", 20.0, y + 25.0, 12.0, Colors::TEXT_DIM);

    // Tooltip for hovered tile
    if let Some(pos) = hovered_pos {
        if let Some(tile) = state.grid.get(pos) {
            if tile.revealed {
                let tooltip_y = 450.0;
                draw_panel(10.0, tooltip_y, 180.0, 80.0);

                draw_text(tile.terrain.name(), 20.0, tooltip_y + 25.0, 16.0, Colors::TEXT);

                if let Some(ref building) = tile.building {
                    let status = if building.powered { "Powered" } else { "No Power!" };
                    let status_color = if building.powered { Colors::SUCCESS } else { Colors::ERROR };
                    draw_text(building.building_type.name(), 20.0, tooltip_y + 45.0, 14.0, Colors::TEXT_DIM);
                    draw_text(status, 20.0, tooltip_y + 62.0, 14.0, status_color);
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
    if is_key_pressed(KeyCode::Key3) {
        state.select_building(BuildingType::PowerNode);
    }
    if is_key_pressed(KeyCode::Key4) {
        state.select_building(BuildingType::WindTurbine);
    }
    if is_key_pressed(KeyCode::Key5) {
        state.select_building(BuildingType::ServerBank);
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
            state.try_place_building(pos);
        }
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
    }

    // Navigation keys
    if is_key_pressed(KeyCode::Escape) {
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
