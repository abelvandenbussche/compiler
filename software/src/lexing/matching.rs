//! Contains the matching struct to match raw source code with the generated grammar rules. Used by
//! The [lexing](crate::lexing) phase to generate tokens
use std::collections::HashMap;

use crate::{
    grammar::{
        RuleSet, contains_recursive, matches_rule, rules_to_map,
        types::{
            GrammarNode::{self, Choice, Literal, Optional, Reference, Sequence, ZeroOrMore},
            Rule, RuleMap,
        },
    },
    lexing::Token,
};

/// A matcher that checks against every rule in its collection and returns the best fitting match
pub struct LexerMatcher {
    rules: RuleMap,
}

impl LexerMatcher {
    /// Create a new `LexerMatcher` instance
    pub fn new(rules: Vec<Rule>) -> Self {
        let hash_map_rules = rules_to_map(rules);
        Self {
            rules: hash_map_rules,
        }
    }

    /// Finds the next best match using a greedy algorithm
    pub fn next_best_match(&self, working_string: &str) -> Option<Token> {
        let mut result_map = HashMap::new();
        // Checking each rule
        for (rule_name, rule_body) in &self.rules {
            let result = matches_rule(working_string, rule_body, &self.rules);
            if let Some(matched_slice) = result {
                result_map.insert(rule_name, matched_slice);
            }
        }
        // Selecting the best
        let mut best: Option<(&String, &str)> = None;
        for (k, e) in result_map.into_iter() {
            if let Some((_best_k, best_e)) = best {
                if e.chars().count() > best_e.chars().count() {
                    best = Some((k, e));
                }
            } else {
                best = Some((k, e))
            }
        }
        // building the token
        let token = Token {
            head: best?.0.clone(),
            body: best?.1.to_owned(),
        };
        Some(token)
    }
}

/// Calculates how deep the references go for a certain rule
fn get_rule_depth(rule: &GrammarNode, rules: &RuleMap) -> u32 {
    /// Recursive inner function of `get_rule_depth`
    fn calc_depth(rule: &GrammarNode, rules: &RuleMap, current_depth: u32) -> u32 {
        match rule {
            Reference(reference) => {
                let referenced_rule = &rules[reference];
                calc_depth(referenced_rule, rules, current_depth) + 1
            }
            Sequence(vector) | Choice(vector) => {
                let mut max = 0;
                for i in vector {
                    let depth = calc_depth(&i, rules, current_depth);
                    max = max.max(depth);
                }
                max
            }
            Optional(value) | ZeroOrMore(value) | GrammarNode::Grouping(value) => {
                calc_depth(value, rules, current_depth)
            }
            Literal(_) => 0,
        }
    }
    calc_depth(rule, rules, 0)
}

#[cfg(test)]
mod tests {
    use crate::{
        grammar::{
            rules_to_map,
            types::{GrammarNode, Rule},
        },
        lexing::{
            Token,
            matching::{LexerMatcher, get_rule_depth},
        },
    };
    #[test]
    fn best_match() {
        let string = "Hello world";
        let rule1 = Rule {
            head: "RULE1".to_owned(),
            body: Some(GrammarNode::Literal("Hello".to_owned())),
        };
        let rule2 = Rule {
            head: "RULE2".to_owned(),
            body: Some(GrammarNode::Literal("Hello wo".to_owned())),
        };
        let matcher = LexerMatcher::new(vec![rule1, rule2]);
        let match_result = matcher.next_best_match(string);
        assert_eq!(
            match_result,
            Some(Token {
                head: "RULE2".to_owned(),
                body: "Hello wo".to_owned()
            })
        );
    }
    #[test]
    fn test_rule_depth() {
        let rules = vec![
            Rule {
                head: "foo".to_owned(),
                body: Some(GrammarNode::Reference("bar".to_owned())),
            },
            Rule {
                head: "bar".to_owned(),
                body: Some(GrammarNode::Sequence(vec![
                    GrammarNode::Reference("random".to_owned()),
                    GrammarNode::Reference("another".to_owned()),
                ])),
            },
            Rule {
                head: "random".to_owned(),
                body: Some(GrammarNode::Literal("Hello world".to_owned())),
            },
            Rule {
                head: "another".to_owned(),
                body: Some(GrammarNode::Optional(Box::new(GrammarNode::Reference(
                    "yet another".to_owned(),
                )))),
            },
            Rule {
                head: "yet another".to_owned(),
                body: Some(GrammarNode::Literal("Testing".to_owned())),
            },
        ];
        let rule_map = rules_to_map(rules.clone());
        assert_eq!(
            get_rule_depth(&rules[0].body.clone().unwrap(), &rule_map),
            3
        );
    }
}
