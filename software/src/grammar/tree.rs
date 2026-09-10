use crate::grammar::lexer::EBNFToken::*;

use super::lexer::EBNFToken;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// Literal characters
    Literal(String),
    /// Reference to another rule
    Reference(String),
    /// A sequence of nodes
    Sequence(Vec<Node>),
    /// Optional node
    Optional(Box<Node>),
    /// 1 or more of a node
    ZeroOrMore(Box<Node>),
    /// Groupes multiple nodes together
    Grouping(Box<Node>),
    /// Choice of a single node
    Choice(Vec<Node>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rule {
    head: String,
    body: Option<Node>,
}

/// Parses a stream of tokens into a tree structure
pub fn parse_tokens(mut tokens: Vec<EBNFToken>) -> Rule {
    // A rule should always start with an identifier
    let body_tokens = tokens.split_off(tokens.iter().position(|i| *i == Equals).unwrap());
    let head = match tokens.into_iter().next() {
        Some(EBNFToken::Identifier(s)) => Rule {
            head: s,
            body: Some(generate_rule_body(&body_tokens)),
        },
        Some(other) => panic!("Unexpected token: {:?}", other),
        None => panic!("Slice does not contain any elements"),
    };
    head
}

fn generate_rule_body(token_stream: &[EBNFToken]) -> Node {
    let mut place_holder_node;
    for (idx, token) in token_stream.iter().enumerate() {
        match token {
            LBrace | LBracket | LParentheses => {
                if let Some(match_location) = find_matching_delimiter(idx, &token_stream) {
                    let contained_slice = &token_stream[idx + 1..match_location];
                    println!("slice: {:?}", contained_slice);
                    place_holder_node = generate_rule_body(contained_slice);
                } else {
                    panic!("Non matching delimiter at {idx} in {:?}", token_stream);
                }
            }
            _ => {}
        }
    }
    unimplemented!();
    place_holder_node
}

/// Finds the position of the matching open delimiter given an index of an open delimiter
fn find_matching_delimiter(left_idx: usize, list: &[EBNFToken]) -> Option<usize> {
    let mut depth = 0;
    for (idx, token) in list.iter().enumerate().skip(left_idx) {
        match token {
            LBracket | LBrace | LParentheses => depth += 1,
            RBracket | RBrace | RParenthese => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            return Some(idx);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::grammar::{
        lexer::EBNFToken::{self, *},
        tree::{
            Node::{self, *},
            Rule, find_matching_delimiter, parse_tokens,
        },
    };

    #[test]
    fn finding_matching_closing_delimiter() {
        let list = vec![
            LBracket,
            LBracket,
            LBrace,
            RBrace,
            LParentheses,
            LBrace,
            RBrace,
            RParenthese,
            RBracket,
        ];
        assert_eq!(find_matching_delimiter(1, &list), Some(8));
        assert_eq!(find_matching_delimiter(2, &list), Some(3));
        assert_eq!(find_matching_delimiter(4, &list), Some(7));
        assert_eq!(find_matching_delimiter(0, &list), None);
    }
    #[test]
    fn very_simple_tree_from_tokens() {
        // foo = "bar";
        let tokens = vec![
            Identifier("foo".to_owned()),
            Equals,
            EBNFToken::Literal("bar".to_owned()),
            Semicolon,
        ];
        let expected = Rule {
            head: "foo".to_owned(),
            body: Some(Node::Literal("bar".to_owned())),
        };
        assert_eq!(parse_tokens(tokens), expected);
    }
    #[test]
    fn simple_tree_from_tokens() {
        // foo = ["hello" | "world"], {digit};
        let tokens = vec![
            Identifier("foo".to_owned()),
            Equals,
            LBracket,
            EBNFToken::Literal("hello".to_owned()),
            Pipe,
            EBNFToken::Literal("world".to_owned()),
            RBracket,
            Comma,
            LBrace,
            Identifier("digit".to_owned()),
            RBrace,
            Semicolon,
        ];
        let expected_output = Rule {
            head: "foo".to_owned(),
            body: Some(Sequence(vec![
                Optional(Box::new(Choice(vec![
                    Node::Literal("hello".to_owned()),
                    Node::Literal("world".to_owned()),
                ]))),
                ZeroOrMore(Box::new(Reference("digit".to_owned()))),
            ])),
        };
        assert_eq!(parse_tokens(tokens), expected_output);
    }
    #[test]
    fn tree_from_tokens() {
        // foo = ["1"], bar | fizz, {"2", "3"};
        let tokens = vec![
            Identifier("foo".to_owned()),
            Equals,
            LBracket,
            EBNFToken::Literal("1".to_owned()),
            RBracket,
            Comma,
            Identifier("bar".to_owned()),
            Pipe,
            Identifier("fizz".to_owned()),
            Comma,
            LBrace,
            EBNFToken::Literal("2".to_owned()),
            Comma,
            EBNFToken::Literal("3".to_owned()),
            RBracket,
            Semicolon,
        ];
        let expected_rule = Rule {
            head: "foo".to_owned(),
            body: Some(Sequence(vec![
                Optional(Box::new(Node::Literal("1".to_owned()))),
                Choice(vec![
                    Reference("bar".to_owned()),
                    Reference("fizz".to_owned()),
                ]),
                ZeroOrMore(Box::new(Sequence(vec![
                    Node::Literal("2".to_owned()),
                    Node::Literal("3".to_owned()),
                ]))),
            ])),
        };
        assert_eq!(parse_tokens(tokens), expected_rule);
    }
}
