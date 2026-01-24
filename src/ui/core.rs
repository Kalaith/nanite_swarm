//! Color schemes and styling

#![allow(dead_code)]

use macroquad::prelude::*;

/// Color palette from PRD
pub struct Colors;

impl Colors {
    pub const PRIMARY: Color = Color::new(0.0, 0.851, 1.0, 1.0);      // #00D9FF - AI Core
    pub const PRIMARY_SOFT: Color = Color::new(0.0, 0.7, 0.85, 1.0);   // Softer primary for accents
    pub const SECONDARY: Color = Color::new(0.4, 0.4, 0.4, 1.0);      // #666666 - Buildings
    pub const ACCENT: Color = Color::new(1.0, 0.42, 0.21, 1.0);       // #FF6B35 - Resources
    pub const BACKGROUND: Color = Color::new(0.04, 0.04, 0.04, 1.0);  // #0A0A0A - Space
    pub const SURFACE: Color = Color::new(0.1, 0.1, 0.1, 1.0);        // #1A1A1A - UI panels
    pub const SURFACE_DARK: Color = Color::new(0.07, 0.07, 0.08, 1.0); // Deeper surface for layering
    pub const PANEL_BORDER: Color = Color::new(0.0, 0.6, 0.75, 0.5);   // Subtle cyan edge
    pub const SUCCESS: Color = Color::new(0.3, 0.69, 0.31, 1.0);      // #4CAF50
    pub const WARNING: Color = Color::new(1.0, 0.6, 0.0, 1.0);        // #FF9800
    pub const ERROR: Color = Color::new(0.96, 0.26, 0.21, 1.0);       // #F44336
    pub const TEXT: Color = Color::new(0.8, 0.8, 0.8, 1.0);           // #CCCCCC
    pub const TEXT_DIM: Color = Color::new(0.67, 0.67, 0.67, 1.0);    // #AAAAAA
}

/// Standard UI dimensions
pub struct Dimensions;

impl Dimensions {
    pub const BUTTON_HEIGHT: f32 = 40.0;
    pub const BUTTON_PADDING: f32 = 16.0;
    pub const PANEL_PADDING: f32 = 12.0;
    pub const FONT_SIZE_LARGE: f32 = 32.0;
    pub const FONT_SIZE_NORMAL: f32 = 20.0;
    pub const FONT_SIZE_SMALL: f32 = 14.0;
}
