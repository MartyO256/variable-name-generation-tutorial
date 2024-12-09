use std::{collections::HashSet, rc::Rc};

use crate::{
    expression::{Expression, ExpressionArena, ExpressionId},
    referencing_environment::ReferencingEnvironment,
    strings::StringId,
};

impl Expression {
    pub fn free_variables(
        environment: Rc<ReferencingEnvironment>,
        expressions: &ExpressionArena,
        expression: ExpressionId,
    ) -> HashSet<StringId> {
        let mut framed_environment = ReferencingEnvironment::new_frame(environment);
        FreeVariables::new(&mut framed_environment, expressions).free_variables(expression)
    }
}

struct FreeVariables<'a> {
    environment: &'a mut ReferencingEnvironment,
    expressions: &'a ExpressionArena,
    free_variables: HashSet<StringId>,
}

impl<'a> FreeVariables<'a> {
    pub fn new(
        referencing_environment: &'a mut ReferencingEnvironment,
        expressions: &'a ExpressionArena,
    ) -> FreeVariables<'a> {
        FreeVariables {
            environment: referencing_environment,
            expressions,
            free_variables: HashSet::new(),
        }
    }

    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable { identifier } => {
                if self.environment.lookup(*identifier).is_none() {
                    self.free_variables.insert(*identifier);
                }
            }
            Expression::NamelessVariable { index: _ } => {}
            Expression::Abstraction { parameter, body } => match parameter {
                Option::Some(parameter) => {
                    self.environment.bind(*parameter);
                    self.visit(*body);
                    self.environment.unbind(*parameter);
                }
                Option::None => {
                    self.environment.shift();
                    self.visit(*body);
                    self.environment.unshift();
                }
            },
            Expression::NamelessAbstraction { body } => {
                self.environment.shift();
                self.visit(*body);
                self.environment.unshift();
            }
            Expression::Application {
                function,
                arguments,
            } => {
                self.visit(*function);
                for &argument in arguments {
                    self.visit(argument);
                }
            }
        }
    }

    pub fn free_variables(mut self, expression: ExpressionId) -> HashSet<StringId> {
        self.visit(expression);
        self.free_variables
    }
}

#[cfg(test)]
mod tests {

    use crate::{referencing_environment::ReferencingEnvironment, strings::StringArena};

    use super::*;

    fn check_free_variables(input: &str, expected: Vec<&str>) {
        let mut strings = StringArena::new();
        let mut expected_free_variables = HashSet::with_capacity(expected.len());
        for e in expected {
            expected_free_variables.insert(strings.intern(e.as_bytes()));
        }

        let mut expressions = ExpressionArena::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::new());

        let expression =
            Expression::parse_mixed_expression(&mut strings, &mut expressions, input.as_bytes())
                .unwrap();

        let free_variables =
            Expression::free_variables(referencing_environment, &expressions, expression);

        assert!(free_variables.eq(&expected_free_variables));
    }

    #[test]
    fn free_variables_computes_the_free_variables_in_the_expression() {
        check_free_variables("x", vec!["x"]);
        check_free_variables("λf. x", vec!["x"]);
        check_free_variables("λf. λx. f x", vec![]);
        check_free_variables("λx. λy. λz. x z (y z)", vec![]);
        check_free_variables("λf. x1 x2 x3", vec!["x1", "x2", "x3"]);
    }
}
