//! A module containing common types used in the compiler

use std::fmt::Display;

use crate::grammar::types::Rule;

#[derive(Debug, PartialEq)]
/// Literal values represented in the source code
pub enum Literal {
    Char(char),
    String(String),
    Int(i128),
    Float(f64),
    Bool(bool),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Role {
    VariableName,
}

/// A `Rule` that is linked to a `Role`
pub struct RoleLink {
    pub rule: Rule,
    pub role: Role,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Bool(bool) => &bool.to_string(),
            Self::Float(float) => &float.to_string(),
            Self::Int(int) => &int.to_string(),
            Self::Char(c) => &c.to_string(),
            Self::String(s) => s,
        };
        f.write_str(string)
    }
}
