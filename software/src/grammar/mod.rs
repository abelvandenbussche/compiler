//! This module parses the grammar from the grammar file into rules that the [crate::parsing] can use on the
//! source code

use crate::{
    common::RoleLink,
    grammar::{
        lexer::{EBNFToken::Semicolon, lex},
        tree::{RuleOrRole, parse_tokens},
        types::{GrammarNode, Rule, RuleMap},
    },
};
use std::collections::HashMap;

mod lexer;
pub mod tree;
pub mod types;

/// A set of rule collections, meant to group the different rule types together
pub struct RuleSet {
    /// Rules used during the lexing phase
    pub lexing: Vec<Rule>,
    /// Rules used during the parsing phase
    pub parsing: Vec<Rule>,
    pub roles: Vec<RoleLink>,
}

/// Parses the program rules and returns them
///
/// # What it returns
/// It returns a tuple containing the token rules and the grammar rules
/// where the first value are the token rules and the second are the grammar rules
pub fn get_rules(ebnf: String) -> RuleSet {
    let tokens = lex(ebnf);
    let mut grammar_rules = vec![];
    let mut token_rules = vec![];
    let mut roles = vec![];
    for i in split_list_on(tokens, Semicolon) {
        match parse_tokens(i) {
            RuleOrRole::Role(new_role) => roles.push(new_role),
            RuleOrRole::Rule(new_rule) => {
                let new_rule_head = &new_rule.head;
                if is_lexing_rule(new_rule_head) {
                    token_rules.push(new_rule);
                } else {
                    grammar_rules.push(new_rule);
                }
            }
        }
    }
    RuleSet {
        lexing: token_rules,
        parsing: grammar_rules,
        roles,
    }
}

/// Splits the list into multiple lists
/// does not include the splitting character\
/// This functions does some expensive move operations
fn split_list_on<T>(list: Vec<T>, on: T) -> Vec<Vec<T>>
where
    T: Eq,
{
    let mut list_of_lists = vec![];
    let mut new_list = vec![];

    for i in list {
        if i == on {
            list_of_lists.push(new_list);
            new_list = vec![];
            continue;
        }
        new_list.push(i);
    }
    // Adding the final list
    if !new_list.is_empty() {
        list_of_lists.push(new_list);
    }
    list_of_lists
}

/// Converts a list of rules to a hashmap for more efficient indexing and finding of rules
pub fn rules_to_map(rules: Vec<Rule>) -> RuleMap {
    let mut map = HashMap::new();
    for rule in rules {
        if let Some(node) = rule.body {
            map.insert(rule.head, node);
        }
    }
    map
}

/// Checks wether a there is recursion within a set of rules
pub fn is_recursive(rule_set: RuleMap) -> bool {
    /// Tests wether a single rule has a recursive definition
    fn test_rule_for_recursion(rule: GrammarNode) -> bool {
        todo!()
    }
    todo!()
}

/// Checks whether a certain string matches a given rule
/// returns `None` if it did not find a match and `Some` if it did
///
/// If it found a match if returns what part of the string it found to match
pub fn matches_rule<'a>(text: &'a str, rule: &GrammarNode, rules: &RuleMap) -> Option<&'a str> {
    match rule {
        GrammarNode::Sequence(list) => {
            let mut first_index = 0;
            for i in list {
                if let Some(matched_slice) = matches_rule(&text[first_index..], i, rules) {
                    first_index += matched_slice.len();
                } else {
                    return None;
                }
            }
            Some(&text[..first_index])
        }
        GrammarNode::Reference(reference) => {
            if let Some(new_rule) = rules.get(reference) {
                matches_rule(text, new_rule, rules)
            } else {
                None
            }
        }
        GrammarNode::Choice(to_choose) => {
            for i in to_choose {
                let result = matches_rule(text, i, rules);
                if result.is_some() {
                    return result;
                }
            }
            None
        }
        GrammarNode::Literal(literal) => {
            if text.starts_with(literal) {
                Some(&text[0..literal.len()])
            } else {
                None
            }
        }
        GrammarNode::ZeroOrMore(internal_rule) => {
            let mut range = 0;
            while let Some(v) = matches_rule(&text[range..], internal_rule, rules) {
                range += v.len()
            }
            Some(&text[..range])
        }
        GrammarNode::Optional(option) => {
            // TODO: write better tests for this
            if let Some(match_slice) = matches_rule(text, option, rules) {
                Some(match_slice)
            } else {
                Some("")
            }
        }
        GrammarNode::Grouping(group) => matches_rule(text, group, rules),
    }
}

/// Returns whether `true` if the rule is a lexing rule and `false` if the rule is another type
pub fn is_lexing_rule(rule_name: &str) -> bool {
    rule_name.to_uppercase() == rule_name
}

#[cfg(test)]
mod tests {
    use crate::grammar::{
        RuleSet, is_recursive,
        types::{GrammarNode, Rule},
    };

    use super::{get_rules, is_lexing_rule, matches_rule, rules_to_map, split_list_on};

    #[test]
    fn lexing_rule_identification() {
        assert!(is_lexing_rule("FOO"));
        assert!(is_lexing_rule("FOO1"));

        assert!(!is_lexing_rule("foo"));
        assert!(!is_lexing_rule("Foo"));
        assert!(!is_lexing_rule("FoO"));
        assert!(!is_lexing_rule("FoO*"));
    }
    #[test]
    fn splitting_list() {
        // Splitting multiple times
        let list = vec![1, 4, 7, 2, 3, 6, 1, 2, 9, 3];
        assert_eq!(
            split_list_on(list, 2),
            vec![vec![1, 4, 7], vec![3, 6, 1], vec![9, 3]]
        );
        // Non-splitting
        let list2 = vec![1, 2, 3];
        assert_eq!(split_list_on(list2, 4), vec![vec![1, 2, 3]]);
    }
    #[test]
    fn rules_to_hash_conversion() {
        let rules = vec![
            Rule {
                head: "TEST".to_owned(),
                body: None,
            },
            Rule {
                head: "foo".to_owned(),
                body: Some(GrammarNode::Literal("bar".to_owned())),
            },
        ];
        let hashmap = rules_to_map(rules);
        assert_eq!(hashmap.get("TEST"), None);
        assert_eq!(
            hashmap.get("foo"),
            Some(&GrammarNode::Literal("bar".to_owned()))
        );
    }
    #[test]
    fn literal_matching() {
        let literal_rule = Rule {
            head: "IF_KEYWORD".to_owned(),
            body: Some(GrammarNode::Literal("if".to_owned())),
        };
        let rules_map = rules_to_map(vec![literal_rule]);
        assert_eq!(
            matches_rule("if true then", &rules_map["IF_KEYWORD"], &rules_map),
            Some("if")
        );
    }
    #[test]
    fn simple_matching() {
        let number_rule = Rule {
            head: "NUMBER".to_owned(),
            body: Some(GrammarNode::Sequence(vec![
                GrammarNode::Reference("DIGIT".to_owned()),
                GrammarNode::ZeroOrMore(Box::new(GrammarNode::Reference("DIGIT".to_owned()))),
            ])),
        };
        let digit_rule = Rule {
            head: "DIGIT".to_owned(),
            body: Some(GrammarNode::Choice(vec![
                GrammarNode::Literal("0".to_owned()),
                GrammarNode::Literal("1".to_owned()),
                GrammarNode::Literal("2".to_owned()),
                GrammarNode::Literal("3".to_owned()),
                GrammarNode::Literal("4".to_owned()),
                GrammarNode::Literal("5".to_owned()),
                GrammarNode::Literal("6".to_owned()),
                GrammarNode::Literal("7".to_owned()),
                GrammarNode::Literal("8".to_owned()),
                GrammarNode::Literal("9".to_owned()),
            ])),
        };
        let rules_map = rules_to_map(vec![number_rule, digit_rule]);
        assert_eq!(
            matches_rule("123", &rules_map["NUMBER"], &rules_map),
            Some("123")
        );
        assert_eq!(
            matches_rule("123hello world", &rules_map["NUMBER"], &rules_map),
            Some("123")
        );
        assert_eq!(
            matches_rule("he5llo world123", &rules_map["NUMBER"], &rules_map),
            None
        );
        assert_eq!(
            matches_rule("123 50", &rules_map["NUMBER"], &rules_map),
            Some("123")
        );
    }
    #[test]
    fn handle_grouping() {
        let rule_with_grouping = Rule {
            head: "GROUPING".to_owned(),
            body: Some(GrammarNode::Grouping(Box::new(GrammarNode::Sequence(
                vec![
                    GrammarNode::Literal("Hello".to_owned()),
                    GrammarNode::Literal(" World".to_owned()),
                ],
            )))),
        };
        let rule_map = rules_to_map(vec![rule_with_grouping]);
        assert_eq!(
            matches_rule("Hello World", &rule_map["GROUPING"], &rule_map),
            Some("Hello World")
        );
    }
    #[test]
    fn handle_optional() {
        let optional_rule = Rule {
            head: "OPTIONAL".to_owned(),
            body: Some(GrammarNode::Sequence(vec![
                GrammarNode::Literal("Hello ".to_owned()),
                GrammarNode::Optional(Box::new(GrammarNode::Literal("Rust ".to_owned()))),
                GrammarNode::Literal("World".to_owned()),
            ])),
        };
        let rule_map = rules_to_map(vec![optional_rule]);
        assert_eq!(
            matches_rule("Hello Rust World", &rule_map["OPTIONAL"], &rule_map,),
            Some("Hello Rust World")
        );
        assert_eq!(
            matches_rule("Hello World", &rule_map["OPTIONAL"], &rule_map,),
            Some("Hello World")
        );
    }
    #[test]
    fn getting_seperated_rules() {
        let ebnf_input = r#"TEST = "something";
foo = "something else";
BaR = "yet another something else";
yet = %another
    "#;
        let RuleSet {
            lexing: lexing_rules,
            parsing: parsing_rules,
            roles: _,
        } = get_rules(ebnf_input.to_owned());
        assert_eq!(
            lexing_rules,
            vec![Rule {
                head: "TEST".to_owned(),
                body: Some(GrammarNode::Literal("something".to_owned()))
            }]
        );
        assert_eq!(
            parsing_rules,
            vec![
                Rule {
                    head: "foo".to_owned(),
                    body: Some(GrammarNode::Literal("something else".to_owned()))
                },
                Rule {
                    head: "BaR".to_owned(),
                    body: Some(GrammarNode::Literal(
                        "yet another something else".to_owned()
                    ))
                },
                Rule {
                    head: "yet".to_owned(),
                    body: Some(GrammarNode::Reference("%another".to_owned()))
                }
            ]
        )
    }

    #[test]
    fn rule_recursion() {
        let rule_map1 = rules_to_map(vec![
            Rule {
                head: "foo".to_owned(),
                body: Some(GrammarNode::Reference("bar".to_owned())),
            },
            Rule {
                head: "bar".to_owned(),
                body: Some(GrammarNode::Reference("foo".to_owned())),
            },
        ]);
        assert!(is_recursive(rule_map1));
        let rule_map2 = rules_to_map(vec![
            Rule {
                head: "foo".to_owned(),
                body: None,
            },
            Rule {
                head: "bar".to_owned(),
                body: Some(GrammarNode::Literal("Hello world".to_owned())),
            },
        ]);
        assert!(!is_recursive(rule_map2));
    }
}
