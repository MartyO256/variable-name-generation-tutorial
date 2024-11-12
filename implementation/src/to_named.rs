use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    expression::{DeBruijnIndex, Expression, ExpressionArena, ExpressionId},
    strings::{StringArena, StringId},
};

#[derive(Clone)]
struct ReferenceSet {
    variables: HashSet<StringId>,
    indices: HashMap<DeBruijnIndex, Rc<RefCell<Option<StringId>>>>,
}

struct BindingStack {
    stack: Vec<Rc<RefCell<Option<StringId>>>>,
}

struct Store {
    reference_sets: Vec<Option<ReferenceSet>>,
}

impl ReferenceSet {
    fn variable(identifier: StringId) -> ReferenceSet {
        let mut variables = HashSet::new();
        variables.insert(identifier);
        ReferenceSet {
            variables,
            indices: HashMap::new(),
        }
    }

    fn index(index: DeBruijnIndex, cell: Rc<RefCell<Option<StringId>>>) -> ReferenceSet {
        let mut indices = HashMap::new();
        indices.insert(index, cell);
        ReferenceSet {
            variables: HashSet::new(),
            indices,
        }
    }

    fn union(sets: Vec<&ReferenceSet>) -> ReferenceSet {
        let mut variables = HashSet::new();
        for &set in sets.iter() {
            for &variable in set.variables.iter() {
                variables.insert(variable);
            }
        }
        let mut indices = HashMap::new();
        for &set in sets.iter() {
            for (&index, name) in set.indices.iter() {
                indices.insert(index, name.clone());
            }
        }
        ReferenceSet { variables, indices }
    }

    fn unshifted_indices(&self) -> HashMap<DeBruijnIndex, Rc<RefCell<Option<StringId>>>> {
        let mut indices = HashMap::new();
        for (&index, name) in self.indices.iter() {
            let i = index.into_usize();
            if i > 1 {
                indices.insert((i - 1).into(), name.clone());
            }
        }
        indices
    }

    fn unshift(&self) -> ReferenceSet {
        let variables = self.variables.clone();
        let indices = self.unshifted_indices();
        ReferenceSet { variables, indices }
    }

    fn unbind(&self, identifier: StringId) -> ReferenceSet {
        let mut variables = self.variables.clone();
        variables.remove(&identifier);
        let indices = self.unshifted_indices();
        ReferenceSet { variables, indices }
    }

    fn unbind_option(&self, identifier: Option<StringId>) -> ReferenceSet {
        match identifier {
            Option::Some(identifier) => self.unbind(identifier),
            Option::None => self.unshift(),
        }
    }

    fn len(&self) -> usize {
        self.variables.len() + self.indices.len()
    }

    fn names(&self) -> HashSet<StringId> {
        let mut names = HashSet::with_capacity(self.len());
        for &name in self.variables.iter() {
            names.insert(name);
        }
        for index in self.indices.values() {
            if let Option::Some(name) = index.borrow().clone() {
                names.insert(name);
            }
        }
        names
    }

    fn lookup_name(&self, index: DeBruijnIndex) -> Option<StringId> {
        self.indices
            .get(&index)
            .and_then(|cell| cell.borrow().clone())
    }

    fn select_name(&self, index: DeBruijnIndex, name: StringId) {
        debug_assert!(self.contains_index(index));
        *self.indices.get(&index).unwrap().borrow_mut() = Option::Some(name);
    }

    fn contains_index(&self, index: DeBruijnIndex) -> bool {
        self.indices.contains_key(&index)
    }
}

impl BindingStack {
    fn new() -> BindingStack {
        BindingStack { stack: Vec::new() }
    }

    #[allow(dead_code)]
    fn with_capacity(capacity: usize) -> BindingStack {
        BindingStack {
            stack: Vec::with_capacity(capacity),
        }
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn push(&mut self) {
        self.stack.push(Rc::new(RefCell::new(Option::None)))
    }

    fn pop(&mut self) {
        debug_assert!(self.len() > 0);
        self.stack.pop().unwrap();
    }

    fn lookup(&self, index: DeBruijnIndex) -> &Rc<RefCell<Option<StringId>>> {
        debug_assert!(index.into_usize() <= self.len());
        &self.stack[self.len() - index.into_usize()]
    }
}

impl Store {
    fn new(n: usize) -> Store {
        Store {
            reference_sets: vec![Option::None; n],
        }
    }

    fn has(&self, expression: ExpressionId) -> bool {
        expression.into_usize() < self.reference_sets.len()
    }

    fn set(&mut self, expression: ExpressionId, reference_set: ReferenceSet) {
        debug_assert!(self.has(expression));
        self.reference_sets[expression.into_usize()] = Option::Some(reference_set)
    }

    fn get(&self, expression: ExpressionId) -> &ReferenceSet {
        debug_assert!(self.has(expression));
        self.reference_sets[expression.into_usize()]
            .as_ref()
            .unwrap()
    }
}

struct StoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    bindings: BindingStack,
    store: Store,
}

impl<'a> StoreBuilder<'a> {
    fn new(expressions: &'a ExpressionArena) -> StoreBuilder<'a> {
        StoreBuilder {
            expressions,
            bindings: BindingStack::new(),
            store: Store::new(expressions.len()),
        }
    }

    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            &Expression::Variable { identifier } => {
                self.store
                    .set(expression, ReferenceSet::variable(identifier));
            }
            &Expression::NamelessVariable { index } => {
                let cell = self.bindings.lookup(index);
                self.store
                    .set(expression, ReferenceSet::index(index, cell.clone()));
            }
            &Expression::Abstraction { parameter, body } => {
                self.bindings.push();
                self.visit(body);
                self.bindings.pop();
                let body_reference_set = self.store.get(body);
                self.store
                    .set(expression, body_reference_set.unbind_option(parameter));
            }
            &Expression::NamelessAbstraction { body } => {
                self.bindings.push();
                self.visit(body);
                self.bindings.pop();
                let body_reference_set = self.store.get(body);
                self.store.set(expression, body_reference_set.unshift());
            }
            &Expression::Application {
                function,
                ref arguments,
            } => {
                self.visit(function);
                for &argument in arguments.iter() {
                    self.visit(argument);
                }
                let mut reference_sets = Vec::new();
                let function_reference_set = self.store.get(function);
                reference_sets.push(function_reference_set);
                for &argument in arguments.iter() {
                    let argument_reference_set = self.store.get(argument);
                    reference_sets.push(argument_reference_set);
                }
                let reference_set = ReferenceSet::union(reference_sets);
                self.store.set(expression, reference_set);
            }
        }
    }

    fn into_store(self) -> Store {
        self.store
    }
}

pub trait FreshVariableNameGenerator {
    fn fresh_name(&self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId;
}

struct NameGeneration<'a, G: FreshVariableNameGenerator> {
    strings: &'a mut StringArena,
    provider: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    store: Store,
    variable_name_generator: G,
}

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.provider[expression] {
            &Expression::Variable { identifier } => self.destination.variable(identifier),
            &Expression::NamelessVariable { index } => {
                let reference_set = self.store.get(expression);
                let identifier = reference_set.lookup_name(index).unwrap();
                self.destination.variable(identifier)
            }
            &Expression::Abstraction { parameter, body } => {
                let named_body = self.to_named(body);
                self.destination.abstraction(parameter, named_body)
            }
            &Expression::NamelessAbstraction { body } => {
                let reference_set = self.store.get(body);
                let parameter = if reference_set.contains_index(1.into()) {
                    let identifiers = reference_set.names();
                    let identifier = self
                        .variable_name_generator
                        .fresh_name(&mut self.strings, &identifiers);
                    reference_set.select_name(1.into(), identifier);
                    Option::Some(identifier)
                } else {
                    Option::None
                };
                let named_body = self.to_named(body);
                self.destination.abstraction(parameter, named_body)
            }
            &Expression::Application {
                function,
                ref arguments,
            } => {
                let named_function = self.to_named(function);
                let named_arguments = arguments
                    .iter()
                    .map(|&argument| self.to_named(argument))
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
    let mut store_builder = StoreBuilder::new(expressions);
    store_builder.visit(expression);
    let store = store_builder.into_store();
    NameGeneration {
        strings,
        provider: expressions,
        destination,
        store,
        variable_name_generator,
    }
    .to_named(expression)
}

pub struct SuffixVariableNameGenerator {}

impl SuffixVariableNameGenerator {
    pub fn new() -> SuffixVariableNameGenerator {
        SuffixVariableNameGenerator {}
    }
}

impl FreshVariableNameGenerator for SuffixVariableNameGenerator {
    fn fresh_name(&self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId {
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

pub fn is_named(expressions: &ExpressionArena, expression: ExpressionId) -> bool {
    match &expressions[expression] {
        &Expression::Variable { identifier: _ } => true,
        &Expression::NamelessVariable { index: _ } => false,
        &Expression::Abstraction { parameter: _, body } => is_named(expressions, body),
        &Expression::NamelessAbstraction { body: _ } => false,
        &Expression::Application {
            function,
            ref arguments,
        } => {
            if !is_named(expressions, function) {
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

#[cfg(test)]
mod tests {

    use crate::{
        alpha_equivalence::alpha_equivalent, equality::equals, parser::parse_expression,
        referencing_environment::ReferencingEnvironment, to_locally_nameless::to_nameless,
    };

    use super::*;

    #[test]
    fn to_named_fx_x() {
        let mut strings: StringArena = StringArena::new();
        let nx = strings.intern_str("x");

        let mut p1 = ExpressionArena::new();

        let x = p1.nameless_variable(1.into());
        let fx_x = p1.nameless_abstraction(x);

        let mut p2 = ExpressionArena::new();

        let variable_namer = SuffixVariableNameGenerator::new();
        let named_fx_x = to_named(&mut strings, &p1, fx_x, &mut p2, variable_namer);

        let mut p3 = ExpressionArena::new();
        let x = p3.variable(nx);
        let fx = p3.abstraction(Option::Some(nx), x);

        assert!(equals((&p2, named_fx_x), (&p3, fx)));
    }

    #[test]
    fn to_named_fxy_x() {
        let mut strings = StringArena::new();
        let nx = strings.intern_str("x");

        let mut p1 = ExpressionArena::new();

        let x = p1.nameless_variable(2.into());
        let fy_x = p1.nameless_abstraction(x);
        let fxy_x = p1.nameless_abstraction(fy_x);

        let mut p2 = ExpressionArena::new();

        let variable_namer = SuffixVariableNameGenerator::new();
        let named_fxy_x = to_named(&mut strings, &p1, fxy_x, &mut p2, variable_namer);

        let mut p3 = ExpressionArena::new();
        let x = p3.variable(nx);
        let fy_x = p3.abstraction(Option::None, x);
        let fxy_x = p3.abstraction(Option::Some(nx), fy_x);

        assert!(equals((&p2, named_fxy_x), (&p3, fxy_x)));
    }

    #[test]
    fn to_named_fxy_y() {
        let mut strings = StringArena::new();
        let nx = strings.intern_str("x");

        let mut p1 = ExpressionArena::new();

        let x = p1.nameless_variable(1.into());
        let fy_x = p1.nameless_abstraction(x);
        let fxy_y = p1.nameless_abstraction(fy_x);

        let mut p2 = ExpressionArena::new();

        let variable_namer = SuffixVariableNameGenerator::new();
        let named_fxy_y = to_named(&mut strings, &p1, fxy_y, &mut p2, variable_namer);

        let mut p3 = ExpressionArena::new();
        let x = p3.variable(nx);
        let fy_x = p3.abstraction(Option::Some(nx), x);
        let fxy_y = p3.abstraction(Option::None, fy_x);

        assert!(equals((&p2, named_fxy_y), (&p3, fxy_y)));
    }

    fn roundtrip_test(input: &str) {
        let mut strings = StringArena::new();
        let mut source_expressions = ExpressionArena::new();
        let mut nameless_expressions = ExpressionArena::new();
        let mut named_expressions = ExpressionArena::new();
        let variable_name_generator = SuffixVariableNameGenerator::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::new());

        let expression =
            parse_expression(&mut strings, &mut source_expressions, input.as_bytes()).unwrap();
        let nameless_expression = to_nameless(
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
        assert!(is_named(&named_expressions, named_expression));

        assert!(alpha_equivalent(
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
}
