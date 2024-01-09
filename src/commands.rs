use crate::utils::exit_with_error;
use crate::Command;

pub fn arg_tokenizer(mut args: Vec<String>) -> Command {
    // removes target/debug/fmap
    args.remove(0);

    if args.is_empty() {
        return Command::Scan(true, false, false, false);
    }

    match args[0].as_str() {
        "config" | "-c" => return Command::CreateConfig,
        "version" | "-v" => return Command::Version,
        "install" | "-i" => return Command::Install,
        "help" | "-h" => return Command::Help,
        "add" | "-add" => {
            if args.len() < 3 {
                exit_with_error("a required argument is missing", true)
            } else if args.len() > 3 {
                exit_with_error("too many arguments found", true)
            }

            match args[1].as_str() {
                "-dir" => {
                    return Command::AddDirectory(args[2].to_string());
                }
                "-file" => {
                    return Command::AddFile(args[2].to_string());
                }
                _ => exit_with_error("invalid subcommand", true),
            }
        }
        "remove" | "-rmv" => {
            if args.len() < 3 {
                exit_with_error("a required argument is missing", true)
            } else if args.len() > 3 {
                exit_with_error("too many arguments found", true)
            }

            match args[1].as_str() {
                "-dir" => {
                    return Command::RemoveDirectory(args[2].to_string());
                }
                "-file" => {
                    return Command::RemoveFile(args[2].to_string());
                }
                _ => exit_with_error("invalid subcommand", true),
            }
        }
        "list" | "-ls" => {
            if args.len() < 2 {
                exit_with_error("a required argument is missing", true)
            } else if args.len() > 2 {
                exit_with_error("too many arguments found", true)
            }

            match args[1].as_str() {
                "-dir" => {
                    return Command::ListIgnoredDirectories;
                }
                "-file" => {
                    return Command::ListIgnoredFiles;
                }
                _ => exit_with_error("invalid subcommand", true),
            }
        }
        "scan" | "-s" => {
            let mut show_endings = false;
            let mut show_file_sizes = false;
            let mut show_directory_sizes = false;
            let mut show_file_counts_in_directories = false;

            if args.len() > 1 {
                for arg in args[1..].iter() {
                    match arg.as_str() {
                        "-e" => show_endings = true,
                        "-fs" => show_file_sizes = true,
                        "-ds" => show_directory_sizes = true,
                        "-fc" => show_file_counts_in_directories = true,
                        _ => exit_with_error("invalid subcommand", true),
                    }
                }
            }

            /*
            optional_extra_args:
            -e: show file endings
            -fs: show file sizes
            -fc: show file count on directory
            -ds: show directory sizes
             */

            return Command::Scan(
                show_endings,
                show_file_sizes,
                show_directory_sizes,
                show_file_counts_in_directories,
            );
        }
        "scan-path" | "-sp" => {
            if args.len() < 2 {
                exit_with_error("a required argument is missing", true)
            }

            let mut show_endings = false;
            let mut show_file_sizes = false;
            let mut show_directory_sizes = false;
            let mut show_file_counts_in_directories = false;

            if args.len() > 2 {
                for arg in args[2..].iter() {
                    match arg.as_str() {
                        "-e" => show_endings = true,
                        "-fs" => show_file_sizes = true,
                        "-ds" => show_directory_sizes = true,
                        "-fc" => show_file_counts_in_directories = true,
                        _ => exit_with_error("invalid subcommand", true),
                    }
                }
            }

            /*
            optional_extra_args:
            -e: show file endings
            -fs: show file sizes
            -fc: show file count on directory
            -ds: show directory sizes
             */

            return Command::ScanPath(
                args[1].to_string(),
                show_endings,
                show_file_sizes,
                show_directory_sizes,
                show_file_counts_in_directories,
            );
        }
        _ => {
            exit_with_error("unknown command", true);
        }
    };

    Command::Help
}
