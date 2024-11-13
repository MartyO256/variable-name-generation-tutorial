use std::{collections::HashSet, rc::Rc};

use crate::{
    expression::{Expression, ExpressionArena, ExpressionId},
    referencing_environment::ReferencingEnvironment,
    strings::{StringArena, StringId},
};

pub fn is_named(expressions: &ExpressionArena, expression: ExpressionId) -> bool {
    match &expressions[expression] {
        Expression::Variable { identifier: _ } => true,
        Expression::NamelessVariable { index: _ } => false,
        Expression::Abstraction { parameter: _, body } => is_named(expressions, *body),
        Expression::NamelessAbstraction { body: _ } => false,
        Expression::Application {
            function,
            arguments,
        } => {
            if !is_named(expressions, *function) {
                false
            } else {
                for &argument in arguments.iter() {
                    if !is_named(expressions, argument) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

pub fn is_locally_nameless(expressions: &ExpressionArena, expression: ExpressionId) -> bool {
    match &expressions[expression] {
        Expression::Variable { identifier: _ } => true,
        Expression::NamelessVariable { index: _ } => true,
        Expression::Abstraction {
            parameter: _,
            body: _,
        } => false,
        Expression::NamelessAbstraction { body } => is_locally_nameless(expressions, *body),
        Expression::Application {
            function,
            arguments,
        } => {
            if !is_locally_nameless(expressions, *function) {
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
            Expression::Abstraction { parameter, body } => {
                self.environment.bind_option(*parameter);
                self.visit(*body);
                self.environment.unbind_option(*parameter);
            }
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

pub trait FreshVariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId;
}

pub struct SuffixVariableNameGenerator {}

impl SuffixVariableNameGenerator {
    pub fn new() -> SuffixVariableNameGenerator {
        SuffixVariableNameGenerator {}
    }
}

impl Default for SuffixVariableNameGenerator {
    fn default() -> SuffixVariableNameGenerator {
        SuffixVariableNameGenerator::new()
    }
}

impl FreshVariableNameGenerator for SuffixVariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId {
        let mut suffix = 0;
        let mut n = strings.intern("x".as_bytes());
        while claimed.contains(&n) {
            suffix += 1;
            let mut t = "x".as_bytes().to_vec();
            t.extend(suffix.to_string().as_bytes());
            n = strings.intern(&t);
        }
        n
    }
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
    fn free_variables_computes_the_free_variables_in_the_expression() {
        check_free_variables("x", vec!["x"]);
        check_free_variables("λf. x", vec!["x"]);
        check_free_variables("λf. λx. f x", vec![]);
        check_free_variables("λx. λy. λz. x z (y z)", vec![]);
        check_free_variables("λf. x1 x2 x3", vec!["x1", "x2", "x3"]);
    }
}
