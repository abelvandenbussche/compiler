use std::{env::args, fs::File, path::PathBuf};

use crate::lexing::Lexer;

mod ast;
mod grammar;
mod ir_generation;
mod lexing;
mod parsing;

fn main() {
    let source_file = PathBuf::from(args().nth(1).expect("No argument given"));
    let mut lexer =
        Lexer::new(File::open(source_file).expect("Failed to open source file")).unwrap();
    while let Some(token) = lexer.next_token() {
        println!("token: {:?}", token);
    }
    println!("Done tokening");
}
