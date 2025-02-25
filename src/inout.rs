use std::{
    fs::File,
    process::{Command, Stdio},
};
use colored::*;

pub fn handle_output_redirection(command: &str, args: &[&str], output_file: Option<&str>) {
    let mut cmd = Command::new(command);
    cmd.args(args);

    if let Some(file) = output_file {
        match File::create(file) {
            Ok(file) => {
                cmd.stdout(Stdio::from(file));
            }
            Err(e) => {
                eprintln!("{}", format!("Failed to create file for redirection: {}", e).red());
                return;
            }
        }
    }

    let status = cmd.status();
    match status {
        Ok(status) => {
            if !status.success() {
                eprintln!("{}", format!("Command failed with status: {}", status).red());
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to execute command '{}': {}", command, e).red());
        }
    }
}

pub fn handle_input_redirection(command: &str, args: &[&str], input_file: Option<&str>) {
    let mut cmd = Command::new(command);
    cmd.args(args);

    if let Some(file) = input_file {
        match File::open(file) {
            Ok(f) => {
                cmd.stdin(Stdio::from(f));
            }
            Err(e) => {
                eprintln!("{}", format!("Error opening file '{}': {}", file, e).red());
                return;
            }
        }
    }

    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if !stdout.is_empty() {
                print!("{}", stdout);
            }

            if !stderr.is_empty() {
                eprint!("{}", stderr.red());
            }

            if !output.status.success() {
                eprintln!("{}", format!("Command failed with status: {}", output.status).red());
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to execute command '{}': {}", command, e).red());
        }
    }
}

pub fn handle_pipe(command1: &str, args1: &[&str], command2: &str, args2: &[&str]) {
    let mut child1 = match Command::new(command1)
        .args(args1)
        .stdout(Stdio::piped())
        .spawn() {
            Ok(child) => child,
            Err(e) => {
                eprintln!("{}", format!("Failed to start process '{}': {}", command1, e).red());
                return;
            }
        };

    let mut child2 = match Command::new(command2)
        .args(args2)
        .stdin(child1.stdout.take().expect("Failed to open pipe"))
        .spawn() {
            Ok(child) => child,
            Err(e) => {
                eprintln!("{}", format!("Failed to start process '{}': {}", command2, e).red());
                return;
            }
        };

    if let Err(e) = child1.wait() {
        eprintln!("{}", format!("Failed to wait on process '{}': {}", command1, e).red());
    }

    if let Err(e) = child2.wait() {
        eprintln!("{}", format!("Failed to wait on process '{}': {}", command2, e).red());
    }
}