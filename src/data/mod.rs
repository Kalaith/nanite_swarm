//! Data types and JSON loading
//!
//! This module contains all data structures and configuration loading.

#![allow(unused)]

mod loader;
mod game_config;
mod defs;

pub use loader::*;
pub use game_config::*;
pub use defs::*;

use std::fs;
use std::sync::OnceLock;

static GAME_DATA: OnceLock<GameData> = OnceLock::new();

pub fn load_game_config() -> GameConfig {
    let json = fs::read_to_string("assets/game_config.json").unwrap_or_default();
    load_json(&json).unwrap_or_else(|_| GameConfig::default())
}

pub fn game_data() -> &'static GameData {
    GAME_DATA.get_or_init(GameData::load)
}
