use std::env;
use std::process;
use colored::*;

pub fn is_builtin(command: &str) -> bool {
    matches!(command, "cd" | "exit" | "help")
}

pub fn execute_builtin(command: &str, args: &[&str]) {
    match command {
        "cd" => {
            if args.is_empty() {
                if let Some(home) = env::var_os("HOME") {
                    if let Err(e) = env::set_current_dir(&home) {
                        eprintln!("{}", format!("Failed to change directory: {}", e).red());
                    }
                }
            } else {
                if let Err(e) = env::set_current_dir(args[0]) {
                    eprintln!("{}", format!("Failed to change directory: {}", e).red());
                }
            }
        }
        "exit" => {
            println!("{}", "Goodbye!".bright_yellow());
            process::exit(0);
        }
        "help" => {
            println!("{}", "Available built-in commands:".bright_cyan());
            println!("  {} - Change directory", "cd".green());
            println!("  {} - Exit the shell", "exit".green());
            println!("  {} - Show this help message", "help".green());
        }
        _ => {
            eprintln!("{}", format!("Unknown built-in command: {}", command).red());
        }
    }
}
