use crate::schema::{BookConfig, MetaConfig, ProjectPaths};
use serde::{Deserialize, Serialize};

/// Data structure for the Book Cover page
/// Matches the SIMLE Cover & Branding Logic for layered architecture.
#[derive(Debug, Serialize, Deserialize)]
pub struct CoverPage {
    pub title: String,
    pub author: String,
    pub layout: String,
    pub bg_landscape_url: String,
    pub bg_portrait_url: String,
}

/// Data structure for the Metadata/Copyright page
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaPage {
    pub title: String,
    pub author: String,
    pub description: String,
    pub publisher: String,
    pub isbn: Option<String>,
}

/// Data structure for the Table of Contents page
#[derive(Debug, Serialize, Deserialize)]
pub struct ToCPage {
    pub title: String,
    pub chapter_count: u32,
    pub chapters: Vec<String>,
}

/// Builds the data required for the Cover page.
/// Resolves asset paths through the standard /assets/ directory.
pub fn build_cover(book_toml: &BookConfig, meta_toml: &MetaConfig, paths: &ProjectPaths) -> CoverPage {
    // Construct base URL for this book's asset folder
    // Format: {base_url}/{book_id}/assets
    let asset_base = format!(
        "{}/{}/assets",
        paths.web_paths.server_books_base,
        book_toml.project.id
    );

    CoverPage {
        title: meta_toml.title.clone(),
        author: meta_toml.author.clone(),
        layout: book_toml.cover.book_cover_layout.clone(),
        // Assets are pulled from the root assets folder per SIMLE spec
        bg_landscape_url: format!("{}/{}", asset_base, book_toml.cover.cover_background_landscape),
        bg_portrait_url: format!("{}/{}", asset_base, book_toml.cover.cover_background_portrait),
    }
}

/// Builds the data required for the Book Metadata page.
/// Combines Localized Identity (meta.toml) with Global structural data.
pub fn build_meta(_book_toml: &BookConfig, meta_toml: &MetaConfig) -> MetaPage {
    MetaPage {
        title: meta_toml.title.clone(),
        author: meta_toml.author.clone(),
        description: meta_toml.description.clone(),
        publisher: meta_toml.publisher.clone(),
        isbn: meta_toml.isbn.clone(),
    }
}

/// Builds the data required for the Table of Contents page.
/// Derived from the Master Sequence in book.toml.
pub fn build_toc(book_toml: &BookConfig, _meta_toml: &MetaConfig) -> ToCPage {
    ToCPage {
        title: "Table of Contents".to_string(),
        chapter_count: book_toml.structure.content.len() as u32,
        chapters: book_toml.structure.content.clone(),
    }
}
