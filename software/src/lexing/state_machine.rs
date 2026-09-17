use crate::lexing::Literal;

use super::{
    ReservedKeyword,
    Token::{self, *},
};
use std::str::FromStr;

/// ## Deterministic Finite Automaton ≈ Finite state machine
/// A state machine that systematically reads characters and changes its state based on it
pub struct DFA {
    state: Option<Token>,
    current_char: char,
    next_char: char,
}

// Make a list of valid identifier characters
// This list does not include numbers because numbers are not valid as the first character
const IDENTIFIER_CHARS: [char; 26 * 2 + 1] = {
    let mut arr = [' '; 53];
    let mut i = 0;
    while i < 26 {
        let c = (b'a' + i as u8) as char;
        // Lowercase
        arr[i] = c;
        // Uppercase
        arr[i + 26] = c.to_ascii_uppercase();
        i += 1;
    }
    arr[52] = '_';
    arr
};

impl DFA {
    /// Create a new DFA instance
    pub fn new(first_char: char) -> Self {
        Self {
            state: None,
            // This will get replaced the first call of advance
            current_char: ' ',
            next_char: first_char,
        }
    }

    /// Resets the state machine to its original state and clears its read string
    pub fn reset(&mut self) {
        self.state = None;
    }

    /// Consumes the next character and returns wether or not it has reached a final token
    pub fn advance(&mut self, next_char: char) -> Option<Token> {
        // Switching it around
        self.current_char = self.next_char;
        self.next_char = next_char;

        println!("char: {}", self.current_char);
        if let Some(state_token) = &mut self.state {
            match state_token {
                Equals => {
                    if self.current_char == '=' {
                        return Some(EqualsEquals);
                    } else {
                        return Some(Equals);
                    }
                }
                Identifier(current_ident) => {
                    current_ident.push(self.current_char);
                    if !IDENTIFIER_CHARS.contains(&self.next_char) {
                        // End of identifier
                        // Checking if it is a reserved keyword
                        return if let Ok(keyword) = ReservedKeyword::from_str(&current_ident) {
                            Some(Keyword(keyword))
                        } else if *current_ident == "false" {
                            Some(Literal(Literal::Bool(false)))
                        } else if *current_ident == "true" {
                            Some(Literal(Literal::Bool(true)))
                        } else {
                            self.state.take()
                        };
                    }
                }
                Literal(variant) => match variant {
                    Literal::Int(int) => {
                        if let Some(new_digit) = self.current_char.to_digit(10) {
                            *int = *int * 10 + new_digit as i128;
                        } else if self.current_char == '.' {
                            println!("float found");
                            self.state = Some(Literal(Literal::Float(*int as f64)));
                            println!("float: {:?}", self.state);
                        } else {
                            return self.state.take();
                        }

                        if !self.next_char.is_digit(10) {
                            return self.state.take();
                        }
                    }
                    Literal::Float(float) => {
                        if self.current_char.is_digit(10) {
                            let mut float_string =
                                format!("{:?}", float).trim_matches('0').to_owned();
                            // Stripping trailing zeros
                            float_string.push(self.current_char);
                            // Safe to unwrap because of above checks
                            *float = f64::from_str(&float_string).unwrap();
                            if !self.next_char.is_digit(10) {
                                return self.state.take();
                            }
                        }
                    }
                    Literal::String(string) => {
                        if self.current_char == '"' {
                            return self.state.take();
                        } else {
                            string.push(self.current_char);
                        }
                    }
                    _ => todo!(),
                },
                _ => {}
            }
        } else {
            self.state = match self.current_char {
                '=' => Some(Equals),
                '"' => Some(Literal(Literal::String(String::new()))),
                '{' => return Some(LBrace),
                '}' => return Some(RBrace),
                '[' => return Some(LBracket),
                ']' => return Some(RBracket),
                '(' => return Some(LParentheses),
                ')' => return Some(RParentheses),
                ';' => return Some(Semicolon),
                ':' => return Some(Colon),
                ' ' => None,
                c => {
                    if IDENTIFIER_CHARS.contains(&c) {
                        Some(Identifier(String::from(c)))
                    } else if let Some(digit) = self.current_char.to_digit(10) {
                        let value = Some(Literal(Literal::Int(digit as i128)));
                        if self.next_char.is_digit(10) || self.next_char == '.' {
                            value
                        } else {
                            return value;
                        }
                    } else {
                        None
                    }
                }
            }
        }
        None
    }
    /// Similar to the `advance` method but does not take a char instead giving it an whitespace
    /// character
    ///
    /// This should be called when at the end of a source file
    pub fn finish(&mut self) -> Option<Token> {
        self.advance(' ')
    }
}

#[cfg(test)]
mod tests {

    use super::super::{
        ReservedKeyword::{self, *},
        Token::{self, *},
    };
    use super::DFA;

    /// A helper function to test the state machine\
    /// Feeds a string into the state machine and returns its first not `None` output or none if it
    /// did not give any
    fn feed_string(string: &str) -> Option<Token> {
        let mut chars_iter = string.chars();
        let first_char = chars_iter.next().unwrap();
        let mut dfa = DFA::new(first_char);
        for c in chars_iter {
            let res = dfa.advance(c);
            if res.is_some() {
                return res;
            }
        }
        dfa.finish()
    }

    #[test]
    fn very_simple_lexing() {
        assert_eq!(feed_string(" = "), Some(Equals));
        assert_eq!(feed_string(" == "), Some(EqualsEquals));
    }
    #[test]
    fn lex_keywords_and_identifiers() {
        assert_eq!(
            feed_string(&format!("{} ", ReservedKeyword::For)),
            Some(Keyword(ReservedKeyword::For))
        );
        assert_eq!(
            feed_string(&format!("{} ", VariableDeclaration)),
            Some(Keyword(VariableDeclaration))
        );
        assert_eq!(feed_string("myvar "), Some(Identifier("myvar".to_owned())));
        assert_eq!(
            feed_string(&format!("{}test ", VariableDeclaration)),
            Some(Identifier(format!("{}test", VariableDeclaration)))
        );
    }
    #[test]
    fn lex_brackets() {
        assert_eq!(feed_string("{"), Some(LBrace));
        assert_eq!(feed_string("}"), Some(RBrace));
        assert_eq!(feed_string("("), Some(LParentheses));
        assert_eq!(feed_string(")"), Some(RParentheses));
        assert_eq!(feed_string("["), Some(LBracket));
        assert_eq!(feed_string("]"), Some(RBracket));
        assert_eq!(feed_string(";"), Some(Semicolon));
        assert_eq!(feed_string(":"), Some(Colon));
    }

    #[test]
    fn lex_literals() {
        assert_eq!(
            feed_string("\"Hello world\""),
            Some(Literal(crate::lexing::Literal::String(
                "Hello world".to_owned()
            )))
        );
        assert_eq!(
            feed_string("10"),
            Some(Literal(crate::lexing::Literal::Int(10)))
        );
        assert_eq!(
            feed_string("2.5"),
            Some(Literal(crate::lexing::Literal::Float(2.5)))
        );
        assert_eq!(
            feed_string("true"),
            Some(Literal(crate::lexing::Literal::Bool(true)))
        );
        assert_eq!(
            feed_string("false"),
            Some(Literal(crate::lexing::Literal::Bool(false)))
        );
    }
}
