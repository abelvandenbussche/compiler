use std::collections::HashMap;

/// Rules used by the parser to match `terminals` produced by the lexer
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    pub head: String,
    pub body: Option<GrammarNode>,
}

impl Rule {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GrammarNode {
    /// Literal characters
    Literal(String),
    /// Reference to another rule
    Reference(String),
    /// A sequence of nodes
    Sequence(Vec<GrammarNode>),
    /// Optional node
    Optional(Box<GrammarNode>),
    /// 0 or more of a node
    ZeroOrMore(Box<GrammarNode>),
    /// Groupes multiple nodes together
    Grouping(Box<GrammarNode>),
    /// Choice of a single node
    Choice(Vec<GrammarNode>),
}
