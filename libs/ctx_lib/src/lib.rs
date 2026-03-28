use std::path::{PathBuf};

pub mod chitra;
pub mod index;



pub fn validate_path(curr_path: Option<PathBuf>)-> PathBuf{
    match curr_path{
        Some(p)=> p,
        None => PathBuf::from(".")
    }
}
