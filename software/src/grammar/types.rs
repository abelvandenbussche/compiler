use std::collections::HashMap;

/// Rules used by the parser to match `terminals` produced by the lexer
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    pub head: String,
    pub body: Option<GrammarNode>,
}

impl Rule {
    fn matches<'a>(
        &self,
        tokens: &'a str,
        rule: &GrammarNode,
        rules: &HashMap<String, GrammarNode>,
    ) -> Option<&'a str> {
        println!("processing rule: {:?}", rule);
        match rule {
            GrammarNode::Sequence(list) => {
                let mut first_index = 0;
                for i in list {
                    if let Some(matched_slice) = self.matches(&tokens[first_index..], i, rules) {
                        first_index += matched_slice.len();
                    } else {
                        println!("sequence none");
                        return None;
                    }
                }
                Some(&tokens[..first_index])
            }
            GrammarNode::Reference(reference) => {
                if let Some(new_rule) = rules.get(reference) {
                    self.matches(tokens, new_rule, rules)
                } else {
                    println!("reference none");
                    None
                }
            }
            GrammarNode::Choice(to_choose) => {
                for i in to_choose {
                    let result = self.matches(tokens, i, rules);
                    if result.is_some() {
                        return result;
                    }
                }
                println!("choice none");
                None
            }
            GrammarNode::Literal(literal) => {
                if let Some(first_token) = tokens.chars().next() {
                    println!("{:?}, {literal}", first_token);
                    if first_token.to_string() == *literal {
                        Some(&tokens[0..1])
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            GrammarNode::ZeroOrMore(internal_rule) => {
                let mut range = 0;
                while let Some(v) = self.matches(&tokens[range..], internal_rule, rules) {
                    println!("looping");
                    range += v.len()
                }
                Some(&tokens[..range])
            }
            other => {
                println!("Got here with: ({:?})", other);
                todo!()
            }
        }
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn simple_matching() {}
}
