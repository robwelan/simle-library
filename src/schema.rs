use serde::{Serialize, Deserialize};

/// Global identity and technical identifiers for the publication.
/// Aligns with the [project] block in book.toml.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectInfo {
    /// Unique slug/ID for library indexing.
    pub id: String,
    /// Global ISBN (one per .simle file).
    pub isbn: String,
    /// Current edition version (e.g., "2.1.0").
    pub version: String,
    /// Internal engine compatibility version (required by lib.rs).
    pub engine_version: String,
}

/// Defines technical file mapping and reading order.
/// Aligns with the [structure] block in book.toml.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureConfig {
    /// Supported language folders in /content/.
    pub locales: Vec<String>,
    /// THE MASTER SEQUENCE: Defines the exact page order.
    pub content: Vec<String>,
    /// Technical manifest list (required by current Rust implementation).
    pub manifest: Vec<String>,
}

/// The root manifest structure for book.toml.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookConfig {
    pub project: ProjectInfo,
    pub structure: StructureConfig,
    pub cover: CoverConfig,
}

/// Media and theme configuration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoverConfig {
    /// Background image path, typically in /assets/.
    pub cover_background_landscape: String,
    /// Theme accent color (e.g., "#ff0000").
    pub accent_color: Option<String>,
}

/// Localized identity found in /content/[locale]/meta.toml.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaConfig {
    /// The language name as it appears in the UI (e.g., "日本語").
    pub locale_name: String,
    /// Translated title of the book.
    pub title: String,
    /// Localized author name.
    pub author: String,
    /// Short localized blurb for library view.
    pub description: String,
    /// Localized publisher branch name.
    pub publisher: String,
    /// Localized search terms for the reader engine.
    pub keywords: Vec<String>,
    /// Optional global ISBN shadowed in the local meta.
    pub isbn: Option<String>,
}

/// Helper for library indexing and engine routing.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryEntry {
    pub id: String,
    pub config_path: String,
    pub meta_path: String,
}
