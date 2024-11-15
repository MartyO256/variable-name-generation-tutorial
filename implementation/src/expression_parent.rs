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

    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable { identifier: _ } => {}
            Expression::NamelessVariable { index: _ } => {}
            Expression::Abstraction { parameter: _, body } => {
                self.parent[body.into_usize()] = Option::Some(expression.clone());
                self.visit(*body);
            }
            Expression::NamelessAbstraction { body } => {
                self.parent[body.into_usize()] = Option::Some(expression.clone());
                self.visit(*body);
            }
            Expression::Application {
                function,
                arguments,
            } => {
                self.parent[function.into_usize()] = Option::Some(expression.clone());
                self.visit(*function);
                for &argument in arguments {
                    self.parent[argument.into_usize()] = Option::Some(expression.clone());
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
