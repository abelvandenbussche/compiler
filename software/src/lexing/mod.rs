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
    dfa: LexerMatcher,
    file_contents: String,
}
impl Lexer {
    /// Creates a new lexer
    pub fn new(mut file_contents: String, lexing_rules: Vec<Rule>) -> io::Result<Self> {
        Ok(Self {
            dfa: LexerMatcher::new(lexing_rules),
            file_contents,
        })
    }
    /// Lexes the next token
    pub fn next_token(&mut self) -> Option<Token> {
        let next_token = self.dfa.next_best_match(&self.file_contents);
        if let Some(token) = &next_token {
            self.file_contents = self.file_contents[token.body.len()..].to_owned();
        }
        next_token
    }
}
/// Internal test module for tokenizing
/// These focus on files and the larger whole rather than individual token lexing
#[cfg(test)]
mod tests {}
