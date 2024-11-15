use std::rc::Rc;

use crate::{
    expression::{Expression, ExpressionArena, ExpressionId},
    referencing_environment::ReferencingEnvironment,
};

impl Expression {
    pub fn to_locally_nameless(
        (environment, arena, expression): (
            Rc<ReferencingEnvironment>,
            &ExpressionArena,
            ExpressionId,
        ),
        destination: &mut ExpressionArena,
    ) -> ExpressionId {
        let mut framed_environment = ReferencingEnvironment::new_frame(environment);
        Indexing::new(&mut framed_environment, arena, destination).convert(expression)
    }
}

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

    fn convert_to_locally_nameless(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.arena[expression] {
            Expression::Variable { identifier } => {
                match self.environment.lookup_index(*identifier) {
                    Option::Some(index) => self.destination.nameless_variable(index),
                    Option::None => self.destination.variable(*identifier),
                }
            }
            Expression::NamelessVariable { index } => self.destination.nameless_variable(*index),
            Expression::Abstraction { parameter, body } => {
                self.environment.bind_option(*parameter);
                let indexed_body = self.convert_to_locally_nameless(*body);
                self.environment.unbind_option(*parameter);
                self.destination.nameless_abstraction(indexed_body)
            }
            Expression::NamelessAbstraction { body } => {
                self.environment.shift();
                let indexed_body = self.convert_to_locally_nameless(*body);
                self.environment.unshift();
                self.destination.nameless_abstraction(indexed_body)
            }
            Expression::Application {
                function,
                arguments,
            } => {
                let indexed_function = self.convert_to_locally_nameless(*function);
                let mut indexed_arguments = Vec::with_capacity(arguments.len());
                for &argument in arguments {
                    let indexed_argument = self.convert_to_locally_nameless(argument);
                    indexed_arguments.push(indexed_argument)
                }
                self.destination
                    .application(indexed_function, indexed_arguments)
            }
        }
    }

    pub fn convert(mut self, expression: ExpressionId) -> ExpressionId {
        self.convert_to_locally_nameless(expression)
    }
}

#[cfg(test)]
mod tests {

    use rand::{thread_rng, Rng};

    use crate::{
        parser::{parse_expression, parse_mixed_expression},
        referencing_environment::ReferencingEnvironment,
        strings::StringArena,
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
        assert!(Expression::is_locally_nameless(
            &parsed_expressions,
            expected_expression
        ));

        let nameless_expression = Expression::to_locally_nameless(
            (environment.clone(), &parsed_expressions, expression),
            &mut converted_expressions,
        );
        assert!(Expression::is_locally_nameless(
            &converted_expressions,
            nameless_expression
        ));

        assert!(
            Expression::free_variables(environment.clone(), &parsed_expressions, expression).eq(
                &Expression::free_variables(
                    environment.clone(),
                    &converted_expressions,
                    nameless_expression
                )
            )
        );

        assert!(Expression::equals(
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

    fn fuzz_test<R: Rng>(rng: &mut R, max_depth: usize) {
        let mut strings = StringArena::new();
        let mut expressions = ExpressionArena::new();
        let environment = Rc::new(ReferencingEnvironment::new());

        let expression =
            Expression::sample(&mut strings, &mut expressions, environment, rng, max_depth);
        eprintln!(
            "{}",
            Expression::to_string(&strings, &expressions, 80, expression).unwrap()
        );
        let environment = Rc::new(ReferencingEnvironment::new());

        let mut converted_expressions = ExpressionArena::new();

        let nameless_expression = Expression::to_locally_nameless(
            (environment.clone(), &expressions, expression),
            &mut converted_expressions,
        );
        assert!(Expression::is_locally_nameless(
            &converted_expressions,
            nameless_expression
        ));

        assert!(
            Expression::free_variables(environment.clone(), &expressions, expression).eq(
                &Expression::free_variables(
                    environment.clone(),
                    &converted_expressions,
                    nameless_expression
                )
            )
        );
    }

    #[test]
    fn fuzz_tests() {
        let mut rng = thread_rng();
        let max_depth = 7;
        let test_count = 50;
        for _ in 0..test_count {
            fuzz_test(&mut rng, max_depth);
        }
    }
}
