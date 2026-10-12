//! This module parses the grammar from the grammar file into rules that the [crate::parsing] can use on the
//! source code

use crate::grammar::{
    lexer::{EBNFToken::Semicolon, lex},
    tree::parse_tokens,
    types::{GrammarNode, Rule, RuleMap},
};
use std::collections::HashMap;

mod lexer;
pub mod tree;
pub mod types;

/// Parses the program rules and returns them
///
/// # What it returns
/// It returns a tuple containing the token rules and the grammar rules
/// where the first value are the token rules and the second are the grammar rules
pub fn get_rules(ebnf: String) -> (Vec<Rule>, Vec<Rule>) {
    let tokens = lex(ebnf);
    let mut grammar_rules = vec![];
    let mut token_rules = vec![];
    for i in split_list_on(tokens, Semicolon) {
        let new_rule = parse_tokens(i);
        let new_rule_head = &new_rule.head;
        if new_rule_head.to_uppercase() == *new_rule_head {
            token_rules.push(new_rule);
        } else {
            grammar_rules.push(new_rule);
        }
    }
    (token_rules, grammar_rules)
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

/// Checks whether a certain string matches a given rule
/// returns `None` if it did not find a match and `Some` if it did
///
/// If if found a match if returns what part of the string it found to match
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

#[cfg(test)]
mod tests {
    use crate::grammar::{
        get_rules, matches_rule, rules_to_map,
        types::{GrammarNode, Rule},
    };

    use super::split_list_on;
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
        let (lexing_rules, parsing_rules) = get_rules(ebnf_input.to_owned());
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
}
