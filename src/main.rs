#[allow(unused_imports)]
mod builtins;
mod command;

use std::io::{self, Write};
use users::get_current_username;

fn main() {
    loop {
        print!("[{:?}] ~> ", get_current_username().unwrap());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        let command = tokens[0];
        let args = &tokens[1..];

        if builtins::is_builtin(command) {
            builtins::execute_builtin(command, args);
        } else {
            command::execute_command(command, args);
        }
    }
}
