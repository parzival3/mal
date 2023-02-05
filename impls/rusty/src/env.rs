use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::types::{Value, Symbol};

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
            entries: HashMap::<Symbol, Value>::new()
        }
    }

    pub fn add(&mut self, name: Symbol, value: Value) {
        self.entries.insert(name, value);
    }

}
