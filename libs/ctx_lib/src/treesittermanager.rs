use anyhow::Error;
use std::path::PathBuf;
use treesitter_lib::ctxdir::{download_ctx_parser, get_ctx_parser_dir};
pub struct TreesitterManager {
    pub bin_dir: PathBuf,
}

impl TreesitterManager {
    pub async fn new() -> Result<Self, Error> {
        let bin_dir = get_ctx_parser_dir().await?;
        Ok(Self { bin_dir })
    }

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
            ".md" => Some("markdown"),
            _ => None,
        }
    }

    pub async fn ensure_treesitter_bianry(&self, lang: &str) -> Result<PathBuf, Error> {
        let binary_path = download_ctx_parser(&self.bin_dir, lang).await?;
        Ok(binary_path)
    }
}
