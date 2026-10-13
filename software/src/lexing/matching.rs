use std::collections::HashMap;

use crate::{
    grammar::{
        matches_rule, rules_to_map,
        types::{Rule, RuleMap},
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

#[cfg(test)]
mod tests {
    use crate::{
        grammar::types::{GrammarNode, Rule},
        lexing::{Token, matching::LexerMatcher},
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
}
