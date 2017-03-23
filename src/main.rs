extern crate regex;

pub mod cli;
pub mod platform;
pub mod commands;

fn main() {
    println!("============================================================================================");
    println!("=== Welcome to regex-tester! Start by entering a command. Type 'help' to show help page. ===");
    println!("============================================================================================");

    loop {
        let res = cli::cli_parse_command();
        if !res {
            break;
        }
    }

    println!("Bye!");
}
