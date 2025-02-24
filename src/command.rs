use std::process::{Command, ExitStatus};

pub fn execute_command(command: &str, args: &[&str]) -> ExitStatus {
    let status = Command::new(command)
        .args(args)
        .status()
        .expect("failed to execute process");

    if !status.success() {
        println!("Command failed with status: {}", status);
    }

    status
}
