use mal::env::default_environment;
use mal::mal::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new().expect("Failed to initialize readline");
    let env = default_environment();
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rep(&env, &line);
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
