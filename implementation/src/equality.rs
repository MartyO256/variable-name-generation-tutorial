use crate::expression::{Expression, ExpressionArena, ExpressionId};

struct Equality<'a> {
    expressions1: &'a ExpressionArena,
    expressions2: &'a ExpressionArena,
}

impl<'a> Equality<'a> {
    fn equals(&self, e1: ExpressionId, e2: ExpressionId) -> bool {
        match (&self.expressions1[e1], &self.expressions2[e2]) {
            (
                &Expression::Variable { identifier: i1 },
                &Expression::Variable { identifier: i2 },
            ) => i1 == i2,
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
            ) => param1 == param2 && self.equals(b1, b2),
            (
                &Expression::NamelessAbstraction { body: b1 },
                &Expression::NamelessAbstraction { body: b2 },
            ) => self.equals(b1, b2),
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
                } else if !self.equals(f1, f2) {
                    false
                } else {
                    for (&a1, &a2) in as1.iter().zip(as2.iter()) {
                        if !self.equals(a1, a2) {
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

pub fn equals(
    (expressions1, e1): (&ExpressionArena, ExpressionId),
    (expressions2, e2): (&ExpressionArena, ExpressionId),
) -> bool {
    Equality {
        expressions1,
        expressions2: expressions2,
    }
    .equals(e1, e2)
}

#[cfg(test)]
mod tests {
    use crate::strings::StringArena;

    use super::*;

    #[test]
    fn equals_same_variable() {
        let mut strings = StringArena::new();
        let nx = strings.intern("x".as_bytes());

        let mut expressions = ExpressionArena::new();
        let r1 = expressions.variable(nx);
        let r2 = expressions.variable(nx);

        assert!(equals((&expressions, r1), (&expressions, r1)));
        assert!(equals((&expressions, r1), (&expressions, r2)));
        assert!(equals((&expressions, r2), (&expressions, r2)));
    }

    #[test]
    fn equals_same_nameless_variable() {
        let mut expressions = ExpressionArena::new();
        let r1 = expressions.nameless_variable(1.into());
        let r2 = expressions.nameless_variable(1.into());

        assert!(equals((&expressions, r1), (&expressions, r1)));
        assert!(equals((&expressions, r1), (&expressions, r2)));
        assert!(equals((&expressions, r2), (&expressions, r2)));
    }
}
