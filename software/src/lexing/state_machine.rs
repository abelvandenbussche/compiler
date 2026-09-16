use super::{
    ReservedKeyword,
    Token::{self, *},
};
use std::str::FromStr;

/// ## Deterministic Finite Automaton ≈ Finite state machine
/// A state machine that systematically reads characters and changes its state based on it
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
        if let Some(state_token) = &mut self.state {
            match state_token {
                Equals => {
                    if next_char == '=' {
                        return Some(EqualsEquals);
                    } else {
                        return Some(Equals);
                    }
                }
                Identifier(current_ident) => {
                    if next_char == ' ' {
                        // End of identifier
                        // Checking if it is a reserved keyword
                        return Some(
                            if let Ok(keyword) = ReservedKeyword::from_str(&current_ident) {
                                Keyword(keyword)
                            } else {
                                Identifier(current_ident.clone())
                            },
                        );
                    }
                    current_ident.push(next_char);
                }
                _ => {}
            }
        } else {
            self.state = match next_char {
                '=' => Some(Equals),
                '{' => return Some(LBrace),
                '}' => return Some(RBrace),
                '[' => return Some(LBracket),
                ']' => return Some(RBracket),
                '(' => return Some(LParentheses),
                ')' => return Some(RParentheses),
                ' ' => None,
                c => Some(Identifier(String::from(c))),
            }
        }
        None
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
        let mut dfa = DFA::new();
        for c in string.chars() {
            let res = dfa.advance(c);
            if res.is_some() {
                return res;
            }
        }
        None
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
    }
}
