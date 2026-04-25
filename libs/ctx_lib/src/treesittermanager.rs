use anyhow::Error;
use std::path::PathBuf;
use treesitter_lib::ctxdir::{download_ctx_parser, get_ctx_parser_dir};

// Manages the Treesitter binary directory and provides language-specific parsers
pub struct TreesitterManager {
    pub bin_dir: PathBuf,
}

impl TreesitterManager {
    // Creates a new TreesitterManager by downloading the parser binaries
    pub async fn new() -> Result<Self, Error> {
        let bin_dir = get_ctx_parser_dir().await?;
        Ok(Self { bin_dir })
    }

    // Returns the language name for a given file extension
    pub fn get_language_from_extension(lang_extension: &str) -> Option<&'static str> {
        match lang_extension {
            "rs" => Some("rust"),
            "cpp" | "cc" => Some("cpp"),
            "java" => Some("java"),
            "js" | "jsx" => Some("javascript"),
            "ts" | "tsx" => Some("typescript"),
            "py" => Some("python"),
            "go" => Some("go"),
            "json" => Some("json"),
            "toml" => Some("toml"),
            "yaml" => Some("yaml"),
            "xml" => Some("xml"),
            "html" => Some("html"),
            "css" => Some("css"),
            "sql" => Some("sql"),
            "Dockerfile" => Some("dockerfile"),
            "md" => Some("markdown"),
            _ => None,
        }
    }

    // Ensures the Treesitter binary for the given language is downloaded and returns its path
    pub async fn ensure_treesitter_binary(&self, lang: &str) -> Result<PathBuf, Error> {
        let binary_path = download_ctx_parser(&self.bin_dir, lang).await?;
        Ok(binary_path)
    }
}
