use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    iter::Rev,
    slice::Iter,
};

use crate::{
    admissible_variable_name_generator::AdmissibleVariableNameGenerator,
    expression::{DeBruijnIndex, Expression, ExpressionArena, ExpressionId},
    strings::{StringArena, StringId},
};

impl Expression {
    pub fn convert_to_named<G: AdmissibleVariableNameGenerator>(
        strings: &mut StringArena,
        expressions: &ExpressionArena,
        expression: ExpressionId,
        destination: &mut ExpressionArena,
        variable_name_generator: G,
    ) -> ExpressionId {
        let mut identifiers = IdentifierArena::new();
        let binders = BinderStoreBuilder::new(expressions, &mut identifiers).build(expression);
        NameGeneration::new(
            strings,
            expressions,
            destination,
            identifiers,
            binders,
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
    #[inline]
    fn new() -> IdentifierArena {
        IdentifierArena {
            identifiers: Vec::new(),
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.identifiers.len()
    }

    #[inline]
    fn has(&self, id: IdentifierId) -> bool {
        id.into_usize() < self.len()
    }

    fn new_identifier(&mut self) -> IdentifierId {
        let index = self.len();
        self.identifiers.push(Option::None);
        IdentifierId::new(index)
    }

    #[inline]
    fn lookup(&self, id: IdentifierId) -> Option<StringId> {
        debug_assert!(self.has(id));
        self.identifiers[id.into_usize()]
    }

    #[inline]
    fn set(&mut self, id: IdentifierId, name: StringId) {
        debug_assert!(self.has(id));
        let previous = self.identifiers[id.into_usize()].replace(name);
        debug_assert!(previous.is_none());
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Constraint {
    String(StringId),
    Identifier(IdentifierId),
}

impl Constraint {
    #[inline]
    fn new_string_constraint(string: StringId) -> Constraint {
        Constraint::String(string)
    }

    #[inline]
    fn new_identifier_constraint(identifier: IdentifierId) -> Constraint {
        Constraint::Identifier(identifier)
    }

    #[inline]
    fn evaluate(&self, identifiers: &IdentifierArena) -> Option<StringId> {
        match self {
            Constraint::Identifier(identifier) => identifiers.lookup(*identifier),
            Constraint::String(string) => Option::Some(*string),
        }
    }
}

struct Binder {
    source_parameter: Option<StringId>,
    destination_parameter: IdentifierId,
    restrictions: HashSet<Constraint>,
    undesirables: HashSet<Constraint>,
    used: bool,
}

impl Binder {
    #[inline]
    fn new(source_parameter: Option<StringId>, destination_parameter: IdentifierId) -> Binder {
        Binder {
            source_parameter,
            destination_parameter,
            restrictions: HashSet::new(),
            undesirables: HashSet::new(),
            used: false,
        }
    }

    #[inline]
    fn is_used(&self) -> bool {
        self.used
    }

    #[inline]
    fn mark_used(&mut self) {
        self.used = true;
    }

    #[inline]
    fn add_restriction(&mut self, restriction: Constraint) {
        self.restrictions.insert(restriction);
    }

    #[inline]
    fn add_string_restriction(&mut self, restriction: StringId) {
        self.add_restriction(Constraint::new_string_constraint(restriction));
    }

    #[inline]
    fn add_identifier_restriction(&mut self, restriction: IdentifierId) {
        self.add_restriction(Constraint::new_identifier_constraint(restriction));
    }

    #[inline]
    fn add_undesirable(&mut self, undesirable: Constraint) {
        self.undesirables.insert(undesirable);
    }

    #[inline]
    fn add_string_undesirable(&mut self, undesirable: StringId) {
        self.add_undesirable(Constraint::new_string_constraint(undesirable));
    }

    #[inline]
    #[allow(dead_code)]
    fn add_identifier_undesirable(&mut self, undesirable: IdentifierId) {
        self.add_undesirable(Constraint::new_identifier_constraint(undesirable));
    }
}

struct BinderStore {
    binders: HashMap<ExpressionId, Binder>,
}

impl BinderStore {
    #[inline]
    fn new() -> BinderStore {
        BinderStore {
            binders: HashMap::new(),
        }
    }

    #[inline]
    fn set(&mut self, expression: ExpressionId, binder: Binder) {
        self.binders.insert(expression, binder);
    }

    #[inline]
    fn get(&self, expression: ExpressionId) -> Option<&Binder> {
        self.binders.get(&expression)
    }

    #[inline]
    fn get_mut(&mut self, expression: ExpressionId) -> Option<&mut Binder> {
        self.binders.get_mut(&expression)
    }
}

impl Default for BinderStore {
    #[inline]
    fn default() -> BinderStore {
        BinderStore::new()
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

struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    #[inline]
    fn new(
        expressions: &'a ExpressionArena,
        identifiers: &'a mut IdentifierArena,
    ) -> BinderStoreBuilder<'a> {
        BinderStoreBuilder {
            expressions,
            identifiers,
            binders: BinderStore::default(),
            environment: ReferencingEnvironment::default(),
        }
    }

    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                if let Option::Some(identifier) = self.environment.lookup(*variable) {
                    // `expression` is a bound variable
                    // Constraints to add to the binder for `expression` to avoid unnecessary renamings
                    let mut undesirables = Vec::new();
                    for binder_expression in self.environment.binders_iter() {
                        let binder = self.binders.get_mut(*binder_expression).unwrap();
                        if binder.destination_parameter == identifier {
                            // Found the binder for bound variable `expression`
                            for avoid in undesirables {
                                binder.add_string_undesirable(avoid);
                            }
                            binder.mark_used();
                            break;
                        }
                        if binder.source_parameter.is_some() {
                            // If the binder for `expression` needs renaming, avoid this binder's given parameter name
                            undesirables.push(binder.source_parameter.unwrap());
                        }
                        binder.add_identifier_restriction(identifier);
                    }
                } else {
                    // `expression` is a free variable
                    for binder in self.environment.binders_iter() {
                        let binder = self.binders.get_mut(*binder).unwrap();
                        binder.add_string_restriction(*variable);
                    }
                };
            }
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let (binder_source_parameter_name, binder_destination_parameter_identifier) = {
                    let binder = self.binders.get(binder_expression).unwrap();
                    (binder.source_parameter, binder.destination_parameter)
                };

                // Constraints to add to the binder for `expression` to avoid unnecessary renamings
                let mut undesirables = Vec::new();
                for sub_binder_expression in
                    self.environment.binders_iter().take(index.into_usize() - 1)
                {
                    let sub_binder = self.binders.get_mut(*sub_binder_expression).unwrap();

                    // `sub_binder` can't use the same parameter as `binder`
                    sub_binder.add_identifier_restriction(binder_destination_parameter_identifier);

                    if sub_binder.source_parameter.is_some()
                        && binder_source_parameter_name != sub_binder.source_parameter
                    {
                        // `binder` should avoid using the same parameter as `sub_binder`
                        undesirables.push(sub_binder.source_parameter.unwrap());
                    }
                }

                let binder = self.binders.get_mut(binder_expression).unwrap();
                binder.mark_used();
                for undesirable in undesirables {
                    binder.add_string_undesirable(undesirable);
                }
            }
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let binder = Binder::new(*parameter, parameter_identifier);
                self.binders.set(expression, binder);
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
                let binder = Binder::new(Option::None, parameter_identifier);
                self.binders.set(expression, binder);
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

    #[inline]
    fn build(mut self, expression: ExpressionId) -> BinderStore {
        self.visit(expression);
        self.binders
    }
}

struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> {
    strings: &'a mut StringArena,
    source: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    identifiers: IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
    variable_name_generator: G,
}

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    #[inline]
    fn new(
        strings: &'a mut StringArena,
        provider: &'a ExpressionArena,
        destination: &'a mut ExpressionArena,
        identifiers: IdentifierArena,
        binders: BinderStore,
        variable_name_generator: G,
    ) -> NameGeneration<'a, G> {
        NameGeneration {
            strings,
            source: provider,
            destination,
            identifiers,
            binders,
            environment: ReferencingEnvironment::default(),
            variable_name_generator,
        }
    }

    fn evaluate_constraint_set(&self, constraints: &HashSet<Constraint>) -> HashSet<StringId> {
        let mut identifiers = HashSet::new();
        for constraint in constraints {
            if let Option::Some(string) = constraint.evaluate(&self.identifiers) {
                identifiers.insert(string);
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
                let binder_expression = self.environment.lookup_binder(*index);
                let binder = self.binders.get(binder_expression).unwrap();
                let name = self
                    .identifiers
                    .lookup(binder.destination_parameter)
                    .unwrap();
                self.destination.variable(name)
            }
            Expression::Abstraction {
                parameter: source_parameter,
                body,
            } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter = if let Option::Some(name) = source_parameter {
                    // A parameter name already exists for `expression`
                    let restrictions = self.evaluate_constraint_set(&binder.restrictions);
                    if restrictions.contains(name) {
                        // `initial_parameter` has to be renamed
                        let undesirables = self.evaluate_constraint_set(&binder.undesirables);
                        let new_name = self
                            .variable_name_generator
                            .generate_admissible_name(self.strings, |name| {
                                !restrictions.contains(&name) && !undesirables.contains(&name)
                            });
                        self.identifiers.set(binder.destination_parameter, new_name);
                        Option::Some(new_name)
                    } else {
                        // `initial_parameter` can be used as is
                        self.identifiers.set(binder.destination_parameter, *name);
                        Option::Some(*name)
                    }
                } else if binder.is_used() {
                    // The parameter for `expression` is used in `body`
                    let restrictions = self.evaluate_constraint_set(&binder.restrictions);
                    let undesirables = self.evaluate_constraint_set(&binder.undesirables);
                    let name = self
                        .variable_name_generator
                        .generate_admissible_name(self.strings, |name| {
                            !restrictions.contains(&name) && !undesirables.contains(&name)
                        });
                    self.identifiers.set(binder.destination_parameter, name);
                    Option::Some(name)
                } else {
                    // The parameter for `expression` is never used in `body`
                    Option::None
                };
                match source_parameter {
                    Option::Some(name) => {
                        self.environment
                            .bind(*name, binder.destination_parameter, expression);
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
                let binder = self.binders.get(expression).unwrap();
                let parameter = if binder.is_used() {
                    // The parameter for `expression` is used in `body`
                    let restrictions = self.evaluate_constraint_set(&binder.restrictions);
                    let undesirables = self.evaluate_constraint_set(&binder.undesirables);
                    let name = self
                        .variable_name_generator
                        .generate_admissible_name(self.strings, |name| {
                            !restrictions.contains(&name) && !undesirables.contains(&name)
                        });
                    self.identifiers.set(binder.destination_parameter, name);
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

    #[inline]
    fn convert(mut self, expression: ExpressionId) -> ExpressionId {
        self.convert_to_named(expression)
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use rand::{thread_rng, Rng};

    use crate::{
        admissible_variable_name_generator::VariableNameGenerator,
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
        test_alpha_equivalence_of_named_mixed_expression("λx. λ. x y z 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λ. x 1");
        test_alpha_equivalence_of_named_mixed_expression("λ_. 1");
        test_alpha_equivalence_of_named_mixed_expression("λ_. λ_. 1");
        test_alpha_equivalence_of_named_mixed_expression("λ_. λ_. 2");
        test_alpha_equivalence_of_named_mixed_expression("λ_. λ_. λ_. 2 1");
        test_alpha_equivalence_of_named_mixed_expression("λ_. λ_. λ_. 3 2");
        test_alpha_equivalence_of_named_mixed_expression("λ_. λ_. λ_. 3 2 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λ_. λ_. 3 2 1");
        test_alpha_equivalence_of_named_mixed_expression("λ_. λx. λ_. 3 2 1");
        test_alpha_equivalence_of_named_mixed_expression("λ_. λ_. λx. 3 2 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λ_. λy. 3 2 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λy. x 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λx. x 1");
        test_alpha_equivalence_of_named_mixed_expression("λx. λy. λx. x 3");
        test_alpha_equivalence_of_named_mixed_expression("λx. λy. λx. x 3");
        test_alpha_equivalence_of_named_mixed_expression("λx. λx. λy. x 3");
        test_alpha_equivalence_of_named_mixed_expression("λ. λx. λ. 3 x 1");
        test_alpha_equivalence_of_named_mixed_expression(
            "λx. λx. 1 2 (λx. 1 2 3 (λx. 1 2 3 4 (λx. 1 2 3 4 5)))",
        );
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
        roundtrip_test("λf. λx. λy. f (λx. f x y) x");
        roundtrip_test("λf. λx. λy. f x");
        roundtrip_test("λf. λx. λy. f x y");
        roundtrip_test("λf. λy. f x y");
        roundtrip_test("λf. λx. f x y");
        roundtrip_test("λx. λy. f x y");
        roundtrip_test("f x y");
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
