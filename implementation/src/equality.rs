use crate::expression::{Expression, ExpressionArena, ExpressionId};

impl Expression {
    pub fn equals(
        (expressions1, e1): (&ExpressionArena, ExpressionId),
        (expressions2, e2): (&ExpressionArena, ExpressionId),
    ) -> bool {
        Equality::new(expressions1, expressions2).check_equality(e1, e2)
    }
}

struct Equality<'a> {
    expressions1: &'a ExpressionArena,
    expressions2: &'a ExpressionArena,
}

impl<'a> Equality<'a> {
    pub fn new(
        expressions1: &'a ExpressionArena,
        expressions2: &'a ExpressionArena,
    ) -> Equality<'a> {
        Equality {
            expressions1,
            expressions2,
        }
    }

    fn equals(&self, e1: ExpressionId, e2: ExpressionId) -> bool {
        match (&self.expressions1[e1], &self.expressions2[e2]) {
            (Expression::Variable { identifier: i1 }, Expression::Variable { identifier: i2 }) => {
                i1 == i2
            }
            (
                Expression::NamelessVariable { index: i1 },
                Expression::NamelessVariable { index: i2 },
            ) => i1 == i2,
            (
                Expression::Abstraction {
                    parameter: param1,
                    body: b1,
                },
                Expression::Abstraction {
                    parameter: param2,
                    body: b2,
                },
            ) => param1 == param2 && self.equals(*b1, *b2),
            (
                Expression::NamelessAbstraction { body: b1 },
                Expression::NamelessAbstraction { body: b2 },
            ) => self.equals(*b1, *b2),
            (
                Expression::Application {
                    function: f1,
                    arguments: as1,
                },
                Expression::Application {
                    function: f2,
                    arguments: as2,
                },
            ) => {
                if as1.len() != as2.len() || !self.equals(*f1, *f2) {
                    false
                } else {
                    for (&a1, &a2) in as1.iter().zip(as2.iter()) {
                        if !self.equals(a1, a2) {
                            return false;
                        }
                    }
                    true
                }
            }
            _ => false,
        }
    }

    pub fn check_equality(self, e1: ExpressionId, e2: ExpressionId) -> bool {
        self.equals(e1, e2)
    }
}

#[cfg(test)]
mod tests {

    use crate::strings::StringArena;

    use super::*;

    #[test]
    fn equals_same_variable() {
        let mut strings = StringArena::new();
        let nx = strings.intern("x".as_bytes());

        let mut expressions = ExpressionArena::new();
        let r1 = expressions.variable(nx);
        let r2 = expressions.variable(nx);

        assert!(Expression::equals((&expressions, r1), (&expressions, r1)));
        assert!(Expression::equals((&expressions, r1), (&expressions, r2)));
        assert!(Expression::equals((&expressions, r2), (&expressions, r2)));
    }

    #[test]
    fn equals_same_nameless_variable() {
        let mut expressions = ExpressionArena::new();
        let r1 = expressions.nameless_variable(1.into());
        let r2 = expressions.nameless_variable(1.into());

        assert!(Expression::equals((&expressions, r1), (&expressions, r1)));
        assert!(Expression::equals((&expressions, r1), (&expressions, r2)));
        assert!(Expression::equals((&expressions, r2), (&expressions, r2)));
    }

    fn check_expression_equality(input1: &str, input2: &str, expected: bool) {
        let mut strings = StringArena::new();
        let mut expressions = ExpressionArena::new();

        let expression1 =
            Expression::parse_mixed_expression(&mut strings, &mut expressions, input1.as_bytes())
                .unwrap();
        let expression2 =
            Expression::parse_mixed_expression(&mut strings, &mut expressions, input2.as_bytes())
                .unwrap();

        assert_eq!(
            Expression::equals((&expressions, expression1), (&expressions, expression2)),
            expected
        );
    }

    #[test]
    fn equals_decides_structural_equality() {
        check_expression_equality("x", "x", true);
        check_expression_equality("λf. x", "λf. x", true);
        check_expression_equality("λ_. x", "λ_. x", true);
        check_expression_equality("λf. λx. f x", "λf. λx. f x", true);
        check_expression_equality("λx. λy. λz. x z (y z)", "λx. λy. λz. x z (y z)", true);
        check_expression_equality("λ. x", "λ. x", true);
        check_expression_equality("λ. λ. 2 1", "λ. λ. 2 1", true);
        check_expression_equality("λ. λ. λ. 3 1 (2 1)", "λ. λ. λ. 3 1 (2 1)", true);

        check_expression_equality("x", "y", false);
        check_expression_equality("x", "λf. y", false);
        check_expression_equality("x", "x y", false);
        check_expression_equality("λf. x", "λf. y", false);
        check_expression_equality("λ_. x", "λf. x", false);
        check_expression_equality("λ_. x", "λ_. y", false);
        check_expression_equality("λf. x", "λg. x", false);
        check_expression_equality("λf. λx. f x", "λf. λy. f y", false);
        check_expression_equality("λf. λx. f x", "λg. λx. g x", false);
        check_expression_equality("λx. λy. λz. x z (y z)", "λx. λy. λz. x z (z y)", false);
        check_expression_equality("λ. x", "λ. y", false);
        check_expression_equality("λ. λ. 2 1", "λ. λ. 1 2", false);
        check_expression_equality("λ. λ. λ. 3 1 (2 1)", "λ. λ. λ. 3 1 (1 2)", false);
    }
}
