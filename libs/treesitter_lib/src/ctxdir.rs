use anyhow::{Context, Error};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::info;

pub async fn get_ctx_parser_dir() -> Result<PathBuf, Error> {
    let project_dir =
        ProjectDirs::from("com", "Chitra", "ctx").context("Failed to find valid project dir")?;

    let parser_dir = project_dir.config_dir().join("treesitter_binary");
    if !parser_dir.exists() {
        fs::create_dir_all(&parser_dir)
            .await
            .context("failed to create ctx parser dir")?;
    }

    Ok(parser_dir)
}

pub async fn download_ctx_parser<P: AsRef<Path>>(
    target_dir: P,
    language: &str,
) -> Result<PathBuf, Error> {
    let target_dir = target_dir.as_ref();

    let extension = if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };

    let filename = format!("tree_sitter_{}.{}", language, extension);
    let file_path = target_dir.join(filename);

    if file_path.exists() {
        return Ok(file_path);
    }
    info!("Downloading ctx parser for language: {}", language);
    // https://github.com/techpankajyadav/tree-sitter/releases/download/latest/tree-sitter-python.so
    let url = format!(
        "https://github.com/techpankajyadav/tree-sitter/releases/download/latest/tree-sitter-{}.{}",
        language, extension
    );
    info!("the tree start downloading and the url is {}", url);
    info!(
        "the tree start downloading and the file path is {:?}",
        &file_path
    );
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;
    fs::write(&file_path, bytes)
        .await
        .context(format!("Failed to create the tree-sitter-{}", language))?;
    info!("Downloadinng complete of tree_sitter_{:?}", language);
    Ok(file_path)
}
