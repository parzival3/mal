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
    UnbalancedArray,
    UnbalancedList,
    UnbalancedMap,
    RandomEscape,
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
            TokenizerError::RandomEscape => write!(f, "Found a random escape not iside a String"),
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
    LeftString(String),
    RightString(String),
    Comment(String),
    Atom(String),
    Escape,
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
    // Tokenizer with regex is fine for now but we should really parse the
    // input by hand in order to save the position of the token so the error report
    // can be more useful
    pub fn tokenize(input: &str) -> TokenizerResult<Reader<T>> {
        let regex = Regex::new(concat!(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###))
        .unwrap();

        let mut tokens = Vec::new();

        for cap in regex.captures_iter(input) {
            match STANDALONE_TOKENS_MAPPING.iter().find(|&x| x.0 == &cap[1]) {
                Some(token_mapping) => tokens.push(token_mapping.1.clone()),
                None => {
                    if cap[1].starts_with(';') {
                        let token = Tokens::Comment(cap[1].to_string());
                        tokens.push(token);
                    } else if cap[1].starts_with('\\') {
                        tokens.push(Tokens::Escape);
                    } else if cap[1].starts_with('\"') && cap[1].ends_with('\"') {
                        tokens.push(Tokens::String(cap[1].to_string()));
                    } else if cap[1].starts_with('\"') {
                        tokens.push(Tokens::LeftString(cap[1].to_string()));
                    } else if cap[1].ends_with('\"') {
                        tokens.push(Tokens::RightString(cap[1].to_string()));
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
            Tokens::TildeAt => self.read_quote(Type::Atom(Atom::SpliceUnquote)),
            Tokens::LeftParen => self.read_sequence_until(
                Tokens::RightParen,
                |child| Type::List(List { child }),
                TokenizerError::UnbalancedList,
            ),
            Tokens::RightParen => Err(TokenizerError::UnbalancedList),
            Tokens::LeftSquareBraket => self.read_sequence_until(
                Tokens::RightSquareBraket,
                |child| Type::Array(List { child }),
                TokenizerError::UnbalancedArray,
            ),
            Tokens::RightSquareBraket => Err(TokenizerError::UnbalancedArray),
            Tokens::LeftBraket => self.read_sequence_until(
                Tokens::RightBraket,
                |child| Type::Map(List { child }),
                TokenizerError::UnbalancedMap,
            ),
            Tokens::RightBraket => Err(TokenizerError::UnbalancedMap),
            Tokens::String(content) => Ok(Type::Atom(Atom::String(content))),
            Tokens::LeftString(content) => self.read_string(content),
            Tokens::RightString(content) => Err(TokenizerError::Quote(format!(
                "unbalanced quote at {}, EOF",
                content
            ))),
            Tokens::Comment(_) => self.read_from(), // skip the current comment
            Tokens::Atom(content) => self.read_atom(content),
            Tokens::Escape => Err(TokenizerError::RandomEscape),
        }
    }

    fn read_string(&mut self, mut content: String) -> TokenizerResult<Type> {
        while let Ok(token) = self.next() {
            match token {
                Tokens::Escape => content += "\\",
                Tokens::Atom(value) => content += &value,
                Tokens::RightString(value) => {
                    return Ok(Type::Atom(Atom::String(content + &value)))
                }
                Tokens::String(value) => return Ok(Type::Atom(Atom::String(content + &value))),
                token => {
                    return Err(TokenizerError::Quote(format!(
                        "err unterminated quote starting at {}, val {:?}",
                        content, token
                    )))
                }
            }
        }
        Err(TokenizerError::Quote(format!(
            "unterminated quote starting at {}",
            content
        )))
    }

    fn read_sequence_until<F>(
        &mut self,
        stop_token: Tokens,
        create_token: F,
        error_condition: TokenizerError,
    ) -> TokenizerResult<Type>
    where
        F: Fn(Vec<Type>) -> Type,
    {
        let mut content = Vec::new();
        while let Ok(token) = self.peek() {
            if token == stop_token {
                let _ = self.next();
                return Ok(create_token(content));
            }
            content.push(self.read_from()?);
        }
        Err(error_condition)
    }

    fn read_quote(&mut self, head: Type) -> TokenizerResult<Type> {
        match self.read_from() {
            Ok(tail) => Ok(Type::List(List {
                child: vec![head, tail],
            })),
            error => error,
        }
    }

    fn read_atom(&mut self, content: String) -> TokenizerResult<Type> {
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

        if content.starts_with('`') {
            return self.read_quote(Type::Atom(Atom::QuasiQuote));
        }

        if content.starts_with('\'') {
            return self.read_quote(Type::Atom(Atom::Quote));
        }

        if content.starts_with('@') {
            return self.read_quote(Type::Atom(Atom::Deref));
        }

        if content.starts_with('^') {
            return self.read_quote(Type::Atom(Atom::WithMeta));
        }

        if content.starts_with('~') {
            return self.read_quote(Type::Atom(Atom::Unquote));
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
    fn testing_escaped_qoute() {
        let mut reader = Reader::<InternalReader>::tokenize(r###""\""###)
            .expect("We should be able to create a Reader");
        let ast = reader
            .read_from();
        println!("AST is {:?}", ast);  // TODO remove
        assert!(ast.is_err());
    }

    #[test]
    fn testing_string_with_qoute() {
        let mut reader = Reader::<InternalReader>::tokenize(r###""abc \" dfg""###)
            .expect("We should be able to create a Reader");
        let ast = reader
            .read_from()
            .expect("We should be able to parse a single atom");
        assert_eq!(ast, Type::Atom(Atom::String("\"abc \\\" dfg\"".to_string())));
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
                child: vec![Type::Atom(Atom::String(String::from("\"Hello World\"")))]
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
