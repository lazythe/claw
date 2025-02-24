mod builtins;
mod command;
mod inout;

use std::io::{self, Write};
use users::get_current_username;
use colored::*;

fn main() {
    loop {
        let username = get_current_username().unwrap();
        print!("[{}] {}> ", username.to_string_lossy().bright_green(), "~".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.contains(">") || input.contains("<") {
                    handle_redirection(input);
                } else if input.contains("|") {
                    handle_pipe(input);
                } else {
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
            Err(error) => {
                eprintln!("{}", format!("Error reading input: {}", error).red());
            }
        }
    }
}

fn handle_redirection(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if let Some(output_idx) = parts.iter().position(|&x| x == ">") {
        let (command_args, output_file) = parts.split_at(output_idx);
        let output_file = output_file.last().unwrap();

        inout::handle_output_redirection(command_args[0], &command_args[1..], Some(output_file));
    } else if let Some(input_idx) = parts.iter().position(|&x| x == "<") {
        let (command_args, input_file) = parts.split_at(input_idx);
        let input_file = input_file.last().unwrap();

        inout::handle_input_redirection(command_args[0], &command_args[1..], Some(input_file));
    }
}

fn handle_pipe(input: &str) {
    let parts: Vec<&str> = input.split("|").collect();
    let command1 = parts[0].trim();
    let command2 = parts[1].trim();

    let tokens1: Vec<&str> = command1.split_whitespace().collect();
    let tokens2: Vec<&str> = command2.split_whitespace().collect();

    inout::handle_pipe(tokens1[0], &tokens1[1..], tokens2[0], &tokens2[1..]);
}
