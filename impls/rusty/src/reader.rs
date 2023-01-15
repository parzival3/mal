use crate::types::*;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenizerError {
    Quote(String), // Un-matched quote
    Paren(String),
    Braket(String),
}

impl std::error::Error for TokenizerError {}
impl core::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::Quote(error) => write!(f, "Tokenizer Error, Quote Error {}", error),
            TokenizerError::Paren(error) => write!(f, "Tokenizer Error, Paren Error {}", error),
            TokenizerError::Braket(error) => write!(f, "Tokenizer Error, Braket Error {}", error),
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

trait ReaderTrait {
    fn new(tokens: Vec<Tokens>) -> Self;
    fn next(&mut self) -> Option<&Tokens>;
    fn peek(&self) -> Option<&Tokens>;
}

#[derive(Debug, Clone, PartialEq)]
struct Reader<T>
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
                                "Missing enclosing qoute \" {}",
                                "TODO: Fix error reporting"
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

    pub fn next(&mut self) -> Option<Tokens> {
        self.internal_reader.next().cloned()
    }

    pub fn peek(&self) -> Option<Tokens> {
        self.internal_reader.peek().cloned()
    }

    pub fn read_from(&mut self) -> Option<Type> {
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

    fn read_list(&mut self) -> Option<Type> {
        let mut content = Vec::new();
        while let Some(token) = self.peek() {
            if token == Tokens::RightParen {
                return Some(Type::List(List { child: content }));
            }
            content.push(self.read_from()?);
        }
        None
    }

    fn read_atom(&mut self, content: String) -> Option<Type> {
        if content.starts_with('\"') {
            return Some(Type::Atom(Atom::String(content.replace('\"', ""))));
        }

        if content == "nil" {
            return Some(Type::Atom(Atom::Nil));
        }

        if let Ok(value) = content.parse::<i64>() {
            return Some(Type::Atom(Atom::Integer(value)));
        }

        if content == "true" {
            return Some(Type::Atom(Atom::True));
        }

        if content == "false" {
            return Some(Type::Atom(Atom::False));
        }

        if content.starts_with(':') {
            return Some(Type::Atom(Atom::Keyword(content)));
        }

        return Some(Type::Atom(Atom::Symbol(content)));
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn testing_tokenizer() {
        let mut reader = Reader::<InternalReader>::tokenize("(+ 1 2 \"Hello\")")
            .expect("We should be able to create a Reader");
        assert_eq!(reader.next(), Some(Tokens::LeftParen));
        assert_eq!(reader.next(), Some(Tokens::Atom(String::from("+"))));
        assert_eq!(reader.next(), Some(Tokens::Atom(String::from("1"))));
        assert_eq!(reader.next(), Some(Tokens::Atom(String::from("2"))));
        assert_eq!(
            reader.next(),
            Some(Tokens::String(String::from("\"Hello\"")))
        );
        assert_eq!(reader.next(), Some(Tokens::RightParen));
        assert_eq!(reader.next(), None);
    }

    #[test]
    fn testing_error_on_unclosed_quote() {
        let reader = Reader::<InternalReader>::tokenize("(+ 1 2 \"Hello)");
        assert!(reader.is_err());
        assert_eq!(
            reader,
            Err(TokenizerError::Quote(format!(
                "Missing enclosing qoute \" {}",
                "TODO: Fix error reporting"
            )))
        );
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
                child: vec![
                    Type::Atom(Atom::String(String::from("Hello World")))]
            })
        );
    }

    #[test]
    fn testing_read_from_unbalanced_list() {
        let mut reader = Reader::<InternalReader>::tokenize("(\"Hello World\"").expect("We should be able to create a Reader");
        let ast = reader.read_from();
        assert!(ast.is_none());
    }
}
