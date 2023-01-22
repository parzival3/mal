use crate::types::*;
// TODO: should this be inside the trait implementing Atom and List? for now I'll simply follow the guide

pub fn pr_str(ast: Type) {
    println!("{}", ast_to_string(ast));
}

fn ast_to_string(ast: Type) -> String {
    match ast {
        Type::Atom(atom) => pr_atom(atom),
        Type::List(list) => pr_sequence(list, "(", ")"),
        Type::Array(list) => pr_sequence(list, "[", "]"),
        Type::Map(list) => pr_sequence(list, "{", "}"),
    }
}

fn pr_sequence(list: List, start: &str, end: &str) -> String {
    let new_output: Vec<String> = list.child.iter().map(|atom| ast_to_string(atom.clone())).collect();
    format!("{}{}{}", start, new_output.join(" "), end)
}

fn pr_atom(atom: Atom) -> String {
    match atom {
        Atom::Integer(value) => value.to_string(),
        Atom::Symbol(value) => value,
        Atom::Nil => "nil".to_string(),
        Atom::True => "true".to_string(),
        Atom::False => "false".to_string(),
        Atom::String(value) => value,
        Atom::Keyword(value) => value,
        Atom::SpliceUnquote => "splice-unquote".to_string(),
        Atom::Unquote => "unquote".to_string(),
        Atom::Deref => "deref".to_string(),
        Atom::Quote => "quote".to_string(),
        Atom::QuasiQuote => "quasiquote".to_string(),
        Atom::WithMeta => "with-meta".to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]

    fn test_printing() {
        let ast = Type::List(List {
            child: vec![
                Type::Atom(Atom::Symbol(String::from("+"))),
                Type::Atom(Atom::Integer(1)),
                Type::Atom(Atom::Integer(2)),
            ],
        });
        assert_eq!(ast_to_string(ast), "(+ 1 2)".to_string());
    }

    #[test]
    fn testing_printing_nested_lists() {
        let ast = Type::List(List {
            child: vec![
                Type::Atom(Atom::Symbol(String::from("+"))),
                Type::List(List {
                    child: vec![Type::Atom(Atom::Symbol(String::from("+")))],
                }),
            ],
        });

        assert_eq!(ast_to_string(ast), "(+ (+))".to_string());
    }
}
