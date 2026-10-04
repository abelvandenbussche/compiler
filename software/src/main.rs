use std::{env::args, fs::File, path::PathBuf};

use crate::{grammar::get_rules, lexing::Lexer};

mod common;
mod grammar;
mod ir_generation;
mod lexing;
mod parsing;

fn main() {
    let source_file = PathBuf::from(args().nth(1).expect("No argument given"));
    let (lexing_rule, grammar_rule) = get_rules();
    let lexer = Lexer::new(File::open(source_file).expect("Failed to open file"));
}
