use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Index,
    rc::Rc,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StringId {
    index: usize,
}

impl StringId {
    #[inline]
    fn new(index: usize) -> Self {
        StringId { index }
    }

    #[inline]
    fn into_usize(self) -> usize {
        self.index
    }
}

pub struct StringArena {
    ids: HashMap<Rc<Box<[u8]>>, StringId>,
    strings: Vec<Rc<Box<[u8]>>>,
}

impl StringArena {
    #[inline]
    pub fn new() -> StringArena {
        StringArena {
            ids: HashMap::new(),
            strings: Vec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> StringArena {
        StringArena {
            ids: HashMap::with_capacity(capacity),
            strings: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn get(&self, reference: StringId) -> &[u8] {
        debug_assert!(self.has(reference));
        &self.strings[reference.into_usize()]
    }

    #[inline]
    pub fn has(&self, reference: StringId) -> bool {
        reference.into_usize() < self.len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn intern(&mut self, value: &[u8]) -> StringId {
        match self.ids.entry(Rc::new(value.to_vec().into_boxed_slice())) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let index = self.strings.len();
                let id = StringId::new(index);
                self.strings.push(entry.key().clone());
                entry.insert(id);
                id
            }
        }
    }

    pub fn intern_str(&mut self, value: &str) -> StringId {
        self.intern(value.as_bytes())
    }
}

impl Index<StringId> for StringArena {
    type Output = [u8];

    #[inline]
    fn index(&self, index: StringId) -> &Self::Output {
        self.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_initially_empty() {
        let strings = StringArena::new();
        assert!(strings.len() == 0);
    }

    #[test]
    fn assigns_unique_ids_for_different_strings() {
        let mut strings = StringArena::with_capacity(3);

        let i1 = strings.intern_str("a");
        let i2 = strings.intern_str("b");
        let i3 = strings.intern_str("c");

        assert!(strings.has(i1));
        assert!(strings.has(i2));
        assert!(strings.has(i3));

        assert!(i1 != i2);
        assert!(i1 != i3);
        assert!(i2 != i3);

        assert!(strings[i1] != strings[i2]);
        assert!(strings[i1] != strings[i3]);
        assert!(strings[i2] != strings[i3]);
    }

    #[test]
    fn returns_same_id_for_same_strings() {
        let mut strings = StringArena::with_capacity(1);

        let i1 = strings.intern_str("a");
        let i2 = strings.intern_str("a");

        assert!(strings.has(i1));
        assert!(strings.has(i2));

        assert!(i1 == i2);

        assert!(strings[i1] == strings[i2]);
    }
}
