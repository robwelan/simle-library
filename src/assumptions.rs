use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileSystemAssumptions {
    pub library_file: String,
    pub book_toml_file: String, // Renamed from book_config_file
    pub meta_toml_file: String, // Renamed from meta_file
    pub assets_dir: String,
    pub content_ext: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyAssumptions {
    pub lib_path_key: String,
    pub lib_locale_key: String,
    pub manifest_key: String,
    pub wordmark_title_key: String,
    pub default_locale: String,
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
                book_toml_file: "book.toml".to_string(),
                meta_toml_file: "meta.toml".to_string(),
                assets_dir: "assets".to_string(),
                content_ext: "md".to_string(),
            },
            keys: KeyAssumptions {
                lib_path_key: "path".to_string(),
                lib_locale_key: "active_locale".to_string(),
                manifest_key: "manifest".to_string(),
                wordmark_title_key: "book_wordmark_title".to_string(),
                default_locale: "en-us".to_string(),
            },
        }
    }
}

impl EngineAssumptions {
    /// Web-safe URL joining for book.toml
    pub fn book_toml_path(&self, book_dir: &str) -> String {
        format!("{}/{}", book_dir, self.fs.book_toml_file)
    }

    /// Web-safe URL joining for meta.toml: {book_dir}/content/{locale}/meta.toml
    /// Updated to match the SIMLE content shadowing directory structure
    pub fn meta_toml_path(&self, book_dir: &str, locale: &str) -> String {
        format!("{}/content/{}/{}", book_dir, locale, self.fs.meta_toml_file)
    }

    /// Path to a specific localized asset: {book_dir}/assets/{locale}/{asset_name}
    pub fn localized_asset_path(&self, book_dir: &str, locale: &str, asset: &str) -> String {
        format!("{}/{}/{}/{}", book_dir, self.fs.assets_dir, locale, asset)
    }

    /// Path to localized content file: {book_dir}/content/{locale}/{filename}.md
    /// Updated to include the mandatory '/content/' segment
    pub fn content_path(&self, book_dir: &str, locale: &str, filename: &str) -> String {
        format!("{}/content/{}/{}.{}", book_dir, locale, filename, self.fs.content_ext)
    }
}
