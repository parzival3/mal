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
            let mut evaluated_list = eval_list_args(env, list)?;
            let args = evaluated_list.drain(1..).collect();
            let fun = evaluated_list;
            call_function(
                env,
                fun.into_iter().next().ok_or_else(|| {
                    RuntimeError::Evaluation(String::from("Couldn't get function"))
                })?,
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
    Err(RuntimeError::Evaluation(String::from(
        "Symbol {val} is not a function",
    )))
}

fn eval_symbol(env: &RcEnv, val: Symbol) -> RuntimeResult<Value> {
    Ok(env
        .try_borrow()?
        .get(&val)
        .ok_or_else(|| RuntimeError::ValueNotFound(format!("Symbol {val}, is not defined")))?)
}

/// Eval each of the element of a list separatedly and return a vector of values
/// Most likely this is not correct, where we are mixing rust vector and lisp list.
fn eval_list_args(env: &RcEnv, args: List<Value>) -> Result<Vec<Value>, RuntimeError> {
    args.iter()
        .map(|val| eval_ast(env, val.clone()))
        .collect::<Result<Vec<Value>, RuntimeError>>()
        .to_owned()
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
        Value::Symbol(val) => eval_symbol(env, val),
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

pub fn rep(input_string: &str) {
    let env = default_environment();
    match read(input_string) {
        Ok(parsed_input) => print(eval(&env, parsed_input)),
        Err(e) => println!("(EOF|end of input|unbalanced): {e}"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inner_list() {
        let env = default_environment();
        let expr = "(+ 1 2 (+ 1 4))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(8));

        let expr = "(- (+ 5 (* 2 3)) 3)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(8));
    }

    #[test]
    fn mal_tests_part_2() {
        let env = default_environment();
        let expr = "(+ 1 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(3));

        let expr = "(+ 5 (* 2 3))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(11));

        let expr = "(- (+ 5 (* 2 3)) 3)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(8));

        let expr = "(/ (- (+ 5 (* 2 3)) 3) 4)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(2));

        let expr = "(/ (- (+ 515 (* 87 311)) 302) 27)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(1010));

        let expr = "(* -3 6)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(-18));

        let expr = "(/ (- (+ 515 (* -87 311)) 296) 27)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(-994));

        let expr = "()";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::List(List::new()));

        let mut array = List::new();
        array = array.prepend(Value::Integer(1));
        array = array.prepend(Value::Integer(2));
        array = array.prepend(Value::Integer(3));
        array = array.reverse();

        let expr = "[1 2 (+ 1 2)]'";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Array(array));

        let mut map = List::new();
        map = map.prepend(Value::String(String::from("\"a\"")));
        map = map.prepend(Value::Integer(15));
        map = map.reverse();

        let expr = "{\"a\" (+ 7 8)}'";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Map(map));

        let mut map = List::new();
        map = map.prepend(Value::Keyword(String::from(":a")));
        map = map.prepend(Value::Integer(15));
        map = map.reverse();

        let expr = "{:a (+ 7 8)}'";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Map(map));


        let map = List::new();
        let expr = "{}'";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Map(map));


        let array = List::new();

        let expr = "[]'";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Array(array));
    }
}
