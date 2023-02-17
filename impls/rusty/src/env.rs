use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    errors::RuntimeError,
    types::{Symbol, Value},
};

pub type RcEnv = Rc<RefCell<Env>>;

/// An environment of symbol bindings. Used for the base environment, for
/// closures, for `let` statements, for function arguments, etc.
#[derive(Debug)]
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

    pub fn add(&mut self, name: Symbol, value: Value) {
        self.entries.insert(name, value);
    }

    /// Walks up the environment hierarchy until it finds the symbol's value or
    /// runs out of environments.
    pub fn get(&self, key: &Symbol) -> Option<Value> {
        if let Some(val) = self.entries.get(&key) {
            Some((*val).clone()) // clone the Rc
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(key)
        } else {
            None
        }
    }
}

pub fn default_environment() -> RcEnv {
    let mut env = Env::new(None);
    env.add(
        Symbol::from("+"),
        Value::NativeFun(|_, args| args.into_iter().reduce(|acc, e| acc + e))
    );

    env.add(
        Symbol::from("-"),
        Value::NativeFun(|_, args| {
            args.into_iter()
                .try_fold(0, |acc, value| {
                    if let Value::Integer(integer) = value {
                        Ok(integer - acc)
                    } else {
                        Err(RuntimeError::Evaluation(format!(
                            "{value} is not an integer"
                        )))
                    }
                })
                .map(Value::Integer)
        }),
    );

    env.add(
        Symbol::from("*"),
        Value::NativeFun(|_, args| {
            args.into_iter()
                .try_fold(0, |acc, value| {
                    if let Value::Integer(integer) = value {
                        Ok(integer * acc)
                    } else {
                        Err(RuntimeError::Evaluation(format!(
                            "{value} is not an integer"
                        )))
                    }
                })
                .map(Value::Integer)
        }),
    );

    env.add(
        Symbol::from("/"),
        Value::NativeFun(|_, args| {
            let first = args.first().ok_or_else(|| RuntimeError::Evaluation(String::from("Division requires two arguments")))?;
            args.into_iter().skip(1)
                .try_fold(first, |acc, value| {
                    if let Value::Integer(integer) = value {
                        Ok(integer / acc)
                    } else {
                        Err(RuntimeError::Evaluation(format!(
                            "{value} is not an integer"
                        )))
                    }
                })
                .map(Value::Integer)
        }),
    );
    return Rc::new(RefCell::new(env));
}
