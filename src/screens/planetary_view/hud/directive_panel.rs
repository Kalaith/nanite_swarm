//! Right sidebar: directive / tutorial panel

use crate::data::UiTheme;
use crate::directives::Directive;
use crate::state::PlanetState;
use crate::ui::{draw_hud_panel, draw_status_row};
use macroquad_toolkit::ui::draw_ui_text;

use super::super::format::fit_text_to_width;
use super::{PanelColors, RightStackLayout};

pub(super) fn draw(
    state: &PlanetState,
    directive: &Directive,
    theme: &UiTheme,
    colors: &PanelColors,
    right: &RightStackLayout,
) {
    let text = colors.text;
    let dim = colors.dim;
    let primary_soft = colors.primary_soft;
    let success = colors.success;
    let warning = colors.warning;
    let right_x = right.right_x;
    let right_w = right.right_w;
    let directive_y = right.directive.y;
    let directive_h = right.directive.h;

    draw_hud_panel(theme, right.directive, Some("TUTORIAL: EXPAND & AUTOMATE"));
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
}
