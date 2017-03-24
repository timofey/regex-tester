extern crate regex;
extern crate rustyline;

pub mod cli;
pub mod platform;
pub mod commands;

use regex::Regex;

use rustyline::Editor;

use std::fs::File;
use std::path::Path;
use std::error::Error;

static HISTORY_FN: &'static str = "history";

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

    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history") {
        let path = Path::new(HISTORY_FN);
        match File::create(&path) {
            Err(err) => panic!("Couldn't create {}: {}", path.display(), err.description()),
            Ok(file) => file
        };
    }

    loop {
        let res = cli::cli_parse_command(&mut current_re, &mut rl);
        if !res {
            break;
        }
    }

    rl.save_history(HISTORY_FN).unwrap();
}
