use std::string::String;
use std::fmt::Write;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use commands::*;
use REHolder;

static CLI_PROMPT: &'static str = "#> ";

fn cli_request(mut command: &mut String, mut reader: &mut Editor<()>) -> bool {

    let input = reader.readline(CLI_PROMPT);
    match input {
        Ok(line) => {
            reader.add_history_entry(&line);
            command.clear();
            command.write_str(line.as_str()).unwrap();
            return true;
        },
        Err(ReadlineError::Interrupted) => {
            return false;
        },
        Err(ReadlineError::Eof) => {
            command.clear();
            command.write_str("exit").unwrap();
            return true;
        },
        Err(err) => {
            panic!("Error: {:?}", err);
        }
    }
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

pub fn cli_parse_command(mut current_re: &mut REHolder, mut reader: &mut Editor<()>) -> bool {
    let mut command = String::new();
    let res = cli_request(&mut command, &mut reader);

    if !res {
        return false;
    }

    if command == "exit" {
        println!("Bye!");
        return false;
    }

    if command.len() > 0 {
        let result = parse_and_execute(&command, &mut current_re);
        println!("{}", result);
    }

    true
}

