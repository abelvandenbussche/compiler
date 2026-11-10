//! Module containing the `Role` enum and its implementations

pub trait Role: Eq + PartialEq {
    /// This should return the name of the role for in the EBNF syntax
    fn get_role_def_string(&self) -> String;
}

#[derive(PartialEq, Eq)]
pub struct VariableIdentifier {}
impl Role for VariableIdentifier {
    fn get_role_def_string(&self) -> String {
        return "@var_ident".to_owned();
    }
}
