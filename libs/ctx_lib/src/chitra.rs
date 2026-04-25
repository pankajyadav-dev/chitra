use anyhow::Error;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::info;

// Initializes the chitra context in the given directory
pub async fn init_chitra<P: AsRef<Path>>(curr_dir: P) -> Result<(), Error> {
    // check if there is any parent chitra folder exist
    let is_present = check_chitra_dir(&curr_dir).await;
    if let Some(chitra_dir) = is_present {
        info!("ctx is already existed in {:?}", chitra_dir);
        return Ok(());
    }
    // create new chitra context folder in the give directory if not exist
    create_chitra_dir(curr_dir).await?;
    Ok(())
}


// Checks if there is a chitra folder in the given directory or its ancestors
pub async fn check_chitra_dir<P: AsRef<Path>>(curr_dir: P) -> Option<PathBuf> {
    let curr_dir = curr_dir.as_ref();
    // traverse up the directory tree to find the nearest chitra folder
    for ancestor in curr_dir.ancestors() {
        let target_dir = ancestor.join(".ctx");
        if let Ok(metadata) = fs::metadata(&target_dir).await
            && metadata.is_dir()
        {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

// Create a new chitra context folder in the given directory
async fn create_chitra_dir<P: AsRef<Path>>(curr_dir: P) -> Result<PathBuf, Error> {
    let root_dir_path = curr_dir.as_ref();
    let chitra_dir_path = root_dir_path.join(".ctx");
    let chitra_ignore_dir_path = root_dir_path.join(".ctxignore");
    fs::create_dir(&chitra_dir_path).await?;
    fs::File::create(&chitra_ignore_dir_path).await?;
    Ok(chitra_dir_path)
}
