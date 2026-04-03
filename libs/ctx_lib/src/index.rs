use anyhow::{Error, anyhow};
use globset::{Glob, GlobSetBuilder};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::info;
use walkdir::WalkDir;

async fn get_chitra_relative_path<P: AsRef<Path>>(
    base_dir: P,
    curr_dir: P,
) -> Result<PathBuf, Error> {
    let base_dir = base_dir.as_ref();
    let curr_dir = curr_dir.as_ref();

    let cononical_root = fs::canonicalize(base_dir).await?;
    let cononical_curr_dir = fs::canonicalize(curr_dir).await?;
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

pub async fn read_chitra_ignore_files<P: AsRef<Path>>(
    chitra_path: P,
) -> Result<Vec<String>, Error> {
    let chitra_path = chitra_path.as_ref().join("./.chitraignore");
    let ignore_file = fs::read_to_string(&chitra_path).await?;
    let ignore_files = ignore_file
        .lines()
        .map(|e| e.trim())
        .map(|e| e.trim_end_matches("/"))
        // .map(|e| e.trim_end_matches("*"))
        .filter(|l| !l.is_empty())
        .filter(|l| !l.starts_with("#"))
        .map(|line| line.to_string())
        .collect();
    Ok(ignore_files)
}

pub async fn filter_index_files<P: AsRef<Path>>(
    chitra_dir: P,
    dir_to_index: P,
) -> Result<Vec<PathBuf>, Error> {
    let chitra_dir = chitra_dir.as_ref();
    let dir_to_index = dir_to_index.as_ref();

    let paths = index_files(dir_to_index)?;

    let ignore_path = read_chitra_ignore_files(&chitra_dir)
        .await
        .unwrap_or_default();
    if ignore_path.is_empty() {
        return Ok(paths);
    }

    let mut builder = GlobSetBuilder::new();

    for pattern in ignore_path {
        match Glob::new(&pattern) {
            Ok(glob) => {
                builder.add(glob);
            }
            Err(e) => {
                info!("Invalid glob pattern {} - {}", pattern, e);
            }
        };
    }

    let glob_set = builder
        .build()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    let mut filter_path = Vec::new();

    for path in paths {
        let relative_path = path.strip_prefix(chitra_dir).unwrap_or(&path);

        let is_ignored = relative_path.ancestors().any(|ancestor| {
            if ancestor.as_os_str().is_empty() {
                return false;
            }
            if glob_set.is_match(ancestor) {
                return true;
            }
            if let Some(name) = ancestor.file_name()
                && glob_set.is_match(name)
            {
                return true;
            }

            false
        });

        if !is_ignored {
            let parsed_path = get_chitra_relative_path(chitra_dir, &path).await?;
            filter_path.push(parsed_path);
        } else {
            info!("Ignoreing the file {:?}", &path);
        }
    }

    Ok(filter_path)
}

fn index_files<P: AsRef<Path>>(dir_path: P) -> Result<Vec<PathBuf>, Error> {
    let dir_path = dir_path.as_ref();
    let files: Vec<PathBuf> = WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();
    Ok(files)
}
