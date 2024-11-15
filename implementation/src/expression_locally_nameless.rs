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
