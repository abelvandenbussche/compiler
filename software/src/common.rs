//! A module containing common types used in the compiler

use crate::parsing::role::Role;
use std::fmt::Display;

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

/// Possible actions that can be used when setting terminal colors
pub enum ColorAction {
    ForeGroundRGB(u8, u8, u8),
    BackGroundRGB(u8, u8, u8),
    Reset,
}

/// Returns a text color escape sequence
pub fn set_text_color(color: ColorAction) -> String {
    match color {
        ColorAction::ForeGroundRGB(r, g, b) => format!("\x1b[38;2;{r};{g};{b}m"),
        ColorAction::BackGroundRGB(r, g, b) => format!("\x1b[48;2;{r};{g};{b}m"),
        ColorAction::Reset => String::from("\x1b[m"),
    }
}
