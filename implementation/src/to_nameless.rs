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
    fn to_nameless(&mut self, expression: ExpressionId) -> Option<ExpressionId> {
        match &self.arena[expression] {
            &Expression::Variable { identifier } => {
                let index = self.environment.lookup_index(identifier)?;
                Option::Some(self.destination.nameless_variable(index.into()))
            }
            &Expression::NamelessVariable { ref index } => {
                Option::Some(self.destination.nameless_variable(*index))
            }
            &Expression::Abstraction { parameter, body } => {
                self.environment.bind_option(parameter);
                let indexed_body = self.to_nameless(body);
                self.environment.bind_option(parameter);
                Option::Some(self.destination.nameless_abstraction(indexed_body?))
            }
            &Expression::NamelessAbstraction { ref body } => {
                self.environment.shift();
                let indexed_body = self.to_nameless(*body);
                self.environment.unshift();
                Option::Some(self.destination.nameless_abstraction(indexed_body?))
            }
            &Expression::Application {
                function,
                ref arguments,
            } => {
                let indexed_function = self.to_nameless(function)?;
                let mut indexed_arguments = Vec::with_capacity(arguments.len());
                for &argument in arguments.iter() {
                    let indexed_argument = self.to_nameless(argument)?;
                    indexed_arguments.push(indexed_argument)
                }
                Option::Some(
                    self.destination
                        .application(indexed_function, indexed_arguments),
                )
            }
        }
    }
}

pub fn to_nameless(
    (environment, arena, expression): (Rc<ReferencingEnvironment>, &ExpressionArena, ExpressionId),
    destination: &mut ExpressionArena,
) -> Option<ExpressionId> {
    let mut framed_environment = ReferencingEnvironment::new_frame(environment);
    Indexing {
        environment: &mut framed_environment,
        arena,
        destination,
    }
    .to_nameless(expression)
}
