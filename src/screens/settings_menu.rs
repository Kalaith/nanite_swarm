//! Settings menu screen

use macroquad::prelude::*;
use crate::ui::{Colors, Dimensions, draw_button_sized, draw_panel};

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
    let header_height = 72.0;

    draw_panel(0.0, 0.0, screen_w, header_height);
    draw_text("Settings", 18.0, 30.0, 18.0, Colors::PRIMARY);
    draw_text("Audio & Display", 18.0, 52.0, 12.0, Colors::TEXT_DIM);

    if draw_button_sized(screen_w - 110.0, 18.0, 80.0, 34.0, "Back") {
        return SettingsAction::Back;
    }

    let panel_w = 320.0;
    let panel_h = 240.0;
    let panel_y = screen_h * 0.3;
    let audio_x = screen_w * 0.5 - panel_w - 20.0;
    let display_x = screen_w * 0.5 + 20.0;

    draw_panel(audio_x, panel_y, panel_w, panel_h);
    draw_text("Audio", audio_x + 16.0, panel_y + 28.0, 16.0, Colors::PRIMARY);

    let mut row_y = panel_y + 70.0;
    draw_text("Music Volume", audio_x + 16.0, row_y, 14.0, Colors::TEXT);
    draw_text(&format!("{:.0}%", settings.music_volume * 100.0), audio_x + 200.0, row_y, 14.0, Colors::PRIMARY_SOFT);
    if draw_button_sized(audio_x + 250.0, row_y - 18.0, 28.0, 26.0, "-") {
        settings.music_volume = (settings.music_volume - 0.1).max(0.0);
    }
    if draw_button_sized(audio_x + 284.0, row_y - 18.0, 28.0, 26.0, "+") {
        settings.music_volume = (settings.music_volume + 0.1).min(1.0);
    }
    row_y += 50.0;
    draw_text("SFX Volume", audio_x + 16.0, row_y, 14.0, Colors::TEXT);
    draw_text(&format!("{:.0}%", settings.sfx_volume * 100.0), audio_x + 200.0, row_y, 14.0, Colors::PRIMARY_SOFT);
    if draw_button_sized(audio_x + 250.0, row_y - 18.0, 28.0, 26.0, "-") {
        settings.sfx_volume = (settings.sfx_volume - 0.1).max(0.0);
    }
    if draw_button_sized(audio_x + 284.0, row_y - 18.0, 28.0, 26.0, "+") {
        settings.sfx_volume = (settings.sfx_volume + 0.1).min(1.0);
    }

    draw_panel(display_x, panel_y, panel_w, panel_h);
    draw_text("Display", display_x + 16.0, panel_y + 28.0, 16.0, Colors::PRIMARY);

    let mut display_row_y = panel_y + 70.0;
    draw_text("UI Scale", display_x + 16.0, display_row_y, 14.0, Colors::TEXT);
    draw_text(&format!("{:.2}x", settings.ui_scale), display_x + 200.0, display_row_y, 14.0, Colors::PRIMARY_SOFT);
    if draw_button_sized(display_x + 250.0, display_row_y - 18.0, 28.0, 26.0, "-") {
        settings.ui_scale = (settings.ui_scale - 0.05).max(0.75);
    }
    if draw_button_sized(display_x + 284.0, display_row_y - 18.0, 28.0, 26.0, "+") {
        settings.ui_scale = (settings.ui_scale + 0.05).min(1.5);
    }
    display_row_y += 50.0;

    let toggle_text = if settings.show_fps { "Show FPS: On" } else { "Show FPS: Off" };
    if draw_button_sized(display_x + 16.0, display_row_y - 16.0, 200.0, 30.0, toggle_text) {
        settings.show_fps = !settings.show_fps;
    }

    draw_text(
        "Press ESC to return",
        20.0,
        screen_h - 20.0,
        Dimensions::FONT_SIZE_SMALL,
        Colors::TEXT_DIM,
    );

    if is_key_pressed(KeyCode::Escape) {
        return SettingsAction::Back;
    }

    SettingsAction::None
}
