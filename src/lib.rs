use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// This represents the "Capability Map" for subscribers
const ENGINE_VERSION: &str = "0.0.1-dev";

#[derive(Serialize, Deserialize)]
pub struct EngineInfo {
    pub version: String,
    pub status: String,
    pub capabilities: Vec<String>,
}

/// Represents the global blueprint from book.toml
#[derive(Serialize, Deserialize)]
pub struct BookManifest {
    pub project_id: String,
    pub version: String,
    pub locales: Vec<String>,
    pub manifest: Vec<String>,
}

#[wasm_bindgen]
pub fn get_engine_info() -> JsValue {
    let info = EngineInfo {
        version: ENGINE_VERSION.to_string(),
        status: "Pre-Alpha / Architectural Draft".to_string(),
        capabilities: vec![
            "directory_structure_v2".to_string(),
            "evergreen_assets_logic".to_string(),
            "parallel_sync_v1".to_string(),
        ],
    };
    serde_wasm_bindgen::to_value(&info).unwrap()
}

/// The Next Step: Parse the actual book.toml content
/// This validates that the provided JSON matches our Standard
#[wasm_bindgen]
pub fn parse_and_validate_manifest(json_input: &str) -> JsValue {
    let manifest: Result<BookManifest, _> = serde_json::from_str(json_input);

    match manifest {
        Ok(m) => serde_wasm_bindgen::to_value(&m).unwrap(),
        Err(_) => JsValue::NULL,
    }
}

/// Logic for "Direct Shadowing"
/// In the next step, this will take the scroll position of the primary language
/// and calculate exactly where the "shadow" (secondary) language should be.
#[wasm_bindgen]
pub fn calculate_shadow_offset(scroll_top: f64, primary_height: f64, shadow_height: f64) -> f64 {
    if primary_height == 0.0 { return 0.0; }

    // Calculate the ratio: How far are we through the first book?
    let ratio = scroll_top / primary_height;

    // Apply that same percentage to the second book's height
    ratio * shadow_height
}

/// Health Check: Verifies the engine is loaded and responsive
#[wasm_bindgen]
pub fn ping() -> String {
    format!("SIMLE Engine {} is active", ENGINE_VERSION)
}
