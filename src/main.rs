pub mod cli;
pub mod display;
pub mod scan;
pub mod utils;

pub mod system {
    pub mod config;
    pub mod local;
}

use crate::display::display;
use crate::scan::scan;
use crate::system::config::{
    add_value_to_setting, create_config_file, get_setting_from_config, remove_value_from_setting,
    ConfigOption,
};
use crate::system::local::install;
use crate::utils::get_current_directory_path;
use cli::{Arg, Cli, Command};

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

fn main() {
    let os = match std::env::consts::OS {
        "windows" => OS::Windows,
        "macos" => OS::Mac,
        _ => panic!("Unsupported OS"),
    };

    create_config_file(&os, false);

    let cli = Cli::new()
        .with_default_command("scan")
        .with_commands(vec![
            Command::new("config", "Creates a new config file").with_short('c'),
            Command::new("version", "Displays the current version of fmap").with_short('v'),
            Command::new("install", "Installs the files and directories"),
            Command::new("add", "Adds a file or directory to the ignore list")
                .with_short('a')
                .with_arg(
                    Arg::new()
                        .with_name("type")
                        .with_short('t')
                        .with_long("type")
                        .with_value_name("TYPE")
                        .with_help("file (file) or directory (dir)"),
                )
                .with_arg(
                    Arg::new()
                        .with_name("name")
                        .with_short('n')
                        .with_long("name")
                        .with_value_name("NAME")
                        .with_help("The name of the file/file ending or directory"),
                ),
            Command::new("remove", "Removes a file or directory to the ignore list")
                .with_short('r')
                .with_arg(
                    Arg::new()
                        .with_name("type")
                        .with_short('t')
                        .with_long("type")
                        .with_value_name("TYPE")
                        .with_help("file (file) or directory (dir)"),
                )
                .with_arg(
                    Arg::new()
                        .with_name("name")
                        .with_short('n')
                        .with_long("name")
                        .with_value_name("NAME")
                        .with_help("The name of the file/file ending or directory"),
                ),
            Command::new("list", "Lists all files or directories in the ignore list")
                .with_short('r')
                .with_arg(
                    Arg::new()
                        .with_name("type")
                        .with_short('t')
                        .with_long("type")
                        .with_value_name("TYPE")
                        .with_help("file (file) or directory (dir)"),
                ),
            Command::new("scan", "Scans the current directory")
                .with_short('s')
                .with_arg(
                    Arg::new()
                        .with_name("endings")
                        .with_short('e')
                        .with_long("endings")
                        .with_help("Shows the file endings"),
                )
                .with_arg(
                    Arg::new()
                        .with_name("file_sizes")
                        .with_short('f')
                        .with_long("file-sizes")
                        .with_help("Shows the file sizes"),
                )
                .with_arg(
                    Arg::new()
                        .with_name("directory_sizes")
                        .with_short('d')
                        .with_long("directory-sizes")
                        .with_help("Shows the directory sizes"),
                )
                .with_arg(
                    Arg::new()
                        .with_name("file_counts_in_directories")
                        .with_short('c')
                        .with_long("file-counts-in-directories")
                        .with_help("Shows the file counts in directories"),
                ),
            Command::new("scan-path", "Scans a specific path")
                .with_short('p')
                .with_arg(
                    Arg::new()
                        .with_name("path")
                        .with_short('p')
                        .with_long("path")
                        .with_value_name("PATH")
                        .with_help("The path to scan"),
                )
                .with_arg(
                    Arg::new()
                        .with_name("endings")
                        .with_short('e')
                        .with_long("endings")
                        .with_help("Shows the file endings"),
                )
                .with_arg(
                    Arg::new()
                        .with_name("file_sizes")
                        .with_short('f')
                        .with_long("file-sizes")
                        .with_help("Shows the file sizes"),
                )
                .with_arg(
                    Arg::new()
                        .with_name("directory_sizes")
                        .with_short('d')
                        .with_long("directory-sizes")
                        .with_help("Shows the directory sizes"),
                )
                .with_arg(
                    Arg::new()
                        .with_name("file_counts_in_directories")
                        .with_short('c')
                        .with_long("file-counts-in-directories")
                        .with_help("Shows the file counts in directories"),
                ),
            Command::new("help", "Helps you with the commands").with_short('h'),
        ]);

    let command = cli.match_commands();

    match command.name {
        "config" => {
            recreate_config_file(&os);
        }
        "version" => {
            cli.version();
        }
        "install" => {
            install(&os);
        }
        "add" => {
            let list_type = command.get_value_of("type").throw_if_none();
            let name = command.get_value_of("name").throw_if_none();

            match list_type.as_str() {
                "file" => {
                    add_value_to_setting(ConfigOption::IgnoredFiles, &name, &os);
                }
                "dir" => {
                    add_value_to_setting(ConfigOption::IgnoredDirectories, &name, &os);
                }
                _ => {
                    println!("Invalid list type, must be \"file\" or \"dir\"");
                }
            }
        }
        "remove" => {
            let list_type = command.get_value_of("type").throw_if_none();
            let name = command.get_value_of("name").throw_if_none();

            match list_type.as_str() {
                "file" => {
                    remove_value_from_setting(ConfigOption::IgnoredFiles, &name, &os);
                }
                "dir" => {
                    remove_value_from_setting(ConfigOption::IgnoredDirectories, &name, &os);
                }
                _ => {
                    println!("Invalid list type, must be \"file\" or \"dir\"");
                }
            }
        }
        "list" => {
            let list_type = command.get_value_of("type").throw_if_none();

            match list_type.as_str() {
                "file" => {
                    let files = get_setting_from_config(ConfigOption::IgnoredFiles, &os).value;
                    println!("Files: {:?}", files);
                }
                "dir" => {
                    let directories =
                        get_setting_from_config(ConfigOption::IgnoredDirectories, &os).value;
                    println!("Directories: {:?}", directories);
                }
                _ => {
                    println!("Invalid list type, must be \"file\" or \"dir\"");
                }
            }
        }
        "scan" => {
            let show_endings = command.has("endings");
            let show_file_sizes = command.has("file_sizes");
            let show_directory_sizes = command.has("directory_sizes");
            let show_file_counts_in_directories = command.has("file_counts_in_directories");

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
        "scan-path" => {
            let path = command.get_value_of("path").throw_if_none();
            let show_endings = command.has("endings");
            let show_file_sizes = command.has("file_sizes");
            let show_directory_sizes = command.has("directory_sizes");
            let show_file_counts_in_directories = command.has("file_counts_in_directories");

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
        "help" => {
            cli.help();
        }
        _ => {
            println!("Invalid command");
        }
    }
}

fn recreate_config_file(os: &OS) {
    create_config_file(&os, true);
    println!("reset config file");
}
