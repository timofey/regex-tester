extern crate regex;

pub mod cli;
pub mod platform;

fn main() {
    println!("Welcome to regex-tester! Start by entering a command. Type 'help' to show help page.");

    cli::cli_parse_command();
}
