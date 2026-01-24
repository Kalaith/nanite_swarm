//! Settings menu screen

use macroquad::prelude::*;
use crate::ui::{Colors, Dimensions, draw_button, draw_panel};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingsAction {
    None,
    Back,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub ui_scale: f32,
    pub show_fps: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            music_volume: 0.6,
            sfx_volume: 0.7,
            ui_scale: 1.0,
            show_fps: false,
        }
    }
}

/// Render the settings menu and return any action taken
pub fn render_settings_menu(settings: &mut Settings) -> SettingsAction {
    clear_background(Colors::BACKGROUND);

    let screen_w = screen_width();
    let screen_h = screen_height();

    let panel_w = 460.0;
    let panel_h = 320.0;
    let panel_x = (screen_w - panel_w) / 2.0;
    let panel_y = screen_h * 0.25;

    draw_panel(panel_x, panel_y, panel_w, panel_h);
    draw_text("Settings", panel_x + 20.0, panel_y + 34.0, 22.0, Colors::PRIMARY);

    let mut y = panel_y + 80.0;
    draw_text("Music Volume", panel_x + 20.0, y, 16.0, Colors::TEXT);
    draw_text(&format!("{:.0}%", settings.music_volume * 100.0), panel_x + 200.0, y, 16.0, Colors::PRIMARY_SOFT);
    if draw_button(panel_x + 280.0, y - 18.0, 36.0, "-") {
        settings.music_volume = (settings.music_volume - 0.1).max(0.0);
    }
    if draw_button(panel_x + 320.0, y - 18.0, 36.0, "+") {
        settings.music_volume = (settings.music_volume + 0.1).min(1.0);
    }
    y += 50.0;

    draw_text("SFX Volume", panel_x + 20.0, y, 16.0, Colors::TEXT);
    draw_text(&format!("{:.0}%", settings.sfx_volume * 100.0), panel_x + 200.0, y, 16.0, Colors::PRIMARY_SOFT);
    if draw_button(panel_x + 280.0, y - 18.0, 36.0, "-") {
        settings.sfx_volume = (settings.sfx_volume - 0.1).max(0.0);
    }
    if draw_button(panel_x + 320.0, y - 18.0, 36.0, "+") {
        settings.sfx_volume = (settings.sfx_volume + 0.1).min(1.0);
    }
    y += 50.0;

    draw_text("UI Scale", panel_x + 20.0, y, 16.0, Colors::TEXT);
    draw_text(&format!("{:.2}x", settings.ui_scale), panel_x + 200.0, y, 16.0, Colors::PRIMARY_SOFT);
    if draw_button(panel_x + 280.0, y - 18.0, 36.0, "-") {
        settings.ui_scale = (settings.ui_scale - 0.05).max(0.75);
    }
    if draw_button(panel_x + 320.0, y - 18.0, 36.0, "+") {
        settings.ui_scale = (settings.ui_scale + 0.05).min(1.5);
    }
    y += 50.0;

    let toggle_text = if settings.show_fps { "Show FPS: On" } else { "Show FPS: Off" };
    if draw_button(panel_x + 20.0, y - 18.0, 200.0, toggle_text) {
        settings.show_fps = !settings.show_fps;
    }

    draw_text(
        "Press ESC to return",
        panel_x + 20.0,
        panel_y + panel_h - 20.0,
        Dimensions::FONT_SIZE_SMALL,
        Colors::TEXT_DIM,
    );

    if is_key_pressed(KeyCode::Escape) {
        return SettingsAction::Back;
    }

    SettingsAction::None
}
