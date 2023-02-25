#[cfg(test)]
mod test {
    use crate::env::*;
    use crate::list::List;
    use crate::types::*;
    use crate::mal::*;
    #[test]
    fn mal_tests_part_2() {
        let env = default_environment();
        let expr = "(+ 1 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(3));

        let expr = "(+ 5 (* 2 3))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(11));

        let expr = "(- (+ 5 (* 2 3)) 3)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(8));

        let expr = "(/ (- (+ 5 (* 2 3)) 3) 4)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(2));

        let expr = "(/ (- (+ 515 (* 87 311)) 302) 27)";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(1010)
        );

        let expr = "(* -3 6)";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(-18)
        );

        let expr = "(/ (- (+ 515 (* -87 311)) 296) 27)";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(-994)
        );

        let expr = "()";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::List(List::new())
        );

        let mut array = List::new();
        array = array.prepend(Value::Integer(1));
        array = array.prepend(Value::Integer(2));
        array = array.prepend(Value::Integer(3));
        array = array.reverse();

        let expr = "[1 2 (+ 1 2)]'";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Array(array)
        );

        let mut map = List::new();
        map = map.prepend(Value::String(String::from("\"a\"")));
        map = map.prepend(Value::Integer(15));
        map = map.reverse();

        let expr = "{\"a\" (+ 7 8)}'";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Map(map));

        let mut map = List::new();
        map = map.prepend(Value::Keyword(String::from(":a")));
        map = map.prepend(Value::Integer(15));
        map = map.reverse();

        let expr = "{:a (+ 7 8)}'";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Map(map));

        let map = List::new();
        let expr = "{}'";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Map(map));

        let array = List::new();

        let expr = "[]'";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Array(array)
        );
    }
}
