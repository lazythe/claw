use std::env;
use std::process::exit;

pub fn is_builtin(command: &str) -> bool {
    match command {
        "cd" | "exit" => true,
        _ => false,
    }
}

pub fn execute_builtin(command: &str, args: &[&str]) {
    match command {
        "cd" => {
            if args.len() == 1 {
                if let Err(e) = env::set_current_dir(args[0]) {
                    println!("cd: {}: {}", args[0], e);
                }
            }
        }
        "exit" => {
            exit(0);
        }
        _ => println!("Unknown command"),
    }
}
