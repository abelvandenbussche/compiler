use std::{env::args, fs::File, path::PathBuf};

use crate::lexing::Lexer;

mod ast;
mod grammar;
mod ir_generation;
mod lexing;

fn main() {
    let source_file = PathBuf::from(args().nth(1).expect("No argument given"));
    let lexer = Lexer::new(File::open(source_file).expect("Failed to open source file"));
}
