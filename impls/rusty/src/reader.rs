use core::str::Bytes;
use regex::Regex;

#[derive(Debug)]
enum Tokens {
    TildeAt,
    LeftParen,
    RightParen,
    LeftSquareBraket,
    RightSquareBraket,
    LeftBraket,
    RightBraket,
    String { content: String },
    Comment { content: String },
    Atom { content: String }, // Captures a sequence of zero or more non special characters (e.g. symbols, numbers, "true", "false")
}

const STANDALONE_TOKENS_MAPPING: [(&str, Tokens); 7] = [
    ("~@", Tokens::TildeAt),
    ("(", Tokens::LeftParen),
    (")", Tokens::RightParen),
    ("[", Tokens::LeftSquareBraket),
    ("]", Tokens::RightSquareBraket),
    ("{", Tokens::LeftBraket),
    ("}", Tokens::RightBraket),
];

struct Reader<'a> {
    input: Bytes<'a>,
}

impl<'a> Reader<'a> {
    pub fn tokenize(input: &'a str) -> Self {
        let regex = Regex::new(concat!("[\\s,]*(~@|[\\[\\]{}()'",
                                       "`~^@]|\"(?:\\.|[^\\\\\"])",
                                       "*\"?|;.*|[^\\s\\[\\]{}('\"`,;)]*)"))
        .unwrap();

        for cap in regex.captures_iter(input) {
            match STANDALONE_TOKENS_MAPPING.iter().find(|&x| x.0 == &cap[1]) {
                Some(token) => println!("{:?}", token),
                None => {
                    if cap[1].starts_with(";") {
                        let token = Tokens::Comment {
                            content: cap[1].to_string(),
                        };
                        println!("{:?}", token)
                    } else if cap[1].starts_with("\"") {
                        let token = Tokens::String {
                            content: cap[1].to_string(),
                        };
                        println!("{:?}", token)
                    } else {
                        let token = Tokens::Atom {
                            content: cap[1].to_string(),
                        };
                        println!("{:?}", token)
                    }
                }
            }
        }

        Reader {
            input: input.bytes(),
        }
    }

    pub fn new(input: &'a str) -> Self {
        Reader {
            input: input.bytes(),
        }
    }

    pub fn next(&mut self) -> Option<u8> {
        self.input.next()
    }

    pub fn peek(self) -> Option<u8> {
        self.input.peekable().peek().copied()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn check_reader() {
        let mut reader = Reader::new("aHello World");
        assert_eq!(reader.next(), Some(b'a'));
        assert_eq!(reader.next(), Some(b'H'));
    }

    #[test]
    fn testing_tokenizer() {
        let mut reader = Reader::tokenize("(+ 1 2 \"Hello\")");
    }
}
