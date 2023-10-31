mod filesystem;
mod args;

use args::Args;
use clap::Parser;

/// A utility to replace text in folders, file names and file content
fn main() {
    let args = Args::parse();

    if args.verbose {
        println!(
            "Search \"{}\" and replace it by \"{}\" in \"{}\"!\n",
            args.search,
            args.replace,
            args.location.display()
        );
    }

    filesystem::walk_through( &args);
}
