use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Index,
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

pub struct StringArena<'a> {
    ids: HashMap<&'a [u8], StringId>,
    strings: Vec<&'a [u8]>,
}

impl<'a> StringArena<'a> {
    #[inline]
    pub fn new() -> StringArena<'a> {
        StringArena {
            ids: HashMap::new(),
            strings: Vec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> StringArena<'a> {
        StringArena {
            ids: HashMap::with_capacity(capacity),
            strings: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn get(&self, reference: StringId) -> &'a [u8] {
        debug_assert!(self.has(reference));
        self.strings[reference.into_usize()]
    }

    #[inline]
    pub fn has(&self, reference: StringId) -> bool {
        reference.into_usize() < self.len()
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn intern(&mut self, value: &'a [u8]) -> StringId {
        match self.ids.entry(value) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let index = self.strings.len();
                let id = StringId::new(index);
                self.strings.push(entry.key());
                entry.insert(id);
                id
            }
        }
    }
}

impl<'a> Index<StringId> for StringArena<'a> {
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

        let i1 = strings.intern("a".as_bytes());
        let i2 = strings.intern("b".as_bytes());
        let i3 = strings.intern("c".as_bytes());

        assert!(strings.has(i1));
        assert!(strings.has(i2));
        assert!(strings.has(i3));

        assert!(i1 != i2);
        assert!(i1 != i3);
        assert!(i2 != i3);

        assert!(strings.get(i1) != strings.get(i2));
        assert!(strings.get(i1) != strings.get(i3));
        assert!(strings.get(i2) != strings.get(i3));
    }

    #[test]
    fn returns_same_id_for_same_strings() {
        let mut strings = StringArena::with_capacity(1);

        let i1 = strings.intern("a".as_bytes());
        let i2 = strings.intern("a".as_bytes());

        assert!(strings.has(i1));
        assert!(strings.has(i2));

        assert!(i1 == i2);

        assert!(strings.get(i1) == strings.get(i2));
    }
}
