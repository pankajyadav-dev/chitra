use anyhow::Error;
use std::path::{Path, PathBuf};
use tracing::info;

pub async fn create_index_tree<P: AsRef<Path>>(
    chitra_path: P,
    file_paths: Vec<PathBuf>,
) -> Result<(), Error> {
    let chitra_path = chitra_path.as_ref();
    let chitra_index_file_path = chitra_path.join("index/index.json");
    info!("Chitra index file path {:?}", chitra_index_file_path);
    info!("File path which we have to index {:?}", file_paths);
    Ok(())
}
