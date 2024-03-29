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

    /// Option to change the input in all cases (same as -luc)
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub all_cases: bool,

    /// Option to change the input in lowercase
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub lowercase: bool,

    /// Option to change the input in uppercase
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub uppercase: bool,

    /// Option to change the input in capitalized case
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub capitalize: bool,

    /// Option to output the changed files/folders
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    /// Option to simulate the changes
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,
}

pub fn parse_existing_path(base_path: &str) -> Result<PathBuf, Error> {
    if !Path::new(base_path).exists() {
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }
    return Ok(PathBuf::from(base_path));
}
