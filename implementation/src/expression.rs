use std::ops::Index;

use crate::strings::StringId;

/// De Bruijn indices denoting the distance between a nameless variable and its
/// binder. These indices start at 1.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DeBruijnIndex {
    index: usize,
}

/// Contiguous store of [expressions](Expression) backed by a vector.
#[derive(Debug)]
pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

/// Expression IDs as indices in [expression arenas](ExpressionArena).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExpressionId {
    index: usize,
}

/// Expressions in mixed representation. This is a flat data implementation for
/// abstract syntax trees.
///
/// These expressions may contain named (free or bound) and nameless variables,
/// named and nameless lambda abstractions, and applications.
///
/// To support annotating expressions with auxiliary data, expressions are
/// constructed and stored in [expression arenas](ExpressionArena). As such,
/// expressions are [referred to by ID](ExpressionId) within an arena.
#[derive(Debug)]
pub enum Expression {
    Variable {
        identifier: StringId,
    },
    NamelessVariable {
        index: DeBruijnIndex,
    },
    Abstraction {
        parameter: Option<StringId>,
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
        debug_assert!(index > 0);
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
    /// Creates a new empty expression arena.
    #[inline]
    pub fn new() -> ExpressionArena {
        ExpressionArena {
            expressions: Vec::new(),
        }
    }

    /// Creates an empty expression arena with at least the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> ExpressionArena {
        ExpressionArena {
            expressions: Vec::with_capacity(capacity),
        }
    }

    /// Retrieves the expression with the corresponding ID in the expression
    /// arena. It is assumed that the expression arena has sufficiently many
    /// expressions in it for the ID to be included in it.
    #[inline]
    pub fn get(&self, id: ExpressionId) -> &Expression {
        debug_assert!(self.has(id));
        &self.expressions[id.into_usize()]
    }

    /// Adds the given expression to the expression arena, and returns that
    /// expression's ID to retrieve it from the expression arena.
    pub fn add(&mut self, e: Expression) -> ExpressionId {
        let i = ExpressionId::new(self.expressions.len());
        self.expressions.push(e);
        i
    }

    #[inline]
    pub fn has(&self, id: ExpressionId) -> bool {
        id.into_usize() < self.len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.expressions.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.expressions.is_empty()
    }

    /// Constructs a variable with the given identifier and adds it to the
    /// expression arena.
    #[inline]
    pub fn variable(&mut self, identifier: StringId) -> ExpressionId {
        self.add(Expression::Variable { identifier })
    }

    /// Constructs a nameless variable with the given de Bruijn index and adds
    /// it to the expression arena.
    #[inline]
    pub fn nameless_variable(&mut self, index: DeBruijnIndex) -> ExpressionId {
        self.add(Expression::NamelessVariable { index })
    }

    /// Constructs a lambda abstraction with the given parameter and body
    /// expression and adds it to the expression arena.
    #[inline]
    pub fn abstraction(&mut self, parameter: Option<StringId>, body: ExpressionId) -> ExpressionId {
        self.add(Expression::Abstraction { parameter, body })
    }

    /// Constructs a nameless lambda abstraction with the given body expression
    /// and adds it to the expression arena.
    #[inline]
    pub fn nameless_abstraction(&mut self, body: ExpressionId) -> ExpressionId {
        self.add(Expression::NamelessAbstraction { body })
    }

    /// Constructs an application with the given function and arguments
    /// sub-expressions and adds it to the expression arena.
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

impl Default for ExpressionArena {
    fn default() -> ExpressionArena {
        ExpressionArena::new()
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
        let expressions = ExpressionArena::default();
        assert!(expressions.len() == 0);
        assert!(expressions.is_empty());
    }

    #[test]
    fn arena_variable_creates_variable() {
        let mut strings = StringArena::new();
        let x = strings.intern_str("x");

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
        let x = strings.intern_str("x");

        let mut expressions = ExpressionArena::new();

        let vx = expressions.variable(x);
        let f = expressions.abstraction(Option::Some(x), vx);

        assert!(expressions.has(f));
        assert!(matches!(
            expressions[f],
            Expression::Abstraction {
                parameter: _,
                body: _
            }
        ));
        if let Expression::Abstraction { parameter, body } = expressions[f] {
            assert!(parameter == Option::Some(x));
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
        let x = strings.intern_str("x");
        let y = strings.intern_str("y");
        let f = strings.intern_str("f");

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
