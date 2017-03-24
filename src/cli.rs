use std::string::String;
use std::io;
use std::io::prelude::*;

use platform::EOL_LEN;
use commands::*;
use REHolder;

static CLI_PROMPT: &'static str = "#> ";

fn cli_request(command: &mut String) {
    print!("{welcome}", welcome = CLI_PROMPT);
    io::stdout().flush().unwrap();
    io::stdin().read_line(command).expect("Failed to read command!");
    let len = command.len();
    command.truncate(len - EOL_LEN as usize);
}

fn parse_and_execute(command: &String, mut current_re: &mut REHolder) -> String {

    let space_pos = match command.find(' ') {
        None => command.len(),
        Some(pos) => pos
    };

    let (com, arg) = command.split_at(space_pos);
    let arg = arg.trim();

    match com.as_ref() {
        "help" | "?" => show_help(),
        "re" => set_regex(&arg, &mut current_re),
        "test" => test_input(&arg, &current_re),
        _ => default_command()
    }
}

pub fn cli_parse_command(mut current_re: &mut REHolder) -> bool {
    let mut command = String::new();
    cli_request(&mut command);

    if command == "exit" {
        return false;
    }

    if command.len() > 0 {
        let result = parse_and_execute(&command, &mut current_re);
        println!("{}", result);
    }

    true
}

