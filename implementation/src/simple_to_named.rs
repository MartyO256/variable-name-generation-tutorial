use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    expression::{DeBruijnIndex, Expression, ExpressionArena, ExpressionId},
    fresh_variable_name_generators::FreshVariableNameGenerator,
    strings::{StringArena, StringId},
};

pub struct ReferencingEnvironment {
    parent: Option<Rc<ReferencingEnvironment>>,
    bindings_map: HashMap<StringId, Vec<()>>,
    bindings_list: Vec<Option<StringId>>,
    size: usize,
}

impl ReferencingEnvironment {
    #[inline]
    pub fn new() -> ReferencingEnvironment {
        ReferencingEnvironment {
            parent: Option::None,
            bindings_map: HashMap::new(),
            bindings_list: Vec::new(),
            size: 0,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> ReferencingEnvironment {
        ReferencingEnvironment {
            parent: Option::None,
            bindings_map: HashMap::with_capacity(capacity),
            bindings_list: Vec::with_capacity(capacity),
            size: 0,
        }
    }

    #[inline]
    pub fn new_frame(refs: Rc<ReferencingEnvironment>) -> ReferencingEnvironment {
        let size = refs.size;
        ReferencingEnvironment {
            parent: Option::Some(refs),
            bindings_map: HashMap::new(),
            bindings_list: Vec::new(),
            size,
        }
    }

    #[inline]
    pub fn new_frame_with_capacity(
        refs: Rc<ReferencingEnvironment>,
        capacity: usize,
    ) -> ReferencingEnvironment {
        let size = refs.size;
        ReferencingEnvironment {
            parent: Option::Some(refs),
            bindings_map: HashMap::with_capacity(capacity),
            bindings_list: Vec::with_capacity(capacity),
            size,
        }
    }

    pub fn bind(&mut self, identifier: StringId) {
        if let Option::Some(stack) = self.bindings_map.get_mut(&identifier) {
            stack.push(());
        } else {
            let stack = vec![()];
            self.bindings_map.insert(identifier, stack);
        }
        self.bindings_list.push(Option::Some(identifier));
        self.size += 1;
    }

    pub fn unbind(&mut self, identifier: StringId) {
        debug_assert!(self.bindings_map.contains_key(&identifier));
        let stack = self.bindings_map.get_mut(&identifier).unwrap();
        debug_assert!(!stack.is_empty());
        stack.pop();
        self.bindings_list.pop();
        self.size -= 1;
    }

    #[inline]
    pub fn shift(&mut self) {
        self.size += 1;
        self.bindings_list.push(Option::None);
    }

    #[inline]
    pub fn unshift(&mut self) {
        debug_assert!(self.size > 0);
        self.size -= 1;
        self.bindings_list.pop();
    }

    #[inline]
    pub fn bind_option(&mut self, identifier: Option<StringId>) {
        match identifier {
            Option::Some(identifier) => self.bind(identifier),
            Option::None => self.shift(),
        }
    }

    #[inline]
    pub fn unbind_option(&mut self, identifier: Option<StringId>) {
        match identifier {
            Option::Some(identifier) => self.unbind(identifier),
            Option::None => self.unshift(),
        }
    }

    pub fn lookup_name(&self, index: DeBruijnIndex) -> Option<StringId> {
        let i: usize = index.into_usize();
        let n = self.bindings_list.len();
        if i <= n {
            self.bindings_list[n - i]
        } else if let Option::Some(parent) = self.parent.to_owned() {
            parent.lookup_name((i - n).into())
        } else {
            unreachable!()
        }
    }
}

impl Default for ReferencingEnvironment {
    fn default() -> ReferencingEnvironment {
        ReferencingEnvironment::new()
    }
}

struct UsedVariables<'a> {
    environment: &'a mut ReferencingEnvironment,
    expressions: &'a ExpressionArena,
    used_variables: HashSet<StringId>,
    used_indices: HashSet<DeBruijnIndex>,
}

impl<'a> UsedVariables<'a> {
    pub fn new(
        environment: &'a mut ReferencingEnvironment,
        expressions: &'a ExpressionArena,
    ) -> UsedVariables<'a> {
        UsedVariables {
            environment,
            expressions,
            used_variables: HashSet::new(),
            used_indices: HashSet::new(),
        }
    }

    fn shift_indices(&mut self) {
        self.used_indices = self
            .used_indices
            .drain()
            .map(|index| {
                let i = index.into_usize();
                (i + 1).into()
            })
            .collect();
    }

    fn unshift_indices(&mut self) {
        self.used_indices = self
            .used_indices
            .drain()
            .filter_map(|index| {
                let i = index.into_usize();
                if i > 1 {
                    Option::Some((i - 1).into())
                } else {
                    Option::None
                }
            })
            .collect();
    }

    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable { identifier } => {
                self.used_variables.insert(*identifier);
            }
            Expression::NamelessVariable { index } => {
                if let Option::Some(name) = self.environment.lookup_name(*index) {
                    self.used_variables.insert(name);
                } else {
                    self.used_indices.insert(*index);
                }
            }
            Expression::Abstraction { parameter, body } => {
                self.shift_indices();
                self.environment.bind_option(*parameter);
                self.visit(*body);
                self.environment.unbind_option(*parameter);
                self.unshift_indices();
            }
            Expression::NamelessAbstraction { body } => {
                self.shift_indices();
                self.environment.shift();
                self.visit(*body);
                self.environment.unshift();
                self.unshift_indices();
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

    pub fn used_variables_and_indices(
        mut self,
        expression: ExpressionId,
    ) -> (HashSet<StringId>, HashSet<DeBruijnIndex>) {
        self.visit(expression);
        (self.used_variables, self.used_indices)
    }
}

struct NameGeneration<'a, G: FreshVariableNameGenerator> {
    strings: &'a mut StringArena,
    provider: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    environment: ReferencingEnvironment,
    variable_name_generator: G,
}

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    pub fn new(
        strings: &'a mut StringArena,
        provider: &'a ExpressionArena,
        destination: &'a mut ExpressionArena,
        variable_name_generator: G,
    ) -> NameGeneration<'a, G> {
        NameGeneration {
            strings,
            provider,
            destination,
            environment: ReferencingEnvironment::new(),
            variable_name_generator,
        }
    }

    pub fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.provider[expression] {
            Expression::Variable { identifier } => self.destination.variable(*identifier),
            Expression::NamelessVariable { index } => {
                let identifier = self.environment.lookup_name(*index).unwrap();
                self.destination.variable(identifier)
            }
            Expression::Abstraction { parameter, body } => {
                self.environment.bind_option(*parameter);
                let named_body = self.convert_to_named(*body);
                self.environment.unbind_option(*parameter);
                self.destination.abstraction(*parameter, named_body)
            }
            Expression::NamelessAbstraction { body } => {
                self.environment.shift();
                let (used_variables, used_indices) =
                    UsedVariables::new(&mut self.environment, self.provider)
                        .used_variables_and_indices(*body);
                self.environment.unshift();
                let parameter = if used_indices.contains(&1.into()) {
                    let identifiers = used_variables;
                    let identifier = self
                        .variable_name_generator
                        .fresh_name(self.strings, &identifiers);
                    Option::Some(identifier)
                } else {
                    Option::None
                };
                self.environment.bind_option(parameter);
                let named_body = self.convert_to_named(*body);
                self.environment.unbind_option(parameter);
                self.destination.abstraction(parameter, named_body)
            }
            Expression::Application {
                function,
                arguments,
            } => {
                let named_function = self.convert_to_named(*function);
                let named_arguments = arguments
                    .iter()
                    .map(|&argument| self.convert_to_named(argument))
                    .collect();
                self.destination
                    .application(named_function, named_arguments)
            }
        }
    }
}

pub fn to_named<G: FreshVariableNameGenerator>(
    strings: &mut StringArena,
    expressions: &ExpressionArena,
    expression: ExpressionId,
    destination: &mut ExpressionArena,
    variable_name_generator: G,
) -> ExpressionId {
    NameGeneration::new(strings, expressions, destination, variable_name_generator)
        .convert_to_named(expression)
}

#[cfg(test)]
mod tests {

    use rand::{thread_rng, Rng};

    use crate::{
        fresh_variable_name_generators::SuffixVariableNameGenerator, parser::parse_expression,
        referencing_environment::ReferencingEnvironment,
    };

    use super::*;

    fn roundtrip_test(input: &str) {
        let mut strings = StringArena::new();
        let mut source_expressions = ExpressionArena::new();
        let mut nameless_expressions = ExpressionArena::new();
        let mut named_expressions = ExpressionArena::new();
        let variable_name_generator = SuffixVariableNameGenerator::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::new());

        let expression =
            parse_expression(&mut strings, &mut source_expressions, input.as_bytes()).unwrap();
        let nameless_expression = Expression::to_locally_nameless(
            (
                referencing_environment.clone(),
                &source_expressions,
                expression,
            ),
            &mut nameless_expressions,
        );
        let named_expression = to_named(
            &mut strings,
            &nameless_expressions,
            nameless_expression,
            &mut named_expressions,
            variable_name_generator,
        );
        assert!(Expression::is_named(&named_expressions, named_expression));

        assert!(Expression::alpha_equivalent(
            (
                referencing_environment.clone(),
                &source_expressions,
                expression
            ),
            (
                referencing_environment.clone(),
                &named_expressions,
                named_expression
            )
        ));
    }

    #[test]
    fn roundtrip_tests() {
        roundtrip_test("λx. x");
        roundtrip_test("λy. x");
        roundtrip_test("λx. λy. x");
        roundtrip_test("λx. λy. y");
        roundtrip_test("λf. λx. λy. f x");
        roundtrip_test("λf. λx. λy. f x y");
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
        let mut nameless_expressions = ExpressionArena::new();
        let mut named_expressions = ExpressionArena::new();
        let variable_name_generator = SuffixVariableNameGenerator::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::new());

        let nameless_expression = Expression::to_locally_nameless(
            (referencing_environment.clone(), &expressions, expression),
            &mut nameless_expressions,
        );
        let named_expression = to_named(
            &mut strings,
            &nameless_expressions,
            nameless_expression,
            &mut named_expressions,
            variable_name_generator,
        );
        eprintln!(
            "{}",
            Expression::to_string(&strings, &named_expressions, 80, named_expression).unwrap()
        );
        assert!(Expression::is_named(&named_expressions, named_expression));

        assert!(Expression::alpha_equivalent(
            (referencing_environment.clone(), &expressions, expression),
            (
                referencing_environment.clone(),
                &named_expressions,
                named_expression
            )
        ));
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
