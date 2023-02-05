use mal::mal::*;
use std::io;
use std::io::prelude::*;

fn read_input(user_query: &str) -> Option<String> {
    let stdin = io::stdin();
    print!("{}", user_query);
    let mut input_line = String::new();
    let _ = std::io::stdout().flush();
    let bytes_read = stdin
        .read_line(&mut input_line)
        .expect("Something was wrong with the input string");
    if bytes_read == 0 {
        // Received an EOF
        None
    } else {
        Some(input_line)
    }
}

fn main() {
    loop {
        match read_input("user> ") {
            Some(input_string) => rep(&input_string),
            None => return,
        }
    }
}
