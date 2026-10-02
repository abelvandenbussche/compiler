//! This module parses the grammar from the grammar file into rules that the [crate::parsing] can use on the
//! source code

use crate::grammar::{
    lexer::{EBNFToken::Semicolon, lex},
    tree::parse_tokens,
    types::{GrammarNode, Rule},
};
use std::collections::HashMap;

mod lexer;
pub mod tree;
pub mod types;

pub fn get_rules() {}
fn get_tree() -> Vec<Rule> {
    let tokens = lex();
    let mut rules = vec![];
    for i in split_list_on(tokens, Semicolon) {
        rules.push(parse_tokens(i));
    }
    rules
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
    if new_list.len() > 0 {
        list_of_lists.push(new_list);
    }
    list_of_lists
}

/// Converts a list of rules to a hashmap for more efficient indexing and finding of rules
pub fn rules_to_map(rules: Vec<Rule>) -> HashMap<String, GrammarNode> {
    let mut map = HashMap::new();
    for rule in rules {
        if let Some(node) = rule.body {
            map.insert(rule.head, node);
        }
    }
    map
}

/// Checks whether a certain string matches a given rules
/// returns `None` if it did not find a match and `Some` if it did
///
/// If if found a match if returns what part of the string it found to match
fn matches_rule<'a>(
    text: &'a str,
    rule: &GrammarNode,
    rules: &HashMap<String, GrammarNode>,
) -> Option<&'a str> {
    println!("processing rule: {:?}", rule);
    match rule {
        GrammarNode::Sequence(list) => {
            let mut first_index = 0;
            for i in list {
                if let Some(matched_slice) = matches_rule(&text[first_index..], i, rules) {
                    first_index += matched_slice.len();
                } else {
                    println!("sequence none");
                    return None;
                }
            }
            Some(&text[..first_index])
        }
        GrammarNode::Reference(reference) => {
            if let Some(new_rule) = rules.get(reference) {
                matches_rule(text, new_rule, rules)
            } else {
                println!("reference none");
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
            println!("choice none");
            None
        }
        GrammarNode::Literal(literal) => {
            if let Some(first_token) = text.chars().next() {
                println!("{:?}, {literal}", first_token);
                if first_token.to_string() == *literal {
                    Some(&text[0..1])
                } else {
                    None
                }
            } else {
                None
            }
        }
        GrammarNode::ZeroOrMore(internal_rule) => {
            let mut range = 0;
            while let Some(v) = matches_rule(&text[range..], internal_rule, rules) {
                println!("looping");
                range += v.len()
            }
            Some(&text[..range])
        }
        other => {
            println!("Got here with: ({:?})", other);
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar::{
        matches_rule, rules_to_map,
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
}
