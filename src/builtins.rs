use std::env;
use std::process;
use colored::*;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::collections::VecDeque;
use lazy_static::lazy_static;

lazy_static! {
    static ref DIR_STACK: std::sync::Mutex<VecDeque<PathBuf>> = std::sync::Mutex::new(VecDeque::new());
}

pub fn is_builtin(command: &str) -> bool {
    matches!(command, "cd" | "exit" | "help" | "clear" | "pushd" | "popd" | "dirs")
}

fn get_dir_stack() -> std::sync::MutexGuard<'static, VecDeque<PathBuf>> {
    DIR_STACK.lock().unwrap()
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
            println!("  {} - Clear the terminal screen", "clear".green());
            println!("  {} - Show directory stack", "dirs".green());
            println!("  {} - Exit the shell", "exit".green());
            println!("  {} - Show this help message", "help".green());
            println!("  {} - Pop directory from stack and cd to it", "popd".green());
            println!("  {} - Push current directory to stack and cd to new one", "pushd".green());
        }
        "clear" => {
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().unwrap();
        }
        "pushd" => {
            if args.is_empty() {
                eprintln!("{}", "pushd: no directory specified".red());
                return;
            }

            let new_dir = Path::new(args[0]);
            if !new_dir.exists() {
                eprintln!("{}", format!("pushd: directory not found: {}", args[0]).red());
                return;
            }

            let current_dir = env::current_dir().unwrap();
            if env::set_current_dir(new_dir).is_err() {
                eprintln!("{}", format!("pushd: failed to change directory: {}", args[0]).red());
                return;
            }

            get_dir_stack().push_front(current_dir);
            print_dirs();
        }
        "popd" => {
            let mut stack = get_dir_stack();
            match stack.pop_front() {
                Some(dir) => {
                    if env::set_current_dir(&dir).is_err() {
                        eprintln!("{}", format!("popd: failed to change to directory: {}", dir.display()).red());
                        stack.push_front(dir);
                        return;
                    }
                    print_dirs();
                }
                None => eprintln!("{}", "popd: directory stack empty".red()),
            }
        }
        "dirs" => {
            print_dirs();
        }
        _ => {
            eprintln!("{}", format!("Unknown built-in command: {}", command).red());
        }
    }
}

fn print_dirs() {
    let current = env::current_dir().unwrap();
    print!("{}", current.display().to_string().bright_blue());
    
    for dir in get_dir_stack().iter() {
        print!(" {}", dir.display().to_string().blue());
    }
    println!();
}
