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

pub fn test_input(input: &str, current_re: &REHolder) -> String {
    if !current_re.re.is_match(input) {
        return "Input does not match the pattern. ".to_string();
    }

    let mut output = String::new();

    for (ind, cap) in current_re.re.captures_iter(input).enumerate() {
        if ind > 0 {
            output += format!("==========={}", EOL).as_ref();
        }
        output += format!(">>> Match #{}, capture groups: {}", ind + 1, EOL).as_ref();
        for c_ind in 0..cap.len() {
            let cg = cap.get(c_ind).unwrap();
            output += format!(">>> <{}>: {}{}", c_ind, cg.as_str(), EOL).as_ref();
        }
    }
    output
}

pub fn default_command() -> String {
    "No such command was found. Type 'help' to show help page.".to_string()
}
