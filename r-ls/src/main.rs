use std::{
    env,
    fs::DirEntry,
    path::{Path, PathBuf},
};

use colored::Colorize;

fn main() {
    let path: PathBuf;
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        path = env::current_dir().unwrap();
    } else {
        path = Path::new(&args[1]).to_path_buf();
    }
    print_entries(path);
}

fn print_entries(path: PathBuf) {
    let mut entries = read_entries(path);

    remove_hidden_entries(&mut entries);
    sort_entries(&mut entries);

    for entry in entries {
        print_single_entry(entry);
        print!("  "); // spacing between entries, looks wonky, TODO: maybe arrange in grid 
    }
    println!();
}

/// Read entries of directory
fn read_entries(path: PathBuf) -> Vec<std::fs::DirEntry> {
    let entries_result = path.read_dir();

    let iter = match entries_result {
        Ok(entries) => entries,
        Err(e) => {
            println!("r-ls: {}: {}", path.display(), e);
            std::process::exit(1);
        }
    };

    iter.map(|entry| entry.unwrap()).collect()
}

/// Remove entries starting with .
fn remove_hidden_entries(entries: &mut Vec<std::fs::DirEntry>) {
    entries.retain(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
}

/// Sort entries by name
fn sort_entries(entries: &mut Vec<std::fs::DirEntry>) {
    entries.sort_by(|a, b| {
        a.file_name()
            .to_string_lossy()
            .to_lowercase()
            .cmp(&b.file_name().to_string_lossy().to_lowercase())
    })
}

/// Print the name of a single entry according to file type
fn print_single_entry(entry: DirEntry) {
    if let Ok(file_type) = entry.file_type() {
        // change blue for directory
        if file_type.is_dir() {
            print!("{}/", entry.file_name().to_string_lossy().blue().bold())
        } else {
            print!("{}", entry.file_name().to_string_lossy())
        }
    }
}
