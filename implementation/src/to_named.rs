use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
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
        let mut identifiers = IdentifierArena::new();
        let constraints =
            ConstraintStoreBuilder::new(expressions, &mut identifiers).build(expression);
        NameGeneration::new(
            strings,
            expressions,
            destination,
            identifiers,
            constraints,
            variable_name_generator,
        )
        .convert(expression)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct IdentifierId {
    index: usize,
}

impl IdentifierId {
    #[inline]
    fn new(index: usize) -> IdentifierId {
        IdentifierId { index }
    }

    #[inline]
    fn into_usize(self) -> usize {
        self.index
    }
}

struct IdentifierArena {
    identifiers: Vec<Option<StringId>>,
}

impl IdentifierArena {
    fn new() -> IdentifierArena {
        IdentifierArena {
            identifiers: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.identifiers.len()
    }

    fn has(&self, id: IdentifierId) -> bool {
        id.into_usize() < self.len()
    }

    fn new_identifier(&mut self) -> IdentifierId {
        let index = self.len();
        self.identifiers.push(Option::None);
        IdentifierId::new(index)
    }

    fn lookup(&self, id: IdentifierId) -> Option<StringId> {
        debug_assert!(self.has(id));
        self.identifiers[id.into_usize()]
    }

    fn set(&mut self, id: IdentifierId, name: StringId) {
        debug_assert!(self.has(id));
        let previous = self.identifiers[id.into_usize()].replace(name);
        debug_assert!(previous.is_none());
    }
}

struct Constraint {
    parameter: IdentifierId,
    restrictions: HashSet<IdentifierId>,
    used: bool,
}

impl Constraint {
    fn new(parameter: IdentifierId) -> Constraint {
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
    fn new() -> ConstraintStore {
        ConstraintStore {
            constraints: HashMap::new(),
        }
    }

    fn set(&mut self, expression: ExpressionId, constraint: Constraint) {
        self.constraints.insert(expression, constraint);
    }

    fn get(&self, expression: ExpressionId) -> Option<&Constraint> {
        self.constraints.get(&expression)
    }

    fn get_mut(&mut self, expression: ExpressionId) -> Option<&mut Constraint> {
        self.constraints.get_mut(&expression)
    }
}

impl Default for ConstraintStore {
    fn default() -> ConstraintStore {
        ConstraintStore::new()
    }
}

struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, Vec<IdentifierId>>,
    binders_stack: Vec<ExpressionId>,
}

impl ReferencingEnvironment {
    #[inline]
    fn new() -> ReferencingEnvironment {
        ReferencingEnvironment {
            bindings_map: HashMap::new(),
            binders_stack: Vec::new(),
        }
    }

    fn bind(&mut self, name: StringId, identifier: IdentifierId, binder: ExpressionId) {
        match self.bindings_map.entry(name) {
            Entry::Occupied(mut stack) => {
                stack.get_mut().push(identifier);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![identifier]);
            }
        };
        self.binders_stack.push(binder);
    }

    fn unbind(&mut self, identifier: StringId) {
        debug_assert!(self.bindings_map.contains_key(&identifier));
        let stack = self.bindings_map.get_mut(&identifier).unwrap();
        debug_assert!(!stack.is_empty());
        stack.pop();
        if stack.is_empty() {
            self.bindings_map.remove(&identifier);
        }
        self.binders_stack.pop();
    }

    #[inline]
    fn shift(&mut self, binder: ExpressionId) {
        self.binders_stack.push(binder);
    }

    #[inline]
    fn unshift(&mut self) {
        debug_assert!(!self.binders_stack.is_empty());
        self.binders_stack.pop();
    }

    fn lookup(&self, identifier: StringId) -> Option<IdentifierId> {
        self.bindings_map
            .get(&identifier)
            .and_then(|stack| stack.last().copied())
    }

    #[inline]
    fn lookup_binder(&self, index: DeBruijnIndex) -> ExpressionId {
        self.binders_stack[self.binders_stack.len() - index.into_usize()]
    }

    #[inline]
    fn binders_iter(&self) -> Rev<Iter<'_, ExpressionId>> {
        self.binders_stack.iter().rev()
    }
}

impl Default for ReferencingEnvironment {
    fn default() -> ReferencingEnvironment {
        ReferencingEnvironment::new()
    }
}

struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn new(
        expressions: &'a ExpressionArena,
        identifiers: &'a mut IdentifierArena,
    ) -> ConstraintStoreBuilder<'a> {
        ConstraintStoreBuilder {
            expressions,
            identifiers,
            constraints: ConstraintStore::default(),
            environment: ReferencingEnvironment::default(),
        }
    }

    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                let identifier =
                    if let Option::Some(identifier) = self.environment.lookup(*variable) {
                        // `expression` is a bound variable
                        identifier
                    } else {
                        // `expression` is a free variable
                        let identifier = self.identifiers.new_identifier();
                        self.identifiers.set(identifier, *variable);
                        identifier
                    };
                for binder in self.environment.binders_iter() {
                    let Constraint {
                        parameter,
                        restrictions,
                        used,
                    } = self.constraints.get_mut(*binder).unwrap();
                    if *parameter == identifier {
                        // Found the binder for bound variable `expression`
                        *used = true;
                        break;
                    }
                    restrictions.insert(identifier);
                }
            }
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let identifier = {
                    let Constraint {
                        parameter,
                        restrictions: _,
                        used: _,
                    } = self.constraints.get(binder).unwrap();
                    *parameter
                };

                // Constraints to add to the binder for `expression`
                let mut additional_restrictions = Vec::new();
                for sub_binder in self.environment.binders_iter().take(index.into_usize() - 1) {
                    let Constraint {
                        parameter,
                        restrictions,
                        used: _,
                    } = self.constraints.get_mut(*sub_binder).unwrap();

                    // `sub_binder` can't use the same parameter as `binder`
                    restrictions.insert(identifier);

                    // `binder` can't use the same parameter as `sub_binder`
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
                let parameter_identifier = self.identifiers.new_identifier();
                let constraint = Constraint::new(parameter_identifier);
                self.constraints.set(expression, constraint);
                match parameter {
                    Option::Some(parameter) => {
                        self.environment
                            .bind(*parameter, parameter_identifier, expression);
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
                let parameter_identifier = self.identifiers.new_identifier();
                let constraint = Constraint::new(parameter_identifier);
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

    fn build(mut self, expression: ExpressionId) -> ConstraintStore {
        self.visit(expression);
        self.constraints
    }
}

struct NameGeneration<'a, G: FreshVariableNameGenerator> {
    strings: &'a mut StringArena,
    source: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    identifiers: IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
    variable_name_generator: G,
}

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn new(
        strings: &'a mut StringArena,
        provider: &'a ExpressionArena,
        destination: &'a mut ExpressionArena,
        identifiers: IdentifierArena,
        constraints: ConstraintStore,
        variable_name_generator: G,
    ) -> NameGeneration<'a, G> {
        NameGeneration {
            strings,
            source: provider,
            destination,
            identifiers,
            constraints,
            environment: ReferencingEnvironment::default(),
            variable_name_generator,
        }
    }

    fn lookup_restriction_set(&self, restrictions: &HashSet<IdentifierId>) -> HashSet<StringId> {
        let mut identifiers = HashSet::new();
        for restriction in restrictions {
            if let Option::Some(identifier) = self.identifiers.lookup(*restriction) {
                identifiers.insert(identifier);
            }
        }
        identifiers
    }

    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Variable { identifier: name } => match self.environment.lookup(*name) {
                Option::Some(identifier) => {
                    // `expression` is a bound variable that may have been renamed
                    let assigned_name = self.identifiers.lookup(identifier).unwrap();
                    self.destination.variable(assigned_name)
                }
                Option::None => {
                    // `expression` is a free variable
                    self.destination.variable(*name)
                }
            },
            Expression::NamelessVariable { index } => {
                // `expression` is a bound nameless variable
                let binder = self.environment.lookup_binder(*index);
                let Constraint {
                    parameter,
                    restrictions: _,
                    used: _,
                } = self.constraints.get(binder).unwrap();
                let identifier = self.identifiers.lookup(*parameter).unwrap();
                self.destination.variable(identifier)
            }
            Expression::Abstraction {
                parameter: initial_parameter,
                body,
            } => {
                let Constraint {
                    parameter,
                    restrictions,
                    used,
                } = self.constraints.get(expression).unwrap();
                let chosen_parameter = if let Option::Some(name) = initial_parameter {
                    // A parameter name already exists for `expression`
                    let claimed_identifiers = self.lookup_restriction_set(restrictions);
                    if claimed_identifiers.contains(name) {
                        // `initial_parameter` has to be renamed
                        let new_name = self
                            .variable_name_generator
                            .fresh_name(self.strings, &claimed_identifiers);
                        self.identifiers.set(*parameter, new_name);
                        Option::Some(new_name)
                    } else {
                        // `initial_parameter` can be used as is
                        self.identifiers.set(*parameter, *name);
                        Option::Some(*name)
                    }
                } else if *used {
                    // The parameter for `expression` is used in `body`
                    let claimed_identifiers = self.lookup_restriction_set(restrictions);
                    let name = self
                        .variable_name_generator
                        .fresh_name(self.strings, &claimed_identifiers);
                    self.identifiers.set(*parameter, name);
                    Option::Some(name)
                } else {
                    // The parameter for `expression` is never used in `body`
                    Option::None
                };
                match initial_parameter {
                    Option::Some(name) => {
                        self.environment.bind(*name, *parameter, expression);
                        let named_body = self.convert_to_named(*body);
                        self.environment.unbind(*name);
                        self.destination.abstraction(chosen_parameter, named_body)
                    }
                    Option::None => {
                        self.environment.shift(expression);
                        let named_body = self.convert_to_named(*body);
                        self.environment.unshift();
                        self.destination.abstraction(chosen_parameter, named_body)
                    }
                }
            }
            Expression::NamelessAbstraction { body } => {
                let Constraint {
                    parameter,
                    restrictions,
                    used,
                } = self.constraints.get(expression).unwrap();
                let parameter = if *used {
                    // The parameter for `expression` is used in `body`
                    let claimed_identifiers = self.lookup_restriction_set(restrictions);
                    let name = self
                        .variable_name_generator
                        .fresh_name(self.strings, &claimed_identifiers);
                    self.identifiers.set(*parameter, name);
                    Option::Some(name)
                } else {
                    // The parameter for `expression` is never used in `body`
                    Option::None
                };
                self.environment.shift(expression);
                let named_body = self.convert_to_named(*body);
                self.environment.unshift();
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

    fn convert(mut self, expression: ExpressionId) -> ExpressionId {
        self.convert_to_named(expression)
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use rand::{thread_rng, Rng};

    use crate::{
        fresh_variable_name_generators::VariableNameGenerator,
        referencing_environment::ReferencingEnvironment,
    };

    use super::*;

    fn test_alpha_equivalence_of_named_mixed_expression(input: &str) {
        let mut strings = StringArena::new();
        let mut source_expressions = ExpressionArena::new();
        let mut named_expressions = ExpressionArena::new();
        let variable_name_generator = VariableNameGenerator::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::default());

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
        test_alpha_equivalence_of_named_mixed_expression("λ_. λ_. 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λ_. λx1. 3 2 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λy. x 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λx. x 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λy. λx. x 3");
        test_alpha_equivalence_of_named_mixed_expression("λx. (λy. λx. x 3) x");
        test_alpha_equivalence_of_named_mixed_expression("λ. λx. λ. 3 x 1");
    }

    fn roundtrip_test(input: &str) {
        let mut strings = StringArena::new();
        let mut source_expressions = ExpressionArena::new();
        let mut nameless_expressions = ExpressionArena::new();
        let mut named_expressions = ExpressionArena::new();
        let variable_name_generator = VariableNameGenerator::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::default());

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
        let environment = Rc::new(ReferencingEnvironment::default());

        let expression =
            Expression::sample(&mut strings, &mut expressions, environment, rng, max_depth);
        let mut nameless_expressions = ExpressionArena::new();
        let mut named_expressions = ExpressionArena::new();
        let variable_name_generator = VariableNameGenerator::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::default());

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
