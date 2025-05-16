pub mod cli;
pub mod display;
pub mod scan;
pub mod styles;
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
use cli::{Arg, Cli, CmdOption, Command};

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
        .with_command(Command::new("help", "Prints help information").with_option(
            CmdOption::new("command", "COMMAND", "The command you want help with").optional(),
        ))
        .with_command(Command::new("version", "Prints version information"))
        .with_command(Command::new("config", "Creates a new config file"))
        .with_command(Command::new("install", "Installs fmap"))
        .with_command(
            Command::new("add", "Add a file or directory to the ignore list")
                .with_option(CmdOption::new(
                    "type",
                    "TYPE",
                    "file (file) or directory (dir)",
                ))
                .with_option(CmdOption::new(
                    "name",
                    "NAME",
                    "The name of the file/file ending or directory",
                )),
        )
        .with_command(
            Command::new("remove", "Remove a file or directory to the ignore list")
                .with_option(CmdOption::new(
                    "type",
                    "TYPE",
                    "file (file) or directory (dir)",
                ))
                .with_option(CmdOption::new(
                    "name",
                    "NAME",
                    "The name of the file/file ending or directory",
                )),
        )
        .with_command(
            Command::new("list", "List all files or directories in the ignore list").with_option(
                CmdOption::new("type", "TYPE", "file (file) or directory (dir)"),
            ),
        )
        .with_command(
            Command::new("scan", "Scan a directory")
                .with_option(CmdOption::new("path", "PATH", "path to a directory").optional())
                .with_arg(Arg::new(
                    "no extentions",
                    "Disable file extentions",
                    "",
                    'x',
                ))
                .with_arg(Arg::new("file sizes", "Show file sizes", "", 'f'))
                .with_arg(Arg::new("directory sizes", "Show directory sizes", "", 'd'))
                .with_arg(Arg::new(
                    "file count",
                    "Show the file count in a directory",
                    "",
                    'c',
                )),
        );

    let command = cli.match_commands();

    match command.name.as_str() {
        "help" => {
            let command = command.get_option("command").to_option();
            cli.help(command.as_deref())
        }
        "version" => cli.version(),
        "config" => recreate_config_file(&os),
        "install" => install(&os),
        "add" => {
            let list_type = command.get_option("type").throw_if_none();
            let name = command.get_option("name").throw_if_none();

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
            let list_type = command.get_option("type").throw_if_none();
            let name = command.get_option("name").throw_if_none();

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
            let list_type = command.get_option("type").throw_if_none();

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
            let raw_path = command.get_option("path").to_option();
            let show_endings = command.has("no extentions");
            let show_file_sizes = command.has("file sizes");
            let show_dir_sizes = command.has("directory sizes");
            let show_file_count = command.has("show_file_count");

            let path = match raw_path {
                Some(path) => path,
                None => get_current_directory_path(),
            };

            let tree = scan(&path, &os);

            display(
                &tree,
                &!show_endings,
                &show_file_sizes,
                &show_dir_sizes,
                &show_file_count,
                &os,
            );
        }
        _ => cli.help(None),
    }
}

fn recreate_config_file(os: &OS) {
    create_config_file(&os, true);
    println!("reset config file");
}
