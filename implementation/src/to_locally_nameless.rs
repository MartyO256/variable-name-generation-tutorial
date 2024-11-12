use std::rc::Rc;

use crate::{
    expression::{Expression, ExpressionArena, ExpressionId},
    referencing_environment::ReferencingEnvironment,
};

struct Indexing<'a> {
    environment: &'a mut ReferencingEnvironment,
    arena: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
}

impl<'a> Indexing<'a> {
    pub fn new(
        environment: &'a mut ReferencingEnvironment,
        arena: &'a ExpressionArena,
        destination: &'a mut ExpressionArena,
    ) -> Indexing<'a> {
        Indexing {
            environment,
            arena,
            destination,
        }
    }

    fn to_locally_nameless(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.arena[expression] {
            &Expression::Variable { identifier } => {
                match self.environment.lookup_index(identifier) {
                    Option::Some(index) => self.destination.nameless_variable(index.into()),
                    Option::None => self.destination.variable(identifier),
                }
            }
            &Expression::NamelessVariable { ref index } => {
                self.destination.nameless_variable(*index)
            }
            &Expression::Abstraction { parameter, body } => {
                self.environment.bind_option(parameter);
                let indexed_body = self.to_locally_nameless(body);
                self.environment.unbind_option(parameter);
                self.destination.nameless_abstraction(indexed_body)
            }
            &Expression::NamelessAbstraction { ref body } => {
                self.environment.shift();
                let indexed_body = self.to_locally_nameless(*body);
                self.environment.unshift();
                self.destination.nameless_abstraction(indexed_body)
            }
            &Expression::Application {
                function,
                ref arguments,
            } => {
                let indexed_function = self.to_locally_nameless(function);
                let mut indexed_arguments = Vec::with_capacity(arguments.len());
                for &argument in arguments.iter() {
                    let indexed_argument = self.to_locally_nameless(argument);
                    indexed_arguments.push(indexed_argument)
                }
                self.destination
                    .application(indexed_function, indexed_arguments)
            }
        }
    }

    pub fn convert(mut self, expression: ExpressionId) -> ExpressionId {
        self.to_locally_nameless(expression)
    }
}

pub fn to_locally_nameless(
    (environment, arena, expression): (Rc<ReferencingEnvironment>, &ExpressionArena, ExpressionId),
    destination: &mut ExpressionArena,
) -> ExpressionId {
    let mut framed_environment = ReferencingEnvironment::new_frame(environment);
    Indexing::new(&mut framed_environment, arena, destination).convert(expression)
}

pub fn is_locally_nameless(expressions: &ExpressionArena, expression: ExpressionId) -> bool {
    match &expressions[expression] {
        &Expression::Variable { identifier: _ } => true,
        &Expression::NamelessVariable { index: _ } => true,
        &Expression::Abstraction {
            parameter: _,
            body: _,
        } => false,
        &Expression::NamelessAbstraction { body } => is_locally_nameless(expressions, body),
        &Expression::Application {
            function,
            ref arguments,
        } => {
            if !is_locally_nameless(expressions, function) {
                false
            } else {
                for &argument in arguments.iter() {
                    if !is_locally_nameless(expressions, argument) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        equality::equals,
        free_variables::free_variables,
        parser::{parse_expression, parse_mixed_expression},
        referencing_environment::ReferencingEnvironment,
        strings::StringArena,
        to_locally_nameless::to_locally_nameless,
    };

    use super::*;

    fn check_to_locally_nameless_structural_equality(input: &str, expected: &str) {
        let mut strings = StringArena::new();
        let mut parsed_expressions = ExpressionArena::new();
        let mut converted_expressions = ExpressionArena::new();
        let environment = Rc::new(ReferencingEnvironment::new());

        let expression =
            parse_expression(&mut strings, &mut parsed_expressions, input.as_bytes()).unwrap();

        let expected_expression =
            parse_mixed_expression(&mut strings, &mut parsed_expressions, expected.as_bytes())
                .unwrap();
        assert!(is_locally_nameless(
            &parsed_expressions,
            expected_expression
        ));

        let nameless_expression = to_locally_nameless(
            (environment.clone(), &parsed_expressions, expression),
            &mut converted_expressions,
        );
        assert!(is_locally_nameless(
            &converted_expressions,
            nameless_expression
        ));

        assert!(
            free_variables(environment.clone(), &parsed_expressions, expression).eq(
                &free_variables(
                    environment.clone(),
                    &converted_expressions,
                    nameless_expression
                )
            )
        );

        assert!(equals(
            (&converted_expressions, nameless_expression),
            (&parsed_expressions, expected_expression)
        ));
    }

    #[test]
    fn converts_to_locally_nameless_representation() {
        check_to_locally_nameless_structural_equality("x", "x");
        check_to_locally_nameless_structural_equality("λx. x", "λ. 1");
        check_to_locally_nameless_structural_equality("λf. x", "λ. x");
        check_to_locally_nameless_structural_equality("λx. λy. x", "λ. λ. 2");
        check_to_locally_nameless_structural_equality("λx. λy. y", "λ. λ. 1");
        check_to_locally_nameless_structural_equality("λf. λx. f x", "λ. λ. 2 1");
        check_to_locally_nameless_structural_equality(
            "λx. λy. λz. x z (y z)",
            "λ. λ. λ. 3 1 (2 1)",
        );
        check_to_locally_nameless_structural_equality("x z (y z)", "x z (y z)");
    }
}
