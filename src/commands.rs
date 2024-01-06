use crate::utils::exit_with_error;
use crate::Command;

pub fn arg_tokenizer(mut args: Vec<String>) -> Command {
    // removes target/debug/fmap
    args.remove(0);

    if args.is_empty() {
        println!("scan");
        return Command::Scan;
    }

    match args[0].as_str() {
        "-c" => return Command::CreateConfig,
        "-v" => return Command::Version,
        "-h" => return Command::Help,
        "-add" => {
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
        "-rmv" => {
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
        "-ls" => {
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
        "-p" => {
            if args.len() < 2 {
                exit_with_error("a required argument is missing", true)
            } else if args.len() > 2 {
                exit_with_error("too many arguments found", true)
            }

            return Command::ScanPath(args[1].to_string());
        }
        _ => {
            exit_with_error("unknown command", true);
        }
    };

    Command::Scan
}
