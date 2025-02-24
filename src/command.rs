use std::process::Command;
use colored::*;

pub fn execute_command(command: &str, args: &[&str]) {
    let output = Command::new(command)
        .args(args)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
            } else {
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr).red());
                }
                eprintln!("{}", format!("Command failed with exit code: {}", output.status).red());
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to execute command: {}", e).red());
        }
    }
}
