use crate::types::*;
// TODO: should this be inside the trait implementing Atom and List? for now I'll simply follow the guide

pub fn pr_str(ast: Type) {
    println!("{}", ast_to_string(ast));
}

fn ast_to_string(ast: Type) -> String {
    match ast {
        Type::Atom(atom) => pr_atom(atom),
        Type::List(list) => pr_list(list),
    }
}

fn pr_list(list: List) -> String {
    if list.child.is_empty() {
        String::from("()")
    } else {
        let mut output = String::from("(");
        list.child
            .iter()
            .for_each(|atom| output += &(ast_to_string(atom.clone()) + " "));
        String::from(&output[0..output.len() - 1]) + ")" // TODO this is not very elegant... but it does the job
    }
}

fn pr_atom(atom: Atom) -> String {
    match atom {
        Atom::Integer(value) => value.to_string(),
        Atom::Symbol(value) => value,
        Atom::Nil => "nil".to_string(),
        Atom::True => "true".to_string(),
        Atom::False => "false".to_string(),
        Atom::String(value) => String::from("\"") + &value + "\"",
        Atom::Keyword(value) => value,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_printing() {
        let ast = Type::List(List { child: vec![] });
        assert_eq!(ast_to_string(ast), "()".to_string());
    }

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
