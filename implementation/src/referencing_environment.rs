use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{expression::DeBruijnIndex, strings::StringId};

pub struct ReferencingEnvironment {
    parent: Option<Rc<ReferencingEnvironment>>,
    bindings_map: HashMap<StringId, Vec<usize>>,
    size: usize,
}

impl ReferencingEnvironment {
    #[inline]
    pub fn new() -> ReferencingEnvironment {
        ReferencingEnvironment {
            parent: Option::None,
            bindings_map: HashMap::new(),
            size: 0,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> ReferencingEnvironment {
        ReferencingEnvironment {
            parent: Option::None,
            bindings_map: HashMap::with_capacity(capacity),
            size: 0,
        }
    }

    #[inline]
    pub fn new_frame(refs: Rc<ReferencingEnvironment>) -> ReferencingEnvironment {
        let size = refs.size;
        ReferencingEnvironment {
            parent: Option::Some(refs),
            bindings_map: HashMap::new(),
            size,
        }
    }

    #[inline]
    pub fn new_frame_with_capacity(
        refs: Rc<ReferencingEnvironment>,
        capacity: usize,
    ) -> ReferencingEnvironment {
        let size = refs.size;
        ReferencingEnvironment {
            parent: Option::Some(refs),
            bindings_map: HashMap::with_capacity(capacity),
            size,
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

    pub fn domain(&self) -> HashSet<StringId> {
        let mut keys: HashSet<StringId> = self.bindings_map.keys().cloned().collect();
        if let Option::Some(parent) = self.parent.as_ref() {
            keys.extend(parent.domain());
        }
        keys
    }

    pub fn domain_len(&self) -> usize {
        self.bindings_map.len()
    }

    pub fn bind(&mut self, identifier: StringId) {
        if let Option::Some(stack) = self.bindings_map.get_mut(&identifier) {
            stack.push(self.size);
        } else {
            let stack = vec![self.size];
            self.bindings_map.insert(identifier, stack);
        }
        self.shift();
    }

    pub fn unbind(&mut self, identifier: StringId) {
        debug_assert!(self.bindings_map.contains_key(&identifier));
        let stack = self.bindings_map.get_mut(&identifier).unwrap();
        debug_assert!(!stack.is_empty());
        stack.pop();
        if stack.is_empty() {
            self.bindings_map.remove(&identifier);
        }
        self.unshift();
    }

    #[inline]
    pub fn shift(&mut self) {
        self.size += 1;
    }

    #[inline]
    pub fn unshift(&mut self) {
        debug_assert!(self.size > 0);
        self.size -= 1;
    }

    pub fn lookup(&self, identifier: StringId) -> Option<usize> {
        self.bindings_map
            .get(&identifier)
            .and_then(|stack| stack.last().copied())
            .or_else(|| self.parent.as_ref()?.lookup(identifier))
    }

    pub fn lookup_index(&self, identifier: StringId) -> Option<DeBruijnIndex> {
        self.lookup(identifier)
            .map(|level| (self.size - level).into())
    }
}

impl Default for ReferencingEnvironment {
    fn default() -> ReferencingEnvironment {
        ReferencingEnvironment::new()
    }
}
