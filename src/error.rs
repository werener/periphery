use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LexicalError {
    #[error("No matches at position {0}")]
    NoMatch(usize),
}
