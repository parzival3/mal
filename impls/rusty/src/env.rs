use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::types::{Type, Symbol};

pub type RcEnv = Rc<RefCell<Env>>;

/// An environment of symbol bindings. Used for the base environment, for
/// closures, for `let` statements, for function arguments, etc.
#[derive(Debug)]
pub struct Env {
    parent: Option<RcEnv>,
    entries: HashMap<Symbol, Type>,
}

impl Env {
    pub fn new(parent: Option<RcEnv>) -> Self {
        Env {
            parent,
            entries: HashMap::<Symbol, Type>::new()
        }
    }

    pub fn add(&mut self, name: Symbol, value: Type) {
        self.entries.insert(name, value);
    }

    pub fn get(&self, key: &Symbol) -> Option<Type> {
        self.entries.get(key).cloned()
    }

}
