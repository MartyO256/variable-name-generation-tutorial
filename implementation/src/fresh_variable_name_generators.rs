use std::collections::HashSet;

use crate::strings::{StringArena, StringId};

pub trait FreshVariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId;
}

pub struct VariableNameGenerator {
    bases: Vec<Box<[u8]>>,
}

impl VariableNameGenerator {
    pub fn new() -> VariableNameGenerator {
        VariableNameGenerator {
            bases: vec![
                Box::from(b"x" as &[u8]),
                Box::from(b"y" as &[u8]),
                Box::from(b"z" as &[u8]),
            ],
        }
    }

    pub fn with_bases(bases: Vec<Box<[u8]>>) -> VariableNameGenerator {
        VariableNameGenerator { bases }
    }
}

impl Default for VariableNameGenerator {
    fn default() -> VariableNameGenerator {
        VariableNameGenerator::new()
    }
}

impl FreshVariableNameGenerator for VariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId {
        let n = self.bases.len();
        let mut attempts = 0;
        let mut suffix = 0;
        loop {
            let mut candidate = self.bases[attempts % n].to_vec();
            if suffix > 0 {
                candidate.extend(suffix.to_string().as_bytes());
            }
            let id = strings.intern(&candidate);
            if !claimed.contains(&id) {
                return id;
            }
            attempts += 1;
            if attempts % n == 0 {
                suffix += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_fresh_names() {
        let mut strings = StringArena::new();
        let mut generator = VariableNameGenerator::with_bases(vec![
            Box::from(b"x" as &[u8]),
            Box::from(b"y" as &[u8]),
            Box::from(b"z" as &[u8]),
        ]);
        let mut claimed = HashSet::new();
        for expected in vec![
            strings.intern(b"x"),
            strings.intern(b"y"),
            strings.intern(b"z"),
            strings.intern(b"x1"),
            strings.intern(b"y1"),
            strings.intern(b"z1"),
            strings.intern(b"x2"),
            strings.intern(b"y2"),
            strings.intern(b"z2"),
        ] {
            let actual = generator.fresh_name(&mut strings, &claimed);
            assert!(!claimed.contains(&actual));
            assert!(expected == actual);
            claimed.insert(actual);
        }
    }
}
