use std::path::{Path, PathBuf};

use anyhow::{Error, anyhow};
use tokio::fs::canonicalize;

async fn get_chitra_relative_path<P: AsRef<Path>>(
    base_dir: P,
    curr_dir: P,
) -> Result<PathBuf, Error> {
    let base_dir = base_dir.as_ref();
    let curr_dir = curr_dir.as_ref();

    let cononical_root = canonicalize(base_dir).await?;
    let cononical_curr_dir = canonicalize(curr_dir).await?;
    match cononical_curr_dir.strip_prefix(&cononical_root) {
        Ok(relative_path) => Ok(relative_path.to_path_buf()),
        Err(e) => Err(anyhow!(
            "Invalid path chitra dir not found in the current project {:?}",
            e
        )),
    }
}

pub async fn index_relative_path<P: AsRef<Path>>(
    base_dir: P,
    curr_dir: P,
) -> Result<PathBuf, Error> {
    let base_dir = base_dir.as_ref();
    let curr_dir = curr_dir.as_ref();
    let relative_path = get_chitra_relative_path(base_dir, curr_dir).await?;
    if relative_path.as_os_str().is_empty() {
        return Ok(PathBuf::from("."));
    }

    Ok(relative_path)
}
