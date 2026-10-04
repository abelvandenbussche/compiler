use std::{env::args, fs::File, path::PathBuf};

use crate::{grammar::get_rules, lexing::Lexer};

mod common;
mod grammar;
mod ir_generation;
mod lexing;
mod parsing;

fn main() {
    let mut arguments = args();
    let source_file = File::open(PathBuf::from(
        arguments.nth(1).expect("No source file given"),
    ))
    .expect("Invalid source file");
    let grammar_file = File::open(PathBuf::from(
        arguments.next().expect("No grammar file given"),
    ))
    .expect("Invalid grammar file");
    let (lexing_rules, grammar_rules) = get_rules(grammar_file);
    let mut lexer = Lexer::new(source_file, lexing_rules).expect("Failed to create lexer");
    while let Some(token) = lexer.next_token() {
        println!("token: {:?}", token);
    }
}
