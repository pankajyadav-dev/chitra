use anyhow::Error;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::info;

pub async fn init_chitra<P: AsRef<Path>>(curr_dir: P) -> Result<(), Error> {
    let is_present = check_chitra_dir(&curr_dir).await;
    if let Some(chitra_dir) = is_present {
        info!("ctx is already existed in {:?}", chitra_dir);
        return Ok(());
    }
    create_chitra_dir(curr_dir).await?;
    Ok(())
}

pub async fn check_chitra_dir<P: AsRef<Path>>(curr_dir: P) -> Option<PathBuf> {
    let curr_dir = curr_dir.as_ref();
    for ancestor in curr_dir.ancestors() {
        let target_dir = ancestor.join(".chitra");
        if let Ok(metadata) = fs::metadata(&target_dir).await
            && metadata.is_dir()
        {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

async fn create_chitra_dir<P: AsRef<Path>>(curr_dir: P) -> Result<PathBuf, Error> {
    let root_dir_path = curr_dir.as_ref();
    let chitra_dir_path = root_dir_path.join(".chitra");
    let chitra_ignore_dir_path = root_dir_path.join(".chitraignore");
    fs::create_dir(&chitra_dir_path).await?;
    fs::File::create(&chitra_ignore_dir_path).await?;
    Ok(chitra_dir_path)
}
