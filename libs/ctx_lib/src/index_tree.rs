use anyhow::{Context, Error};
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
        tokio::fs::write(&chitra_index_file_path, b"{}")
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

    let mut required_language = HashSet::new();
    for file in &file_paths {
        if let Some(ext) = file.extension().and_then(|ex| ex.to_str())
            && let Some(lang) = TreesitterManager::get_language_from_extension(ext)
        {
            required_language.insert(lang);
        }
    }
    info!("Required binary for language {:?}", required_language);

    let mut set = JoinSet::new();
    for lang in required_language {
        let manager_clone = Arc::clone(&ts_manager);
        set.spawn(async move { manager_clone.ensure_treesitter_bianry(lang).await });
    }

    while let Some(result) = set.join_next().await {
        let lang_file_path = result??;
        info!("Tree sitter bianry path is {:?}", lang_file_path);
    }

    Ok(())
}

// pub async fn check_treesitter_binary(){
//
// }
