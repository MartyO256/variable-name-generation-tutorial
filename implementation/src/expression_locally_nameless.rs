use crate::expression::{Expression, ExpressionArena, ExpressionId};

impl Expression {
    pub fn is_locally_nameless(expressions: &ExpressionArena, expression: ExpressionId) -> bool {
        IsLocallyNameless::new(expressions).check_is_locally_nameless(expression)
    }
}

struct IsLocallyNameless<'a> {
    expressions: &'a ExpressionArena,
}

impl<'a> IsLocallyNameless<'a> {
    pub fn new(expressions: &'a ExpressionArena) -> IsLocallyNameless<'a> {
        IsLocallyNameless { expressions }
    }

    fn is_locally_nameless(&self, expression: ExpressionId) -> bool {
        match &self.expressions[expression] {
            Expression::Variable { identifier: _ } => true,
            Expression::NamelessVariable { index: _ } => true,
            Expression::Abstraction {
                parameter: _,
                body: _,
            } => false,
            Expression::NamelessAbstraction { body } => self.is_locally_nameless(*body),
            Expression::Application {
                function,
                arguments,
            } => {
                if !self.is_locally_nameless(*function) {
                    false
                } else {
                    for &argument in arguments {
                        if !self.is_locally_nameless(argument) {
                            return false;
                        }
                    }
                    true
                }
            }
        }
    }

    pub fn check_is_locally_nameless(self, expression: ExpressionId) -> bool {
        self.is_locally_nameless(expression)
    }
}

#[cfg(test)]
mod tests {

    use crate::strings::StringArena;

    use super::*;

    fn check_locally_nameless(input1: &str, input2: &str, expected: bool) {
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
        check_locally_nameless("x", "x", true);
        check_locally_nameless("λf. x", "λf. x", true);
        check_locally_nameless("λ_. x", "λ_. x", true);
        check_locally_nameless("λf. λx. f x", "λf. λx. f x", true);
        check_locally_nameless("λx. λy. λz. x z (y z)", "λx. λy. λz. x z (y z)", true);
        check_locally_nameless("λ. x", "λ. x", true);
        check_locally_nameless("λ. λ. 2 1", "λ. λ. 2 1", true);
        check_locally_nameless("λ. λ. λ. 3 1 (2 1)", "λ. λ. λ. 3 1 (2 1)", true);

        check_locally_nameless("x", "y", false);
        check_locally_nameless("x", "λf. y", false);
        check_locally_nameless("x", "x y", false);
        check_locally_nameless("λf. x", "λf. y", false);
        check_locally_nameless("λ_. x", "λf. x", false);
        check_locally_nameless("λ_. x", "λ_. y", false);
        check_locally_nameless("λf. x", "λg. x", false);
        check_locally_nameless("λf. λx. f x", "λf. λy. f y", false);
        check_locally_nameless("λf. λx. f x", "λg. λx. g x", false);
        check_locally_nameless("λx. λy. λz. x z (y z)", "λx. λy. λz. x z (z y)", false);
        check_locally_nameless("λ. x", "λ. y", false);
        check_locally_nameless("λ. λ. 2 1", "λ. λ. 1 2", false);
        check_locally_nameless("λ. λ. λ. 3 1 (2 1)", "λ. λ. λ. 3 1 (1 2)", false);
    }
}
