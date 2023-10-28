use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn parse_existing_path(base_path: &str) -> Result<PathBuf, Error> {
    if !Path::new(base_path).exists() {
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }
    return Ok(PathBuf::from(base_path));
}

pub fn walk_through(base_path: &str, search: &str, replace: &str) {
    for entry in WalkDir::new(base_path)
        .contents_first(true)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();
        if filename.contains(search) {
            rename_file(&entry, search, replace);
        }
        if filename.contains(&search.to_uppercase()) {
            rename_file(&entry, &search.to_uppercase(), &replace.to_uppercase());
        }
    }
}

fn rename_file(filename: &DirEntry, search: &str, replace: &str) {
    let _ = fs::rename(
        filename.path(),
        Path::new(&format!("{}/{}",
            filename.path().parent().unwrap().to_string_lossy(),
            (filename.file_name().to_string_lossy().replace(search, replace)))
        ),
    );
}
