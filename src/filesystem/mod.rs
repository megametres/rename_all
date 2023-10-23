use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn parse_existing_path(arg: &str) -> Result<PathBuf, Error> {
    if !Path::new(arg).exists() {
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }
    return Ok(PathBuf::from(arg));
}

pub fn walk_through(arg: &str) -> Result<bool, Error> {
    for entry in WalkDir::new(arg)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();
        println!("{}", filename);
    }
    return Ok(true);
}
