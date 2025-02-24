#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::process::Command;
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

        let status = Command::new(command)
            .args(args)
            .status()
            .expect("failed to execute process");

        if !status.success() {
            println!("command failed with status: {}", status);
        }
    }
}
