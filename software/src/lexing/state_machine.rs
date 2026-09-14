use crate::lexing::Token::{Equals, EqualsEquals, Identifier};

use super::Token;

/// ## Deterministic Finite Automaton ≈ Finite state machine
/// A state machine that systematically reads characters and changes its state based on it
pub struct DFA {
    state: Option<Token>,
}

impl DFA {
    /// Create a new DFA instance
    pub fn new() -> Self {
        Self { state: None }
    }

    /// Resets the state machine to its original state and clears its read string
    pub fn reset(&mut self) {
        self.state = None;
    }

    /// Consumes the next character and returns wether or not it has reached a final token
    pub fn advance(&mut self, next_char: char) -> Option<Token> {
        if let Some(state_token) = &self.state {
            match state_token {
                Equals => {
                    if next_char == '=' {
                        return Some(EqualsEquals);
                    } else {
                        return Some(Equals);
                    }
                }
                _ => {}
            }
        } else {
            self.state = match next_char {
                '=' => Some(Equals),
                ' ' => None,
                c => Some(Identifier(String::from(c))),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::Token::{self, *};
    use super::DFA;

    /// A helper function to test the state machine\
    /// Feeds a string into the state machine and returns its first not `None` output or none if it
    /// did not give any
    fn feed_string(string: &str) -> Option<Token> {
        let mut dfa = DFA::new();
        for c in string.chars() {
            let res = dfa.advance(c);
            if res.is_some() {
                return res;
            }
        }
        None
    }

    #[test]
    fn very_simple_lexing() {
        assert_eq!(feed_string(" = "), Some(Equals));
        assert_eq!(feed_string(" == "), Some(EqualsEquals));
    }
}
