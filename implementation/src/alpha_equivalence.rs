use std::rc::Rc;

use crate::{
    expression::{Expression, ExpressionArena, ExpressionId},
    referencing_environment::ReferencingEnvironment,
};

struct AlphaEquivalence<'a> {
    expressions1: &'a ExpressionArena,
    environment1: &'a mut ReferencingEnvironment,
    expressions2: &'a ExpressionArena,
    environment2: &'a mut ReferencingEnvironment,
}

impl<'a> AlphaEquivalence<'a> {
    pub fn alpha_equivalent(&mut self, e1: ExpressionId, e2: ExpressionId) -> bool {
        match (&self.expressions1[e1], &self.expressions2[e2]) {
            (
                &Expression::Variable { identifier: i1 },
                &Expression::Variable { identifier: i2 },
            ) => self.environment1.lookup_index(i1) == self.environment2.lookup_index(i2),
            (
                &Expression::Variable { identifier: i1 },
                &Expression::NamelessVariable { index: i2 },
            ) => match self.environment1.lookup_index(i1) {
                Option::Some(l1) => l1 == i2,
                Option::None => false,
            },
            (
                &Expression::NamelessVariable { index: i1 },
                &Expression::Variable { identifier: i2 },
            ) => match self.environment2.lookup_index(i2) {
                Option::Some(l2) => i1 == l2,
                Option::None => false,
            },
            (
                &Expression::NamelessVariable { index: i1 },
                &Expression::NamelessVariable { index: i2 },
            ) => i1 == i2,
            (
                &Expression::Abstraction {
                    parameter: param1,
                    body: b1,
                },
                &Expression::Abstraction {
                    parameter: param2,
                    body: b2,
                },
            ) => {
                self.environment1.bind_option(param1);
                self.environment2.bind_option(param2);
                let r = self.alpha_equivalent(b1, b2);
                self.environment2.unbind_option(param2);
                self.environment1.unbind_option(param1);
                r
            }
            (
                &Expression::Abstraction {
                    parameter: param1,
                    body: b1,
                },
                &Expression::NamelessAbstraction { body: b2 },
            ) => {
                self.environment1.bind_option(param1);
                self.environment2.shift();
                let r = self.alpha_equivalent(b1, b2);
                self.environment2.unshift();
                self.environment1.unbind_option(param1);
                r
            }
            (
                &Expression::NamelessAbstraction { body: b1 },
                &Expression::Abstraction {
                    parameter: param2,
                    body: b2,
                },
            ) => {
                self.environment1.shift();
                self.environment2.bind_option(param2);
                let r = self.alpha_equivalent(b1, b2);
                self.environment2.unbind_option(param2);
                self.environment1.unshift();
                r
            }
            (
                &Expression::NamelessAbstraction { body: b1 },
                &Expression::NamelessAbstraction { body: b2 },
            ) => {
                self.environment1.shift();
                self.environment2.shift();
                let r = self.alpha_equivalent(b1, b2);
                self.environment2.unshift();
                self.environment1.unshift();
                r
            }
            (
                &Expression::Application {
                    function: f1,
                    arguments: ref as1,
                },
                &Expression::Application {
                    function: f2,
                    arguments: ref as2,
                },
            ) => {
                if as1.len() != as2.len() {
                    false
                } else if !self.alpha_equivalent(f1, f2) {
                    false
                } else {
                    for (&a1, &a2) in as1.iter().zip(as2.iter()) {
                        if !self.alpha_equivalent(a1, a2) {
                            return false;
                        }
                    }
                    true
                }
            }
            _ => false,
        }
    }
}

pub fn alpha_convertible(
    (environment1, expressions1, e1): (Rc<ReferencingEnvironment>, &ExpressionArena, ExpressionId),
    (environment2, expressions2, e2): (Rc<ReferencingEnvironment>, &ExpressionArena, ExpressionId),
) -> bool {
    let mut framed_env1 = ReferencingEnvironment::new_frame(environment1);
    let mut framed_env2 = ReferencingEnvironment::new_frame(environment2);
    AlphaEquivalence {
        expressions1,
        environment1: &mut framed_env1,
        expressions2,
        environment2: &mut framed_env2,
    }
    .alpha_equivalent(e1, e2)
}
