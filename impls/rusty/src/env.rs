use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::types::{Type, Symbol};

pub type RcEnv = Option<Rc<RefCell<Env>>>;

/// An environment of symbol bindings. Used for the base environment, for
/// closures, for `let` statements, for function arguments, etc.
#[derive(Debug)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    entries: HashMap<Symbol, Type>,
}

impl Env {
    fn new(parent: Option<Rc<RefCell<Env>>>) -> Self {
        Env {
            parent,
            entries: HashMap::<Symbol, Type>::new()
        }
    }
}
