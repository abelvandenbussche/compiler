//! Module containing the `Role` enum and its implementations

use Role::*;
use std::str::FromStr;

/// Roles that need to be filled in the grammar document so that the compiler can understand how the
/// language should function
#[derive(PartialEq, Eq, Debug)]
pub enum Role {
    VariableIdentifier,
    VariableValue,
    VariableDeclaration,
}

impl FromStr for Role {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // This controls the written name is of each role
        match s {
            "@var_ident" => Ok(VariableIdentifier),
            "@var_decl" => Ok(VariableDeclaration),
            "@var_val" => Ok(VariableValue),
            _ => Err("Invalid role name"),
        }
    }
}
