pub mod commands;
pub mod config;
pub mod display;
pub mod scan;
pub mod utils;

use crate::commands::arg_tokenizer;
use crate::config::{
    add_value_to_setting, create_config_file, get_setting_from_config, remove_value_from_setting,
    ConfigOption,
};
use crate::display::display_tree;
use crate::scan::scan;
use crate::utils::get_current_directory_path;
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub enum OS {
    Windows,
    Mac,
}

pub enum Command {
    Scan(bool, bool, bool, bool),
    ScanPath(String, bool, bool, bool, bool),
    CreateConfig,
    AddDirectory(String),
    AddFile(String),
    RemoveDirectory(String),
    RemoveFile(String),
    ListIgnoredDirectories,
    ListIgnoredFiles,
    Version,
    Help,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let os = match std::env::consts::OS {
        "windows" => OS::Windows,
        "macos" => OS::Mac,
        _ => panic!("Unsupported OS"),
    };

    create_config_file(&os, false);

    let command = arg_tokenizer(args);

    match command {
        Command::CreateConfig => recreate_config_file(&os),
        Command::Version => print_version(),
        Command::Help => print_help(),
        Command::ListIgnoredDirectories => {
            println!(
                "Ignored Directories:\n{}",
                get_setting_from_config(ConfigOption::IgnoredDirectories, &os).value
            )
        }
        Command::ListIgnoredFiles => println!(
            "Ignored Files:\n{}",
            get_setting_from_config(ConfigOption::IgnoredFiles, &os).value
        ),
        Command::AddDirectory(dir) => {
            add_value_to_setting(ConfigOption::IgnoredDirectories, dir, &os)
        }
        Command::AddFile(file) => add_value_to_setting(ConfigOption::IgnoredFiles, file, &os),
        Command::RemoveDirectory(dir) => {
            remove_value_from_setting(ConfigOption::IgnoredDirectories, dir, &os)
        }
        Command::RemoveFile(file) => {
            remove_value_from_setting(ConfigOption::IgnoredFiles, file, &os)
        }
        Command::Scan(
            show_endings,
            show_file_sizes,
            show_directory_sizes,
            show_file_counts_in_directories,
        ) => {
            let current_dir_path = get_current_directory_path();
            let tree = scan(&current_dir_path, &os);
            println!("{}", tree.name);
            display_tree(
                &tree,
                0,
                "",
                &show_endings,
                &show_file_sizes,
                &show_directory_sizes,
                &show_file_counts_in_directories,
            );
        }
        Command::ScanPath(
            path,
            show_endings,
            show_file_sizes,
            show_directory_sizes,
            show_file_counts_in_directories,
        ) => {
            let tree = scan(&path, &os);
            println!("{}", tree.name);
            display_tree(
                &tree,
                0,
                "",
                &show_endings,
                &show_file_sizes,
                &show_directory_sizes,
                &show_file_counts_in_directories,
            );
        }
    }
}

fn recreate_config_file(os: &OS) {
    create_config_file(&os, true);
    println!("reset config file");
}

fn print_version() {
    println!("fmap v-{}", VERSION);
}

fn print_help() {
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const BLUE: &str = "\x1b[38;2;73;107;190m";
    const RESET: &str = "\x1b[0m";

    println!("{}fmap v-{}{}", GREEN, VERSION, RESET);
    println!(
        "By: {}cqb13{} - {}https://github.com/cqb13{}",
        YELLOW, RESET, BLUE, RESET
    );
    println!("A CLI tool for displaying a tree-like view of files and directories.");
    println!("");
    println!("{}Usage:{} fmap [COMMAND] [OPTIONS]", YELLOW, RESET);
    println!("");
    println!("{}Commands:{}\n", YELLOW, RESET);

    println!("{}  scan{}", GREEN, RESET);
    println!("      Scans from the current directory.");
    println!("      Options:");
    println!("        -e, --endings        Show file endings");
    println!("        -fs, --file-sizes   Show file sizes");
    println!("        -ds, --dir-sizes    Show directory sizes");
    println!("        -fc, --file-counts  Show file counts in directories\n");

    println!("{}  scan-path <path>{}", GREEN, RESET);
    println!("      Scans from a custom relative path.");
    println!("      Options:");
    println!("        -e, --endings        Show file endings");
    println!("        -fs, --file-sizes   Show file sizes");
    println!("        -ds, --dir-sizes    Show directory sizes");
    println!("        -fc, --file-counts  Show file counts in directories\n");

    println!("{}  config, -c{}", GREEN, RESET);
    println!("      Creates a configuration file.\n");

    println!("{}  add -dir <directory>", GREEN);
    println!("  add -file <filename>{}", RESET);
    println!("      Adds a directory or file to the ignored list.\n");

    println!("{}  rmv -dir <directory>", GREEN);
    println!("  rmv -file <filename>{}", RESET);
    println!("      Removes a directory or file from the ignored list.\n");

    println!("{}  ls -dir", GREEN);
    println!("  ls -file{}", RESET);
    println!("      Lists all ignored directories or files.\n");

    println!("{}  version, -v{}", GREEN, RESET);
    println!("      Prints the version.\n");

    println!("{}  help, -h{}", GREEN, RESET);
    println!("      Prints this help message.\n");

    println!("{}For more information, visit:{}", YELLOW, RESET);
    println!("  {}https://github.com/cqb13/fmap{}", BLUE, RESET);
}
