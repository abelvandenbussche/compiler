use super::Token;

/// ## Deterministic Finite Automaton
/// A state machine that systematically reads characters and changes state based on it
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
        // TODO:
        // Implement advance method
        unimplemented!();
    }
}
