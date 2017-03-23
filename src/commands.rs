use std::string::String;

use platform::EOL;

pub fn show_help() -> String {
    "This is help page.".to_string() + EOL.as_ref() +
    "Or at leas will be....".as_ref()
}

pub fn default_command() -> String {
    "No such command was found. Type 'help' to show help page.".to_string()
}
