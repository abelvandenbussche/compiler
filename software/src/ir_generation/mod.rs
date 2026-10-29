//! This module converts the AST created by the [crate::parsing]
use std::io::{BufWriter, Write};

/// Deals with generating IR
pub struct IRWriter<OutputType: Write> {
    output: BufWriter<OutputType>,
}

impl<T: Write> IRWriter<T> {
    /// Create a new `ÌRWriter` instance
    pub fn new(output_stream: T) -> Self {
        Self {
            output: BufWriter::new(output_stream),
        }
    }
}

#[cfg(test)]
mod tests {}
