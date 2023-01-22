#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Integer(i64),
    Symbol(String),
    Nil,
    True,
    False,
    String(String),
    Keyword(String),
    SpliceUnquote,
    Unquote,
    Deref,
    Quote,
    WithMeta
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub child: Vec<Type>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Atom(Atom),
    List(List),
    Array(List),
    Map(List),
}
