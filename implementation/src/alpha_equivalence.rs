use std::rc::Rc;

use crate::{
    expression::{Expression, ExpressionArena, ExpressionId},
    referencing_environment::ReferencingEnvironment,
};

impl Expression {
    pub fn alpha_equivalent(
        (environment1, expressions1, e1): (
            Rc<ReferencingEnvironment>,
            &ExpressionArena,
            ExpressionId,
        ),
        (environment2, expressions2, e2): (
            Rc<ReferencingEnvironment>,
            &ExpressionArena,
            ExpressionId,
        ),
    ) -> bool {
        AlphaEquivalence::new(
            expressions1,
            &mut ReferencingEnvironment::new_frame(environment1),
            expressions2,
            &mut ReferencingEnvironment::new_frame(environment2),
        )
        .check_alpha_equivalence(e1, e2)
    }
}

struct AlphaEquivalence<'a> {
    expressions1: &'a ExpressionArena,
    environment1: &'a mut ReferencingEnvironment,
    expressions2: &'a ExpressionArena,
    environment2: &'a mut ReferencingEnvironment,
}

impl<'a> AlphaEquivalence<'a> {
    pub fn new(
        expressions1: &'a ExpressionArena,
        environment1: &'a mut ReferencingEnvironment,
        expressions2: &'a ExpressionArena,
        environment2: &'a mut ReferencingEnvironment,
    ) -> AlphaEquivalence<'a> {
        AlphaEquivalence {
            expressions1,
            environment1,
            expressions2,
            environment2,
        }
    }

    fn alpha_equivalent(&mut self, e1: ExpressionId, e2: ExpressionId) -> bool {
        match (&self.expressions1[e1], &self.expressions2[e2]) {
            (Expression::Variable { identifier: i1 }, Expression::Variable { identifier: i2 }) => {
                match (
                    self.environment1.lookup_index(*i1),
                    self.environment2.lookup_index(*i2),
                ) {
                    (Option::Some(i1), Option::Some(i2)) => i1 == i2,
                    (Option::None, Option::None) => i1 == i2,
                    _ => false,
                }
            }
            (
                Expression::Variable { identifier: i1 },
                Expression::NamelessVariable { index: i2 },
            ) => match self.environment1.lookup_index(*i1) {
                Option::Some(l1) => l1 == *i2,
                Option::None => false,
            },
            (
                Expression::NamelessVariable { index: i1 },
                Expression::Variable { identifier: i2 },
            ) => match self.environment2.lookup_index(*i2) {
                Option::Some(l2) => *i1 == l2,
                Option::None => false,
            },
            (
                Expression::NamelessVariable { index: i1 },
                Expression::NamelessVariable { index: i2 },
            ) => i1 == i2,
            (
                Expression::Abstraction {
                    parameter: param1,
                    body: b1,
                },
                Expression::Abstraction {
                    parameter: param2,
                    body: b2,
                },
            ) => match (param1, param2) {
                (Option::Some(param1), Option::Some(param2)) => {
                    self.environment1.bind(*param1);
                    self.environment2.bind(*param2);
                    let r = self.alpha_equivalent(*b1, *b2);
                    self.environment2.unbind(*param2);
                    self.environment1.unbind(*param1);
                    r
                }
                (Option::Some(param1), Option::None) => {
                    self.environment1.bind(*param1);
                    self.environment2.shift();
                    let r = self.alpha_equivalent(*b1, *b2);
                    self.environment2.unshift();
                    self.environment1.unbind(*param1);
                    r
                }
                (Option::None, Option::Some(param2)) => {
                    self.environment1.shift();
                    self.environment2.bind(*param2);
                    let r = self.alpha_equivalent(*b1, *b2);
                    self.environment2.unbind(*param2);
                    self.environment1.unshift();
                    r
                }
                (Option::None, Option::None) => {
                    self.environment1.shift();
                    self.environment2.shift();
                    let r = self.alpha_equivalent(*b1, *b2);
                    self.environment2.unshift();
                    self.environment1.unshift();
                    r
                }
            },
            (
                Expression::Abstraction {
                    parameter: param1,
                    body: b1,
                },
                Expression::NamelessAbstraction { body: b2 },
            ) => match param1 {
                Option::Some(param1) => {
                    self.environment1.bind(*param1);
                    self.environment2.shift();
                    let r = self.alpha_equivalent(*b1, *b2);
                    self.environment2.unshift();
                    self.environment1.unbind(*param1);
                    r
                }
                Option::None => {
                    self.environment1.shift();
                    self.environment2.shift();
                    let r = self.alpha_equivalent(*b1, *b2);
                    self.environment2.unshift();
                    self.environment1.unshift();
                    r
                }
            },
            (
                Expression::NamelessAbstraction { body: b1 },
                Expression::Abstraction {
                    parameter: param2,
                    body: b2,
                },
            ) => match param2 {
                Option::Some(param2) => {
                    self.environment1.shift();
                    self.environment2.bind(*param2);
                    let r = self.alpha_equivalent(*b1, *b2);
                    self.environment2.unbind(*param2);
                    self.environment1.unshift();
                    r
                }
                Option::None => {
                    self.environment1.shift();
                    self.environment2.shift();
                    let r = self.alpha_equivalent(*b1, *b2);
                    self.environment2.unshift();
                    self.environment1.unshift();
                    r
                }
            },
            (
                Expression::NamelessAbstraction { body: b1 },
                Expression::NamelessAbstraction { body: b2 },
            ) => {
                self.environment1.shift();
                self.environment2.shift();
                let r = self.alpha_equivalent(*b1, *b2);
                self.environment2.unshift();
                self.environment1.unshift();
                r
            }
            (
                Expression::Application {
                    function: f1,
                    arguments: as1,
                },
                Expression::Application {
                    function: f2,
                    arguments: as2,
                },
            ) => {
                if as1.len() != as2.len() || !self.alpha_equivalent(*f1, *f2) {
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

    pub fn check_alpha_equivalence(mut self, e1: ExpressionId, e2: ExpressionId) -> bool {
        self.alpha_equivalent(e1, e2)
    }
}

#[cfg(test)]
mod tests {

    use crate::{referencing_environment::ReferencingEnvironment, strings::StringArena};

    use super::*;

    fn check_alpha_equivalence(input1: &str, input2: &str, expected: bool) {
        let mut strings = StringArena::new();
        let mut expressions = ExpressionArena::new();
        let referencing_environment = Rc::new(ReferencingEnvironment::new());

        let expression1 =
            Expression::parse_mixed_expression(&mut strings, &mut expressions, input1.as_bytes())
                .unwrap();
        let expression2 =
            Expression::parse_mixed_expression(&mut strings, &mut expressions, input2.as_bytes())
                .unwrap();

        assert_eq!(
            Expression::alpha_equivalent(
                (referencing_environment.clone(), &expressions, expression1),
                (referencing_environment.clone(), &expressions, expression2)
            ),
            expected
        );
    }

    #[test]
    fn alpha_equivalence_tests() {
        check_alpha_equivalence("x", "x", true);
        check_alpha_equivalence("λx. x", "λx. x", true);
        check_alpha_equivalence("λx. x", "λy. y", true);
        check_alpha_equivalence("λ. 1", "λx. x", true);
        check_alpha_equivalence("λ. 1", "λ. 1", true);
        check_alpha_equivalence("λ_. λx. x", "λx. λy. y", true);
        check_alpha_equivalence("λf. λx. f x", "λg. λy. g y", true);
        check_alpha_equivalence("λ. λ. 1", "λx. λx. x", true);
        check_alpha_equivalence("λy. λy. y", "λx. λx. x", true);

        check_alpha_equivalence("x", "y", false);
        check_alpha_equivalence("λx. λf. f x", "λg. λy. g y", false);
        check_alpha_equivalence("λx. x", "λ_. y", false);
        check_alpha_equivalence("λ. 1", "λ_. y", false);
        check_alpha_equivalence("λ_. y", "λ. 1", false);
        check_alpha_equivalence("x1 x2 x3", "x1 x2", false);
        check_alpha_equivalence("(λx. x) z", "(λy. y) w", false);
        check_alpha_equivalence("λx. x", "w", false);
        check_alpha_equivalence("λ_. λx. x", "λ. λ. 2", false);
    }
}
