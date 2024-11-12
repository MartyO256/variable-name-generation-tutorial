use std::{collections::HashSet, rc::Rc};

use crate::{
    expression::{Expression, ExpressionArena, ExpressionId},
    referencing_environment::ReferencingEnvironment,
    strings::StringId,
};

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
            &Expression::Variable { identifier } => {
                if let Option::None = self.environment.lookup(identifier) {
                    self.free_variables.insert(identifier);
                }
            }
            &Expression::NamelessVariable { index: _ } => {}
            &Expression::Abstraction { parameter, body } => {
                self.environment.bind_option(parameter);
                self.visit(body);
                self.environment.unbind_option(parameter);
            }
            &Expression::NamelessAbstraction { body } => {
                self.environment.shift();
                self.visit(body);
                self.environment.unshift();
            }
            &Expression::Application {
                function,
                ref arguments,
            } => {
                self.visit(function);
                for &argument in arguments.iter() {
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

pub fn free_variables(
    environment: Rc<ReferencingEnvironment>,
    expressions: &ExpressionArena,
    expression: ExpressionId,
) -> HashSet<StringId> {
    let mut framed_environment = ReferencingEnvironment::new_frame(environment);
    FreeVariables::new(&mut framed_environment, expressions).free_variables(expression)
}

#[cfg(test)]
mod tests {

    use crate::{
        parser::parse_mixed_expression, referencing_environment::ReferencingEnvironment,
        strings::StringArena,
    };

    use super::*;

    fn check_free_variables(input: &str, expected: Vec<&str>) {
        let mut strings = StringArena::new();
        let mut expected_free_variables = HashSet::with_capacity(expected.len());
        for e in expected.iter() {
            expected_free_variables.insert(strings.intern(e.as_bytes()));
        }

        let mut expressions = ExpressionArena::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::new());

        let expression =
            parse_mixed_expression(&mut strings, &mut expressions, input.as_bytes()).unwrap();

        let free_variables = free_variables(referencing_environment, &expressions, expression);

        assert!(free_variables.eq(&expected_free_variables));
    }

    #[test]
    fn converts_to_locally_nameless_representation() {
        check_free_variables("x", vec!["x"]);
        check_free_variables("λf. x", vec!["x"]);
        check_free_variables("λf. λx. f x", vec![]);
        check_free_variables("λx. λy. λz. x z (y z)", vec![]);
        check_free_variables("λf. x1 x2 x3", vec!["x1", "x2", "x3"]);
    }
}
