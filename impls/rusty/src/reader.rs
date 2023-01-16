use crate::types::*;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenizerError {
    Quote(String), // Un-matched quote
    Paren(String),
    Braket(String),
    NoMoreTokens,
    ReadAtom(usize, String),
    ReadList(usize, String),
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
        }
    }
}

pub type TokenizerResult<T> = std::result::Result<T, TokenizerError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    TildeAt,
    LeftParen,
    RightParen,
    LeftSquareBraket,
    RightSquareBraket,
    LeftBraket,
    RightBraket,
    String(String),
    Comment(String),
    Atom(String),
} // Captures a sequence of zero or more non special characters (e.g. symbols, numbers, "true", "false")

const STANDALONE_TOKENS_MAPPING: [(&str, Tokens); 7] = [
    ("~@", Tokens::TildeAt),
    ("(", Tokens::LeftParen),
    (")", Tokens::RightParen),
    ("[", Tokens::LeftSquareBraket),
    ("]", Tokens::RightSquareBraket),
    ("{", Tokens::LeftBraket),
    ("}", Tokens::RightBraket),
];

pub trait ReaderTrait {
    fn new(tokens: Vec<Tokens>) -> Self;
    fn index(&mut self) -> usize;
    fn next(&mut self) -> Option<&Tokens>;
    fn peek(&self) -> Option<&Tokens>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reader<T>
where
    T: ReaderTrait,
{
    internal_reader: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InternalReader {
    tokens: Vec<Tokens>,
    counter: usize,
}

impl ReaderTrait for InternalReader {
    fn new(tokens: Vec<Tokens>) -> Self {
        InternalReader { tokens, counter: 0 }
    }

    fn index(&mut self) -> usize {
        self.counter
    }

    fn next(&mut self) -> Option<&Tokens> {
        let token = self.tokens.get(self.counter);
        self.counter += 1;
        token
    }

    fn peek(&self) -> Option<&Tokens> {
        self.tokens.get(self.counter)
    }
}

impl<T: ReaderTrait> Reader<T> {
    pub fn tokenize(input: &str) -> TokenizerResult<Reader<T>> {
        let regex = Regex::new(concat!(
            "[\\s,]*(~@|[\\[\\]{}()'",
            "`~^@]|\"(?:\\.|[^\\\\\"])",
            "*\"?|;.*|[^\\s\\[\\]{}('\"`,;)]*)"
        ))
        .unwrap();

        let mut tokens = Vec::new();

        for cap in regex.captures_iter(input) {
            match STANDALONE_TOKENS_MAPPING.iter().find(|&x| x.0 == &cap[1]) {
                Some(token_mapping) => tokens.push(token_mapping.1.clone()),
                None => {
                    if cap[1].starts_with(';') {
                        let token = Tokens::Comment(cap[1].to_string());
                        tokens.push(token);
                    } else if cap[1].starts_with('\"') {
                        if !cap[1].ends_with('\"') {
                            return Err(TokenizerError::Quote(format!(
                                "Missing enclosing qoute \" starting at {}",
                                cap[1].to_string()
                            )));
                        }
                        let token = Tokens::String(cap[1].to_string());
                        tokens.push(token);
                    } else {
                        let token = Tokens::Atom(cap[1].to_string());
                        tokens.push(token);
                    }
                }
            }
        }
        let intr = Reader {
            internal_reader: T::new(tokens),
        };

        Ok(intr)
    }

    pub fn next(&mut self) -> TokenizerResult<Tokens> {
        self.internal_reader
            .next()
            .cloned()
            .ok_or(TokenizerError::NoMoreTokens)
    }

    pub fn peek(&self) -> TokenizerResult<Tokens> {
        self.internal_reader
            .peek()
            .cloned()
            .ok_or(TokenizerError::NoMoreTokens)
    }

    pub fn read_from(&mut self) -> TokenizerResult<Type> {
        match self.next()? {
            Tokens::TildeAt => todo!(),
            Tokens::LeftParen => self.read_list(),
            Tokens::RightParen => todo!(),
            Tokens::LeftSquareBraket => todo!(),
            Tokens::RightSquareBraket => todo!(),
            Tokens::LeftBraket => todo!(),
            Tokens::RightBraket => todo!(),
            Tokens::String(content) => self.read_atom(content),
            Tokens::Comment(_) => todo!(),
            Tokens::Atom(content) => self.read_atom(content),
        }
    }

    fn read_list(&mut self) -> TokenizerResult<Type> {
        let mut content = Vec::new();
        while let Ok(token) = self.peek() {
            if token == Tokens::RightParen {
                let _ = self.next();
                return Ok(Type::List(List { child: content }));
            }
            content.push(self.read_from()?);
        }
        Err(TokenizerError::NoMoreTokens)
    }

    fn read_atom(&mut self, content: String) -> TokenizerResult<Type> {
        if content.starts_with('\"') {
            return Ok(Type::Atom(Atom::String(content.replace('\"', ""))));
        }

        if content == "nil" {
            return Ok(Type::Atom(Atom::Nil));
        }

        if let Ok(value) = content.parse::<i64>() {
            return Ok(Type::Atom(Atom::Integer(value)));
        }

        if content == "true" {
            return Ok(Type::Atom(Atom::True));
        }

        if content == "false" {
            return Ok(Type::Atom(Atom::False));
        }

        if content.starts_with(':') {
            return Ok(Type::Atom(Atom::Keyword(content)));
        }

        return Ok(Type::Atom(Atom::Symbol(content)));
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn testing_tokenizer() {
        let mut reader = Reader::<InternalReader>::tokenize("(+ 1 2 \"Hello\")")
            .expect("We should be able to create a Reader");
        assert_eq!(reader.next(), Ok(Tokens::LeftParen));
        assert_eq!(reader.next(), Ok(Tokens::Atom(String::from("+"))));
        assert_eq!(reader.next(), Ok(Tokens::Atom(String::from("1"))));
        assert_eq!(reader.next(), Ok(Tokens::Atom(String::from("2"))));
        assert_eq!(reader.next(), Ok(Tokens::String(String::from("\"Hello\""))));
        assert_eq!(reader.next(), Ok(Tokens::RightParen));
        assert_eq!(reader.next(), Err(TokenizerError::NoMoreTokens));
    }

    #[test]
    fn testing_read_from_symbol() {
        let mut reader = Reader::<InternalReader>::tokenize("(+)")
            .expect("We should be able to create a Reader");
        let ast = reader
            .read_from()
            .expect("We should be able to parse a single atom");
        assert_eq!(
            ast,
            Type::List(List {
                child: vec![Type::Atom(Atom::Symbol(String::from("+")))]
            })
        );
    }

    #[test]
    fn testing_read_from_nested_lists() {
        let mut reader = Reader::<InternalReader>::tokenize("(+ (+) 1)")
            .expect("We should be able to create a Reader");
        let ast = reader
            .read_from()
            .expect("We should be able to parse a single atom");
        assert_eq!(
            ast,
            Type::List(List {
                child: vec![
                    Type::Atom(Atom::Symbol(String::from("+"))),
                    Type::List(List {
                        child: vec![Type::Atom(Atom::Symbol(String::from("+")))]
                    }),
                    Type::Atom(Atom::Integer(1))
                ]
            })
        );
    }

    #[test]
    fn testing_read_from_numeric_expression() {
        let mut reader = Reader::<InternalReader>::tokenize("(+ 1 2)")
            .expect("We should be able to create a Reader");
        let ast = reader
            .read_from()
            .expect("We should be able to parse a single atom");
        assert_eq!(
            ast,
            Type::List(List {
                child: vec![
                    Type::Atom(Atom::Symbol(String::from("+"))),
                    Type::Atom(Atom::Integer(1)),
                    Type::Atom(Atom::Integer(2))
                ]
            })
        );
    }

    #[test]
    fn testing_read_from_keyword() {
        let mut reader = Reader::<InternalReader>::tokenize("(+ :test)")
            .expect("We should be able to create a Reader");
        let ast = reader
            .read_from()
            .expect("We should be able to parse a single atom");
        assert_eq!(
            ast,
            Type::List(List {
                child: vec![
                    Type::Atom(Atom::Symbol(String::from("+"))),
                    Type::Atom(Atom::Keyword(String::from(":test")))
                ]
            })
        );
    }

    #[test]
    fn testing_read_from_nil_true_false() {
        let mut reader = Reader::<InternalReader>::tokenize("(+ nil true false)")
            .expect("We should be able to create a Reader");
        let ast = reader
            .read_from()
            .expect("We should be able to parse a single atom");
        assert_eq!(
            ast,
            Type::List(List {
                child: vec![
                    Type::Atom(Atom::Symbol(String::from("+"))),
                    Type::Atom(Atom::Nil),
                    Type::Atom(Atom::True),
                    Type::Atom(Atom::False)
                ]
            })
        );
    }

    #[test]
    fn testing_read_from_string() {
        let mut reader = Reader::<InternalReader>::tokenize("(\"Hello World\")")
            .expect("We should be able to create a Reader");
        let ast = reader
            .read_from()
            .expect("We should be able to parse a single atom");
        assert_eq!(
            ast,
            Type::List(List {
                child: vec![Type::Atom(Atom::String(String::from("Hello World")))]
            })
        );
    }

    #[test]
    fn testing_read_from_unbalanced_list() {
        let mut reader = Reader::<InternalReader>::tokenize("(\"Hello World\"")
            .expect("We should be able to create a Reader");
        let ast = reader.read_from();
        assert!(ast.is_err());
    }
}
