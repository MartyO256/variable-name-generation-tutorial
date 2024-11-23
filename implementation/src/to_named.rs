use std::{
    collections::{HashMap, HashSet},
    iter::Rev,
    slice::Iter,
};

use crate::{
    expression::{DeBruijnIndex, Expression, ExpressionArena, ExpressionId},
    fresh_variable_name_generators::FreshVariableNameGenerator,
    strings::{StringArena, StringId},
};

impl Expression {
    pub fn convert_to_named<G: FreshVariableNameGenerator>(
        strings: &mut StringArena,
        expressions: &ExpressionArena,
        expression: ExpressionId,
        destination: &mut ExpressionArena,
        variable_name_generator: G,
    ) -> ExpressionId {
        let mut variables = VariableNameArena::new();
        let constraints =
            ConstraintStoreBuilder::new(expressions, &mut variables).build(expression);
        NameGeneration::new(
            strings,
            expressions,
            destination,
            variables,
            constraints,
            variable_name_generator,
        )
        .convert(expression)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct VariableNameId {
    index: usize,
}

impl VariableNameId {
    #[inline]
    pub fn new(index: usize) -> VariableNameId {
        VariableNameId { index }
    }

    #[inline]
    pub fn into_usize(self) -> usize {
        self.index
    }
}

struct VariableNameArena {
    names: Vec<Option<StringId>>,
}

impl VariableNameArena {
    pub fn new() -> VariableNameArena {
        VariableNameArena { names: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn has(&self, id: VariableNameId) -> bool {
        id.into_usize() < self.len()
    }

    pub fn variable_name(&mut self) -> VariableNameId {
        let index = self.len();
        self.names.push(Option::None);
        VariableNameId::new(index)
    }

    pub fn lookup(&self, id: VariableNameId) -> Option<StringId> {
        debug_assert!(self.has(id));
        self.names[id.into_usize()]
    }

    pub fn set(&mut self, id: VariableNameId, name: StringId) {
        debug_assert!(self.has(id));
        let previous = self.names[id.into_usize()].replace(name);
        debug_assert!(previous.is_none());
    }
}

#[derive(Clone)]
struct Constraint {
    parameter: VariableNameId,
    restrictions: HashSet<VariableNameId>,
    used: bool,
}

impl Constraint {
    pub fn new(parameter: VariableNameId) -> Constraint {
        Constraint {
            parameter,
            restrictions: HashSet::new(),
            used: false,
        }
    }
}

struct ConstraintStore {
    constraints: HashMap<ExpressionId, Constraint>,
}

impl ConstraintStore {
    pub fn new() -> ConstraintStore {
        ConstraintStore {
            constraints: HashMap::new(),
        }
    }

    pub fn set(&mut self, expression: ExpressionId, constraint: Constraint) {
        self.constraints.insert(expression, constraint);
    }

    pub fn get(&self, expression: ExpressionId) -> Option<&Constraint> {
        self.constraints.get(&expression)
    }

    pub fn get_mut(&mut self, expression: ExpressionId) -> Option<&mut Constraint> {
        self.constraints.get_mut(&expression)
    }
}

pub struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, Vec<VariableNameId>>,
    binders: Vec<ExpressionId>,
    size: usize,
}

impl ReferencingEnvironment {
    #[inline]
    pub fn new() -> ReferencingEnvironment {
        ReferencingEnvironment {
            bindings_map: HashMap::new(),
            binders: Vec::new(),
            size: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn bind(&mut self, identifier: StringId, variable: VariableNameId, binder: ExpressionId) {
        if let Option::Some(stack) = self.bindings_map.get_mut(&identifier) {
            stack.push(variable);
        } else {
            self.bindings_map.insert(identifier, vec![variable]);
        }
        self.binders.push(binder);
        self.size += 1;
    }

    pub fn unbind(&mut self, identifier: StringId) {
        debug_assert!(self.bindings_map.contains_key(&identifier));
        let stack = self.bindings_map.get_mut(&identifier).unwrap();
        debug_assert!(!stack.is_empty());
        stack.pop();
        if stack.is_empty() {
            self.bindings_map.remove(&identifier);
        }
        self.binders.pop();
        self.size -= 1;
    }

    #[inline]
    pub fn shift(&mut self, binder: ExpressionId) {
        self.binders.push(binder);
        self.size += 1;
    }

    #[inline]
    pub fn unshift(&mut self) {
        debug_assert!(self.size > 0);
        self.binders.pop();
        self.size -= 1;
    }

    fn lookup(&self, identifier: StringId) -> Option<VariableNameId> {
        self.bindings_map
            .get(&identifier)
            .and_then(|stack| stack.last().copied())
    }

    #[inline]
    pub fn lookup_binder(&self, index: DeBruijnIndex) -> ExpressionId {
        self.binders[self.binders.len() - index.into_usize()]
    }

    #[inline]
    pub fn binders_iter(&self) -> Rev<Iter<'_, ExpressionId>> {
        self.binders.iter().rev()
    }
}

impl Default for ReferencingEnvironment {
    fn default() -> ReferencingEnvironment {
        ReferencingEnvironment::new()
    }
}

struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    variables: &'a mut VariableNameArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    pub fn new(
        expressions: &'a ExpressionArena,
        variables: &'a mut VariableNameArena,
    ) -> ConstraintStoreBuilder<'a> {
        ConstraintStoreBuilder {
            expressions,
            variables,
            constraints: ConstraintStore::new(),
            environment: ReferencingEnvironment::new(),
        }
    }

    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable { identifier } => {
                let variable = if let Option::Some(variable) = self.environment.lookup(*identifier)
                {
                    variable
                } else {
                    let variable = self.variables.variable_name();
                    self.variables.set(variable, *identifier);
                    variable
                };
                for binder in self.environment.binders_iter() {
                    let Constraint {
                        parameter,
                        restrictions,
                        used,
                    } = self.constraints.get_mut(*binder).unwrap();
                    if let Option::Some(parameter) = self.variables.lookup(*parameter) {
                        if parameter == *identifier {
                            *used = true;
                            break;
                        }
                    }
                    restrictions.insert(variable);
                }
            }
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let variable = {
                    let Constraint {
                        parameter,
                        restrictions: _,
                        used: _,
                    } = self.constraints.get(binder).unwrap();
                    *parameter
                };
                let mut additional_restrictions = Vec::new();
                for binder in self.environment.binders_iter().take(index.into_usize() - 1) {
                    let Constraint {
                        parameter,
                        restrictions,
                        used: _,
                    } = self.constraints.get_mut(*binder).unwrap();
                    restrictions.insert(variable);
                    additional_restrictions.push(*parameter);
                }
                let Constraint {
                    parameter: _,
                    restrictions,
                    used,
                } = self.constraints.get_mut(binder).unwrap();
                *used = true;
                for additional_restriction in additional_restrictions {
                    restrictions.insert(additional_restriction);
                }
            }
            Expression::Abstraction { parameter, body } => {
                let variable = self.variables.variable_name();
                if let Option::Some(name) = parameter {
                    self.variables.set(variable, *name);
                }
                let constraint = Constraint::new(variable);
                self.constraints.set(expression, constraint);
                match parameter {
                    Option::Some(parameter) => {
                        self.environment.bind(*parameter, variable, expression);
                        self.visit(*body);
                        self.environment.unbind(*parameter);
                    }
                    Option::None => {
                        self.environment.shift(expression);
                        self.visit(*body);
                        self.environment.unshift();
                    }
                }
            }
            Expression::NamelessAbstraction { body } => {
                let variable = self.variables.variable_name();
                let constraint = Constraint::new(variable);
                self.constraints.set(expression, constraint);
                self.environment.shift(expression);
                self.visit(*body);
                self.environment.unshift();
            }
            Expression::Application {
                function,
                arguments,
            } => {
                self.visit(*function);
                for argument in arguments {
                    self.visit(*argument);
                }
            }
        }
    }

    pub fn build(mut self, expression: ExpressionId) -> ConstraintStore {
        self.visit(expression);
        self.constraints
    }
}

struct NameGeneration<'a, G: FreshVariableNameGenerator> {
    strings: &'a mut StringArena,
    provider: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    variables: VariableNameArena,
    constraints: ConstraintStore,
    binders: Vec<ExpressionId>,
    variable_name_generator: G,
}

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    pub fn new(
        strings: &'a mut StringArena,
        provider: &'a ExpressionArena,
        destination: &'a mut ExpressionArena,
        variables: VariableNameArena,
        constraints: ConstraintStore,
        variable_name_generator: G,
    ) -> NameGeneration<'a, G> {
        NameGeneration {
            strings,
            provider,
            destination,
            variables,
            constraints,
            binders: Vec::new(),
            variable_name_generator,
        }
    }

    fn lookup_restriction_set(&self, restrictions: &HashSet<VariableNameId>) -> HashSet<StringId> {
        let mut identifiers = HashSet::new();
        for restriction in restrictions {
            if let Option::Some(identifier) = self.variables.lookup(*restriction) {
                identifiers.insert(identifier);
            }
        }
        identifiers
    }

    fn assign(&mut self, id: VariableNameId, name: StringId) {
        self.variables.set(id, name);
    }

    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.provider[expression] {
            Expression::Variable { identifier } => self.destination.variable(*identifier),
            Expression::NamelessVariable { index } => {
                let binder = self.binders[self.binders.len() - index.into_usize()];
                let Constraint {
                    parameter,
                    restrictions: _,
                    used: _,
                } = self.constraints.get(binder).unwrap();
                let identifier = self.variables.lookup(*parameter).unwrap();
                self.destination.variable(identifier)
            }
            Expression::Abstraction { parameter: _, body } => {
                let Constraint {
                    parameter,
                    restrictions,
                    used,
                } = self.constraints.get(expression).unwrap();
                let parameter =
                    if let parameter @ Option::Some(_) = self.variables.lookup(*parameter) {
                        parameter
                    } else if *used {
                        let claimed_identifiers = self.lookup_restriction_set(restrictions);
                        let name = self
                            .variable_name_generator
                            .fresh_name(self.strings, &claimed_identifiers);
                        self.assign(*parameter, name);
                        Option::Some(name)
                    } else {
                        Option::None
                    };
                self.binders.push(expression);
                let named_body = self.convert_to_named(*body);
                self.binders.pop();
                self.destination.abstraction(parameter, named_body)
            }
            Expression::NamelessAbstraction { body } => {
                let Constraint {
                    parameter,
                    restrictions,
                    used,
                } = self.constraints.get(expression).unwrap();
                let parameter = if *used {
                    let claimed_identifiers = self.lookup_restriction_set(restrictions);
                    let name = self
                        .variable_name_generator
                        .fresh_name(self.strings, &claimed_identifiers);
                    self.assign(*parameter, name);
                    Option::Some(name)
                } else {
                    Option::None
                };
                self.binders.push(expression);
                let named_body = self.convert_to_named(*body);
                self.binders.pop();
                self.destination.abstraction(parameter, named_body)
            }
            Expression::Application {
                function,
                arguments,
            } => {
                let named_function = self.convert_to_named(*function);
                let mut named_arguments = Vec::with_capacity(arguments.len());
                for &argument in arguments {
                    let named_argument = self.convert_to_named(argument);
                    named_arguments.push(named_argument);
                }
                self.destination
                    .application(named_function, named_arguments)
            }
        }
    }

    pub fn convert(mut self, expression: ExpressionId) -> ExpressionId {
        self.convert_to_named(expression)
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use rand::{thread_rng, Rng};

    use crate::{
        fresh_variable_name_generators::SuffixVariableNameGenerator,
        referencing_environment::ReferencingEnvironment,
    };

    use super::*;

    fn test_alpha_equivalence_of_named_mixed_expression(input: &str) {
        let mut strings = StringArena::new();
        let mut source_expressions = ExpressionArena::new();
        let mut named_expressions = ExpressionArena::new();
        let variable_name_generator = SuffixVariableNameGenerator::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::new());

        let expression = Expression::parse_mixed_expression(
            &mut strings,
            &mut source_expressions,
            input.as_bytes(),
        )
        .unwrap();
        let named_expression = Expression::convert_to_named(
            &mut strings,
            &source_expressions,
            expression,
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
    fn test_alpha_equivalence_of_named_mixed_expressions() {
        test_alpha_equivalence_of_named_mixed_expression("λ. x");
        test_alpha_equivalence_of_named_mixed_expression("λy. x");
        test_alpha_equivalence_of_named_mixed_expression("λx. λy. x");
        test_alpha_equivalence_of_named_mixed_expression("λx. λy. y");
        test_alpha_equivalence_of_named_mixed_expression("λf. λx. λy. f x");
        test_alpha_equivalence_of_named_mixed_expression("λ. λx. λy. 3 x y");
        test_alpha_equivalence_of_named_mixed_expression("λf. λ. λy. f 2 y");
        test_alpha_equivalence_of_named_mixed_expression("λf. λx. λ. f x 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λ. f x");
        test_alpha_equivalence_of_named_mixed_expression("λx. λ. x x1 x2 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λ. x 1");
        test_alpha_equivalence_of_named_mixed_expression("λ_. 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λ_. λx1. 3 2 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λy. x 1");
        test_alpha_equivalence_of_named_mixed_expression("λ. λx. λ. 3 x 1");
    }

    fn roundtrip_test(input: &str) {
        let mut strings = StringArena::new();
        let mut source_expressions = ExpressionArena::new();
        let mut nameless_expressions = ExpressionArena::new();
        let mut named_expressions = ExpressionArena::new();
        let variable_name_generator = SuffixVariableNameGenerator::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::new());

        let expression =
            Expression::parse_expression(&mut strings, &mut source_expressions, input.as_bytes())
                .unwrap();
        let nameless_expression = Expression::convert_to_locally_nameless(
            (
                referencing_environment.clone(),
                &source_expressions,
                expression,
            ),
            &mut nameless_expressions,
        );
        let named_expression = Expression::convert_to_named(
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
        let mut nameless_expressions = ExpressionArena::new();
        let mut named_expressions = ExpressionArena::new();
        let variable_name_generator = SuffixVariableNameGenerator::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::new());

        let nameless_expression = Expression::convert_to_locally_nameless(
            (referencing_environment.clone(), &expressions, expression),
            &mut nameless_expressions,
        );
        let named_expression = Expression::convert_to_named(
            &mut strings,
            &nameless_expressions,
            nameless_expression,
            &mut named_expressions,
            variable_name_generator,
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
        let test_count = 400;
        for _ in 0..test_count {
            fuzz_test(&mut rng, max_depth);
        }
    }
}
