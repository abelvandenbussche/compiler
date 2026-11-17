//! A module containing common types used in the compiler

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FundamentalType {
    Char,
    Bool,
    Int,
    Float,
}

impl Display for FundamentalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = match self {
            Self::Int => "#int",
            Self::Float => "#float",
            Self::Char => "#char",
            Self::Bool => "#bool",
            _ => return Err(std::fmt::Error),
        };
        f.write_str(stringified)
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
