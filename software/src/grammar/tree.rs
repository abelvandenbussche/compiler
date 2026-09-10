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
    let head = match tokens.remove(0) {
        EBNFToken::Identifier(s) => Rule {
            head: s,
            body: None,
        },
        other => panic!("Unexpected token: {:?}", other),
    };
    head
}

#[cfg(test)]
mod tests {
    use crate::grammar::{
        lexer::EBNFToken::{self, *},
        tree::{
            Node::{self, *},
            Rule, parse_tokens,
        },
    };

    #[test]
    fn rule_to_tree() {
        let tokens = vec![
            Identifier("foo".to_owned()),
            Equals,
            EBNFToken::Literal("bar".to_owned()),
        ];
        let expected = Rule {
            head: "foo".to_owned(),
            body: Some(Node::Literal("bar".to_owned())),
        };
        assert_eq!(parse_tokens(tokens), expected);

        // TODO:
        // foo = ["hello" | "world"], {}
        let tokens = vec![Identifier("foo".to_owned()), Equals];
    }
}
