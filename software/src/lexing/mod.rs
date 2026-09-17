use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    str::FromStr,
};

use crate::lexing::{
    ReservedKeyword::{For, Return, VariableDeclaration, While},
    state_machine::DFA,
};

mod state_machine;

#[derive(Debug, PartialEq)]
pub enum Token {
    /// Things like variable or function names
    Identifier(String),
    /// Reserved keywords like "for" or "while"
    Keyword(ReservedKeyword),
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
}

#[derive(Debug, PartialEq)]
pub enum ReservedKeyword {
    Function,
    VariableDeclaration,
    While,
    For,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Char(char),
    String(String),
    Int(i128),
    Float(f64),
    Bool(bool),
}

// Allows for the conversion from enum member to string
// This automatically implements the ToString trait and thus allows conversion to String
impl Display for ReservedKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Function => "fn",
            VariableDeclaration => "let",
            While => "while",
            For => "for",
            Return => "return",
        };
        f.write_str(output)
    }
}

// Allow for the conversion from string to keyword enum member
impl FromStr for ReservedKeyword {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fn" => Ok(Self::Function),
            "let" => Ok(VariableDeclaration),
            "while" => Ok(While),
            "for" => Ok(For),
            "return" => Ok(Return),
            _ => Err("String is not a valid keyword".to_owned()),
        }
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
#[cfg(test)]
mod tests {
    use super::{
        Lexer, Literal,
        ReservedKeyword::{self, *},
        Token::*,
    };
    use std::{fs::File, path::PathBuf, str::FromStr};
    const TEST_FILE_DIR: &str = "test_files";
    #[test]
    fn keyword_parsing() {
        /// Tests if you can turn the keyword into a string and then turn it back into a keyword
        fn test_variant(variant: ReservedKeyword) {
            assert_eq!(ReservedKeyword::from_str(&variant.to_string()), Ok(variant));
        }
        test_variant(ReservedKeyword::For);
        test_variant(ReservedKeyword::While);
        test_variant(ReservedKeyword::VariableDeclaration);
        test_variant(ReservedKeyword::Function);
        test_variant(ReservedKeyword::Return);
    }
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
                Keyword(VariableDeclaration),
                Identifier("foo".to_owned()),
                Equals,
                Literal(Literal::Int(1)),
                Semicolon,
                Keyword(VariableDeclaration),
                Identifier("bar".to_owned()),
                Colon,
                Identifier("int".to_owned()),
                Equals,
                Literal(Literal::Int(2)),
                Semicolon,
            ]
        )
    }
}
