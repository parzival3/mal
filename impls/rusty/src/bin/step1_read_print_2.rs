use mal::mal2::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new().expect("Failed to initialize readline");
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rep(&line);
                rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
