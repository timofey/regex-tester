use std::string::String;
use std::io;
use std::io::prelude::*;

use platform::EOL_LEN;

static CLI_WELCOME: &'static str = "#> ";

fn cli_request(command: &mut String) {
    print!("\n{welcome}", welcome = CLI_WELCOME);
    io::stdout().flush().unwrap();
    io::stdin().read_line(command).expect("Failed to read command!");
    let len = command.len();
    command.truncate(len - EOL_LEN as usize);
}

pub fn cli_parse_command() {
    let mut command = String::new();
    cli_request(&mut command);
    
    println!("Parsing '{}' ...", command);
}

