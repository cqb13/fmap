pub mod cli;
pub mod display;
pub mod scan;
pub mod styles;

use cli::{Arg, Cli, CmdOption, Command};
use std::env;

fn main() {
    let cli = Cli::new().with_command(
        Command::new("test", "testing")
            .with_option(CmdOption::new("opt", "OPT", "opt thing"))
            .with_arg(Arg::new("arg", "arg d", "", 'a'))
            .with_arg(Arg::new("argB", "arg b", "", 'b')),
    );

    let command = cli.match_commands();

    match command.name.as_str() {
        "test" => {
            let opt = command.get_option("test").to_option();
            let arg_a = command.has("arg");
            let arg_b = command.has("argB");

            println!("{:?}, {}, {}", opt, arg_a, arg_b);
        }
        _ => {}
    }
}
