//! This module converts the characters read from a file into tokens that are more easily parsed by
//! the [crate::parsing]

use crate::{grammar::types::Rule, lexing::matching::LexerMatcher};

mod matching;

#[derive(Debug, PartialEq, Eq, Clone)]
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
    matcher: LexerMatcher,
    file_contents: String,
}
impl Lexer {
    /// Creates a new lexer
    pub fn new(file_contents: String, lexing_rules: Vec<Rule>) -> Self {
        Self {
            matcher: LexerMatcher::new(lexing_rules),
            file_contents,
        }
    }
    /// Lexes the next token
    pub fn next_token(&mut self) -> Option<Token> {
        let next_token = self.matcher.next_best_match(&self.file_contents);
        if let Some(token) = &next_token {
            self.file_contents = self.file_contents[token.body.len()..].to_owned();
        }
        next_token
    }
}
/// Internal test module for tokenizing
/// These focus on files and the larger whole rather than individual token lexing
#[cfg(test)]
mod tests {
    use crate::{
        grammar::types::{GrammarNode, Rule},
        lexing::{Lexer, Token},
    };

    #[test]
    fn simple_lexing() {
        let char_rule = Rule {
            head: "CHAR".to_owned(),
            body: Some(crate::grammar::types::GrammarNode::Choice(vec![
                GrammarNode::Literal("a".to_owned()),
                GrammarNode::Literal("b".to_owned()),
                GrammarNode::Literal("c".to_owned()),
                GrammarNode::Literal("d".to_owned()),
                GrammarNode::Literal("e".to_owned()),
                GrammarNode::Literal("f".to_owned()),
                GrammarNode::Literal("g".to_owned()),
                GrammarNode::Literal("h".to_owned()),
                GrammarNode::Literal("i".to_owned()),
                GrammarNode::Literal("j".to_owned()),
                GrammarNode::Literal("k".to_owned()),
                GrammarNode::Literal("l".to_owned()),
                GrammarNode::Literal("m".to_owned()),
                GrammarNode::Literal("n".to_owned()),
                GrammarNode::Literal("o".to_owned()),
                GrammarNode::Literal("p".to_owned()),
                GrammarNode::Literal("q".to_owned()),
                GrammarNode::Literal("r".to_owned()),
                GrammarNode::Literal("s".to_owned()),
                GrammarNode::Literal("t".to_owned()),
                GrammarNode::Literal("u".to_owned()),
                GrammarNode::Literal("v".to_owned()),
                GrammarNode::Literal("w".to_owned()),
                GrammarNode::Literal("x".to_owned()),
                GrammarNode::Literal("y".to_owned()),
                GrammarNode::Literal("z".to_owned()),
                GrammarNode::Literal(" ".to_owned()),
            ])),
        };
        let string_rule = Rule {
            head: "STRING".to_owned(),
            body: Some(crate::grammar::types::GrammarNode::Sequence(vec![
                GrammarNode::Literal("\"".to_owned()),
                GrammarNode::ZeroOrMore(Box::new(GrammarNode::Reference("CHAR".to_owned()))),
                GrammarNode::Literal("\"".to_owned()),
            ])),
        };
        let rules = vec![char_rule, string_rule];
        let contents = "\"hello world\"\"abcdefghijklmnopqrstuvwxyz\"".to_owned();
        let mut lexer = Lexer::new(contents, rules);
        let mut tokens = vec![];
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        assert_eq!(
            tokens,
            vec![
                Token {
                    head: "STRING".to_owned(),
                    body: "\"hello world\"".to_owned()
                },
                Token {
                    head: "STRING".to_owned(),
                    body: "\"abcdefghijklmnopqrstuvwxyz\"".to_owned(),
                }
            ]
        );
    }
}
