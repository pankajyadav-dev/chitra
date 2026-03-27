use std::path::{PathBuf, Path};
use tokio::fs;

pub async fn check_chitra_dir(curr_dir: &Path) -> Option<PathBuf> {
    for ancestor in curr_dir.ancestors() {
        let target_dir = ancestor.join(".chitra");
        if let Ok(metadata) = fs::metadata(&target_dir).await {
            if metadata.is_dir() {
                return Some(ancestor.to_path_buf());
            }
        }
    }
    None
}
