use crate::expression::{Expression, ExpressionArena, ExpressionId};

impl Expression {
    pub fn parent_expressions(
        expressions: &ExpressionArena,
        expression: ExpressionId,
    ) -> Vec<Option<ExpressionId>> {
        Parent::new(expressions).compute_parent_expressions(expression)
    }
}

struct Parent<'a> {
    expressions: &'a ExpressionArena,
    parent: Vec<Option<ExpressionId>>,
}

impl<'a> Parent<'a> {
    pub fn new(expressions: &'a ExpressionArena) -> Parent<'a> {
        Parent {
            expressions,
            parent: vec![Option::None; expressions.len()],
        }
    }

    #[inline]
    fn set_parent(&mut self, expression: ExpressionId, parent: ExpressionId) {
        self.parent[expression.into_usize()] = Option::Some(parent);
    }

    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable { identifier: _ } => {}
            Expression::NamelessVariable { index: _ } => {}
            Expression::Abstraction { parameter: _, body } => {
                self.set_parent(*body, expression);
                self.visit(*body);
            }
            Expression::NamelessAbstraction { body } => {
                self.set_parent(*body, expression);
                self.visit(*body);
            }
            Expression::Application {
                function,
                arguments,
            } => {
                self.set_parent(*function, expression);
                self.visit(*function);
                for &argument in arguments {
                    self.set_parent(argument, expression);
                    self.visit(argument);
                }
            }
        }
    }

    pub fn compute_parent_expressions(
        mut self,
        expression: ExpressionId,
    ) -> Vec<Option<ExpressionId>> {
        self.visit(expression);
        self.parent
    }
}

#[cfg(test)]
mod tests {

    use crate::strings::StringArena;

    use super::*;

    fn perform_check_parent_expressions(
        expressions: &ExpressionArena,
        expression: ExpressionId,
        parents: &Vec<Option<ExpressionId>>,
        parent: Option<ExpressionId>,
    ) {
        assert_eq!(parents[expression.into_usize()], parent);
        match &expressions[expression] {
            Expression::Variable { identifier: _ } => {}
            Expression::NamelessVariable { index: _ } => {}
            Expression::Abstraction { parameter: _, body } => {
                perform_check_parent_expressions(
                    expressions,
                    *body,
                    parents,
                    Option::Some(expression),
                );
            }
            Expression::NamelessAbstraction { body } => {
                perform_check_parent_expressions(
                    expressions,
                    *body,
                    parents,
                    Option::Some(expression),
                );
            }
            Expression::Application {
                function,
                arguments,
            } => {
                perform_check_parent_expressions(
                    expressions,
                    *function,
                    parents,
                    Option::Some(expression),
                );
                for argument in arguments {
                    perform_check_parent_expressions(
                        expressions,
                        *argument,
                        parents,
                        Option::Some(expression),
                    );
                }
            }
        }
    }

    fn check_parent_expressions(input: &str) {
        let mut strings = StringArena::new();
        let mut expressions = ExpressionArena::new();

        let expression =
            Expression::parse_mixed_expression(&mut strings, &mut expressions, input.as_bytes())
                .unwrap();

        let parents = Expression::parent_expressions(&expressions, expression);

        perform_check_parent_expressions(&expressions, expression, &parents, Option::None);
    }

    #[test]
    fn parent_expressions_computes_parent_expressions() {
        check_parent_expressions("x");
        check_parent_expressions("λf. x");
        check_parent_expressions("λf. λx. f x");
        check_parent_expressions("λx. λy. λz. x z (y z)");
        check_parent_expressions("λ. x");
        check_parent_expressions("λ. λ. 2 1");
        check_parent_expressions("λ. λ. λ. 3 1 (2 1)");
    }
}
