use std::fmt::Debug;

use crate::error::LexicalError;

use super::token::*;

#[derive(Default)]
pub struct Lexer {
    input: String,
    position: usize,

    tokens: Vec<TokenKind>,
}
impl Debug for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {:?}", self.input, self.tokens)
    }
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    pub(crate) fn advance(&mut self, step: usize) {
        self.position += step;
    }

    pub(crate) fn consume(&mut self, token: TokenKind, matched: regex::Match) {
        self.tokens.push(token);
        self.advance(matched.len());
    }

    pub(crate) fn at_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    pub(crate) fn remainder(&self) -> String {
        self.input[self.position..].to_string()
    }
}

/// Accepts a string as an input, and returns a vector of tokens, if the provided string has no lexical errors. 
/// Otherwise, returns a `LexicalError`
pub fn tokenize<S: Into<String>>(input: S) -> Result<Vec<TokenKind>, LexicalError> {
    let mut lexer = Lexer::new(input);
    let patterns = crate::lexer::token::patterns::all();
    while !lexer.at_eof() {
        let mut has_matched = false;
        for pattern in patterns.iter() {
            if let Some(matched) = pattern.regex.find(lexer.remainder().as_str()) {
                if matched.start() == 0 {
                    log::debug!("Matched pattern '{}' to '{}' (remainder: {})", pattern.regex, matched.as_str(), lexer.remainder());
                    (pattern.handler)(&mut lexer, matched);
                    has_matched = true;
                }
            }
        }
        if !has_matched {
            log::debug!("No matches at position remainder '{}' (position {})", lexer.remainder(), lexer.position);
            return Err(LexicalError::NoMatch(lexer.position));
        }
    }

    return Ok(lexer.tokens);
}
