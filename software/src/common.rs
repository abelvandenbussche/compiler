//! A module containing common types used in the compiler

use std::{fmt::Display, str::FromStr};

use crate::{common::Role::VariableIdentifier, grammar::types::Rule};

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
    VariableIdentifier,
}

impl FromStr for Role {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // This controls the written name is of each role
        match s {
            "@var_ident" => Ok(VariableIdentifier),
            _ => Err("Invalid role name"),
        }
    }
}
/// A `Rule` that is linked to a `Role`
#[derive(PartialEq, Eq, Debug)]
pub struct RoleLink {
    pub rule: String,
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
