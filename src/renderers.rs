use crate::schema::{BookConfig, MetaConfig};
use serde::{Deserialize, Serialize};

/// Data structure for the Book Cover page
#[derive(Debug, Serialize, Deserialize)]
pub struct CoverPage {
    pub title: String,
    pub author: String,
    pub background_url: String,
    pub accent_color: String,
}

/// Data structure for the Metadata/Copyright page
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaPage {
    pub title: String,
    pub author: String,
    pub description: String,
    pub publisher: String,
    pub isbn: Option<String>,
    pub engine_version: String,
}

/// Data structure for the Table of Contents page
#[derive(Debug, Serialize, Deserialize)]
pub struct ToCPage {
    pub title: String,
    pub chapter_count: u32,
    pub chapters: Vec<String>,
}

/// Builds the data required for the Cover page
pub fn build_cover(config: &BookConfig, meta: &MetaConfig) -> CoverPage {
    CoverPage {
        title: meta.title.clone(),
        author: meta.author.clone(),
        background_url: config.cover.cover_background_landscape.clone(),
        accent_color: config.cover.accent_color.clone().unwrap_or_else(|| "#000000".to_string()),
    }
}

/// Builds the data required for the Book Metadata page
pub fn build_meta(config: &BookConfig, meta: &MetaConfig) -> MetaPage {
    MetaPage {
        title: meta.title.clone(),
        author: meta.author.clone(),
        description: meta.description.clone(),
        publisher: meta.publisher.clone(),
        isbn: meta.isbn.clone(),
        engine_version: config.project.engine_version.clone(),
    }
}

/// Builds the data required for the Table of Contents page.
/// Dynamically calculates chapter_count from the content master sequence.
pub fn build_toc(config: &BookConfig, _meta: &MetaConfig) -> ToCPage {
    ToCPage {
        title: "Table of Contents".to_string(),
        // Fix: Use .len() on the content array instead of a hardcoded field
        chapter_count: config.structure.content.len() as u32,
        chapters: config.structure.content.clone(),
    }
}

/// Legacy renderer for full HTML output if needed
pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn render_book(config: &BookConfig, meta: &MetaConfig) -> String {
        let cover = build_cover(config, meta);
        format!(
            "<div style='background-image: url({}); color: {}'><h1>{}</h1><p>{}</p></div>",
            cover.background_url, cover.accent_color, cover.title, cover.author
        )
    }
}
