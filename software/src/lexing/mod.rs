//! This module converts the characters read from a file into tokens that are more easily parsed by
//! the [crate::parsing]

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use crate::{common::Literal, lexing::state_machine::LexerMatcher};

mod state_machine;

#[derive(Debug, PartialEq)]
/// A token that represents a match between characters and a rule
pub struct Token {
    pub head: String,
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
    pub fn new(mut file: File) -> io::Result<Self> {
        let mut buf: [u8; 1] = [0];
        file.read_exact(&mut buf)?;
        // HACK: this should actually contain rules and is just to appease the compiler
        let rules = vec![];
        let first_char = buf
            .iter()
            .map(|e| *e as char)
            .next()
            .expect("Failed to take first char");
        Ok(Self {
            reader: BufReader::new(file),
            dfa: LexerMatcher::new(first_char, rules),
            current_line: vec![],
        })
    }
    /// Lexes the next token
    pub fn next_token(&mut self) -> Option<Token> {
        loop {
            if let Some(next_char) = self.next_char() {
                let result = self.dfa.advance(next_char);
                if let Some(token) = result {
                    self.dfa.reset();
                    return Some(token);
                }
            } else {
                break;
            }
        }
        self.dfa.finish()
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
