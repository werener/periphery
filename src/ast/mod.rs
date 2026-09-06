pub mod expression;
pub mod statement;

use crate::error::ParseError;
pub type Result<T> = std::result::Result<T, ParseError>;

pub trait Statement {
    fn stmt();
}

pub trait Expression {
    fn expr();
}
