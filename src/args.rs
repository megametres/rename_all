use clap::Parser;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// String to search
    #[arg()]
    pub search: String,

    /// String to replace
    #[arg()]
    pub replace: String,

    /// Where to search/replace
    #[arg(value_parser = parse_existing_path)]
    pub location: std::path::PathBuf,

    /// Option to output details
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    /// Option to simulate the changes
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,

    /// Option to also change in lowercase
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub lowercase: bool,

    /// Option to also change in uppercase
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub uppercase: bool,


}

pub fn parse_existing_path(base_path: &str) -> Result<PathBuf, Error> {
    if !Path::new(base_path).exists() {
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }
    return Ok(PathBuf::from(base_path));
}
