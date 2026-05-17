//! Save/load functionality

use super::PlanetState;
use macroquad::miniquad;
use serde_json;
use std::fs;
use std::io;

fn unix_seconds_now() -> i64 {
    (miniquad::date::now() as i64).max(0)
}

/// Serialize game state to JSON string
pub fn save_to_json(state: &mut PlanetState) -> Result<String, serde_json::Error> {
    state.last_saved_unix = unix_seconds_now();
    serde_json::to_string_pretty(state)
}

/// Deserialize game state from JSON string
pub fn load_from_json(json: &str) -> Result<PlanetState, serde_json::Error> {
    let mut state: PlanetState = serde_json::from_str(json)?;
    let now = unix_seconds_now();
    if state.last_saved_unix > 0 && now > state.last_saved_unix {
        let offline_seconds = (now - state.last_saved_unix) as f32;
        state.apply_offline_progress(offline_seconds);
    }
    state.last_saved_unix = now;
    Ok(state)
}

pub fn save_to_file(state: &mut PlanetState, path: &str) -> Result<(), io::Error> {
    let json = save_to_json(state).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    macroquad_toolkit::persistence::save_string_atomic(path, &json)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn load_from_file(path: &str) -> Result<PlanetState, io::Error> {
    let json = fs::read_to_string(path)?;
    load_from_json(&json).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
