use crate::env::*;
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

pub fn eval(env: &RcEnv, ast: Value) -> RuntimeResult<Value> {
    match ast {
        Value::List(list) if list != List::NIL => {
            let mut evaluated_list = list
                .iter()
                .map(|val| eval_ast(env, val.clone()))
                .collect::<Result<Vec<Value>, RuntimeError>>()?
                .to_owned();
            let args = evaluated_list.drain(1..).collect();
            let fun = evaluated_list;
            call_function(
                env,
                fun.into_iter()
                    .next()
                    .ok_or_else(|| RuntimeError::Evaluation(String::from("Couldn't get function")))?,
                args,
            )
        }
        _ => eval_ast(env, ast),
    }
}

fn call_function(env: &RcEnv, func: Value, args: Vec<Value>) -> RuntimeResult<Value> {
    if let Value::NativeFun(native_func) = func {
        return native_func(env.clone(), args);
    }
    Err(RuntimeError::Evaluation(String::from("Not a Function")))
}

fn eval_symbol(env: &RcEnv, val: Symbol) -> RuntimeResult<Value> {
    Ok(env.try_borrow()?.get(&val).ok_or_else(|| {
        RuntimeError::ValueNotFound(format!("Symbol {}, is not defined", val.to_string()))
    })?)
}

pub fn eval_ast(env: &RcEnv, ast: Value) -> RuntimeResult<Value> {
    match ast {
        Value::List(list) => list
            .iter()
            .map(|val| eval(env, val.clone()))
            .collect::<Result<List<Value>, RuntimeError>>()
            .map(Value::List),
        Value::Symbol(val) => eval_symbol(env, val),
        _ => Ok(ast.clone()),
    }
}

pub fn print(ast: RuntimeResult<Value>) {
    match ast {
        Ok(ast) => println!("{}", ast.to_string()),
        Err(e) => println!("(EOF|end of input|unbalanced) {e}"), // TODO change this
    }
}

pub fn rep(input_string: &str) {
    let env = default_environment();
    match read(input_string)
    {
        Ok(parsed_input) => print(eval(&env, parsed_input)),
        Err(e) => println!("(EOF|end of input|unbalanced) {}", e)
    }

}
