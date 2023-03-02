use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    errors::{eval_err, RuntimeError, RuntimeResult},
    list::List,
    types::{arithmetic_function, comp_function, Symbol, Value},
};

pub type RcEnv = Rc<RefCell<Env>>;

/// An environment of symbol bindings. Used for the base environment, for
/// closures, for `let` statements, for function arguments, etc.
#[derive(Debug, PartialEq)]
pub struct Env {
    parent: Option<RcEnv>,
    entries: HashMap<Symbol, Value>,
}

impl Env {
    pub fn new(parent: Option<RcEnv>) -> Self {
        Env {
            parent,
            entries: HashMap::<Symbol, Value>::new(),
        }
    }

    pub fn new_bindings(
        parent: Option<RcEnv>,
        params: Vec<Value>,
        args: Vec<Value>,
    ) -> RuntimeResult<Self> {
        if params.len() != args.len() {
            return Err(eval_err(&format!(
                "Parameters {:?} don't match arguments {:?}",
                params, args
            )));
        }

        let mut env = Env::new(parent);
        params
            .iter()
            .zip(args.into_iter())
            .try_for_each(|(param, arg)| {
                let name = param.expect_symbol()?;
                env.add(name.clone(), arg);
                Ok::<(), RuntimeError>(())
            })?;
        Ok(env)
    }

    pub fn add(&mut self, name: Symbol, value: Value) {
        self.entries.insert(name, value);
    }

    /// Walks up the environment hierarchy until it finds the symbol's value or
    /// runs out of environments.
    pub fn get(&self, key: &Symbol) -> Option<Value> {
        if let Some(val) = self.entries.get(key) {
            Some((*val).clone()) // clone the Rc
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(key)
        } else {
            None
        }
    }
}

pub fn new_env(env: RcEnv) -> RcEnv {
    RcEnv::new(RefCell::new(Env::new(Some(env))))
}

pub fn new_env_bindings(env: RcEnv, params: Vec<Value>, args: Vec<Value>) -> RuntimeResult<RcEnv> {
    Ok(RcEnv::new(RefCell::new(Env::new_bindings(Some(env), params, args)?)))
}

pub fn default_environment() -> RcEnv {
    let mut env = Env::new(None);
    env.add(
        Symbol::from("+"),
        Value::NativeFun(|_, args| arithmetic_function(args, |acc, e| acc + e)),
    );

    env.add(
        Symbol::from("-"),
        Value::NativeFun(|_, args| arithmetic_function(args, |acc, e| acc - e)),
    );

    env.add(
        Symbol::from("*"),
        Value::NativeFun(|_, args| arithmetic_function(args, |acc, e| acc * e)),
    );

    env.add(
        Symbol::from("/"),
        Value::NativeFun(|_, args| arithmetic_function(args, |acc, e| acc / e)),
    );

    env.add(
        Symbol::from("<"),
        Value::NativeFun(|_, args| comp_function(args, |acc, e| acc < e)),
    );

    env.add(
        Symbol::from("<="),
        Value::NativeFun(|_, args| comp_function(args, |acc, e| acc <= e)),
    );

    env.add(
        Symbol::from(">"),
        Value::NativeFun(|_, args| comp_function(args, |acc, e| acc > e)),
    );

    env.add(
        Symbol::from(">="),
        Value::NativeFun(|_, args| comp_function(args, |acc, e| acc >= e)),
    );

    env.add(
        Symbol::from("="),
        Value::NativeFun(|_, args| comp_function(args, |acc, e| acc == e)),
    );

    env.add(
        Symbol::from("list?"),
        Value::NativeFun(|_, args| {
            let first = args
                .first()
                .ok_or_else(|| eval_err("list? requires a list as argument, none given"))?;
            match first.expect_list() {
                Ok(_) => Ok(Value::True),
                Err(_) => Ok(Value::False),
            }
        }),
    );

    env.add(
        Symbol::from("empty?"),
        Value::NativeFun(|_, args| {
            let first = args
                .first()
                .ok_or_else(|| eval_err("empty? requires a list as argument, none given"))?;
            first.expect_list().map(|list| list.empty().into())
        }),
    );

    env.add(
        Symbol::from("count"),
        Value::NativeFun(|_, args| {
            let first = args
                .first()
                .ok_or_else(|| eval_err("count requires a list as argument, none given"))?;
            if matches!(first, Value::Nil) {
                Ok(Value::Integer(0))
            } else {
                first
                    .expect_list()
                    .map(|list| (list.iter().count() as i64).into())
            }
        }),
    );

    env.add(
        Symbol::from("list"),
        Value::NativeFun(|_, args| {
            let arg_iter = args.into_iter();
            let mut list = List::new();
            for item in arg_iter {
                list = list.prepend(item);
            }
            Ok(Value::List(list.reverse()))
        }),
    );
    Rc::new(RefCell::new(env))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn testing_env_bindings() {
        // what should I do here?
        let params = vec![Value::Symbol(Symbol::from("a"))];
        let args = vec![Value::Integer(10)];

        let env = Env::new_bindings(None, params, args)
            .expect("It should be possible to create an new enviroment");

        assert_eq!(env.get(&Symbol::from("a")), Some(Value::Integer(10)));
    }

    #[test]
    fn testing_env_bindings_errors() {
        // what should I do here?
        let params = vec![Value::String(String::from("a"))];
        let args = vec![Value::Integer(10)];
        assert!(Env::new_bindings(None, params, args).is_err());
        let params = vec![Value::String(String::from("a"))];
        let args = vec![];
        assert!(Env::new_bindings(None, params, args).is_err());
   }
}
