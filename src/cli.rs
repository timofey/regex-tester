use std::string::String;
use std::io;
use std::io::prelude::*;

use platform::EOL_LEN;
use commands::*;

static CLI_WELCOME: &'static str = "#> ";

fn cli_request(command: &mut String) {
    print!("\n{welcome}", welcome = CLI_WELCOME);
    io::stdout().flush().unwrap();
    io::stdin().read_line(command).expect("Failed to read command!");
    let len = command.len();
    command.truncate(len - EOL_LEN as usize);
}

fn parse_and_execute(command: & String) -> String {
    match command.as_ref() {
        "help" => show_help(),
        _ => default_command()
    }
}

pub fn cli_parse_command() -> bool {
    let mut command = String::new();
    cli_request(&mut command);

    if command == "exit" {
        return false;
    }
    let result = parse_and_execute(&command);
    println!("{}", result);

    true
}

