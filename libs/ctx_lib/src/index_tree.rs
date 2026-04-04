use anyhow::{Context, Error};
use core::str;
use std::sync::Arc;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use tokio::task::JoinSet;
use tracing::info;

use crate::treesittermanager::TreesitterManager;

pub async fn create_index_tree<P: AsRef<Path>>(
    chitra_path: P,
    file_paths: Vec<PathBuf>,
) -> Result<(), Error> {
    let chitra_path = chitra_path.as_ref();
    let chitra_index_file_path = chitra_path.join(".chitra/index/index.json");
    if let Some(parent_dir) = chitra_index_file_path.parent() {
        tokio::fs::create_dir_all(parent_dir)
            .await
            .context(format!(
                "Falied to create the index directory  {:?}",
                parent_dir
            ))?;
    }
    if !chitra_index_file_path.exists() {
        tokio::fs::write(&chitra_index_file_path, b"{}\n")
            .await
            .context("Failed to create the index.json")?;
        info!("index.json file is created");
    }
    info!("Chitra index file path {:?}", chitra_index_file_path);
    info!("File path which we have to index {:?}", file_paths);

    let ts_manager = Arc::new(TreesitterManager::new().await?);
    info!(
        "the binary cofig path of os filesystem {:?}",
        ts_manager.bin_dir
    );

    let required_language = get_file_extension(&file_paths)?;

    info!("Required binary for language {:?}", required_language);

    check_treesitter_binary_exist(required_language, ts_manager).await?;

    Ok(())
}

async fn check_treesitter_binary_exist(
    required_binary_lang: HashSet<&'static str>,
    tree_manager: Arc<TreesitterManager>,
) -> Result<(), Error> {
    let mut set = JoinSet::new();
    for lang in required_binary_lang {
        let manager_clone = Arc::clone(&tree_manager);
        set.spawn(async move { manager_clone.ensure_treesitter_binary(lang).await });
    }
    while let Some(result) = set.join_next().await {
        let lang_file_path = result??;
        info!("tree sitter binary path is {:?}", lang_file_path);
    }
    Ok(())
}

fn get_file_extension(file_paths: &[PathBuf]) -> Result<HashSet<&'static str>, Error> {
    let mut required_language = HashSet::new();
    for file in file_paths {
        let mut detected_lang = None;
        if let Some(lang) = file.extension().and_then(|e| e.to_str()) {
            detected_lang = TreesitterManager::get_language_from_extension(lang);
        }

        if detected_lang.is_none()
            && let Some(file_name) = file.file_name().and_then(|name| name.to_str())
        {
            let name_lower_case = file_name.to_lowercase();
            detected_lang = match name_lower_case.as_str() {
                "dockerfile" => Some("dockerfile"),
                "makefile" => Some("make"),
                name if name.starts_with("dockerfile") => Some("dockerfile"),
                _ => None,
            }
        }

        if let Some(lang) = detected_lang {
            required_language.insert(lang);
        }
    }
    Ok(required_language)
}
