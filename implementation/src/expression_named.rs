use crate::expression::{Expression, ExpressionArena, ExpressionId};

impl Expression {
    pub fn is_named(expressions: &ExpressionArena, expression: ExpressionId) -> bool {
        IsNamed::new(expressions).check_is_named(expression)
    }
}

struct IsNamed<'a> {
    expressions: &'a ExpressionArena,
}

impl<'a> IsNamed<'a> {
    pub fn new(expressions: &'a ExpressionArena) -> IsNamed<'a> {
        IsNamed { expressions }
    }

    fn is_named(&self, expression: ExpressionId) -> bool {
        match &self.expressions[expression] {
            Expression::Variable { identifier: _ } => true,
            Expression::NamelessVariable { index: _ } => false,
            Expression::Abstraction { parameter: _, body } => self.is_named(*body),
            Expression::NamelessAbstraction { body: _ } => false,
            Expression::Application {
                function,
                arguments,
            } => {
                if !self.is_named(*function) {
                    false
                } else {
                    for &argument in arguments {
                        if !self.is_named(argument) {
                            return false;
                        }
                    }
                    true
                }
            }
        }
    }

    pub fn check_is_named(self, expression: ExpressionId) -> bool {
        self.is_named(expression)
    }
}
