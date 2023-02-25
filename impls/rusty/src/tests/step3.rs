#[cfg(test)]
mod test {
    use crate::env::*;
    use crate::mal::*;
    use crate::types::*;
    #[test]
    fn tests() {
        let env = default_environment();
        let expr = "(def! a 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(2));

        let expr = "(+ (def! a 2) (def! b 3))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(5));

        let expr = "(+ 1 2)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(3));

        let expr = "(def! x 3)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(3));

        let expr = "(def! x 4)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(4));

        let expr = "(def! y (+ 1 7))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(8));

        let expr = "(def! mynum 111)";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(111)
        );

        let expr = "(def! MYNUM 222)";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(222)
        );

        let expr = "MYNUM"; // it should be tested after mynum is defined
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(222)
        );

        let expr = "mynum";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(111)
        );

        let expr = "(abc 1 2 3)";
        assert!(eval(&env, read(expr).unwrap()).is_err());

        let expr = "(def! w 123)";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(123)
        );

        let expr = "(def! w (abc))";
        assert!(eval(&env, read(expr).unwrap()).is_err());

        let expr = "w";
        assert_eq!(
            eval(&env, read(expr).unwrap()).unwrap(),
            Value::Integer(123)
        );

        let expr = "(let* (z 9) z)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(9));

        let expr = "(let* (x 9) x)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(9));

        let expr = "(let* (z (+ 2 3)) (+ 1 z))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(6));

        let expr = "(let* (p (+ 2 3) q (+ 2 p)) (+ p q))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(12));

        let expr = "(let* (p (+ 2 3) q (+ 2 p)) (+ p q))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(12));

        let expr = "(def! y (let* (z 7) z))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(7));

        let expr = "(def! a 4)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(4));

        let expr = "(let* (q 9) q)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(9));

        let expr = "(let* (q 9) a)";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(4));

        let expr = "(let* (z 2) (let* (q 9) a))";
        assert_eq!(eval(&env, read(expr).unwrap()).unwrap(), Value::Integer(4));
    }
}
