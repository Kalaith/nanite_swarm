//! Main grid gameplay screen

mod entity_render;
mod format;
mod hud;
mod input;
mod metrics;
mod terrain_render;

use crate::assets::GameTextures;
use crate::data::UiTheme;
use crate::directives::Directive;
use crate::state::PlanetState;
use crate::ui::color_from_rgba;
use macroquad::prelude::*;
use macroquad_toolkit::math::{lerp, pulse01_at};

use metrics::{is_cursor_over_ui, screen_to_grid, HudMetrics};

/// Actions from the planetary view
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlanetaryAction {
    None,
    OpenResearch,
    OpenInterplanetary,
    OpenMenu,
}

/// Render the planetary grid view
pub fn render_planetary_view(
    state: &mut PlanetState,
    textures: &GameTextures,
    directive: &Directive,
    theme: &UiTheme,
) -> PlanetaryAction {
    clear_background(color_from_rgba(&theme.colors.background));

    let screen_w = screen_width();
    let screen_h = screen_height();
    let metrics = HudMetrics::for_screen(theme, screen_w, screen_h);
    let pulse = pulse01_at(state.time_played, 2.5);
    let global_pulse = lerp(0.8, 1.0, pulse01_at(state.time_played, 2.0));
    let time = state.time_played as f32;

    let (mouse_x, mouse_y) = mouse_position();
    let hovered_pos = if is_cursor_over_ui(mouse_x, mouse_y, screen_w, screen_h, metrics) {
        None
    } else {
        screen_to_grid(mouse_x, mouse_y, metrics)
    };

    terrain_render::draw_planetary_background(screen_w, screen_h, time);
    terrain_render::draw_grid_tiles(state, textures, metrics, hovered_pos, pulse, global_pulse);
    entity_render::draw_drones(state, metrics, time);
    entity_render::draw_particles(state, metrics);

    let ui_action = hud::draw_ui_panels(
        state,
        screen_w,
        screen_h,
        hovered_pos,
        directive,
        textures,
        theme,
        metrics,
    );

    if ui_action != PlanetaryAction::None {
        ui_action
    } else {
        input::handle_input(state, hovered_pos)
    }
}
