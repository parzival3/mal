#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Atom(Atom),
    List(List),
}
