use std::path::PathBuf;

pub mod chitra;
pub mod index;
pub mod index_tree;
pub mod treesittermanager;



// Validates the current path, returning a default path if none is provided
pub fn validate_path(curr_path: Option<PathBuf>) -> PathBuf {
    match curr_path {
        Some(p) => p,
        None => PathBuf::from("."),
    }
}
