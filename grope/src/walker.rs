use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn list_directories(path: &Path) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for entries in WalkDir::new(path).into_iter().skip(1) {
        let entry = entries?;

        if let Some(name) = entry.file_name().to_str() {
            if !name.starts_with('.') {
                paths.push(entry.into_path());
            }
        }
    }
    Ok(paths)
}
