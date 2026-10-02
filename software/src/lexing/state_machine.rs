use crate::grammar::types::Rule;

use super::Token;

/// ## Deterministic Finite Automaton ≈ Finite state machine
/// A state machine that systematically reads characters and changes its state based on it
pub struct LexerMatcher {
    working_string: String,
    rules: Vec<Rule>,
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
    pub fn new(first_char: char, rules: Vec<Rule>) -> Self {
        Self {
            working_string: String::new(),
            rules,
        }
    }

    /// Resets the state machine to its original state and clears its read string
    pub fn reset(&mut self) {
        self.working_string = String::new();
    }

    /// Consumes the next character and returns wether or not it has reached a final token
    pub fn advance(&mut self, next_char: char) -> Option<Token> {
        self.working_string.push(next_char);
        None
    }
    /// Similar to the [crate::lexing::state_machine::DFA::advance] method but does not take a char
    /// instead giving it an whitespace character
    ///
    /// This should be called when at the end of a source file
    pub fn finish(&mut self) -> Option<Token> {
        self.advance(' ')
    }
}

/// These tests focus on the capability to identify single tokens
#[cfg(test)]
mod tests {

    use crate::grammar::types::{GrammarNode, Rule};

    use super::super::Token;
    use super::LexerMatcher;

    /// A helper function to test the state machine\
    /// Feeds a string into the state machine and returns its first not [None] output or none if it
    /// did not give any
    fn feed_string(string: &str, rules: Vec<Rule>) -> Option<Token> {
        let mut chars_iter = string.chars();
        let first_char = chars_iter.next().unwrap();
        let mut dfa = LexerMatcher::new(first_char, rules);
        for c in chars_iter {
            let res = dfa.advance(c);
            if res.is_some() {
                return res;
            }
        }
        dfa.finish()
    }

    #[test]
    fn very_simple_lexing() {
        let equals_rule = Rule {
            head: "EQUALS".to_owned(),
            body: Some(GrammarNode::Literal("=".to_owned())),
        };
        let comparison_rule = Rule {
            head: "COMPARISON_OPERATOR".to_owned(),
            body: Some(GrammarNode::Literal("==".to_owned())),
        };
        let rules = vec![equals_rule, comparison_rule];
        assert_eq!(
            feed_string("=", rules.clone()),
            Some(Token {
                head: "EQUALS".to_owned(),
                body: "=".to_string(),
            })
        );
        assert_eq!(
            feed_string("==", rules),
            Some(Token {
                head: "COMPARISON_OPERATOR".to_owned(),
                body: "==".to_owned(),
            })
        );
    }
}
