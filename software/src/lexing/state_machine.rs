use super::Token;

/// The different possible states of the DFA
enum State {}

/// ## Deterministic Finite Automaton
/// A state machine that systematically reads characters and changes state based on it
pub struct DFA {
    state: State,
    read: String,
}

impl DFA {
    /// Create a new DFA instance
    pub fn new() -> Self {
        // TODO:
        // Make an initial DFA
        unimplemented!();
    }

    /// Resets the state machine to its original state and clears its read string
    pub fn reset(&mut self) {
        // TODO:
        // Implement this method, similair to the new method but not creating a new instance
        unimplemented!()
    }

    /// Consumes the next character and returns wether or not it has reached a final token
    pub fn advance(&mut self, next_char: char) -> Option<Token> {
        // TODO:
        // Implement advance method
        unimplemented!();
    }
}
