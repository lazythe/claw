#[allow(unused_imports)]
use std::io::{self, Write};
use users::get_current_username;

fn main() {
    loop {
        print!("[{:?}] ~> ", get_current_username().unwrap());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }
    }
}
