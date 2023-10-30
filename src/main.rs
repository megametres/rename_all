use clap::Parser;
use once_cell::sync::OnceCell;
mod filesystem;

static VERBOSE: OnceCell<bool> = OnceCell::new();

/// A utility to replace text in folders, file names and file content
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// String to search
    #[arg()]
    search: String,

    /// String to replace
    #[arg()]
    replace: String,

    /// Where to search/replace
    #[arg(value_parser = filesystem::parse_existing_path)]
    location: std::path::PathBuf,

    /// Option to output details
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    VERBOSE.get_or_init(|| args.verbose);

    if *VERBOSE.get().unwrap() {
        println!(
            "Search \"{}\" and replace it by \"{}\" in \"{}\"!\n",
            args.search,
            args.replace,
            args.location.display()
        );
    }

    filesystem::walk_through(
        &args.location.into_os_string().into_string().unwrap(),
        &args.search,
        &args.replace,
    );
}
