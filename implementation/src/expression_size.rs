use crate::expression::{Expression, ExpressionArena, ExpressionId};

impl Expression {
    pub fn size(expressions: &ExpressionArena, expression: ExpressionId) -> usize {
        ExpressionSize::new(expressions).compute_expression_size(expression)
    }
}

struct ExpressionSize<'a> {
    expressions: &'a ExpressionArena,
}

impl<'a> ExpressionSize<'a> {
    pub fn new(expressions: &'a ExpressionArena) -> ExpressionSize<'a> {
        ExpressionSize { expressions }
    }

    fn size(&self, expression: ExpressionId) -> usize {
        match &self.expressions[expression] {
            Expression::Variable { identifier: _ } => 1,
            Expression::NamelessVariable { index: _ } => 1,
            Expression::Abstraction { parameter: _, body } => 1 + self.size(*body),
            Expression::NamelessAbstraction { body } => 1 + self.size(*body),
            Expression::Application {
                function,
                arguments,
            } => {
                let mut s = 1;
                s += self.size(*function);
                for &argument in arguments {
                    s += self.size(argument);
                }
                s
            }
        }
    }

    pub fn compute_expression_size(self, expression: ExpressionId) -> usize {
        self.size(expression)
    }
}

#[cfg(test)]
mod tests {

    use crate::strings::StringArena;

    use super::*;

    fn check_expression_size(input: &str, expected: usize) {
        let mut strings = StringArena::new();
        let mut expressions = ExpressionArena::new();

        let expression =
            Expression::parse_mixed_expression(&mut strings, &mut expressions, input.as_bytes())
                .unwrap();

        let size = Expression::size(&expressions, expression);

        assert_eq!(size, expected);
    }

    #[test]
    fn size_computes_expression_size() {
        check_expression_size("x", 1);
        check_expression_size("λf. x", 2);
        check_expression_size("λf. λx. f x", 5);
        check_expression_size("λx. λy. λz. x z (y z)", 9);
    }
}
