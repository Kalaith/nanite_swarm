//! JSON deserialization and asset loading

use serde::de::DeserializeOwned;

/// Load and parse JSON data from a string
pub fn load_json<T: DeserializeOwned>(json_str: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(json_str)
}
