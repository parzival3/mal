use std::cell::{BorrowError, BorrowMutError};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenizerError {
    Quote(String), // Un-matched quote
    Paren(String),
    Braket(String),
    NoMoreTokens,
    ReadAtom(usize, String),
    ReadList(usize, String),
    UnbalancedArray,
    UnbalancedList,
    UnbalancedMap,
}

impl std::error::Error for TokenizerError {}
impl core::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::Quote(error) => write!(f, "Tokenizer Error, Quote Error {}", error),
            TokenizerError::Paren(error) => write!(f, "Tokenizer Error, Paren Error {}", error),
            TokenizerError::Braket(error) => write!(f, "Tokenizer Error, Braket Error {}", error),
            TokenizerError::NoMoreTokens => {
                write!(f, "Tokenizer Error, No more tokens in the tokenizer")
            }
            TokenizerError::ReadAtom(index, message) => write!(
                f,
                "Tokenizer Error, Atom Error at index {}, {}",
                index, message
            ),
            TokenizerError::ReadList(index, message) => write!(
                f,
                "Tokenizer Error, List Error at index {}, {}",
                index, message
            ),
            TokenizerError::UnbalancedList => write!(f, "EOF while parsing List"),
            TokenizerError::UnbalancedArray => write!(f, "EOF while parsing Array"),
            TokenizerError::UnbalancedMap => write!(f, "EOF while parsing Map"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    Evaluation(String),
    EnviromentBorrowDispute(String), // Trying accessing the enviroment from two part of the code
    ValueNotFound(String),
}
impl std::error::Error for RuntimeError {}
impl core::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::Evaluation(e) => write!(f, "Evaluation Error {}", e),
            RuntimeError::EnviromentBorrowDispute(val) => {
                write!(f, "EnviromentDispute Error {}", val)
            }
            RuntimeError::ValueNotFound(val) => write!(f, "Value not found in eviroment {}", val),
        }
    }
}

impl From<BorrowError> for RuntimeError {
    fn from(e: BorrowError) -> Self {
        RuntimeError::EnviromentBorrowDispute(e.to_string())
    }
}

impl From<BorrowMutError> for RuntimeError {
    fn from(e: BorrowMutError) -> Self {
        RuntimeError::EnviromentBorrowDispute(e.to_string())
    }
}

pub type TokenizerResult<T> = std::result::Result<T, TokenizerError>;
pub type RuntimeResult<T> = std::result::Result<T, RuntimeError>;
