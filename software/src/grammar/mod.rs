//! This module parses the grammar from the grammar file into rules that the [crate::parsing] can use on the
//! source code

use crate::grammar::{
    lexer::{EBNFToken::Semicolon, lex},
    tree::parse_tokens,
    types::Rule,
};

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

#[cfg(test)]
mod tests {
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
}
