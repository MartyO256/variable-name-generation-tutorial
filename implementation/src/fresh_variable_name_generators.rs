use std::collections::HashSet;

use crate::strings::{StringArena, StringId};

pub trait FreshVariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId;
}

pub struct SuffixVariableNameGenerator {}

impl SuffixVariableNameGenerator {
    pub fn new() -> SuffixVariableNameGenerator {
        SuffixVariableNameGenerator {}
    }
}

impl Default for SuffixVariableNameGenerator {
    fn default() -> SuffixVariableNameGenerator {
        SuffixVariableNameGenerator::new()
    }
}

impl FreshVariableNameGenerator for SuffixVariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId {
        let mut suffix = 1;
        let mut id = strings.intern(b"x");
        while claimed.contains(&id) {
            let mut candidate = b"x".to_vec();
            candidate.extend(suffix.to_string().as_bytes());
            id = strings.intern(&candidate);
            suffix += 1;
        }
        id
    }
}
