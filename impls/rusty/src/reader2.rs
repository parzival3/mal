use crate::errors::*;
use crate::list::*;
use crate::types2::*;
use regex::Regex;

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

// The reader trait is not needed, the idea behind it was to
// allow to easilly swap the "regex" implementation and the "scanner"
// implementation with minimal effor and without breaking the
// "regex" one
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
        let regex = Regex::new(concat!(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
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
                        if !cap[1].ends_with('\"') || cap[1].len() < 2 {
                            return Err(TokenizerError::Quote(format!(
                                "unterminated quote starting at {}",
                                &cap[1]
                            )));
                        }
                        tokens.push(Tokens::String(cap[1].to_string()));
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

    pub fn read_from(&mut self) -> TokenizerResult<Value> {
        match self.next()? {
            Tokens::TildeAt => self.read_quote(Value::SpliceUnquote),
            Tokens::LeftParen => self.read_sequence_until(
                Tokens::RightParen,
                |child| Value::List(child),
                TokenizerError::UnbalancedList,
            ),
            Tokens::RightParen => Err(TokenizerError::UnbalancedList),
            Tokens::LeftSquareBraket => self.read_sequence_until(
                Tokens::RightSquareBraket,
                |child| Value::Array(child),
                TokenizerError::UnbalancedArray,
            ),
            Tokens::RightSquareBraket => Err(TokenizerError::UnbalancedArray),
            Tokens::LeftBraket => self.read_sequence_until(
                Tokens::RightBraket,
                |child| Value::Map(child),
                TokenizerError::UnbalancedMap,
            ),
            Tokens::RightBraket => Err(TokenizerError::UnbalancedMap),
            Tokens::String(content) => self.validate_string(content),
            Tokens::Comment(_) => self.read_from(), // skip the current comment
            Tokens::Atom(content) => self.read_atom(content),
        }
    }

    fn validate_string(&mut self, content: String) -> TokenizerResult<Value> {
        // Black magic for parsing the content of the string, not very proud of it, but
        // it works.
        // The StringChecks "accumulator" is a struct where the first field represent if we
        // have a properly escaped string and the second one represent if the last double quote (") was
        // an escaped one. The struct is not actually need a pair of boolean will suffice but makes this
        // clumsy algorithm more easy to understand

        struct StringChecks {
            pub missing_escape: bool,
            pub is_last_quote_escaped: bool,
        }

        let escaped = content.chars().fold(
            StringChecks {
                missing_escape: false,
                is_last_quote_escaped: false,
            },
            |escaped, ch| {
                if escaped.missing_escape {
                    if ch == '\"' {
                        StringChecks {
                            missing_escape: false,
                            is_last_quote_escaped: true,
                        }
                    } else {
                        StringChecks {
                            missing_escape: false,
                            is_last_quote_escaped: false,
                        }
                    }
                } else {
                    if ch == '\\' {
                        StringChecks {
                            missing_escape: true,
                            is_last_quote_escaped: false,
                        }
                    } else {
                        StringChecks {
                            missing_escape: false,
                            is_last_quote_escaped: false,
                        }
                    }
                }
            },
        );

        if escaped.missing_escape || escaped.is_last_quote_escaped {
            Err(TokenizerError::Quote(format!(
                "unterminated quote starting at {}",
                content
            )))
        } else {
            return Ok(Value::String(content));
        }
    }

    fn read_sequence_until<F>(
        &mut self,
        stop_token: Tokens,
        create_token: F,
        error_condition: TokenizerError,
    ) -> TokenizerResult<Value>
    where
        F: Fn(List<Value>) -> Value,
    {
        // This could be a simple fold if the inner type was an iterator peekable :-)
        let mut content = Vec::new();
        while let Ok(token) = self.peek() {
            if token == stop_token {
                let _ = self.next();
                return Ok(create_token(
                    content
                        .into_iter()
                        .rfold(List::new(), |acc, elem| acc.prepend(elem)),
                ));
            }
            content.push(self.read_from()?);
        }
        Err(error_condition)
    }

    fn read_quote(&mut self, head: Value) -> TokenizerResult<Value> {
        self.read_from()
            .map(|val| Value::List(List::new().prepend(val).prepend(head)))
    }

    fn read_with_meta(&mut self) -> TokenizerResult<Value> {
        let first_arg = self.read_from()?; // TODO maybe this can be improved
        let second_arg = self.read_from()?;
        Ok(Value::List(
            List::new().prepend(second_arg).prepend(first_arg),
        ))
    }

    fn read_atom(&mut self, content: String) -> TokenizerResult<Value> {
        if content == "nil" {
            return Ok(Value::Nil);
        }

        if let Ok(value) = content.parse::<i64>() {
            return Ok(Value::Integer(value));
        }

        if content == "true" {
            return Ok(Value::True);
        }

        if content == "false" {
            return Ok(Value::False);
        }

        if content.starts_with(':') {
            return Ok(Value::Keyword(content));
        }

        if content.starts_with('`') {
            return self.read_quote(Value::QuasiQuote);
        }

        if content.starts_with('\'') {
            return self.read_quote(Value::Quote);
        }

        if content.starts_with('@') {
            return self.read_quote(Value::Deref);
        }

        if content.starts_with('^') {
            return self.read_with_meta();
        }

        if content.starts_with('~') {
            return self.read_quote(Value::Unquote);
        }

        return Ok(Value::Symbol(Symbol(content)));
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
            Value::List(List::new().prepend(Value::Symbol(Symbol::from("+"))))
        );
    }

    #[test]
    fn testing_escaped_qoute() {
        let mut reader = Reader::<InternalReader>::tokenize(r###""\""###)
            .expect("We should be able to create a Reader");
        let ast = reader.read_from();
        println!("AST is {:?}", ast); // TODO remove
        assert!(ast.is_err());
    }

    #[test]
    fn testing_string_with_qoute() {
        let mut reader = Reader::<InternalReader>::tokenize(r###""abc \" dfg""###)
            .expect("We should be able to create a Reader");
        let ast = reader
            .read_from()
            .expect("We should be able to parse a single atom");
        assert_eq!(ast, Value::String("\"abc \\\" dfg\"".to_string()));
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
            Value::List(
                List::new()
                    .prepend(Value::Integer(1))
                    .prepend(Value::List(
                        List::new().prepend(Value::Symbol(Symbol::from("+")))
                    ))
                    .prepend(Value::Symbol(Symbol::from("+")))
            )
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
            Value::List(
                List::new()
                    .prepend(Value::Integer(2))
                    .prepend(Value::Integer(1))
                    .prepend(Value::Symbol(Symbol::from("+")))
            )
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
            Value::List(
                List::new()
                    .prepend(Value::Keyword(String::from(":test")))
                    .prepend(Value::Symbol(Symbol::from("+")))
            )
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
            Value::List(
                List::new()
                    .prepend(Value::False)
                    .prepend(Value::True)
                    .prepend(Value::Nil)
                    .prepend(Value::Symbol(Symbol::from("+")))
            )
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
            Value::List(List::new().prepend(Value::String(String::from("\"Hello World\""))))
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
