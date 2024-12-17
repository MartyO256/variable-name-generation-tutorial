use crate::strings::{StringArena, StringId};

pub trait AdmissibleVariableNameGenerator {
    fn generate_admissible_name<F: Fn(StringId) -> bool>(
        &mut self,
        strings: &mut StringArena,
        is_admissible: F,
    ) -> StringId;
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

impl AdmissibleVariableNameGenerator for VariableNameGenerator {
    fn generate_admissible_name<F: Fn(StringId) -> bool>(
        &mut self,
        strings: &mut StringArena,
        is_admissible: F,
    ) -> StringId {
        let n = self.bases.len();
        let mut attempts = 0;
        let mut suffix = 0;
        loop {
            let mut candidate = self.bases[attempts % n].to_vec();
            if suffix > 0 {
                candidate.extend(suffix.to_string().as_bytes());
            }
            let id = strings.intern(&candidate);
            if is_admissible(id) {
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
    use std::collections::HashSet;

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
            let actual = generator
                .generate_admissible_name(&mut strings, |string| !claimed.contains(&string));
            assert!(!claimed.contains(&actual));
            assert!(expected == actual);
            claimed.insert(actual);
        }
    }
}
