use serde::{Serialize, Deserialize};

// ==========================================================
// 1. GLOBAL CONFIGURATION (book.toml)
// ==========================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub id: String,
    pub version: String,
    pub global_isbn: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureConfig {
    pub locales: Vec<String>,
    pub content: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoverConfig {
    pub book_cover_layout: String,
    pub cover_background_landscape: String,
    pub cover_background_portrait: String,
}

/// The Master Blueprint parsed from the root book.toml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookConfig {
    pub project: ProjectConfig,
    pub structure: StructureConfig,
    pub cover: CoverConfig,
}

// ==========================================================
// 2. LOCALIZED IDENTITY (meta.toml)
// ==========================================================

/// Represents the localized metadata found in /content/{locale}/meta.toml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaConfig {
    pub locale_name: String, // e.g., "English" or "简体中文"
    pub title: String,
    pub author: String,
    pub description: String,
    pub publisher: String,
    pub keywords: Vec<String>,
    pub isbn: Option<String>,
}

// ==========================================================
// 3. INFRASTRUCTURE (paths.json)
// ==========================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebPaths {
    pub server_books_base: String,
}

/// System-level path resolution parsed from paths.json
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectPaths {
    pub web_paths: WebPaths,
}

// ==========================================================
// 4. LIBRARY INDEX (library.json)
// ==========================================================

/// Represents a single entry in the bookshelf index
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryEntry {
    pub path: String,
    pub active_locale: String,
}
