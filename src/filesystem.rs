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
            rename_file(&entry, &args.search, &args.replace, &args.verbose);
        }
        if filename.contains(&args.search.to_uppercase()) {
            rename_file(
                &entry,
                &args.search.to_uppercase(),
                &args.replace.to_uppercase(),
                &args.verbose,
            );
        }
        if filename.contains(&args.search.capitalize()) {
            rename_file(
                &entry,
                &args.search.capitalize(),
                &args.replace.capitalize(),
                &args.verbose,
            );
        }
    }
}

fn rename_file(filename: &DirEntry, search: &str, replace: &str, verbose: &bool) {
    let from_file = filename.path();
    let mut to_file = filename
        .file_name()
        .to_string_lossy()
        .replace(search, replace);

    let parent_folder = filename.path().parent().unwrap().to_string_lossy();

    if parent_folder != "" {
        to_file = format!("{}/{}", parent_folder, to_file)
    }

    if *verbose {
        println!(
            "Renaming  \n\"{}\"\nto\n\"{}\"\n",
            from_file.to_string_lossy(),
            to_file
        );
    }

    let _ = fs::rename(from_file, to_file);
}
