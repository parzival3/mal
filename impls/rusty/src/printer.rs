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
        Type::NativeFun(_) => pr_fun(),
    }
}

fn pr_fun() -> String {
    String::from("<function>")
}

fn pr_sequence(list: List, start: &str, end: &str) -> String {
    let new_output: Vec<String> = list
        .child
        .iter()
        .map(|atom| ast_to_string(atom.clone()))
        .collect();
    format!("{}{}{}", start, new_output.join(" "), end)
}

fn pr_atom(atom: Value) -> String {
    match atom {
        Value::Integer(value) => value.to_string(),
        Value::Symbol(value) => value.to_string(),
        Value::Nil => "nil".to_string(),
        Value::True => "true".to_string(),
        Value::False => "false".to_string(),
        Value::String(value) => value,
        Value::Keyword(value) => value,
        Value::SpliceUnquote => "splice-unquote".to_string(),
        Value::Unquote => "unquote".to_string(),
        Value::Deref => "deref".to_string(),
        Value::Quote => "quote".to_string(),
        Value::QuasiQuote => "quasiquote".to_string(),
        Value::WithMeta => "with-meta".to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]

    fn test_printing() {
        let ast = Type::List(List {
            child: vec![
                Type::Atom(Value::Symbol(Symbol(String::from("+")))),
                Type::Atom(Value::Integer(1)),
                Type::Atom(Value::Integer(2)),
            ],
        });
        assert_eq!(ast_to_string(ast), "(+ 1 2)".to_string());
    }

    #[test]
    fn testing_printing_nested_lists() {
        let ast = Type::List(List {
            child: vec![
                Type::Atom(Value::Symbol(Symbol(String::from("+")))),
                Type::List(List {
                    child: vec![Type::Atom(Value::Symbol(Symbol(String::from("+"))))],
                }),
            ],
        });

        assert_eq!(ast_to_string(ast), "(+ (+))".to_string());
    }
}
