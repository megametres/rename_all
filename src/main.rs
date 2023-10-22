use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// String to search
    #[arg(id = "search")]
    search: String,

    /// String to replace
    #[arg(id = "replace")]
    replace: String,

    /// Where to search
    #[arg(id = "location")]
    location: String,
}

fn main() {
    let args = Args::parse();

    println!(
        "Search {} and replace it by {} in {}!",
        args.search, args.replace, args.location
    )
}
