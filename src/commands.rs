use std::string::String;

use regex::Regex;

use platform::EOL;
use REHolder;

pub fn show_help() -> String {
    "This is help page.".to_string() + EOL.as_ref() +
    "Or at leas will be....".as_ref()
}

pub fn set_regex(regex: &str, current_re: &mut REHolder) -> String {
    current_re.set(Regex::new(regex).unwrap());
    format!("Regex was set: {}\n", regex)
}

pub fn default_command() -> String {
    "No such command was found. Type 'help' to show help page.".to_string()
}
