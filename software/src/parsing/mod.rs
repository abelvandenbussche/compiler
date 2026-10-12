//! This module uses the rules from the [EBNF module](crate::grammar) to generate the structure of
//! the source code.

use crate::grammar::types::RuleMap;

use super::lexing::Token;

/// Represents the combination of the `grammar` rules to the `Lexer` tokens
#[derive(Debug, PartialEq, Eq)]
pub enum MatchNode {
    Token(Token),
    Rule {
        name: String,
        children: Vec<MatchNode>,
    },
}

/// A struct that matches higher level parsing rules on the tokens that are produced by the lexer
/// this is also responsible for integrating the functionality of the language
pub struct ParserMatcher {
    /// The syntax tree that the parser builds up
    pub tree: MatchNode,
    /// A map of all rules so it can find references
    rules: RuleMap,
}

impl ParserMatcher {
    /// Creates a new `ParserMatcher`
    pub fn new(rules: RuleMap) -> Self {
        Self {
            tree: MatchNode::Rule {
                name: "program".to_owned(),
                children: vec![],
            },
            rules,
        }
    }
    /// Tries to match the next given token and build out its tree
    pub fn match_token(&mut self, token: Token) {}
}

#[cfg(test)]
mod tests {}
