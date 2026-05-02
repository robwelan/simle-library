use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

mod schema;
mod renderers;

// Synchronized imports with schema.rs
use crate::schema::{BookConfig, MetaConfig, ProjectPaths};
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
    paths: ProjectPaths,
    is_local: bool,
}

#[wasm_bindgen]
impl SimleEngine {
    /// The constructor takes strings from the JS host.
    /// is_local determines whether to use '../' (local) or './' (server) paths from paths.json.
    #[wasm_bindgen(constructor)]
    pub fn new(book_toml: &str, meta_toml: &str, paths_json: &str, is_local: bool) -> Result<SimleEngine, JsValue> {
        // Parse Global Controller (book.toml)
        let config: BookConfig = toml::from_str(book_toml)
            .map_err(|e| JsValue::from_str(&format!("WASM: book.toml Parse Error: {}", e)))?;

        // Parse Localized Identity (meta.toml)
        let meta: MetaConfig = toml::from_str(meta_toml)
            .map_err(|e| JsValue::from_str(&format!("WASM: meta.toml Parse Error: {}", e)))?;

        // Parse Infrastructure (paths.json)
        let paths: ProjectPaths = serde_json::from_str(paths_json)
            .map_err(|e| JsValue::from_str(&format!("WASM: paths.json Parse Error: {}", e)))?;

        Ok(SimleEngine { config, meta, paths, is_local })
    }

    /// Fetches a specific page by index.
    /// 0: Cover, 1: Metadata, 2: Table of Contents, 3+: Content Sequence.
    pub fn get_page(&self, index: u32) -> JsValue {
        let page = match index {
            0 => VirtualPage::Cover(renderers::build_cover(&self.config, &self.meta, &self.paths)),
            1 => VirtualPage::BookMeta(renderers::build_meta(&self.config, &self.meta)),
            2 => VirtualPage::ToC(renderers::build_toc(&self.config, &self.meta)),
            _ => {
                let content_idx = index as i32 - 3;
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

    /// Resolves a path for book-specific assets (e.g. cover images inside a book folder).
    /// Dynamically selects between local_books_base and server_books_base.
    pub fn resolve_asset_path(&self, asset_name: &str) -> String {
        let base = if self.is_local {
            &self.paths.web_paths.local_books_base
        } else {
            &self.paths.web_paths.server_books_base
        };
        format!("{}/{}", base, asset_name)
    }

    /// Resolves a path for global shared assets (fonts, styles, html snippets).
    /// Dynamically selects between local_shared_base and server_shared_base from paths.json.
    /// Usage: engine.resolve_shared_path("fonts", "Inter.woff2")
    pub fn resolve_shared_path(&self, category: &str, asset_name: &str) -> String {
        let base = if self.is_local {
            &self.paths.web_paths.local_shared_base
        } else {
            &self.paths.web_paths.server_shared_base
        };
        format!("{}/{}/{}", base, category, asset_name)
    }

    /// Total pages = Static pages (3) + Length of the Master Content Sequence.
    pub fn total_pages(&self) -> u32 {
        3 + self.config.structure.content.len() as u32
    }

    /// Returns the localized title from meta.toml.
    pub fn get_title(&self) -> String {
        self.meta.title.clone()
    }

    /// Returns the primary locale as defined in the global manifest (book.toml).
    pub fn get_default_locale(&self) -> String {
        self.config.structure.locales.first()
            .cloned()
            .unwrap_or_else(|| "en-us".to_string())
    }
}
