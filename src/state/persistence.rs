//! Save/load functionality

use super::PlanetState;
use serde_json;

/// Serialize game state to JSON string
pub fn save_to_json(state: &PlanetState) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(state)
}

/// Deserialize game state from JSON string
pub fn load_from_json(json: &str) -> Result<PlanetState, serde_json::Error> {
    serde_json::from_str(json)
}
