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
#[cfg(target_arch = "wasm32")]
use macroquad::prelude::*;

static GAME_DATA: OnceLock<GameData> = OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
pub fn load_game_config() -> GameConfig {
    let json = fs::read_to_string("assets/game_config.json").unwrap_or_default();
    load_json(&json).unwrap_or_else(|_| GameConfig::default())
}

#[cfg(target_arch = "wasm32")]
pub async fn load_game_config() -> GameConfig {
    let json = load_string("assets/game_config.json").await.unwrap_or_default();
    load_json(&json).unwrap_or_else(|_| GameConfig::default())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_game_data() -> GameData {
    GameData::load()
}

#[cfg(target_arch = "wasm32")]
pub async fn load_game_data() -> GameData {
    GameData::load_async().await
}

pub fn set_game_data(data: GameData) {
    GAME_DATA.set(data).unwrap();
}

pub fn game_data() -> &'static GameData {
    GAME_DATA.get().expect("Game data not loaded yet")
}
