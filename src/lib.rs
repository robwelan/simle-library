use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

mod schema;
mod renderers;

use crate::schema::{BookConfig, MetaConfig};
use crate::renderers::{CoverPage, MetaPage, ToCPage};

/// Defines the structure of pages sent to the JavaScript frontend.
/// Follows the SIMLE rendering standard for predictable JSON output.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum VirtualPage {
    Cover(CoverPage),
    BookMeta(MetaPage),
    ToC(ToCPage),
    Chapter {
        title: String,
        content: String,
        source_file: String
    },
    Empty,
}

#[wasm_bindgen]
pub struct SimleEngine {
    config: BookConfig,
    meta: MetaConfig,
}

#[wasm_bindgen]
impl SimleEngine {
    /// The constructor takes JSON strings from the JS host.
    /// Validates the structure against the SIMLE Schema (ProjectInfo & StructureConfig).
    #[wasm_bindgen(constructor)]
    pub fn new(config_json: &str, meta_json: &str) -> Result<SimleEngine, JsValue> {
        let config: BookConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("WASM: Config Parse Error: {}", e)))?;

        let meta: MetaConfig = serde_json::from_str(meta_json)
            .map_err(|e| JsValue::from_str(&format!("WASM: Meta Parse Error: {}", e)))?;

        Ok(SimleEngine { config, meta })
    }

    /// Fetches a specific page by index.
    /// 0: Cover, 1: Metadata, 2: Table of Contents, 3+: Content Sequence.
    pub fn get_page(&self, index: u32) -> JsValue {
        let page = match index {
            0 => VirtualPage::Cover(renderers::build_cover(&self.config, &self.meta)),
            1 => VirtualPage::BookMeta(renderers::build_meta(&self.config, &self.meta)),
            2 => VirtualPage::ToC(renderers::build_toc(&self.config, &self.meta)),
            _ => {
                // Adjust index for static offset (Cover, Meta, ToC)
                let content_idx = index as i32 - 3;

                // Validate against the Master Sequence defined in book.toml
                if content_idx >= 0 && content_idx < self.config.structure.content.len() as i32 {
                    let filename = &self.config.structure.content[content_idx as usize];

                    VirtualPage::Chapter {
                        title: format!("Chapter {}", content_idx + 1),
                        content: format!("Placeholder content for {}", filename),
                        source_file: filename.clone(),
                    }
                } else {
                    VirtualPage::Empty
                }
            }
        };

        serde_wasm_bindgen::to_value(&page).unwrap_or(JsValue::NULL)
    }

    /// Total pages = Static pages (3) + Length of the Master Content Sequence.
    pub fn total_pages(&self) -> u32 {
        3 + self.config.structure.content.len() as u32
    }

    pub fn get_title(&self) -> String {
        self.meta.title.clone()
    }

    /// Returns the primary locale as defined in the global manifest.
    pub fn get_default_locale(&self) -> String {
        self.config.structure.locales.first()
            .cloned()
            .unwrap_or_else(|| "en-us".to_string())
    }
}
