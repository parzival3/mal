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
        Value::Symbol(symb) if symb == Symbol::from("let") => eval_let(env, list),
        _ => eval_function(env, list),
    }
}

fn eval_err(input: &str) -> RuntimeError {
    RuntimeError::Evaluation(String::from(input))
}

fn add_to_env(env: &RcEnv, symbol: Symbol, value: Value) -> RuntimeResult<()> {
    Ok(env.try_borrow_mut()?.add(symbol, value))
}

fn eval_let(env: &RcEnv, list: List<Value>) -> RuntimeResult<Value> {
    let new_env = new_env(env.clone());
    let first = list
        .car_n(
            1,
            eval_err("let expects at least a definition and a body; none given"),
        )?
        .clone();

    let mut list_iter = first.expect_list()?.iter();
    while let Some(val) = list_iter.next() {
        let def = val.expect_list()?;
        let symbol_name = def.car(eval_err(
            "There must be at least a symbol and definition in the let binding, none given",
        ))?;
        let name = symbol_name.expect_symbol()?;
        let body = def.car_n(
            1,
            eval_err("There must be a definition body for symbol {name}"),
        )?;
        let evaluated = eval(&new_env, body.clone())?;
        add_to_env(&new_env, name.clone(), evaluated)?;
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
            RuntimeError::Evaluation(format!(
                "def! expects at least a definition and a body; none given"
            )),
        )?
        .clone();

    if let Value::Symbol(definition) = first {
        let body = list.iter().skip(2).next().ok_or_else(|| RuntimeError::Evaluation(format!("def! expects at least a definition and a body, definition is {definition} body is empty")))?;
        let evaluated = eval(env, body.clone())?;
        env.try_borrow_mut()?
            .add(definition.clone(), evaluated.clone());
        return Ok(evaluated);
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
    }
    Err(RuntimeError::Evaluation(format!(
        "Symbol {func} is not a function",
    )))
}

fn get_symbol(env: &RcEnv, val: Symbol) -> RuntimeResult<Value> {
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
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(1010)
        );

        let expr = "(* -3 6)";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(-18)
        );

        let expr = "(/ (- (+ 515 (* -87 311)) 296) 27)";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(-994)
        );

        let expr = "()";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::List(List::new())
        );

        let mut array = List::new();
        array = array.prepend(Value::Integer(1));
        array = array.prepend(Value::Integer(2));
        array = array.prepend(Value::Integer(3));
        array = array.reverse();

        let expr = "[1 2 (+ 1 2)]'";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Array(array)
        );

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
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Array(array)
        );
    }

    #[test]
    fn mal_tests_part_3() {
        let env = default_environment();
        let expr = "(def! a 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(2));

        let expr = "(+ (def! a 2) (def! b 3))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(5));

        let expr = "(let ((a 2) (b 3)) (+ a b))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(5));
    }
}
