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

pub fn to_nameless(
    (environment, arena, expression): (Rc<ReferencingEnvironment>, &ExpressionArena, ExpressionId),
    destination: &mut ExpressionArena,
) -> ExpressionId {
    let mut framed_environment = ReferencingEnvironment::new_frame(environment);
    Indexing::new(&mut framed_environment, arena, destination).convert(expression)
}
