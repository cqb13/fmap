pub mod commands;
pub mod config;

use crate::commands::arg_tokenizer;
use crate::config::{create_config_file, get_setting_from_config, ConfigOption};
use std::env;

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

/**
 * Args:
 * -c: creates a new config file
 * -add -dir/-file "directory/file name": adds a directory or file to respective list
 * -rmv -dir/-file "directory/file name": removes a directory or file from respective list
 * -ls -dir/-file: lists all directories or files in respective list
 * -v: version
 * -h: help
 * -p: "path": a custom relative path to use instead of current directory
 *
 * nothing: scans from current directory
 */

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
        _ => (),
    }
}

fn recreate_config_file(os: &OS) {
    create_config_file(&os, true);
    println!("reset config file");
}

fn exit_with_error(error: &str, show_help: bool) {
    println!("{}", error);
    if show_help {
        println!("use -h for a list of all commands");
    }
    std::process::exit(0);
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
    println!("fmap -p: \"path\": a custom relative path to use instead of current directory");
    println!("fmap -c: creates a new config file, overwriting the old one if it exists");
    println!(
        "fmap -add -dir/-file \"directory/file name\": adds a directory or file to respective list"
    );
    println!("fmap -rmv -dir/-file \"directory/file name\": removes a directory or file from respective list");
    println!("fmap -ls -dir/-file: lists all directories or files in respective list");
    println!("fmap -v: version");
    println!("fmap -h: help");
    println!("");
    println!("https://github.com/cqb13/fmap");
}
