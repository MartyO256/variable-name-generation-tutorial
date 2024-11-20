use std::collections::{HashMap, HashSet};

use crate::{
    expression::{DeBruijnIndex, Expression, ExpressionArena, ExpressionId},
    fresh_variable_name_generators::FreshVariableNameGenerator,
    strings::{StringArena, StringId},
};

impl Expression {
    pub fn to_named<G: FreshVariableNameGenerator>(
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
        .convert_to_named(expression)
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

    pub fn new_name(&mut self) -> VariableNameId {
        let index = self.len();
        self.names.push(Option::None);
        VariableNameId::new(index)
    }

    pub fn lookup_name(&self, id: VariableNameId) -> Option<StringId> {
        debug_assert!(self.has(id));
        self.names[id.into_usize()].clone()
    }

    pub fn update_name(&mut self, id: VariableNameId, name: StringId) {
        debug_assert!(self.has(id));
        self.names[id.into_usize()] = Option::Some(name);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ParameterRestriction {
    BoundVariable { id: VariableNameId },
    FreeVariable { id: StringId },
}

#[derive(Clone)]
enum Constraint {
    Abstraction {
        parameter: Option<StringId>,
    },
    NamelessAbstraction {
        parameter: VariableNameId,
        restrictions: HashSet<ParameterRestriction>,
        used: bool,
    },
    Variable {
        name: VariableNameId,
    },
}

struct ConstraintStore {
    constraints: Vec<Option<Constraint>>,
}

impl ConstraintStore {
    pub fn new(size: usize) -> ConstraintStore {
        ConstraintStore {
            constraints: vec![Option::None; size],
        }
    }

    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    pub fn has(&self, id: ExpressionId) -> bool {
        id.into_usize() < self.len()
    }

    pub fn set(&mut self, expression: ExpressionId, constraint: Constraint) {
        self.constraints[expression.into_usize()] = Option::Some(constraint)
    }

    pub fn get(&self, expression: ExpressionId) -> Option<&Constraint> {
        debug_assert!(self.has(expression));
        self.constraints[expression.into_usize()].as_ref()
    }

    pub fn get_mut(&mut self, expression: ExpressionId) -> Option<&mut Constraint> {
        debug_assert!(self.has(expression));
        self.constraints[expression.into_usize()].as_mut()
    }
}

pub struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, usize>,
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

    pub fn is_bound(&self, identifier: StringId) -> bool {
        self.bindings_map.contains_key(&identifier)
    }

    #[inline]
    pub fn is_free(&self, identifier: StringId) -> bool {
        !self.bindings_map.contains_key(&identifier)
    }

    pub fn domain_len(&self) -> usize {
        self.bindings_map.len()
    }

    pub fn bind(&mut self, identifier: StringId, binder: ExpressionId) {
        if let Option::Some(count) = self.bindings_map.get_mut(&identifier) {
            *count += 1;
        } else {
            self.bindings_map.insert(identifier, 1);
        }
        self.binders.push(binder);
        self.size += 1;
    }

    pub fn unbind(&mut self, identifier: StringId) {
        debug_assert!(self.bindings_map.contains_key(&identifier));
        let count = self.bindings_map.get_mut(&identifier).unwrap();
        debug_assert!(*count > 0);
        *count -= 1;
        if *count == 0 {
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

    #[inline]
    pub fn bind_option(&mut self, identifier: Option<StringId>, binder: ExpressionId) {
        match identifier {
            Option::Some(identifier) => self.bind(identifier, binder),
            Option::None => self.shift(binder),
        }
    }

    #[inline]
    pub fn unbind_option(&mut self, identifier: Option<StringId>) {
        match identifier {
            Option::Some(identifier) => self.unbind(identifier),
            Option::None => self.unshift(),
        }
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
            constraints: ConstraintStore::new(expressions.len()),
            environment: ReferencingEnvironment::new(),
        }
    }

    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable { identifier } => {
                if self.environment.is_free(*identifier) {
                    for binder in self.environment.binders.iter() {
                        match self.constraints.get_mut(*binder) {
                            Option::Some(Constraint::NamelessAbstraction {
                                parameter: _,
                                restrictions,
                                used: _,
                            }) => {
                                restrictions
                                    .insert(ParameterRestriction::FreeVariable { id: *identifier });
                            }
                            Option::Some(Constraint::Abstraction { parameter: _ }) => {}
                            _ => unreachable!(),
                        }
                    }
                }
            }
            Expression::NamelessVariable { index } => {
                let parameter = self.lookup(*index);
                self.constraints
                    .set(expression, Constraint::Variable { name: parameter });
                let mut additional_restrictions = HashSet::new();
                for binder in self
                    .environment
                    .binders
                    .iter()
                    .rev()
                    .take(index.into_usize() - 1)
                {
                    match self.constraints.get_mut(*binder) {
                        Option::Some(Constraint::NamelessAbstraction {
                            parameter: _,
                            restrictions,
                            used: _,
                        }) => {
                            restrictions
                                .insert(ParameterRestriction::BoundVariable { id: parameter });
                        }
                        Option::Some(Constraint::Abstraction {
                            parameter: Option::Some(parameter),
                        }) => {
                            additional_restrictions
                                .insert(ParameterRestriction::FreeVariable { id: *parameter });
                        }
                        Option::Some(Constraint::Abstraction {
                            parameter: Option::None,
                        }) => {}
                        _ => unreachable!(),
                    }
                }

                let binder =
                    self.environment.binders[self.environment.binders.len() - index.into_usize()];
                if let Option::Some(Constraint::NamelessAbstraction {
                    parameter: _,
                    restrictions,
                    used,
                }) = self.constraints.get_mut(binder)
                {
                    *used = true;
                    for additional_restriction in additional_restrictions {
                        restrictions.insert(additional_restriction);
                    }
                } else {
                    unreachable!()
                }
            }
            Expression::Abstraction { parameter, body } => {
                self.constraints.set(
                    expression,
                    Constraint::Abstraction {
                        parameter: *parameter,
                    },
                );
                match parameter {
                    Option::Some(parameter) => {
                        self.environment.bind(*parameter, expression);
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
                let variable = self.variables.new_name();
                self.constraints.set(
                    expression,
                    Constraint::NamelessAbstraction {
                        parameter: variable,
                        restrictions: HashSet::new(),
                        used: false,
                    },
                );
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

    fn lookup(&mut self, index: DeBruijnIndex) -> VariableNameId {
        let binder = self.environment.binders[self.environment.binders.len() - index.into_usize()];
        if let Option::Some(Constraint::NamelessAbstraction {
            parameter,
            restrictions: _,
            used: _,
        }) = self.constraints.get(binder)
        {
            parameter.clone()
        } else {
            unreachable!()
        }
    }
}

struct NameGeneration<'a, G: FreshVariableNameGenerator> {
    strings: &'a mut StringArena,
    provider: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    variables: VariableNameArena,
    constraints: ConstraintStore,
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
            variable_name_generator,
        }
    }

    fn to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.provider[expression] {
            Expression::Variable { identifier } => self.destination.variable(*identifier),
            Expression::NamelessVariable { index: _ } => {
                if let Option::Some(Constraint::Variable { name }) =
                    self.constraints.get(expression)
                {
                    let identifier = self.variables.lookup_name(*name).unwrap();
                    self.destination.variable(identifier)
                } else {
                    unreachable!()
                }
            }
            Expression::Abstraction { parameter, body } => {
                let named_body = self.to_named(*body);
                self.destination.abstraction(*parameter, named_body)
            }
            Expression::NamelessAbstraction { body } => {
                if let Option::Some(Constraint::NamelessAbstraction {
                    parameter,
                    restrictions,
                    used,
                }) = self.constraints.get(expression)
                {
                    let parameter = if *used {
                        let mut identifiers = HashSet::new();
                        for &restriction in restrictions {
                            match restriction {
                                ParameterRestriction::BoundVariable { id } => {
                                    if let Option::Some(identifier) = self.variables.lookup_name(id)
                                    {
                                        identifiers.insert(identifier);
                                    }
                                }
                                ParameterRestriction::FreeVariable { id } => {
                                    identifiers.insert(id);
                                }
                            }
                        }
                        let identifier = self
                            .variable_name_generator
                            .fresh_name(self.strings, &identifiers);
                        self.variables.update_name(*parameter, identifier);
                        Option::Some(identifier)
                    } else {
                        Option::None
                    };
                    let named_body = self.to_named(*body);
                    self.destination.abstraction(parameter, named_body)
                } else {
                    unreachable!()
                }
            }
            Expression::Application {
                function,
                arguments,
            } => {
                let named_function = self.to_named(*function);
                let mut named_arguments = Vec::with_capacity(arguments.len());
                for &argument in arguments {
                    let named_argument = self.to_named(argument);
                    named_arguments.push(named_argument);
                }
                self.destination
                    .application(named_function, named_arguments)
            }
        }
    }

    pub fn convert_to_named(mut self, expression: ExpressionId) -> ExpressionId {
        self.to_named(expression)
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

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
        let named_expression = Expression::to_named(
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
        let named_expression = Expression::to_named(
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
        let test_count = 400;
        for _ in 0..test_count {
            fuzz_test(&mut rng, max_depth);
        }
    }
}
