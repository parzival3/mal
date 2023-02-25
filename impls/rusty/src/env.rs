use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    errors::eval_err,
    list::List,
    types::{arithmetic_function, Symbol, Value, comp_function},
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

pub fn new_env(env: RcEnv) -> RcEnv {
    RcEnv::new(RefCell::new(Env::new(Some(env.clone()))))
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
                return Ok(Value::Integer(0));
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
            let mut arg_iter = args.into_iter();
            let mut list = List::new();
            while let Some(item) = arg_iter.next() {
                list = list.prepend(item);
            }
            Ok(Value::List(list.reverse()))
        }),
    );
    return Rc::new(RefCell::new(env));
}
