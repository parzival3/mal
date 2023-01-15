use crate::printer::pr_str;
use crate::reader::*;
use crate::types::*;

pub fn read(input_string: &str) -> Option<Type> {
    let mut string = input_string.to_owned();
    string.pop();
    let mut reader = Reader::<InternalReader>::tokenize(input_string).ok()?;
    reader.read_from()
}

pub fn eval(ast: Option<Type>) -> Option<Type> {
    ast
}

pub fn print(ast: Option<Type>) {
    match ast {
        Some(ast) => pr_str(ast),
        _ => todo!(),
    }
}

pub fn rep(input_string: &str) {
    print(eval(read(input_string)));
}
