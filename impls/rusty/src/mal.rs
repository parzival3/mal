use crate::env::RcEnv;
use crate::errors::*;
use crate::reader::*;
use crate::types::*;

pub fn read(input_string: &str) -> TokenizerResult<Value> {
    let mut string = input_string.to_owned();
    string.pop();
    let mut reader = Reader::<InternalReader>::tokenize(input_string)?;
    reader.read_from()
}

pub fn eval(ast: TokenizerResult<Value>) -> TokenizerResult<Value> {
    ast
}

pub fn eval_ast(env: RcEnv, ast: Value) -> RuntimeResult<Value> {
        let res = match ast {
            Value::SpliceUnquote => todo!(),
            Value::Unquote => todo!(),
            Value::Deref => todo!(),
            Value::Quote => todo!(),
            Value::QuasiQuote => todo!(),
            Value::WithMeta => todo!(),
            Value::Array(_) => todo!(),
            Value::List(list) => list.iter().try_fold(List::new(), |acc,
            Value::Map(_) => todo!(),
            Value::NativeFun(_) => todo!(),
            Value::Integer(val) => Value::Integer(val),
            Value::String(val) => Value::String(val),
            Value::Nil => Value::Nil,
            Value::True => Value::True,
            Value::False => Value::False,
            Value::Keyword(val) => Value::Keyword(val),
            Value::Symbol(val) => env.try_borrow()?.get(&val).ok_or_else(|| RuntimeError::ValueNotFound(format!("Symbol {}, is not defined", val.to_string())))?,
        };
    Err(RuntimeError::Evaluation)
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
