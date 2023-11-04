use crate::args::Args;
use capitalize::Capitalize;
use std::fs;
use walkdir::{DirEntry, WalkDir};

pub fn walk_through(args: &Args) {
    for entry in WalkDir::new(&args.location)
        .contents_first(true)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();
        if filename.contains(&args.search) {
            rename_file(
                &entry,
                &args.search,
                &args.replace,
                &args.verbose,
                &args.dry_run,
            );
        }
        if args.lowercase && filename.contains(&args.search.to_lowercase()) {
            rename_file(
                &entry,
                &args.search.to_lowercase(),
                &args.replace.to_lowercase(),
                &args.verbose,
                &args.dry_run,
            );
        }
        if args.uppercase && filename.contains(&args.search.to_uppercase()) {
            rename_file(
                &entry,
                &args.search.to_uppercase(),
                &args.replace.to_uppercase(),
                &args.verbose,
                &args.dry_run,
            );
        }
        if args.capitalize && filename.contains(&args.search.capitalize()) {
            rename_file(
                &entry,
                &args.search.capitalize(),
                &args.replace.capitalize(),
                &args.verbose,
                &args.dry_run,
            );
        }
    }
}

fn rename_file(filename: &DirEntry, search: &str, replace: &str, verbose: &bool, dry_run: &bool) {
    let from_file = filename.path();
    let mut to_file = filename
        .file_name()
        .to_string_lossy()
        .replace(search, replace);

    let parent_folder = filename.path().parent().unwrap().to_string_lossy();

    if parent_folder != "" {
        to_file = format!("{}/{}", parent_folder, to_file)
    }

    if *verbose || *dry_run {
        println!(
            "Renaming  \n\"{}\"\nto\n\"{}\"\n",
            from_file.to_string_lossy(),
            to_file
        );
    }

    if !*dry_run {
        let _ = fs::rename(from_file, to_file);
    }
}
