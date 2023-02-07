use crate::env::RcEnv;
use crate::errors::*;
use crate::list::List;
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

pub fn eval_new(ast: &Value) -> RuntimeResult<Value> {
    Ok(Value::Integer(10))
}

trait LogRuntimeError<T> {
    fn rerr_to_option(&self) -> Option<T>;
}

impl<T: Clone> LogRuntimeError<T> for RuntimeResult<T> {
    fn rerr_to_option(&self) -> Option<T> {
        match self {
            Ok(val) => Some((*val).clone()),
            Err(e) => {
                println!("Runtime error {}", e);
                None
            },
        }
    }
}

fn eval_list(env: RcEnv, list: List<Value>) -> RuntimeResult<Value> {
    let list_evaluated = list.iter().try_fold(List::new(), |acc, val| {
        eval_new(val).rerr_to_option().and_then(|val| Some(acc.prepend(val)))
    }).ok_or_else(|| RuntimeError::Evaluation)?;
    Ok(Value::List(list_evaluated.reverse()))
}

fn eval_symbol(env: RcEnv, val: Symbol) -> RuntimeResult<Value> {
   Ok(env.try_borrow()?.get(&val).ok_or_else(|| RuntimeError::ValueNotFound(format!("Symbol {}, is not defined", val.to_string())))?)
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
            Value::List(list) => eval_list(env, list),
            Value::Map(_) => todo!(),
            Value::NativeFun(_) => todo!(),
            Value::Integer(val) => Ok(Value::Integer(val)),
            Value::String(val) => Ok(Value::String(val)),
            Value::Nil => Ok(Value::Nil),
            Value::True => Ok(Value::True),
            Value::False => Ok(Value::False),
            Value::Keyword(val) => Ok(Value::Keyword(val)),
            Value::Symbol(val) => eval_symbol(env, val),
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
