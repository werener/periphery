use crate::error::LexicalError;

use super::token::*;

#[derive(Default, Debug)]
pub struct Lexer {
    input: String,
    position: usize,

    tokens: Vec<Token>,
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

    pub(crate) fn consume(&mut self, token: Token, matched: regex::Match) {
        self.tokens.push(token);
        self.advance(matched.len());
    }

    fn at_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    #[inline]
    fn remainder(&self) -> String {
        self.input[self.position..].to_string()
    }
}

/// Accepts a string as an input, and returns a vector of tokens, if the provided string has no lexical errors.
/// Otherwise, returns a `LexicalError`
pub fn tokenize<S: Into<String>>(input: S) -> crate::lexer::Result<Vec<Token>> {
    let mut lexer = Lexer::new(input);
    let patterns = crate::lexer::token::patterns::all();
    log::debug!("Entered lexing phase");

    while !lexer.at_eof() {
        let mut has_matched = false;
        let remainder = lexer.remainder();

        for pattern in patterns.iter() {
            if let Some(matched) = pattern.regex.find(&remainder) {
                if matched.start() == 0 {
                    log::debug!(
                        "Matched pattern '{}' to '{}' (in: '{}')",
                        pattern.regex,
                        matched.as_str(),
                        lexer.remainder()
                    );
                    (pattern.handler)(&mut lexer, matched)?;
                    has_matched = true;
                    break;
                }
            }
        }
        if !has_matched {
            log::debug!(
                "No matches at remainder '{}' (position {})",
                lexer.remainder(),
                lexer.position
            );
            return Err(LexicalError::NoMatch(lexer.position, lexer.tokens));
        }
    }
    lexer.tokens.push(Token::Eof);

    log::debug!("Finished lexing phase");
    return Ok(lexer.tokens);
}
