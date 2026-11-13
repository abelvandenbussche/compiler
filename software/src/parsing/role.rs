//! Module containing the `Role` enum and its implementations

use crate::grammar::types::Rule;

#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    VariableDeclaration { identifier: Rule, value: Rule },
}

#[derive(Debug, PartialEq, Eq)]
pub struct RoleAssignment {
    /// The role to which it is assigned
    pub role: Role,
    /// The name of the assigned rule
    pub assigned_rule: String,
}
