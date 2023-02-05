use crate::env::*;
use crate::errors::*;
use crate::printer::pr_str;
use crate::reader::*;
use crate::types::*;

pub fn read(input_string: &str) -> TokenizerResult<Type> {
    let mut string = input_string.to_owned();
    string.pop();
    let mut reader = Reader::<InternalReader>::tokenize(input_string)?;
    reader.read_from()
}

pub fn eval(ast: TokenizerResult<Type>) -> TokenizerResult<Type> {
    let mut env = Env::new(None);
    let sum = Type::NativeFun(|env, args| Ok(Type::Atom(Value::Integer(3))));
    env.add(Symbol::from("+"), sum);
    ast
}

pub fn eval_to_change(ast: &Type) -> RuntimeResult<Type> {
    let mut env = Env::new(None);
    let sum = Type::NativeFun(|env, args| Ok(Type::Atom(Value::Integer(3))));
    env.add(Symbol::from("+"), sum);
    Ok(Type::Atom(Value::Integer(3)))
}

pub fn eval_ast(ast: Type, env: RcEnv) -> RuntimeResult<Type> {
    match ast {
        Type::Atom(val) => {
            match val {
                Value::Symbol(val) => {
                    let hash_map = env.try_borrow()?;
                    hash_map.get(&val).ok_or_else(|| RuntimeError::ValueNotFound(val.to_string()))
                }
                _ => todo!()
            }
        },
        Type::List(list) => {
            // returns new list wich is the result of calling eval to each element
            let result: RuntimeResult<Vec<Type>> = list
                .child
                .iter()
                .try_fold(
                    Vec::<Type>::new(),
                    |mut acc, t_to_eval| match eval_to_change(t_to_eval) {
                        Ok(val) => {
                            acc.push(val);
                            Some(acc)
                        }
                        Err(e) => {
                            println!("Evaluation Error {:?}", e); // TODO: better error message
                            None
                        }
                    },
                )
                .ok_or_else(|| RuntimeError::Evaluation);

            Ok(Type::Atom(Value::Integer(2)))
        }
        others => Ok(others),
    }
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
