use std::{
    fs::File,
    process::{Command, Stdio},
};

pub fn handle_output_redirection(command: &str, args: &[&str], output_file: Option<&str>) {
    let mut cmd = Command::new(command);
    cmd.args(args);

    if let Some(file) = output_file {
        let file = File::create(file).expect("Failed to create file for redirection");
        cmd.stdout(Stdio::from(file));
    }

    let status = cmd.status().expect("Failed to execute command");
    if !status.success() {
        println!("Command failed with status {}", status);
    }
}

pub fn handle_input_redirection(command: &str, args: &[&str], input_file: Option<&str>) {
    let mut cmd = Command::new(command);
    cmd.args(args);

    if let Some(file) = input_file {
        match File::open(file) {
            Ok(f) => {
                println!(
                    "Input file '{}' successfully opened for stdin redirection",
                    file
                );
                cmd.stdin(Stdio::from(f));
            }
            Err(e) => {
                eprintln!("{}:{}", file, e);
                return;
            }
        }
    }

    let output = cmd.output().expect("Failed to execute command");

    println!("Command exited with status: {}", output.status);
    if !output.stdout.is_empty() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("Command output: {}", output_str);
    }

    if !output.stderr.is_empty() {
        let error_str = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error output: {}", error_str);
    }
}

pub fn handle_pipe(command1: &str, args1: &[&str], command2: &str, args2: &[&str]) {
    let mut child1 = Command::new(command1)
        .args(args1)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process 1");

    let mut child2 = Command::new(command2)
        .args(args2)
        .stdout(child1.stdout.take().expect("Failed to open pipe"))
        .spawn()
        .expect("Failed to start process 2");

    let _ = child1.wait().expect("Failed to wait on process 1");
    let _ = child2.wait().expect("Failed to wait on process 2");
}
