use clap::Parser;

mod filesystem;

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
}

fn main() {
    let args = Args::parse();

    println!(
        "Search '{}' and replace it by '{}' in '{}'!",
        args.search,
        args.replace,
        args.location.display()
    );
    filesystem::walk_through(
        &args.location.into_os_string().into_string().unwrap(),
        &args.search,
        &args.replace,
    );
}
