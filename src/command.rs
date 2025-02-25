use std::process::Command;
use colored::*;

pub fn execute_command(command: &str, args: &[&str]) {
    match Command::new(command).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.is_empty() {
                    eprintln!("{}", stderr.red());
                }
                eprintln!("{}", format!("Command failed with exit code: {}", output.status).red());
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to execute command '{}': {}", command, e).red());
        }
    }
}
