use crate::printer::pr_str;
use crate::reader::*;
use crate::types::*;
use crate::env::*;

pub fn read(input_string: &str) -> TokenizerResult<Type> {
    let mut string = input_string.to_owned();
    string.pop();
    let mut reader = Reader::<InternalReader>::tokenize(input_string)?;
    reader.read_from()
}

pub fn eval(ast: TokenizerResult<Type>) -> TokenizerResult<Type> {
    let env = Env::new(None);
    let sum = NativeFun(|_, __| -> Ok(Type::Atom(Value::Integer{3})));
    ast
}

pub fn print(ast: TokenizerResult<Type>) {
    match ast {
        Ok(ast) => pr_str(ast),
        _ => println!("(EOF|end of input|unbalanced)"), // TODO change this
    }
}

pub fn rep(input_string: &str) {
    print(eval(read(input_string)));
}
