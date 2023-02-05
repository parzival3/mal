use crate::errors::*;
use crate::reader2::*;
use crate::types2::*;

pub fn read(input_string: &str) -> TokenizerResult<Value> {
    let mut string = input_string.to_owned();
    string.pop();
    let mut reader = Reader::<InternalReader>::tokenize(input_string)?;
    reader.read_from()
}

pub fn eval(ast: TokenizerResult<Value>) -> TokenizerResult<Value> {
    ast
}

pub fn print(ast: TokenizerResult<Value>) {
    match ast {
        Ok(ast) => println!("{}", ast.to_string()),
        _ => println!("(EOF|end of input|unbalanced)"), // TODO change this
    }
}

pub fn rep(input_string: &str) {
    print(eval(read(input_string)));
}
