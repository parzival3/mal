use crate::{env::RcEnv, errors::RuntimeError};

pub type NativeFun = fn(env: RcEnv, args: Vec<Value>) -> Result<Type, RuntimeError>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub String);
impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        Symbol(String::from(s))
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Symbol(Symbol),
    Nil,
    True,
    False,
    String(String),
    Keyword(String),
    SpliceUnquote,
    Unquote,
    Deref,
    Quote,
    QuasiQuote,
    WithMeta,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub child: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Atom(Value),
    List(List),
    Array(List),
    Map(List),
    NativeFun(NativeFun)
}
