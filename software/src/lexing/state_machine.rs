use std::collections::HashMap;

use crate::grammar::{
    rules_to_map,
    types::{GrammarNode, Rule},
};

/// ## Deterministic Finite Automaton ≈ Finite state machine
/// A state machine that systematically reads characters and changes its state based on it
pub struct LexerMatcher {
    rules: HashMap<String, GrammarNode>,
}

// Make a list of valid identifier characters
// This list does not include numbers because numbers are not valid as the first character
const IDENTIFIER_CHARS: [char; 26 * 2 + 1] = {
    let mut arr = [' '; 53];
    let mut i = 0;
    while i < 26 {
        let c = (b'a' + i as u8) as char;
        // Lowercase
        arr[i] = c;
        // Uppercase
        arr[i + 26] = c.to_ascii_uppercase();
        i += 1;
    }
    arr[52] = '_';
    arr
};

impl LexerMatcher {
    /// Create a new DFA instance
    pub fn new(rules: Vec<Rule>) -> Self {
        let hash_map_rules = rules_to_map(rules);
        Self {
            rules: hash_map_rules,
        }
    }

    /// Finds the next best match using a greedy algorithm
    pub fn next_best_match(&self, working_string: String) {}
}

/// These tests focus on the capability to identify single tokens
#[cfg(test)]
mod tests {}
