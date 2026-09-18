//! A module containing common types used by the compiler

#[derive(Debug, PartialEq)]
/// Literal values represented in the source code
pub enum Literal {
    Char(char),
    String(String),
    Int(i128),
    Float(f64),
    Bool(bool),
}
