use std::{env::args, fs::File, io::Read, path::PathBuf};

use crate::{
    grammar::{RuleSet, get_rules, rules_to_map},
    lexing::Lexer,
    parsing::build_syntax_tree,
};

mod common;
mod grammar;
mod ir_generation;
mod lexing;
mod parsing;

/// The main function of the compiler, it orchestrates everything that happens on a high level
fn main() {
    // Opening both files and reading their contents
    let mut arguments = args();
    let mut source_file = File::open(PathBuf::from(
        arguments.nth(1).expect("No source file given"),
    ))
    .expect("Invalid source file");
    let mut grammar_file = File::open(PathBuf::from(
        arguments.next().expect("No grammar file given"),
    ))
    .expect("Invalid grammar file");

    let mut source_file_contents = String::new();
    source_file
        .read_to_string(&mut source_file_contents)
        .expect("Failed to read source file");
    let mut grammar_file_contents = String::new();
    grammar_file
        .read_to_string(&mut grammar_file_contents)
        .expect("Failed to read grammar file");

    // Parsing the contents of the grammar file
    let RuleSet {
        lexing: lexing_rules,
        parsing: parsing_rules,
        roles: _roles,
    } = get_rules(grammar_file_contents);

    // Lexing the contents of the source file
    let mut lexer = Lexer::new(source_file_contents, lexing_rules);
    let mut tokens = vec![];
    while let Some(token) = lexer.next_token() {
        println!("token: {:?}", token);
        tokens.push(token);
    }

    // parsing the tokens
    let tree = build_syntax_tree(tokens, rules_to_map(parsing_rules), None);
    println!("build up tree: {:?}", tree);
}
