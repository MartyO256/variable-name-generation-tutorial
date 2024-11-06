use std::ops::Index;

use crate::strings::StringId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DeBruijnIndex {
    index: usize,
}

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExpressionId {
    index: usize,
}

pub enum Expression {
    Variable {
        identifier: StringId,
    },
    NamelessVariable {
        index: DeBruijnIndex,
    },
    Abstraction {
        parameter: StringId,
        body: ExpressionId,
    },
    NamelessAbstraction {
        body: ExpressionId,
    },
    Application {
        function: ExpressionId,
        arguments: Vec<ExpressionId>,
    },
}

impl DeBruijnIndex {
    #[inline]
    pub fn new(index: usize) -> DeBruijnIndex {
        DeBruijnIndex { index }
    }

    #[inline]
    pub fn into_usize(self) -> usize {
        self.index
    }
}

impl From<usize> for DeBruijnIndex {
    #[inline]
    fn from(value: usize) -> DeBruijnIndex {
        DeBruijnIndex::new(value)
    }
}

impl From<DeBruijnIndex> for usize {
    #[inline]
    fn from(value: DeBruijnIndex) -> usize {
        value.into_usize()
    }
}

impl ExpressionId {
    #[inline]
    pub fn new(index: usize) -> ExpressionId {
        ExpressionId { index }
    }

    #[inline]
    pub fn into_usize(self) -> usize {
        self.index
    }
}

impl ExpressionArena {
    #[inline]
    pub fn new() -> ExpressionArena {
        ExpressionArena {
            expressions: Vec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> ExpressionArena {
        ExpressionArena {
            expressions: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn get(&self, reference: ExpressionId) -> &Expression {
        debug_assert!(self.has(reference));
        &self.expressions[reference.into_usize()]
    }

    pub fn add(&mut self, e: Expression) -> ExpressionId {
        let i = ExpressionId::new(self.expressions.len());
        self.expressions.push(e);
        i
    }

    #[inline]
    pub fn has(&self, reference: ExpressionId) -> bool {
        reference.into_usize() < self.len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.expressions.len()
    }

    #[inline]
    pub fn variable(&mut self, identifier: StringId) -> ExpressionId {
        self.add(Expression::Variable { identifier })
    }

    #[inline]
    pub fn nameless_variable(&mut self, index: DeBruijnIndex) -> ExpressionId {
        self.add(Expression::NamelessVariable { index })
    }

    #[inline]
    pub fn abstraction(&mut self, parameter: StringId, body: ExpressionId) -> ExpressionId {
        self.add(Expression::Abstraction { parameter, body })
    }

    #[inline]
    pub fn nameless_abstraction(&mut self, body: ExpressionId) -> ExpressionId {
        self.add(Expression::NamelessAbstraction { body })
    }

    #[inline]
    pub fn application(
        &mut self,
        function: ExpressionId,
        arguments: Vec<ExpressionId>,
    ) -> ExpressionId {
        debug_assert!(!arguments.is_empty());
        self.add(Expression::Application {
            function,
            arguments,
        })
    }
}

impl Index<ExpressionId> for ExpressionArena {
    type Output = Expression;

    #[inline]
    fn index(&self, index: ExpressionId) -> &Self::Output {
        self.get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::strings::StringArena;

    use super::*;

    #[test]
    fn arena_is_initially_empty() {
        let expressions = ExpressionArena::new();
        assert!(expressions.len() == 0);
    }

    #[test]
    fn arena_variable_creates_variable() {
        let mut strings = StringArena::new();
        let x = strings.intern("x".as_bytes());

        let mut expressions = ExpressionArena::new();

        let vx = expressions.variable(x);

        assert!(expressions.has(vx));
        assert!(matches!(
            expressions[vx],
            Expression::Variable { identifier: _ }
        ));
        if let Expression::Variable { identifier } = expressions[vx] {
            assert!(identifier == x);
        }
    }

    #[test]
    fn arena_nameless_variable_creates_nameless_variable() {
        let mut expressions = ExpressionArena::new();

        let vx = expressions.nameless_variable(1.into());

        assert!(expressions.has(vx));
        assert!(matches!(
            expressions[vx],
            Expression::NamelessVariable { index: _ }
        ));
        if let Expression::NamelessVariable { index } = expressions[vx] {
            assert!(index == 1.into());
        }
    }

    #[test]
    fn arena_abstraction_creates_abstraction() {
        let mut strings = StringArena::new();
        let x = strings.intern("x".as_bytes());

        let mut expressions = ExpressionArena::new();

        let vx = expressions.variable(x);
        let f = expressions.abstraction(x, vx);

        assert!(expressions.has(f));
        assert!(matches!(
            expressions[f],
            Expression::Abstraction {
                parameter: _,
                body: _
            }
        ));
        if let Expression::Abstraction { parameter, body } = expressions[f] {
            assert!(parameter == x);
            assert!(body == vx);
        }
    }

    #[test]
    fn arena_nameless_abstraction_creates_nameless_abstraction() {
        let mut expressions = ExpressionArena::new();

        let vx = expressions.nameless_variable(1.into());
        let f = expressions.nameless_abstraction(vx);

        assert!(expressions.has(f));
        assert!(matches!(
            expressions[f],
            Expression::NamelessAbstraction { body: _ }
        ));
        if let Expression::NamelessAbstraction { body } = expressions[f] {
            assert!(body == vx);
        }
    }

    #[test]
    fn arena_application_creates_application() {
        let mut strings = StringArena::new();
        let x = strings.intern("x".as_bytes());
        let y = strings.intern("y".as_bytes());
        let f = strings.intern("f".as_bytes());

        let mut expressions = ExpressionArena::with_capacity(4);

        let vx = expressions.variable(x);
        let vy = expressions.variable(y);
        let vf = expressions.variable(f);
        let app = expressions.application(vf, vec![vx, vy]);

        assert!(expressions.has(app));
        assert!(matches!(
            expressions[app],
            Expression::Application {
                function: _,
                arguments: _
            }
        ));
        if let Expression::Application {
            function,
            ref arguments,
        } = expressions[app]
        {
            assert!(function == vf);
            assert!(*arguments == vec![vx, vy]);
        }
    }
}
