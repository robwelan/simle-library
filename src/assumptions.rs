use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileSystemAssumptions {
    pub library_file: String,        // "library.json"
    pub book_config_file: String,     // "book.toml"
    pub meta_file: String,           // "meta.toml"
    pub assets_dir: String,          // "assets"
    pub content_ext: String,         // ".md"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyAssumptions {
    pub lib_path_key: String,
    pub lib_locale_key: String,
    pub manifest_key: String,
    pub wordmark_title_key: String,
    pub default_locale: String,      // Added: Fallback if active_locale is missing
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EngineAssumptions {
    pub fs: FileSystemAssumptions,
    pub keys: KeyAssumptions,
}

impl Default for EngineAssumptions {
    fn default() -> Self {
        Self {
            fs: FileSystemAssumptions {
                library_file: "library.json".to_string(),
                book_config_file: "book.toml".to_string(),
                meta_file: "meta.toml".to_string(),
                assets_dir: "assets".to_string(),
                content_ext: ".md".to_string(),
            },
            keys: KeyAssumptions {
                lib_path_key: "path".to_string(),
                lib_locale_key: "active_locale".to_string(),
                manifest_key: "manifest".to_string(),
                wordmark_title_key: "book_wordmark_title".to_string(),
                default_locale: "en-US".to_string(),
            },
        }
    }
}

impl EngineAssumptions {
    /// Path to a specific book's book.toml
    /// Uses PathBuf to ensure cross-platform compatibility
    pub fn book_toml_path(&self, book_dir: &str) -> PathBuf {
        Path::new(book_dir).join(&self.fs.book_config_file)
    }

    /// Path to meta.toml: {book_dir}/{locale}/meta.toml
    pub fn meta_toml_path(&self, book_dir: &str, locale: &str) -> PathBuf {
        Path::new(book_dir).join(locale).join(&self.fs.meta_file)
    }

    /// Path to a specific localized asset: {book_dir}/assets/{locale}/{asset_name}
    pub fn localized_asset_path(&self, book_dir: &str, locale: &str, asset: &str) -> PathBuf {
        Path::new(book_dir)
            .join(&self.fs.assets_dir)
            .join(locale)
            .join(asset)
    }

    /// Path to localized content file: {book_dir}/{locale}/{filename}.md
    pub fn content_path(&self, book_dir: &str, locale: &str, filename: &str) -> PathBuf {
        let mut path = Path::new(book_dir).join(locale).join(filename);
        path.set_extension(&self.fs.content_ext.trim_start_matches('.'));
        path
    }
}
