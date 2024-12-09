use crate::expression::{Expression, ExpressionArena, ExpressionId};

impl Expression {
    pub fn height(expressions: &ExpressionArena, expression: ExpressionId) -> usize {
        ExpressionHeight::new(expressions).compute_expression_height(expression)
    }
}

struct ExpressionHeight<'a> {
    expressions: &'a ExpressionArena,
}

impl<'a> ExpressionHeight<'a> {
    pub fn new(expressions: &'a ExpressionArena) -> ExpressionHeight<'a> {
        ExpressionHeight { expressions }
    }

    fn height(&self, expression: ExpressionId) -> usize {
        match &self.expressions[expression] {
            Expression::Variable { identifier: _ } => 0,
            Expression::NamelessVariable { index: _ } => 0,
            Expression::Abstraction { parameter: _, body } => 1 + self.height(*body),
            Expression::NamelessAbstraction { body } => 1 + self.height(*body),
            Expression::Application {
                function,
                arguments,
            } => {
                let mut s = 0;
                s = std::cmp::max(s, self.height(*function));
                for &argument in arguments {
                    s = std::cmp::max(s, self.height(argument));
                }
                s
            }
        }
    }

    pub fn compute_expression_height(self, expression: ExpressionId) -> usize {
        self.height(expression)
    }
}

#[cfg(test)]
mod tests {

    use crate::strings::StringArena;

    use super::*;

    fn check_expression_height(input: &str, expected: usize) {
        let mut strings = StringArena::new();
        let mut expressions = ExpressionArena::new();

        let expression =
            Expression::parse_mixed_expression(&mut strings, &mut expressions, input.as_bytes())
                .unwrap();

        let height = Expression::height(&expressions, expression);

        assert_eq!(height, expected);
    }

    #[test]
    fn height_computes_expression_height() {
        check_expression_height("x", 0);
        check_expression_height("λf. x", 1);
        check_expression_height("λf. λx. f x", 2);
        check_expression_height("λx. λy. λz. x z (y z)", 3);
        check_expression_height("λ. x", 1);
        check_expression_height("λ. λ. 2 1", 2);
        check_expression_height("λ. λ. λ. 3 1 (2 1)", 3);
    }
}
