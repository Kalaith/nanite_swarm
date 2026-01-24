//! Solar system overview screen

use macroquad::prelude::*;
use crate::ui::{Colors, Dimensions, draw_panel};

/// Planet data for the solar system
#[derive(Debug, Clone)]
pub struct PlanetInfo {
    pub name: &'static str,
    pub orbit_radius: f32,
    pub color: Color,
    pub size: f32,
    pub description: &'static str,
    pub difficulty: &'static str,
}

impl PlanetInfo {
    const fn new(
        name: &'static str,
        orbit_radius: f32,
        color: Color,
        size: f32,
        description: &'static str,
        difficulty: &'static str,
    ) -> Self {
        Self {
            name,
            orbit_radius,
            color,
            size,
            description,
            difficulty,
        }
    }
}

/// Get the default planet data
pub fn get_planets() -> [PlanetInfo; 5] {
    [
        PlanetInfo::new(
            "Mercury", 80.0,
            Color::new(0.6, 0.6, 0.6, 1.0), 8.0,
            "Scorched world rich in metals",
            "Hard - Extreme temperatures",
        ),
        PlanetInfo::new(
            "Venus", 120.0,
            Color::new(0.9, 0.7, 0.3, 1.0), 11.0,
            "Thick atmosphere, volcanic",
            "Hard - Corrosive atmosphere",
        ),
        PlanetInfo::new(
            "Mars", 170.0,
            Color::new(0.8, 0.4, 0.3, 1.0), 10.0,
            "Red planet with polar ice caps",
            "Normal - Starting world",
        ),
        PlanetInfo::new(
            "Jupiter", 240.0,
            Color::new(0.8, 0.6, 0.4, 1.0), 18.0,
            "Gas giant with many moons",
            "Medium - Moon colonization",
        ),
        PlanetInfo::new(
            "Saturn", 310.0,
            Color::new(0.9, 0.8, 0.5, 1.0), 16.0,
            "Ringed giant, resource-rich moons",
            "Medium - Distant orbit",
        ),
    ]
}

/// Actions from the interplanetary view
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterplanetaryAction {
    None,
    Close,
    SelectPlanet(usize),
    LaunchMassDriver(usize),
}

/// Render the interplanetary solar system view
pub fn render_interplanetary_view(
    current_planet: usize,
    has_mass_driver: bool,
    colonized_planets: &[bool],
) -> InterplanetaryAction {
    clear_background(Colors::BACKGROUND);

    let screen_w = screen_width();
    let screen_h = screen_height();
    let (mouse_x, mouse_y) = mouse_position();

    // Header
    draw_panel(0.0, 0.0, screen_w, 50.0);
    draw_text("Solar System", 20.0, 35.0, Dimensions::FONT_SIZE_LARGE, Colors::PRIMARY);

    if has_mass_driver {
        draw_text(
            "Mass Driver: ONLINE",
            screen_w - 200.0, 35.0,
            Dimensions::FONT_SIZE_NORMAL,
            Colors::SUCCESS,
        );
    } else {
        draw_text(
            "Mass Driver: OFFLINE",
            screen_w - 200.0, 35.0,
            Dimensions::FONT_SIZE_NORMAL,
            Colors::TEXT_DIM,
        );
    }

    // Draw sun at center
    let center_x = screen_w / 2.0;
    let center_y = screen_h / 2.0;

    // Sun glow
    draw_circle(center_x, center_y, 50.0, Color::new(1.0, 0.8, 0.2, 0.2));
    draw_circle(center_x, center_y, 45.0, Color::new(1.0, 0.7, 0.1, 0.3));
    draw_circle(center_x, center_y, 40.0, Colors::WARNING);

    let planets = get_planets();
    let mut hovered_planet: Option<usize> = None;
    let mut action = InterplanetaryAction::None;

    for (i, planet) in planets.iter().enumerate() {
        // Draw orbit
        let orbit_color = if i == current_planet {
            Color::new(0.0, 0.85, 1.0, 0.3)
        } else {
            Colors::SECONDARY
        };
        draw_circle_lines(center_x, center_y, planet.orbit_radius, 1.0, orbit_color);

        // Calculate planet position
        let angle = (i as f32) * 1.2 + get_time() as f32 * 0.02 * (1.0 / (i as f32 + 1.0));
        let px = center_x + planet.orbit_radius * angle.cos();
        let py = center_y + planet.orbit_radius * angle.sin();

        // Check hover
        let dist = ((mouse_x - px).powi(2) + (mouse_y - py).powi(2)).sqrt();
        let is_hovered = dist < planet.size + 5.0;
        if is_hovered {
            hovered_planet = Some(i);
        }

        let planet_size = if i == current_planet { planet.size * 1.3 } else { planet.size };
        let is_colonized = colonized_planets.get(i).copied().unwrap_or(false);

        // Draw colonization glow
        if is_colonized {
            draw_circle(px, py, planet_size + 6.0, Color::new(0.0, 1.0, 0.5, 0.2));
        }

        // Draw planet
        draw_circle(px, py, planet_size, planet.color);

        // Hover highlight
        if is_hovered {
            draw_circle_lines(px, py, planet_size + 4.0, 2.0, Colors::PRIMARY);
        }

        // Current planet indicator
        if i == current_planet {
            draw_circle_lines(px, py, planet_size + 8.0, 2.0, Colors::SUCCESS);
        }

        // Draw name
        let name_color = if is_colonized { Colors::SUCCESS } else { Colors::TEXT_DIM };
        draw_text(planet.name, px - 20.0, py + planet_size + 15.0, 12.0, name_color);
    }

    // Info panel for hovered planet
    if let Some(i) = hovered_planet {
        let planet = &planets[i];
        let is_colonized = colonized_planets.get(i).copied().unwrap_or(false);
        let is_current = i == current_planet;

        let panel_x = 10.0;
        let panel_y = screen_h - 180.0;
        draw_panel(panel_x, panel_y, 280.0, 170.0);

        draw_text(planet.name, panel_x + 15.0, panel_y + 30.0, 24.0, planet.color);
        draw_text(planet.description, panel_x + 15.0, panel_y + 55.0, 14.0, Colors::TEXT);
        draw_text(planet.difficulty, panel_x + 15.0, panel_y + 75.0, 12.0, Colors::TEXT_DIM);

        if is_colonized {
            draw_text("Status: COLONIZED", panel_x + 15.0, panel_y + 100.0, 14.0, Colors::SUCCESS);
        } else {
            draw_text("Status: Unexplored", panel_x + 15.0, panel_y + 100.0, 14.0, Colors::WARNING);
        }

        if is_current {
            draw_text("(Current Location)", panel_x + 15.0, panel_y + 120.0, 12.0, Colors::PRIMARY);
        } else if has_mass_driver && !is_colonized {
            draw_text("Click to launch probe", panel_x + 15.0, panel_y + 140.0, 14.0, Colors::SUCCESS);
        } else if !has_mass_driver && !is_colonized {
            draw_text("Requires: Mass Driver", panel_x + 15.0, panel_y + 140.0, 12.0, Colors::ERROR);
        } else if is_colonized && !is_current {
            draw_text("Click to travel", panel_x + 15.0, panel_y + 140.0, 14.0, Colors::PRIMARY);
        }

        // Handle click
        if is_mouse_button_pressed(MouseButton::Left) {
            if is_colonized && !is_current {
                action = InterplanetaryAction::SelectPlanet(i);
            } else if has_mass_driver && !is_colonized {
                action = InterplanetaryAction::LaunchMassDriver(i);
            }
        }
    }

    // Legend
    draw_panel(screen_w - 180.0, screen_h - 100.0, 170.0, 90.0);
    draw_circle(screen_w - 165.0, screen_h - 75.0, 6.0, Colors::SUCCESS);
    draw_text("Colonized", screen_w - 150.0, screen_h - 70.0, 12.0, Colors::TEXT_DIM);
    draw_circle(screen_w - 165.0, screen_h - 55.0, 6.0, Colors::WARNING);
    draw_text("Unexplored", screen_w - 150.0, screen_h - 50.0, 12.0, Colors::TEXT_DIM);
    draw_circle(screen_w - 165.0, screen_h - 35.0, 6.0, Colors::PRIMARY);
    draw_text("Current", screen_w - 150.0, screen_h - 30.0, 12.0, Colors::TEXT_DIM);

    // Instructions
    draw_text(
        "Press ESC to return | M to toggle map",
        20.0,
        screen_h - 20.0,
        Dimensions::FONT_SIZE_SMALL,
        Colors::TEXT_DIM,
    );

    if is_key_pressed(KeyCode::Escape) {
        return InterplanetaryAction::Close;
    }

    action
}
