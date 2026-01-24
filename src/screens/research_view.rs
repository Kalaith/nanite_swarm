//! Neural network research interface

use macroquad::prelude::*;
use crate::ui::{Colors, Dimensions, draw_panel};
use crate::engine::{ResearchTree, ResearchState};

const NODE_RADIUS: f32 = 25.0;
const GRID_SCALE: f32 = 100.0;

/// Actions from the research view
#[derive(Debug, Clone, PartialEq)]
pub enum ResearchAction {
    None,
    Close,
    StartResearch(String),
}

/// Render the research neural network view
pub fn render_research_view(
    research_state: &ResearchState,
    research_tree: &ResearchTree,
    data_available: f32,
) -> ResearchAction {
    clear_background(Colors::BACKGROUND);

    let screen_w = screen_width();
    let screen_h = screen_height();
    let center_x = screen_w / 2.0;
    let center_y = screen_h / 2.0 - 50.0;
    let pulse = (get_time() as f32 * 2.0).sin().abs();

    // Header
    draw_panel(0.0, 0.0, screen_w, 50.0);
    draw_text("Neural Network", 20.0, 35.0, Dimensions::FONT_SIZE_LARGE, Colors::PRIMARY);
    draw_text(
        &format!("Data: {:.0}", data_available),
        screen_w - 150.0,
        35.0,
        Dimensions::FONT_SIZE_NORMAL,
        Colors::PRIMARY,
    );

    if let Some(current) = &research_state.current_research {
        if let Some(node) = research_tree.get_node(current) {
            let progress = research_state.research_progress.min(node.data_cost);
            let pct = if node.data_cost > 0.0 { progress / node.data_cost } else { 1.0 };
            let bar_w = 220.0;
            let bar_x = screen_w * 0.5 - bar_w * 0.5;
            let bar_y = 18.0;
            draw_rectangle(bar_x, bar_y, bar_w, 10.0, Colors::SURFACE);
            draw_rectangle(bar_x, bar_y, bar_w * pct, 10.0, Colors::PRIMARY);
            draw_text(&format!("Research: {}", node.name), bar_x, bar_y - 4.0, 12.0, Colors::TEXT_DIM);
        }
    }

    // Get mouse position
    let (mouse_x, mouse_y) = mouse_position();
    let mut hovered_node: Option<&str> = None;

    // Draw connections first (behind nodes)
    for (from, to) in research_tree.get_connections() {
        let from_unlocked = research_state.is_unlocked(&from.id);
        let to_unlocked = research_state.is_unlocked(&to.id);

        let from_x = center_x + from.position.0 * GRID_SCALE;
        let from_y = center_y - from.position.1 * GRID_SCALE;
        let to_x = center_x + to.position.0 * GRID_SCALE;
        let to_y = center_y - to.position.1 * GRID_SCALE;

        let line_color = if from_unlocked && to_unlocked {
            Colors::PRIMARY
        } else if from_unlocked {
            Color::new(0.0, 0.5, 0.7, 0.8)
        } else {
            Colors::SECONDARY
        };

        draw_line(from_x, from_y, to_x, to_y, 2.0, line_color);
    }

    // Draw nodes
    for node in &research_tree.nodes {
        let node_x = center_x + node.position.0 * GRID_SCALE;
        let node_y = center_y - node.position.1 * GRID_SCALE;

        let is_unlocked = research_state.is_unlocked(&node.id);
        let can_select = research_tree.can_select(&node.id, &research_state.unlocked);
        let can_research_now = research_tree.can_research(&node.id, &research_state.unlocked, data_available);
        let is_current = research_state.current_research.as_ref() == Some(&node.id);

        // Check if mouse is hovering
        let dist = ((mouse_x - node_x).powi(2) + (mouse_y - node_y).powi(2)).sqrt();
        let is_hovered = dist < NODE_RADIUS;
        if is_hovered {
            hovered_node = Some(&node.id);
        }

        // Node colors
        let (fill_color, border_color) = if is_unlocked {
            (Colors::PRIMARY, Colors::PRIMARY)
        } else if is_current {
            (Color::new(0.0, 0.5, 0.7, 1.0), Colors::WARNING)
        } else if can_research_now {
            (Colors::SURFACE, Colors::SUCCESS)
        } else if can_select {
            (Colors::SURFACE, Colors::PRIMARY_SOFT)
        } else {
            (Colors::SURFACE, Colors::SECONDARY)
        };

        // Draw glow for unlocked nodes
        if is_unlocked {
            let glow_outer = NODE_RADIUS + 6.0 + pulse * 3.0;
            let glow_inner = NODE_RADIUS + 3.0 + pulse * 1.5;
            draw_circle(node_x, node_y, glow_outer, Color::new(0.0, 0.85, 1.0, 0.18 + pulse * 0.08));
            draw_circle(node_x, node_y, glow_inner, Color::new(0.0, 0.85, 1.0, 0.25 + pulse * 0.1));
        }

        // Draw node
        draw_circle(node_x, node_y, NODE_RADIUS, fill_color);
        draw_circle_lines(node_x, node_y, NODE_RADIUS, 2.0, border_color);

        // Hover effect
        if is_hovered {
            draw_circle_lines(node_x, node_y, NODE_RADIUS + 5.0 + pulse * 2.0, 2.0, Colors::PRIMARY);
        }

        // Draw abbreviated name
        let abbrev = &node.name[..node.name.len().min(6)];
        let text_size = measure_text(abbrev, None, 12, 1.0);
        let text_color = if is_unlocked { Colors::BACKGROUND } else { Colors::TEXT };
        draw_text(abbrev, node_x - text_size.width / 2.0, node_y + 4.0, 12.0, text_color);

        // Draw cost below if not unlocked
        if !is_unlocked && node.data_cost > 0.0 {
            let cost_str = format!("{:.0}", node.data_cost);
            let cost_color = if can_research_now { Colors::SUCCESS } else { Colors::TEXT_DIM };
            draw_text(&cost_str, node_x - 10.0, node_y + NODE_RADIUS + 15.0, 12.0, cost_color);
        }
    }

    // Draw tooltip for hovered node
    if let Some(node_id) = hovered_node {
        if let Some(node) = research_tree.get_node(node_id) {
            let tooltip_x = 10.0;
            let tooltip_y = screen_h - 140.0;
            draw_panel(tooltip_x, tooltip_y, 300.0, 130.0);

            draw_text(&node.name, tooltip_x + 15.0, tooltip_y + 30.0, 20.0, Colors::PRIMARY);
            draw_text(&node.description, tooltip_x + 15.0, tooltip_y + 55.0, 14.0, Colors::TEXT);

            if !research_state.is_unlocked(node_id) {
                let cost_text = format!("Cost: {:.0} Data", node.data_cost);
                draw_text(&cost_text, tooltip_x + 15.0, tooltip_y + 80.0, 14.0, Colors::ACCENT);

                if research_tree.can_select(node_id, &research_state.unlocked) {
                    if research_tree.can_research(node_id, &research_state.unlocked, data_available) {
                        draw_text("Click to research", tooltip_x + 15.0, tooltip_y + 100.0, 14.0, Colors::SUCCESS);
                    } else {
                        draw_text("Click to select (insufficient Data)", tooltip_x + 15.0, tooltip_y + 100.0, 12.0, Colors::WARNING);
                    }
                } else if !node.prerequisites.iter().all(|p| research_state.is_unlocked(p)) {
                    draw_text("Prerequisites not met", tooltip_x + 15.0, tooltip_y + 100.0, 14.0, Colors::ERROR);
                } else {
                    draw_text("Not enough Data", tooltip_x + 15.0, tooltip_y + 100.0, 14.0, Colors::WARNING);
                }
            } else {
                draw_text("UNLOCKED", tooltip_x + 15.0, tooltip_y + 80.0, 16.0, Colors::SUCCESS);
            }
        }
    }

    // Instructions
    draw_text(
        "Press ESC to return",
        20.0,
        screen_h - 20.0,
        Dimensions::FONT_SIZE_SMALL,
        Colors::TEXT_DIM,
    );

    // Handle input
    if is_key_pressed(KeyCode::Escape) {
        return ResearchAction::Close;
    }

    // Click to research
    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(node_id) = hovered_node {
            if research_tree.can_select(node_id, &research_state.unlocked) {
                return ResearchAction::StartResearch(node_id.to_string());
            }
        }
    }

    ResearchAction::None
}
