use std::collections::HashMap;

/// Rules used by the compiler that represent the syntax of the language that is defined in the
/// grammar file
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    pub head: String,
    pub body: Option<GrammarNode>,
}

/// The different kinds of EBNF structures
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
    /// Groups multiple nodes together
    Grouping(Box<GrammarNode>),
    /// Choice of a single node
    Choice(Vec<GrammarNode>),
}

// Alias to make it clearer what the point of the type is
pub type RuleMap = HashMap<String, GrammarNode>;
