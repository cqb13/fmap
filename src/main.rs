pub mod commands;
pub mod config;
pub mod scan;
pub mod utils;
pub mod display;

use crate::commands::arg_tokenizer;
use crate::config::{
    add_value_to_setting, create_config_file, get_setting_from_config, remove_value_from_setting,
    ConfigOption,
};
use crate::scan::scan;
use crate::utils::get_current_directory_path;
use std::env;
use crate::display::display_tree;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub enum OS {
    Windows,
    Mac,
}

pub enum Command {
    Scan,
    ScanPath(String),
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
        Command::Scan => {
            let current_dir_path = get_current_directory_path();
            let tree = scan(&current_dir_path, &os);
            println!("{}", tree.name);
            display_tree(&tree, 0, "");
        }
        Command::ScanPath(path) => {
            let tree = scan(&path, &os);
            println!("{}", tree.name);
            display_tree(&tree, 0, "");
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
    println!("fmap v-{}", VERSION);
    println!("By: cqb13 - https://github.com/cqb13");
    println!("A CLI tool for displaying a tree like view of files and directories.");
    println!("");
    println!("Commands:");
    println!("fmap: scans from current directory");
    println!("-p: \"path\": a custom relative path to use instead of current directory");
    println!("-c: creates a new config file, overwriting the old one if it exists");
    println!(
        "-add -dir/-file \"directory/file name\": adds a directory or file to respective list"
    );
    println!(
        "-rmv -dir/-file \"directory/file name\": removes a directory or file from respective list"
    );
    println!("-ls -dir/-file: lists all directories or files in respective list");
    println!("-v: version");
    println!("-h: help");
    println!("");
    println!("https://github.com/cqb13/fmap");
}
