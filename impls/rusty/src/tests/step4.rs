#[cfg(test)]
mod test {
    use crate::env::*;
    use crate::mal::*;
    use crate::types::*;
    use crate::list::*;

    #[test]
    fn extra_tests() {
        let env = default_environment();

        let expr = "(> 10 2 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(< 10 2 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(< 10 11 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(= 10 10 10)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(= 10 1 10)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(= 10 \"a\" 10)";
        assert!(eval(&env, read(expr).unwrap()).is_err());
    }

    #[test]
    fn list_tests() {
        let env = default_environment();

        let expr = "(list)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::List(List::NIL));

        let expr = "(list? (list))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(empty? (list))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(empty? (list 1))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(list 1 2 3)";
        let list = List::new().prepend(Value::Integer(3)).prepend(Value::Integer(2)).prepend(Value::Integer(1));
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::List(list));

        let expr = "(count (list 1 2 3))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(3));

        let expr = "(count (list))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(0));

        let expr = "(count nil)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(0));

        let expr = "(if (> (count (list 1 2 3)) 3) 89 78)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(78));

        let expr = "(if (>= (count (list 1 2 3)) 3) 89 78)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(89));
    }
}
