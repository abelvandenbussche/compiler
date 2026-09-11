use std::{fs::File, io::BufReader};

use crate::lexing::state_machine::DFA;

mod state_machine;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Keyword(ReservedKeyword),
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
}
impl Lexer {
    /// Creates a new lexer
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
        }
    }
    /// Lexes the next token
    pub fn next_token(&mut self) -> Option<Token> {
        unimplemented!();
    }
}
/// Internal test module for tokenizing
const TEST_FILE_DIR: &str = "test_files";
mod tests {
    fn token_test1() {}
}
