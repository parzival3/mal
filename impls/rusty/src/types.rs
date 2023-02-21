use crate::list::*;
use std::ops::*;

use crate::{env::RcEnv, errors::RuntimeError, errors::RuntimeResult};

pub type NativeFun = fn(env: RcEnv, args: Vec<Value>) -> Result<Value, RuntimeError>;

pub type IntType = i64;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(IntType),
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

impl Value {
    pub fn expect_list(&self) -> RuntimeResult<&List<Value>> {
        match self {
            Value::List(list) => Ok(list),
            val => Err(RuntimeError::Evaluation(format!("Value '{val}' is not a list")))
        }
    }

    pub fn expect_symbol(&self) -> RuntimeResult<&Symbol> {
        match self {
            Value::Symbol(symbol) => Ok(symbol),
            val => Err(RuntimeError::Evaluation(format!("Value '{val}' is not a symbol")))
        }
    }

    pub fn expect_list_arr(&self) -> RuntimeResult<&List<Value>> {
        match self {
            Value::List(list) => Ok(list),
            Value::Array(array) => Ok(array),
            val => Err(RuntimeError::Evaluation(format!("Value '{val}' is not a list")))
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(val) => write!(f, "{}", val),
            Value::Symbol(val) => write!(f, "{}", val),
            Value::Nil => write!(f, "nil"),
            Value::True => write!(f, "true"),
            Value::False => write!(f, "false"),
            Value::String(val) => write!(f, "{}", val),
            Value::Keyword(val) => write!(f, "{}", val),
            Value::SpliceUnquote => write!(f, "splice-unquote"),
            Value::Unquote => write!(f, "unquote"),
            Value::Deref => write!(f, "deref"),
            Value::Quote => write!(f, "quote"),
            Value::QuasiQuote => write!(f, "quasiquote"),
            Value::WithMeta => write!(f, "with-meta"),
            Value::Array(array) => write!(f, "{}", print_seq(array, "[", "]")),
            Value::List(list) => write!(f, "{}", print_seq(list, "(", ")")),
            Value::Map(map) => write!(f, "{}", print_seq(map, "{", "}")),
            Value::NativeFun(func) => write!(f, "<nativefunc> {:?}", func),
        }
    }
}

fn print_seq(list: &List<Value>, start: &str, end: &str) -> String {
    let new_output: Vec<String> = list.iter().map(|val| val.to_string()).collect();
    format!("{}{}{}", start, new_output.join(" "), end)
}

impl FromIterator<Value> for List<Value> {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut list = List::new();
        for val in iter {
            list = list.prepend(val);
        }

        return list.reverse();
    }
}

impl Add<&Value> for &Value {
    type Output = Result<Value, RuntimeError>;

    fn add(self, other: &Value) -> Self::Output {
        match (self, other) {
            // same type
            (Value::Integer(this), Value::Integer(other)) => Ok(Value::from(this + other)),
            (Value::String(this), Value::String(other)) => Ok(Value::from(this.clone() + other)),

            // non-string + string
            (Value::String(this), Value::Integer(other)) => {
                Ok(Value::from(this.clone() + &other.to_string()))
            }
            (Value::Integer(this), Value::String(other)) => {
                Ok(Value::from(this.to_string() + other))
            }

            (a, b) => Err(RuntimeError::Evaluation(format!(
                "Can't add {a} with {b}"
            ))),
        }
    }
}

impl Add<Value> for Value {
    type Output = Result<Value, RuntimeError>;

    fn add(self, other: Value) -> Self::Output {
        &self + &other
    }
}

impl Sub<&Value> for &Value {
    type Output = Result<Value, RuntimeError>;

    fn sub(self, other: &Value) -> Self::Output {
        match (self, other) {
            (Value::Integer(this), Value::Integer(other)) => Ok(Value::from(this - other)),
            (a, b) => Err(RuntimeError::Evaluation(format!(
                "Can't subtract {a} with {b}"
            ))),
        }
    }
}

impl Sub<Value> for Value {
    type Output = Result<Value, RuntimeError>;

    fn sub(self, other: Value) -> Self::Output {
        &self - &other
    }
}

impl Mul<&Value> for &Value {
    type Output = Result<Value, RuntimeError>;

    fn mul(self, other: &Value) -> Self::Output {
        match (self, other) {
            (Value::Integer(this), Value::Integer(other)) => Ok(Value::from(this * other)),
            (a, b) => Err(RuntimeError::Evaluation(format!(
                "Can't multiply {a} with {b}"
            ))),
        }
    }
}

impl Mul<Value> for Value {
    type Output = Result<Value, RuntimeError>;

    fn mul(self, other: Value) -> Self::Output {
        &self * &other
    }
}

impl Div<&Value> for &Value {
    type Output = Result<Value, RuntimeError>;

    fn div(self, other: &Value) -> Self::Output {
        match (self, other) {
            (Value::Integer(this), Value::Integer(other)) => Ok(Value::from(this / other)),
            (a, b) => Err(RuntimeError::Evaluation(format!(
                "Can't divide {a} with {b}"
            ))),
        }
    }
}

impl Div<Value> for Value {
    type Output = Result<Value, RuntimeError>;

    fn div(self, other: Value) -> Self::Output {
        &self / &other
    }
}

impl From<IntType> for Value {
    fn from(i: IntType) -> Self {
        Value::Integer(i)
    }
}

impl From<String> for Value {
    fn from(i: String) -> Self {
        Value::String(i)
    }
}

impl From<Symbol> for Value {
    fn from(i: Symbol) -> Self {
        Value::Symbol(i)
    }
}

impl From<List<Value>> for Value {
    fn from(i: List<Value>) -> Self {
        Value::List(i)
    }
}

pub fn arithmetic_function<F: FnMut(Value, Value) -> Result<Value, RuntimeError>>(
    args: Vec<Value>,
    function: F,
) -> RuntimeResult<Value> {
    let first = args.first().ok_or_else(|| {
        RuntimeError::Evaluation(String::from(
            "arithmetic_function needs at least 2 arguments 0 provided",
        ))
    })?.clone();
    args.into_iter().skip(1).try_fold(first, function)
}
