use std::{collections::HashSet, rc::Rc};

use crate::{
    expression::{Expression, ExpressionArena, ExpressionId},
    referencing_environment::ReferencingEnvironment,
    strings::{StringArena, StringId},
};

struct ExpressionSize<'a> {
    expressions: &'a ExpressionArena,
}

impl<'a> ExpressionSize<'a> {
    pub fn new(expressions: &'a ExpressionArena) -> ExpressionSize<'a> {
        ExpressionSize { expressions }
    }

    fn size(&self, expression: ExpressionId) -> usize {
        match &self.expressions[expression] {
            Expression::Variable { identifier: _ } => 1,
            Expression::NamelessVariable { index: _ } => 1,
            Expression::Abstraction { parameter: _, body } => 1 + self.size(*body),
            Expression::NamelessAbstraction { body } => 1 + self.size(*body),
            Expression::Application {
                function,
                arguments,
            } => {
                let mut s = 1;
                s += self.size(*function);
                for &argument in arguments {
                    s += self.size(argument);
                }
                s
            }
        }
    }

    pub fn compute_expression_size(self, expression: ExpressionId) -> usize {
        self.size(expression)
    }
}

pub fn size(expressions: &ExpressionArena, expression: ExpressionId) -> usize {
    ExpressionSize::new(expressions).compute_expression_size(expression)
}

struct ExpressionHeight<'a> {
    expressions: &'a ExpressionArena,
}

impl<'a> ExpressionHeight<'a> {
    pub fn new(expressions: &'a ExpressionArena) -> ExpressionHeight<'a> {
        ExpressionHeight { expressions }
    }

    fn height(&self, expression: ExpressionId) -> usize {
        match &self.expressions[expression] {
            Expression::Variable { identifier: _ } => 0,
            Expression::NamelessVariable { index: _ } => 0,
            Expression::Abstraction { parameter: _, body } => 1 + self.height(*body),
            Expression::NamelessAbstraction { body } => 1 + self.height(*body),
            Expression::Application {
                function,
                arguments,
            } => {
                let mut s = 0;
                s = std::cmp::max(s, self.height(*function));
                for &argument in arguments {
                    s = std::cmp::max(s, self.height(argument));
                }
                s
            }
        }
    }

    pub fn compute_expression_height(self, expression: ExpressionId) -> usize {
        self.height(expression)
    }
}

pub fn height(expressions: &ExpressionArena, expression: ExpressionId) -> usize {
    ExpressionHeight::new(expressions).compute_expression_height(expression)
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

pub fn parent_expressions(
    expressions: &ExpressionArena,
    expression: ExpressionId,
) -> Vec<Option<ExpressionId>> {
    Parent::new(expressions).compute_parent_expressions(expression)
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

pub fn is_named(expressions: &ExpressionArena, expression: ExpressionId) -> bool {
    IsNamed::new(expressions).check_is_named(expression)
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

pub fn is_locally_nameless(expressions: &ExpressionArena, expression: ExpressionId) -> bool {
    IsLocallyNameless::new(expressions).check_is_locally_nameless(expression)
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
        let mut suffix = 1;
        let mut id = strings.intern(b"x");
        while claimed.contains(&id) {
            let mut candidate = b"x".to_vec();
            candidate.extend(suffix.to_string().as_bytes());
            id = strings.intern(&candidate);
            suffix += 1;
        }
        id
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        parser::parse_mixed_expression, referencing_environment::ReferencingEnvironment,
        strings::StringArena,
    };

    use super::*;

    fn check_expression_size(input: &str, expected: usize) {
        let mut strings = StringArena::new();
        let mut expressions = ExpressionArena::new();

        let expression =
            parse_mixed_expression(&mut strings, &mut expressions, input.as_bytes()).unwrap();

        let size = size(&expressions, expression);

        assert_eq!(size, expected);
    }

    #[test]
    fn size_computes_expression_size() {
        check_expression_size("x", 1);
        check_expression_size("λf. x", 2);
        check_expression_size("λf. λx. f x", 5);
        check_expression_size("λx. λy. λz. x z (y z)", 9);
    }

    fn check_expression_height(input: &str, expected: usize) {
        let mut strings = StringArena::new();
        let mut expressions = ExpressionArena::new();

        let expression =
            parse_mixed_expression(&mut strings, &mut expressions, input.as_bytes()).unwrap();

        let height = height(&expressions, expression);

        assert_eq!(height, expected);
    }

    #[test]
    fn height_computes_expression_height() {
        check_expression_height("x", 0);
        check_expression_height("λf. x", 1);
        check_expression_height("λf. λx. f x", 2);
        check_expression_height("λx. λy. λz. x z (y z)", 3);
    }

    fn check_free_variables(input: &str, expected: Vec<&str>) {
        let mut strings = StringArena::new();
        let mut expected_free_variables = HashSet::with_capacity(expected.len());
        for e in expected {
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
