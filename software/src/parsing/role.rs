//! Module containing the `Role` enum and its implementations

use std::fmt::Debug;

pub trait Role: Eq + Debug {}

#[derive(Eq, PartialEq, Debug)]
pub struct VariableDeclaration {
    placeholder: String,
}
impl Role for VariableDeclaration {}
