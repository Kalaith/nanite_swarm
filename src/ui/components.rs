//! Game-specific UI widgets

use super::core::{Colors, Dimensions};
use macroquad::prelude::*;
use macroquad_toolkit::{input::is_hovered, ui::draw_text_centered_in_box};

/// Draw a styled button and return true if clicked
#[allow(dead_code)]
pub fn draw_button(x: f32, y: f32, width: f32, text: &str) -> bool {
    draw_button_sized(x, y, width, Dimensions::BUTTON_HEIGHT, text)
}

/// Draw a styled button with a custom size
pub fn draw_button_sized(x: f32, y: f32, width: f32, height: f32, text: &str) -> bool {
    let hovered = is_hovered(x, y, width, height);
    let pressed = hovered && is_mouse_button_down(MouseButton::Left);

    let base_color = if pressed {
        Colors::PRIMARY_SOFT
    } else if hovered {
        Colors::PRIMARY
    } else {
        Colors::SURFACE_DARK
    };
    let text_color = if hovered {
        Colors::BACKGROUND
    } else {
        Colors::TEXT
    };

    let surface = macroquad_toolkit::ui::SurfaceStyle::new(base_color)
        .with_shadow(vec2(2.0, 3.0), Color::new(0.0, 0.0, 0.0, 0.35))
        .with_border(2.0, Colors::PANEL_BORDER)
        .with_top_highlight(3.0, Color::new(1.0, 1.0, 1.0, 0.08));
    macroquad_toolkit::ui::draw_surface(Rect::new(x, y, width, height), &surface);

    let font_size = if height >= 38.0 {
        Dimensions::FONT_SIZE_NORMAL
    } else {
        Dimensions::FONT_SIZE_SMALL
    };
    draw_text_centered_in_box(
        text,
        x + 8.0,
        y,
        width - 16.0,
        height,
        font_size,
        text_color,
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

/// Draw a panel background
pub fn draw_panel(x: f32, y: f32, width: f32, height: f32) {
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(Colors::SURFACE)
        .with_shadow(vec2(3.0, 4.0), Color::new(0.0, 0.0, 0.0, 0.32))
        .with_header(6.0, Colors::SURFACE_DARK)
        .with_top_highlight(2.0, Color::new(1.0, 1.0, 1.0, 0.06))
        .with_border(1.0, Colors::PANEL_BORDER);
    macroquad_toolkit::ui::draw_surface(Rect::new(x, y, width, height), &surface);
}

/// Draw resource display
#[allow(dead_code)]
pub fn draw_resource(x: f32, y: f32, label: &str, value: f32, color: Color) {
    draw_text(
        &format!("{}: {:.0}", label, value),
        x,
        y,
        Dimensions::FONT_SIZE_NORMAL,
        color,
    );
}
