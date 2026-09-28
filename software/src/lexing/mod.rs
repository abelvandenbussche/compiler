//! This module converts the characters read from a file into tokens that are more easily parsed by
//! the [crate::parsing]

use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use crate::{common::Literal, lexing::state_machine::DFA};

mod state_machine;

#[derive(Debug, PartialEq)]
pub enum Token {
    /// Things like variable or function names
    Identifier(String),
    /// Any literal
    Literal(Literal),
    /// ==
    EqualsEquals,
    /// =
    Equals,
    /// {
    LBrace,
    /// }
    RBrace,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// (
    LParentheses,
    /// )
    RParentheses,
    /// ;
    Semicolon,
    /// :
    Colon,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    MultiplicationSign,
    /// /
    DivisionSign,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::DivisionSign => "/",
            Self::MultiplicationSign => "*",
            Self::Equals => "=",
            Self::EqualsEquals => "==",
            Self::LBrace => "{",
            Self::RBrace => "}",
            Self::LBracket => "[",
            Self::RBracket => "]",
            Self::LParentheses => "(",
            Self::RParentheses => ")",
            Self::Semicolon => ";",
            Self::Colon => ":",
            Self::Identifier(ident) => ident,
            Self::Literal(lit) => &lit.to_string(),
        };
        f.write_str(stringified)
    }
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
    pub fn new(mut file: File) -> io::Result<Self> {
        let mut buf: [u8; 1] = [0];
        file.read_exact(&mut buf)?;
        let first_char = buf
            .iter()
            .map(|e| *e as char)
            .next()
            .expect("Failed to take first char");
        Ok(Self {
            reader: BufReader::new(file),
            dfa: DFA::new(first_char),
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
mod tests {
    use super::{Lexer, Literal, Token::*};
    use std::{fs::File, path::PathBuf};
    const TEST_FILE_DIR: &str = "test_files";
    #[test]
    fn token_test1() {
        let test_dir = PathBuf::from(format!("{TEST_FILE_DIR}/token_test1.txt"));
        let mut lexer = Lexer::new(File::open(test_dir).unwrap()).unwrap();
        let mut tokens = vec![];
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        assert_eq!(
            tokens,
            vec![
                Identifier("let".to_owned()),
                Identifier("foo".to_owned()),
                Equals,
                Literal(Literal::Int(1)),
                Semicolon,
                Identifier("let".to_owned()),
                Identifier("bar".to_owned()),
                Colon,
                Identifier("int".to_owned()),
                Equals,
                Literal(Literal::Int(2)),
                Semicolon,
            ]
        )
    }
    #[test]
    fn token_test2() {
        let test_dir = PathBuf::from(format!("{TEST_FILE_DIR}/token_test2.txt"));
        let mut lexer = Lexer::new(File::open(test_dir).unwrap()).unwrap();
        let mut tokens = vec![];
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        assert_eq!(
            tokens,
            vec![
                Identifier("let".to_owned()),
                Identifier("my_var".to_owned()),
                Equals,
                Literal(Literal::String("Hello world".to_owned())),
                Semicolon,
                Identifier("if".to_owned()),
                Identifier("my_var".to_owned()),
                EqualsEquals,
                Literal(Literal::String("Hello world".to_owned())),
                LBrace,
                Identifier("print".to_owned()),
                LParentheses,
                Literal(Literal::String("Yep".to_owned())),
                RParentheses,
                Semicolon,
                RBrace,
                Identifier("else".to_owned()),
                LBrace,
                Identifier("print".to_owned()),
                LParentheses,
                Literal(Literal::String("Nope".to_owned())),
                RParentheses,
                Semicolon,
                RBrace
            ]
        );
    }
    #[test]
    fn token_test_operations() {
        let test_dir = PathBuf::from(format!("{TEST_FILE_DIR}/token_test_operations.txt"));
        let mut lexer = Lexer::new(File::open(test_dir).unwrap()).unwrap();
        let mut tokens = vec![];
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        assert_eq!(
            tokens,
            vec![
                Literal(Literal::Int(1)),
                Plus,
                LParentheses,
                Literal(Literal::Int(2)),
                MultiplicationSign,
                Literal(Literal::Int(4)),
                RParentheses,
                DivisionSign,
                Literal(Literal::Int(6)),
                Semicolon,
                Literal(Literal::Int(5)),
                DivisionSign,
                Literal(Literal::Int(4)),
                DivisionSign,
                Literal(Literal::Int(2)),
                Plus,
                LParentheses,
                Literal(Literal::Int(5)),
                MultiplicationSign,
                Literal(Literal::Int(30)),
                Minus,
                Literal(Literal::Int(15)),
                RParentheses,
            ]
        )
    }
}
