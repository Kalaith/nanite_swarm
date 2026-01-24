//! Game-specific UI widgets

use macroquad::prelude::*;
use super::core::{Colors, Dimensions};

/// Draw a styled button and return true if clicked
pub fn draw_button(x: f32, y: f32, width: f32, text: &str) -> bool {
    let height = Dimensions::BUTTON_HEIGHT;
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + width && mouse.1 >= y && mouse.1 <= y + height;

    let bg_color = if hovered { Colors::PRIMARY } else { Colors::SURFACE_DARK };
    let text_color = if hovered { Colors::BACKGROUND } else { Colors::TEXT };

    // Shadow
    draw_rectangle(x + 2.0, y + 3.0, width, height, Color::new(0.0, 0.0, 0.0, 0.35));

    draw_rectangle(x, y, width, height, bg_color);
    draw_rectangle_lines(x, y, width, height, 2.0, Colors::PANEL_BORDER);
    draw_rectangle(x + 2.0, y + 2.0, width - 4.0, 3.0, Color::new(1.0, 1.0, 1.0, 0.06));

    let text_size = measure_text(text, None, Dimensions::FONT_SIZE_NORMAL as u16, 1.0);
    draw_text(
        text,
        x + (width - text_size.width) / 2.0,
        y + (height + text_size.height) / 2.0 - 4.0,
        Dimensions::FONT_SIZE_NORMAL,
        text_color,
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

/// Draw a panel background
pub fn draw_panel(x: f32, y: f32, width: f32, height: f32) {
    // Shadow for depth
    draw_rectangle(x + 3.0, y + 4.0, width, height, Color::new(0.0, 0.0, 0.0, 0.3));

    draw_rectangle(x, y, width, height, Colors::SURFACE_DARK);
    draw_rectangle_lines(x, y, width, height, 1.0, Colors::PANEL_BORDER);
    draw_rectangle(x + 1.0, y + 1.0, width - 2.0, 2.0, Color::new(1.0, 1.0, 1.0, 0.04));
}

/// Draw resource display
pub fn draw_resource(x: f32, y: f32, label: &str, value: f32, color: Color) {
    draw_text(
        &format!("{}: {:.0}", label, value),
        x,
        y,
        Dimensions::FONT_SIZE_NORMAL,
        color,
    );
}
