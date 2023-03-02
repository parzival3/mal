#[cfg(test)]
mod test {
    use crate::env::*;
    use crate::list::*;
    use crate::mal::*;
    use crate::types::*;

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
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::List(List::NIL)
        );

        let expr = "(list? (list))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(empty? (list))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(empty? (list 1))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(list 1 2 3)";
        let list = List::new()
            .prepend(Value::Integer(3))
            .prepend(Value::Integer(2))
            .prepend(Value::Integer(1));
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

    #[test]
    fn if_tests() {
        let env = default_environment();

        let expr = "(if true 7 8)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(7));

        let expr = "(if false 7 8)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(8));

        let expr = "(if false 7 false)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(if true (+ 1 7) (+ 1 8))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(8));

        let expr = "(if false (+ 1 7) (+ 1 8))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(9));

        let expr = "(if 0 7 8)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(7));

        let expr = "(if (list) 7 8)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(7));

        let expr = "(if (list 1 2 3) 7 8)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(7));

        let expr = "(= (list) nil)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);
    }

    #[test]
    fn one_way_if() {
        let env = default_environment();

        let expr = "(if false (+ 1 7))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Nil);

        let expr = "(if nil 8)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Nil);

        let expr = "(if nil 8 7)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(7));

        let expr = "(if true (+ 1 7))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(8));
    }

    #[test]
    fn basic_conditionals() {
        let env = default_environment();

        let expr = "(= 2 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(= 1 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(= 1 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(= 1 (+ 1 1))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(= 2 (+ 1 1))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(= nil 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(= nil nil)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(> 2 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(> 1 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);
        let expr = "(> 1 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(>= 2 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(>= 1 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(>= 1 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(< 2 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);
        let expr = "(< 1 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);
        let expr = "(< 1 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(<= 2 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);
        let expr = "(<= 1 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(<= 1 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
    }

    #[test]
    fn test_equality() {
        let env = default_environment();
        let expr = "(= 1 1)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(= 0 0)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(= 1 0)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);
        let expr = "(= true true)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(= false false)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(= nil nil)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);

        let expr = "(= (list) (list))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(= (list) ())";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(= (list 1 2) (list 1 2))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::True);
        let expr = "(= (list 1) (list))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);
        let expr = "(= (list) (list 1))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);

        let expr = "(= 0 (list))";
        assert!(eval(&env, read(expr).unwrap()).is_err());

        let expr = "(= (list) 0)";
        assert!(eval(&env, read(expr).unwrap()).is_err());

        let expr = "(= (list nil) (list))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::False);
    }

    #[test]
    fn test_builting_and_user_defined_function() {
        let env = default_environment();

        let expr = "(+ 1 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(3));

        let expr = "((fn* (a b) (+ b a)) 3 4)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(7));

        let expr = "((fn* () 4) )";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(4));

        let expr = "( (fn* (f x) (f x)) (fn* (a) (+ 1 a)) 7)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(8));

        println!("------------------------------------------------------------------------------------------");
        let expr = "(((fn* (a) (fn* (b) (+ b a))) 5) 7)";
        let parsed = read(expr).unwrap();
        println!("Parsed {parsed}");
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(12));
    }
}
