#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Integer(i64),
    Symbol(String),
    Nil,
    True,
    False,
    String(String),
    Keyword(String)
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub child: Vec<Type>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Atom(Atom),
    List(List),
}
