use std::{env::args, path::PathBuf};

use crate::tokenizing::tokenize;

mod ast;
mod grammar;
mod ir_generation;
mod tokenizing;

fn main() {
    let source_file = PathBuf::from(args().nth(1).expect("No argument given"));
    tokenize(&source_file).unwrap();
}
