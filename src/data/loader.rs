//! JSON deserialization and asset loading

use serde::de::DeserializeOwned;

/// Load and parse JSON data from a string
pub fn load_json<T: DeserializeOwned>(json_str: &str) -> Result<T, String> {
    macroquad_toolkit::data_loader::load_embedded_json(json_str)
}
