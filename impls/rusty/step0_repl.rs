use std::io;
use std::io::prelude::*;

fn read(input_string: &str) -> String {
    let mut string = input_string.to_owned();
    string.pop();
    return string;
}

fn eval(input_string: &str) -> String {
    return input_string.to_owned();
}

fn print(input_string: &str) {
    println!("{}", input_string.to_owned());
}

fn rep(input_string: &str) {
    print(&eval(&read(input_string)));
}

fn read_input(user_query: &str) -> Option<String> {
    let stdin = io::stdin();
    print!("{}", user_query);
    let mut input_line = String::new();
    let _ = std::io::stdout().flush();
    let bytes_read = stdin.read_line(&mut input_line).expect("Something was wrong with the input string");
    if bytes_read == 0 { // Received an EOF
        None
    } else {
        Some(input_line)
    }
}

fn main() {
    loop {
        match read_input("user> ") {
            Some(input_string) => rep(&input_string),
            None => return
        }
    }
}
