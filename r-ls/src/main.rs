use clap::{Arg, ArgAction, Command};
use colored::Colorize;
use std::{fs::DirEntry, path::Path};

fn main() {
    let matches = define_command().get_matches();

    // get paths from args
    let path_args_result =
        matches.get_many::<String>("FILE");
    let paths: Vec<&Path> = match path_args_result {
        None => vec![Path::new(".")],
        Some(path_args) => path_args.map(|path| Path::new(path)).collect(),
    };

    // get all flag from arg
    let all_arg = matches.get_one::<bool>("all").unwrap_or(&false);

    for path in &paths {
        if paths.len() > 1 {
            println!("{}:", path.display());
        }
        print_entries(path, all_arg);

        // empty line between directories, not at the end
        if path != paths.last().unwrap() {
            println!();
        }
    }
}

fn define_command() -> clap::Command {
    Command::new("r-ls")
        .version("0.1.0")
        .about("List information about the FILEs (the current directory by default).")
        .arg(Arg::new("FILE").action(ArgAction::Append))
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue),
        )
}

fn print_entries(path: &Path, show_hidden: &bool) {
    let mut entries = read_entries(path);

    if !show_hidden {
        remove_hidden_entries(&mut entries);
    }
    sort_entries(&mut entries);

    for entry in entries {
        print_single_entry(entry);
        print!("  "); // spacing between entries, looks wonky, TODO: maybe arrange in grid
    }
    println!();
}

/// Read entries of directory
fn read_entries(path: &Path) -> Vec<std::fs::DirEntry> {
    let entries_result = path.read_dir();

    let iter = match entries_result {
        Ok(entries) => entries,
        Err(e) => {
            println!("r-ls: cannot access '{}': {}", path.display(), e);
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
