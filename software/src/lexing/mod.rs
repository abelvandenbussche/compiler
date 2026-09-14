use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::lexing::state_machine::DFA;

mod state_machine;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Keyword(ReservedKeyword),
    EqualsEquals,
    Equals,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReservedKeyword {
    Function,
    VariableDeclaration,
    While,
    For,
}

/// A lexer that can emit a stream of tokens instead of loading a full files worth of tokens in to
/// memory
pub struct Lexer {
    reader: BufReader<File>,
    dfa: DFA,
    current_line: Vec<char>,
}
impl Lexer {
    /// Creates a new lexer
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
            dfa: DFA::new(),
            current_line: vec![],
        }
    }
    /// Lexes the next token
    pub fn next_token(&mut self) -> Option<Token> {
        let next_char = self.next_char()?;
        self.dfa.advance(next_char)
    }

    /// Returns the next character of the file
    fn next_char(&mut self) -> Option<char> {
        if self.current_line.len() == 0 {
            let mut new_line = String::new();
            let result = self
                .reader
                .read_line(&mut new_line)
                .expect("Failed to read file");
            if result == 0 {
                // Probably eof
                return None;
            }
            self.current_line = new_line.chars().collect();
        }
        // Returning the next char
        Some(self.current_line.remove(0))
    }
}
/// Internal test module for tokenizing
mod tests {
    const TEST_FILE_DIR: &str = "test_files";
    fn token_test1() {}
}
