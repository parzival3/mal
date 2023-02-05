use crate::list::*;

use crate::{env::RcEnv, errors::RuntimeError};

pub type NativeFun = fn(env: RcEnv, args: Vec<Value>) -> Result<Value, RuntimeError>;

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


#[derive(Debug, PartialEq)]
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
    Array(List<Value>),
    List(List<Value>),
    Map(List<Value>),
    NativeFun(NativeFun),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(val) => write!(f, "{}", val),
            Value::Symbol(val) => write!(f, "{}", val),
            Value::Nil => write!(f, "Nil"),
            Value::True => write!(f, "true"),
            Value::False => write!(f, "false"),
            Value::String(val) => write!(f, "{}", val),
            Value::Keyword(val) => write!(f, "{}", val),
            Value::SpliceUnquote => write!(f, "splice-unqoute"),
            Value::Unquote => write!(f, "unquote"),
            Value::Deref => write!(f, "deref"),
            Value::Quote => write!(f, "quote"),
            Value::QuasiQuote => write!(f, "quasiquote"),
            Value::WithMeta => write!(f, "with-meta"),
            Value::Array(array) => write!(f, "{}", print_seq(array, "[", "]")),
            Value::List(list) => write!(f, "{}", print_seq(list, "(", ")")),
            Value::Map(map) => write!(f, "{}", print_seq(map, "{", "}")),
            Value::NativeFun(func) => write!(f, "<nativefunc> {:?}", func)
        }
    }
}

fn print_seq(list: &List<Value>, start: &str, end: &str) -> String {
    let new_output: Vec<String> = list
        .iter()
        .map(|val| val.to_string())
        .collect();
    format!("{}{}{}", start, new_output.join(" "), end)
}
