use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub fn parse_existing_path(arg: &str) -> Result<PathBuf, Error> {
    if !Path::new(arg).exists() {
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }

    return Ok(PathBuf::from(arg));
}
