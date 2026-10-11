//! This module uses the rules from the [EBNF module](crate::grammar) to generate the structure of
//! the source code.

use super::lexing::Token;

/// Represents the combination of the `grammar` rules to the `Lexer` tokens
enum MatchNode {
    Token(Token),
    Rule {
        name: String,
        children: Vec<MatchNode>,
    },
}
