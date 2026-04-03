use std::path::PathBuf;

pub mod chitra;
pub mod index;
pub mod index_tree;
pub mod treesittermanager;

pub fn validate_path(curr_path: Option<PathBuf>) -> PathBuf {
    match curr_path {
        Some(p) => p,
        None => PathBuf::from("."),
    }
}
