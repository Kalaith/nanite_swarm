//! Main menu screen

use macroquad::prelude::*;
use crate::ui::{Colors, Dimensions, draw_button, draw_panel};

/// Actions that can be taken from the main menu
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    None,
    NewGame,
    Continue,
    Settings,
    Quit,
}

/// Render the main menu and return any action taken
pub fn render_main_menu(has_save: bool) -> MenuAction {
    clear_background(Colors::BACKGROUND);

    let screen_w = screen_width();
    let screen_h = screen_height();

    // Title
    let title = "NANITE SWARM";
    let title_size = measure_text(title, None, 48, 1.0);
    draw_text(
        title,
        (screen_w - title_size.width) / 2.0,
        screen_h * 0.25,
        48.0,
        Colors::PRIMARY,
    );

    // Subtitle
    let subtitle = "Consume. Evolve. Expand.";
    let sub_size = measure_text(subtitle, None, Dimensions::FONT_SIZE_NORMAL as u16, 1.0);
    draw_text(
        subtitle,
        (screen_w - sub_size.width) / 2.0,
        screen_h * 0.25 + 40.0,
        Dimensions::FONT_SIZE_NORMAL,
        Colors::TEXT_DIM,
    );

    // Menu panel
    let panel_w = 300.0;
    let panel_h = 220.0;
    let panel_x = (screen_w - panel_w) / 2.0;
    let panel_y = screen_h * 0.4;
    draw_panel(panel_x, panel_y, panel_w, panel_h);

    // Buttons
    let btn_w = panel_w - 40.0;
    let btn_x = panel_x + 20.0;
    let mut btn_y = panel_y + 20.0;
    let btn_spacing = 50.0;

    if draw_button(btn_x, btn_y, btn_w, "New Game") {
        return MenuAction::NewGame;
    }
    btn_y += btn_spacing;

    if has_save && draw_button(btn_x, btn_y, btn_w, "Continue") {
        return MenuAction::Continue;
    }
    btn_y += btn_spacing;

    if draw_button(btn_x, btn_y, btn_w, "Settings") {
        return MenuAction::Settings;
    }
    btn_y += btn_spacing;

    if draw_button(btn_x, btn_y, btn_w, "Quit") {
        return MenuAction::Quit;
    }

    MenuAction::None
}
