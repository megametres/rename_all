use crate::args::Args;
use capitalize::Capitalize;
use std::fs::{metadata, rename, File};
use std::io::{Read, Write};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn walk_through(args: &Args) {
    for entry in WalkDir::new(&args.location)
        .contents_first(true)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();
        rename_entry(
            &entry,
            &args.search,
            &args.replace,
            &args.verbose,
            &args.dry_run,
        );

        if (args.lowercase || args.all_cases) && &args.search != &args.search.to_lowercase() {
            rename_entry(
                &entry,
                &args.search.to_lowercase(),
                &args.replace.to_lowercase(),
                &args.verbose,
                &args.dry_run,
            );
        }
        if (args.uppercase || args.all_cases) && &args.search != &args.search.to_uppercase() {
            rename_entry(
                &entry,
                &args.search.to_uppercase(),
                &args.replace.to_uppercase(),
                &args.verbose,
                &args.dry_run,
            );
        }
        if (args.capitalize || args.all_cases) && &args.search != &args.search.capitalize() {
            println!("{}", &args.search.capitalize());
            println!("aaa");
            rename_entry(
                &entry,
                &args.search.capitalize(),
                &args.replace.capitalize(),
                &args.verbose,
                &args.dry_run,
            );
        }
    }
}

fn rename_entry(entry: &DirEntry, search: &str, replace: &str, verbose: &bool, dry_run: &bool) {
    let filename = entry.file_name().to_string_lossy();
    let file_metadata = match metadata(entry.path()) {
        Ok(path) => path,
        Err(_) => return,
    };

    if file_metadata.is_file() {
        rename_file_content(entry, search, replace, verbose, dry_run);
    }

    if filename.contains(search) {
        rename_pathname(entry, search, replace, verbose, dry_run);
    }
}

fn rename_file_content(
    entry: &DirEntry,
    search: &str,
    replace: &str,
    verbose: &bool,
    dry_run: &bool,
) {
    let file_data = read_file_content(entry.path());
    let replaced_data = file_data.replace(search, replace);

    if file_data != replaced_data {
        if *verbose || *dry_run {
            println!(
                "Changing content of file  \n\"{}\"\n",
                entry.path().to_string_lossy()
            );
        }
        if !*dry_run {
            let mut file_dst = File::create(entry.path()).unwrap();
            let _ = file_dst.write(replaced_data.as_bytes());
        }
    }
}

fn rename_pathname(entry: &DirEntry, search: &str, replace: &str, verbose: &bool, dry_run: &bool) {
    let from_file = entry.path();
    let to_file = destination_file(entry, search, replace);

    if *verbose || *dry_run {
        println!(
            "Renaming  \n\"{}\"\nto\n\"{}\"\n",
            from_file.to_string_lossy(),
            to_file
        );
    }

    if !*dry_run {
        let _ = rename(from_file, to_file);
    }
}

fn destination_file(filename: &DirEntry, search: &str, replace: &str) -> String {
    let mut to_file = filename
        .file_name()
        .to_string_lossy()
        .replace(search, replace);

    let parent_folder = filename.path().parent().unwrap().to_string_lossy();

    if parent_folder != "" {
        to_file = format!("{}/{}", parent_folder, to_file)
    }
    to_file
}

pub fn read_file_content(file_path: &Path) -> String {
    let mut read_file = File::open(file_path).expect("Unable to open the file");
    let mut file_content = String::new();
    let _ = read_file.read_to_string(&mut file_content);
    file_content
}
