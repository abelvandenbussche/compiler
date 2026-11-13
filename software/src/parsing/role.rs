//! Module containing the `Role` enum and its implementations

use crate::grammar::types::Rule;

#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    VariableDeclaration { identifier: Rule, value: Rule },
}
