extern crate regex;

pub mod cli;
pub mod platform;
pub mod commands;

use regex::Regex;

pub struct REHolder {
    re: Regex
}

impl REHolder {

    pub fn set(&mut self, regex: Regex) {
        self.re = regex;
    }
}

fn main() {
    println!("============================================================================================");
    println!("=== Welcome to regex-tester! Start by entering a command. Type 'help' to show help page. ===");
    println!("============================================================================================");

    let mut current_re: REHolder = REHolder{ re: Regex::new("").unwrap() };

    loop {
        let res = cli::cli_parse_command(&mut current_re);
        if !res {
            break;
        }
    }

    println!("Last re was: {:?}", current_re.re);
    println!("Bye!");
}
