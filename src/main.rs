pub mod commands;
pub mod config;
pub mod display;
pub mod scan;
pub mod utils;

use crate::commands::arg_tokenizer;
use crate::config::{
    add_value_to_setting, create_config_file, get_setting_from_config, get_user_home_dir,
    remove_value_from_setting, ConfigOption,
};
use crate::display::display;
use crate::scan::scan;
use crate::utils::get_current_directory_path;
use std::env;
use std::io::Write;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub enum OS {
    Windows,
    Mac,
}

impl OS {
    fn get_name(&self) -> &str {
        match self {
            OS::Windows => "Windows",
            OS::Mac => "Mac",
        }
    }
}

pub enum Command {
    Scan(bool, bool, bool, bool),
    ScanPath(String, bool, bool, bool, bool),
    CreateConfig,
    Install,
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
        Command::Install => install(&os),
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
            display(
                &tree,
                &show_endings,
                &show_file_sizes,
                &show_directory_sizes,
                &show_file_counts_in_directories,
                &os,
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
            display(
                &tree,
                &show_endings,
                &show_file_sizes,
                &show_directory_sizes,
                &show_file_counts_in_directories,
                &os,
            );
        }
    }
}

fn recreate_config_file(os: &OS) {
    create_config_file(&os, true);
    println!("reset config file");
}

fn install(os: &OS) {
    println!("starting install on {}", os.get_name());

    let home_dir = get_user_home_dir(os);

    match os {
        OS::Windows => {
            let app_data_path = format!("{}/AppData/Roaming/fmap", home_dir);
            if !std::path::Path::new(&app_data_path).exists() {
                println!("Creating AppData/Roaming/fmap directory");
                std::fs::create_dir_all(&app_data_path).unwrap();
            }

            let new_binary_path = format!("{}/fmap.exe", app_data_path);
            if !std::path::Path::new(&new_binary_path).exists() {
                println!("Moving binary to AppData/Roaming/fmap");
                std::fs::copy(
                    format!("{}/fmap.exe", get_current_directory_path()),
                    &new_binary_path,
                )
                .unwrap();
            } else {
                println!("Replacing binary in AppData/Roaming/fmap");
                std::fs::remove_file(&new_binary_path).unwrap();
                std::fs::copy(
                    format!("{}/fmap.exe", get_current_directory_path()),
                    &new_binary_path,
                )
                .unwrap();
            }

            if let Err(e) = modify_registry_path(&app_data_path) {
                eprintln!("Failed to modify system PATH: {}", e);
                eprintln!("This action may require administrator permissions.");
            }
        }
        OS::Mac => {
            let local_bin_path = format!("{}/.local/bin", home_dir);
            if !std::path::Path::new(&local_bin_path).exists() {
                println!("creating .local/bin directory");
                std::fs::create_dir_all(&local_bin_path).unwrap();
            }

            let new_binary_path = format!("{}/fmap", local_bin_path);
            if !std::path::Path::new(&new_binary_path).exists() {
                println!("moving binary to .local/bin");
                std::fs::copy(
                    format!("{}/fmap", get_current_directory_path()),
                    &new_binary_path,
                )
                .unwrap();
            } else {
                println!("replacing binary in .local/bin");
                std::fs::remove_file(&new_binary_path).unwrap();
                std::fs::copy(
                    format!("{}/fmap", get_current_directory_path()),
                    &new_binary_path,
                )
                .unwrap();
            }
            let zprofile_path = format!("{}/.zprofile", home_dir);
            let zprofile_content = std::fs::read_to_string(&zprofile_path).unwrap();
            if !zprofile_content.contains("export PATH=\"$PATH:$HOME/.local/bin\"") {
                println!("adding .local/bin to path");
                let mut zprofile_file = std::fs::OpenOptions::new()
                    .append(true)
                    .open(&zprofile_content)
                    .unwrap();
                zprofile_file
                    .write_all(b"export PATH=\"$PATH:$HOME/.local/bin\"\n")
                    .unwrap();
            }
        }
    }

    println!("install complete");
}

fn modify_registry_path(new_path: &str) -> std::io::Result<()> {
    use std::process::Command;

    // Escape percent signs by doubling them
    let escaped_path = new_path.replace("%", "%%");

    // Prepare the command to modify the registry
    let status = Command::new("reg")
        .args(&[
            "ADD",
            "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
            "/v",
            "Path",
            "/t",
            "REG_EXPAND_SZ",
            "/d",
            &escaped_path,
            "/f",
        ])
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to modify registry",
        ));
    }

    Ok(())
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
