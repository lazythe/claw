mod builtins;
mod command;
mod inout;

use users::get_current_username;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::env;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    
    if rl.load_history("history.txt").is_err() {
        println!("{}", "No previous history.".yellow());
    }

    loop {
        let username = get_current_username().unwrap();
        let prompt = format!("[{}] {} {}> ", username.to_string_lossy().bright_green(), env::current_dir().unwrap().display().to_string().bright_blue(), "~".bright_blue());
        
        match rl.readline(&prompt) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                if input.contains(">") || input.contains("<") {
                    handle_redirection(input);
                } else if input.contains("|") {
                    handle_pipe(input);
                } else {
                    let tokens: Vec<&str> = input.split_whitespace().collect();
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
            Err(ReadlineError::Interrupted) => {
                println!("{}", "^C".red());
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "exit".bright_yellow());
                break;
            }
            Err(err) => {
                eprintln!("{}", format!("Error: {}", err).red());
                break;
            }
        }
    }

    if let Err(err) = rl.save_history("history.txt") {
        eprintln!("{}", format!("Error saving history: {}", err).red());
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
