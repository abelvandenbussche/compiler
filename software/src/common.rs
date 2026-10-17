//! A module containing common types used in the compiler

use std::{fmt::Display, str::FromStr};

use crate::common::Role::VariableIdentifier;

/// Roles that need to be filled in the grammar document so that the compiler can understand how the
/// language should function
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
    /// The name of a rule
    pub rule: String,
    /// The role that is being linked with a rule
    pub role: Role,
}

/// Literal values represented in the source code
#[derive(Debug, PartialEq)]
pub enum Literal {
    Char(char),
    String(String),
    Int(i128),
    Float(f64),
    Bool(bool),
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
