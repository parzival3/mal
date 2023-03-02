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
        Value::List(list) if list != List::NIL => eval_list(env, list),
        _ => eval_ast(env, ast),
    }
}

fn eval_list(env: &RcEnv, list: List<Value>) -> RuntimeResult<Value> {
    let first = list.iter().next().unwrap().clone(); // this is safe because we check if the list is empty above
    match first {
        Value::Symbol(symb) if symb == Symbol::from("def!") => eval_definition(env, list),
        Value::Symbol(symb) if symb == Symbol::from("let*") => eval_let(env, list),
        Value::Symbol(symb) if symb == Symbol::from("if") => eval_if(env, list),
        Value::Symbol(symb) if symb == Symbol::from("fn*") => define_closure(env, list),
        Value::Symbol(symb) if symb == Symbol::from("do") => eval_do(env, list),
        _ => eval_function(env, list),
    }
}


fn eval_do(env: &RcEnv, list: List<Value>) -> Result<Value, RuntimeError> {
    let expressions = list.into_vec();

    let iter = expressions.iter().skip(1);

    let mut res = Value::Nil;
    for expr in iter {
        res = eval(env, expr.clone())?;
    }

    Ok(res)
}


fn define_closure(env: &RcEnv, list: List<Value>) -> Result<Value, RuntimeError> {
    // this is basically a lambda
    // This returns a Mal or
    let args = list.car_n(1, eval_err("closure missing list of arguments"))?;
    let params = args.expect_list()?.into_vec();
    let body = list.car_n(2, eval_err("closure needs a body"))?;
    Ok(Value::LispClosure(
        Symbol::from("closure"),
        LispClosure::new(params, body.clone()),
        env.clone(),
    ))
}

fn eval_if(env: &RcEnv, list: List<Value>) -> Result<Value, RuntimeError> {
    let condition = list
        .car_n(
            1,
            eval_err("if expects at least a condition and a body; none given"),
        )?
        .clone();
    let body = list.car_n(2, eval_err("if doesn't have a body"))?.clone();

    let else_body = list.iter().nth(3).unwrap_or_else(|| &Value::Nil).clone();

    match eval(env, condition)? {
        Value::Nil | Value::False => eval(env, else_body),
        _ => eval(env, body),
    }
}

fn add_to_env(env: &RcEnv, symbol: Symbol, value: Value) -> RuntimeResult<()> {
    env.try_borrow_mut()?.add(symbol, value);
    Ok(())
}

fn eval_let(env: &RcEnv, list: List<Value>) -> RuntimeResult<Value> {
    let new_env = new_env(env.clone());
    let first = list
        .car_n(
            1,
            eval_err("let expects at least a definition and a body; none given"),
        )?
        .clone();

    let mut list_iter = first.expect_list_arr()?.iter();
    loop {
        let symbol_name = list_iter.next();
        let body = list_iter.next();
        match (symbol_name, body) {
            (None, None) => break,
            (None, Some(body)) => {
                return Err(eval_err(&format!(
                    "let form has a body but not a definition, body is '{body}'"
                )))
            }
            (Some(def), None) => {
                return Err(eval_err(&format!(
                    "let form has a definition but not a body, definition is '{def}'"
                )))
            }
            (Some(Value::Symbol(name)), Some(body)) => {
                let evaluated = eval(&new_env, body.clone())?;
                add_to_env(&new_env, name.clone(), evaluated)?;
            }
            (Some(value), Some(_)) => {
                return Err(eval_err(&format!(
                    "let form needs a symbol as definition, got '{value}'"
                )))
            }
        }
    }

    let body = list.car_n(
        2,
        eval_err(&format!(
            "let expects at least a definition and a body, definition is {first} body is empty"
        )),
    )?;
    eval(&new_env, body.clone())
}

fn eval_definition(env: &RcEnv, list: List<Value>) -> RuntimeResult<Value> {
    let first = list
        .car_n(
            1,
            RuntimeError::Evaluation(String::from(
                "def! expects at least a definition and a body; none given",
            )),
        )?
        .clone();

    if let Value::Symbol(definition) = first {
        let body = list.iter().nth(2).ok_or_else(|| RuntimeError::Evaluation(format!("def! expects at least a definition and a body, definition is {definition} body is empty")))?;
        let evaluated = eval(env, body.clone())?;
        env.try_borrow_mut()?
            .add(definition.clone(), evaluated.clone());
        Ok(evaluated)
    } else {
        return Err(RuntimeError::Evaluation(format!(
            "First element of def! must be a symbol instead is {first}"
        )));
    }
}

fn eval_function(env: &RcEnv, list: List<Value>) -> RuntimeResult<Value> {
    let mut evaluated_list = eval_list_args(env, list)?;
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

fn call_function(env: &RcEnv, func: Value, args: Vec<Value>) -> RuntimeResult<Value> {
    if let Value::NativeFun(native_func) = func {
        return native_func(env.clone(), args);
    } else if let Value::LispClosure(_, closure, lisp_env) = func {
        // TODO: is passing the lisp_env enough? probalby we need to prevent shadowing?
        return call_closure(lisp_env, closure, args);
    }
    Err(RuntimeError::Evaluation(format!(
        "Symbol {func} is not a function",
    )))
}

fn call_closure(env: RcEnv, closure: LispClosure, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let new_env = new_env_bindings(env, closure.params().clone(), args)?;
    eval(&new_env, closure.body().clone())
}

fn get_symbol(env: &RcEnv, val: Symbol) -> RuntimeResult<Value> {
    env.try_borrow()?
        .get(&val)
        .ok_or_else(|| RuntimeError::ValueNotFound(format!("Symbol '{val}' not found")))
}

/// Eval each of the element of a list separatedly and return a vector of values
/// Most likely this is not correct, where we are mixing rust vector and lisp list.
fn eval_list_args(env: &RcEnv, args: List<Value>) -> Result<Vec<Value>, RuntimeError> {
    args.iter()
        .map(|val| eval_ast(env, val.clone()))
        .collect::<Result<Vec<Value>, RuntimeError>>()
}

/// Eval the entire list and return a new list wich is the result of the evaluation
fn eval_array(env: &RcEnv, list: List<Value>) -> RuntimeResult<Value> {
    list.iter()
        .map(|val| eval(env, val.clone()))
        .collect::<Result<List<Value>, RuntimeError>>()
        .map(Value::Array)
}

fn eval_map(env: &RcEnv, list: List<Value>) -> RuntimeResult<Value> {
    list.iter()
        .map(|val| eval(env, val.clone()))
        .collect::<Result<List<Value>, RuntimeError>>()
        .map(Value::Map)
}

pub fn eval_ast(env: &RcEnv, ast: Value) -> RuntimeResult<Value> {
    match ast {
        Value::List(ref x) if *x != List::NIL => eval(env, ast),
        Value::Symbol(val) => get_symbol(env, val),
        Value::Array(array) => eval_array(env, array),
        Value::Map(map) => eval_map(env, map),
        _ => Ok(ast.clone()),
    }
}

pub fn print(ast: RuntimeResult<Value>) {
    match ast {
        Ok(ast) => println!("{ast}"),
        Err(e) => println!("{e}"),
    }
}

pub fn rep(env: &RcEnv, input_string: &str) {
    match read(input_string) {
        Ok(parsed_input) => print(eval(env, parsed_input)),
        Err(e) => println!("(EOF|end of input|unbalanced): {e}"),
    }
}
