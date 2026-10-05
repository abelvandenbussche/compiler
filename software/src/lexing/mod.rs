//! This module converts the characters read from a file into tokens that are more easily parsed by
//! the [crate::parsing]

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use crate::{grammar::types::Rule, lexing::state_machine::LexerMatcher};

mod state_machine;

#[derive(Debug, PartialEq)]
/// A token that represents a match between characters and a rule
pub struct Token {
    /// Rule name
    pub head: String,
    /// Matched part
    pub body: String,
}

/// A lexer that can emit a stream of tokens instead of loading a full files worth of tokens in to
/// memory
pub struct Lexer {
    reader: BufReader<File>,
    dfa: LexerMatcher,
    current_line: Vec<char>,
}
impl Lexer {
    /// Creates a new lexer
    pub fn new(mut file: File, lexing_rules: Vec<Rule>) -> io::Result<Self> {
        let mut buf: [u8; 1] = [0];
        file.read_exact(&mut buf)?;
        Ok(Self {
            reader: BufReader::new(file),
            dfa: LexerMatcher::new(lexing_rules),
            current_line: vec![],
        })
    }
    /// Lexes the next token
    pub fn next_token(&mut self) -> Option<Token> {
        todo!()
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
/// These focus on files and the larger whole rather than individual token lexing
#[cfg(test)]
mod tests {}
