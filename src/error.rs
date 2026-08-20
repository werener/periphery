use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, PartialEq)]
pub enum LexicalError {
    #[error("No matches at position: {0}")]
    NoMatch(usize),
    #[error("Invalid literal: {0}. \"{1}\" isn't base {2}")]
    InvalidIntegerLiteral(String, char, u32),
}

impl Debug for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}