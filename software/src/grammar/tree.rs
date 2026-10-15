use crate::grammar::{
    lexer::EBNFToken::{Literal, *},
    types::{
        GrammarNode::{self, Choice, Grouping, Optional, Reference, Sequence, ZeroOrMore},
        Rule,
    },
};

use super::lexer::EBNFToken;

/// Parses a stream of tokens into a tree structure
pub fn parse_tokens(mut tokens: Vec<EBNFToken>) -> Rule {
    // A rule should always start with an identifier
    let body_tokens = tokens.split_off(tokens.iter().position(|i| *i == Equals).unwrap());
    match tokens.into_iter().next() {
        Some(EBNFToken::Identifier(s)) => Rule {
            head: s,
            // Also removes the leading equals
            body: Some(generate_rule_body(&body_tokens[1..body_tokens.len()])),
        },
        Some(other) => panic!("Unexpected token: {:?}", other),
        None => panic!("Slice does not contain any elements"),
    }
}

/// Takes in a stream of tokens and uses it to generate a `GrammarNode` that is the body of a rule
fn generate_rule_body(token_stream: &[EBNFToken]) -> GrammarNode {
    let mut nodes = vec![];
    let mut token_iter = token_stream.iter().enumerate();
    let mut choosing = false;
    while let Some((idx, token)) = token_iter.next() {
        let result = match token {
            LBrace | LBracket | LParentheses => {
                if let Some(match_location) = find_matching_delimiter(idx, token_stream) {
                    let contained_slice = &token_stream[idx + 1..match_location];
                    let result = generate_rule_body(contained_slice);
                    let product = match token {
                        LBrace => ZeroOrMore(Box::new(result)),
                        LBracket => Optional(Box::new(result)),
                        LParentheses => Grouping(Box::new(result)),
                        _ => unreachable!(),
                    };
                    // Skipping to the end of the slice in the current iterator
                    for _ in idx..match_location {
                        let _ = token_iter.next();
                    }
                    Some(product)
                } else {
                    panic!("Non matching delimiter at {idx} in {:?}", token_stream);
                }
            }
            Literal(s) => Some(GrammarNode::Literal(s.clone())),
            Identifier(s) => Some(Reference(s.clone())),
            Pipe => {
                choosing = true;
                // Moving the last node into this one
                if let Some(last_node) = nodes.pop() {
                    // Check if we are already building a choice
                    if let GrammarNode::Choice(_) = last_node {
                        Some(last_node)
                    } else {
                        Some(Choice(vec![last_node]))
                    }
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(node) = result {
            // Checking if the last element is a choice or not
            let possible_last = nodes.pop();
            if choosing && let Some(Choice(mut choice_list)) = possible_last {
                choice_list.push(node);
                choosing = false;
                nodes.push(Choice(choice_list));
            } else {
                // Re-add the last element if needed
                if let Some(last_node) = possible_last {
                    nodes.push(last_node);
                }
                nodes.push(node);
            }
        }
    }
    match nodes.len() {
        0 => panic!("No nodes to parse"),
        1 => nodes.remove(0),
        _ => Sequence(nodes),
    }
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
            GrammarNode::{self, *},
            Rule, find_matching_delimiter, generate_rule_body, parse_tokens,
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
            body: Some(GrammarNode::Literal("bar".to_owned())),
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
                    GrammarNode::Literal("hello".to_owned()),
                    GrammarNode::Literal("world".to_owned()),
                ]))),
                ZeroOrMore(Box::new(Reference("digit".to_owned()))),
            ])),
        };
        assert_eq!(parse_tokens(tokens), expected_output);
    }
    #[test]
    fn choice_tree() {
        // "1" | "2" | "3"
        let tokens = vec![
            EBNFToken::Literal("1".to_owned()),
            Pipe,
            EBNFToken::Literal("2".to_owned()),
            Pipe,
            EBNFToken::Literal("3".to_owned()),
            Pipe,
            EBNFToken::Literal("4".to_owned()),
        ];
        let expected = GrammarNode::Choice(vec![
            GrammarNode::Literal("1".to_owned()),
            GrammarNode::Literal("2".to_owned()),
            GrammarNode::Literal("3".to_owned()),
            GrammarNode::Literal("4".to_owned()),
        ]);
        assert_eq!(generate_rule_body(&tokens), expected);
    }
    #[test]
    fn mixed_choice_tree() {
        // "1" | {"2"} | "3"
        let tokens = vec![
            EBNFToken::Literal("1".to_owned()),
            Pipe,
            LBrace,
            EBNFToken::Literal("2".to_owned()),
            RBrace,
            Pipe,
            EBNFToken::Literal("3".to_owned()),
        ];
        let expected = GrammarNode::Choice(vec![
            GrammarNode::Literal("1".to_owned()),
            ZeroOrMore(Box::new(GrammarNode::Literal("2".to_owned()))),
            GrammarNode::Literal("3".to_owned()),
        ]);
        assert_eq!(generate_rule_body(&tokens), expected);
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
            RBrace,
            Semicolon,
        ];
        let expected_rule = Rule {
            head: "foo".to_owned(),
            body: Some(Sequence(vec![
                Optional(Box::new(GrammarNode::Literal("1".to_owned()))),
                Choice(vec![
                    Reference("bar".to_owned()),
                    Reference("fizz".to_owned()),
                ]),
                ZeroOrMore(Box::new(Sequence(vec![
                    GrammarNode::Literal("2".to_owned()),
                    GrammarNode::Literal("3".to_owned()),
                ]))),
            ])),
        };
        assert_eq!(parse_tokens(tokens), expected_rule);
    }
}
