use regex::Regex;
use handlers::*;
/// TokenKind represents the type of a token.
///
/// The order, in which these kinds are placed is the same order,
/// in which the lexer tries to match them to their corresponding regex pattern.
#[derive(Debug, Clone, Copy, PartialEq, strum_macros::EnumIter)]
pub enum TokenKind {
    Number(i32),
    String,
    Identifier,

    WhiteSpace,
    Eof,

    Equals,
    NotEquals,
    Less,
    LessOrEquals,
    Greater,
    GreaterOrEquals,

    Or,
    Not,
    And,

    Plus,
    Minus,
    Slash,
    Asterisk,
    Percent,

    Dot,
    Assign,

    LeftBracket,
    RightBracket,
    LeftCurly,
    RightCurly,
    LeftParen,
    RightParen,

    Semicolon,
    Colon,
    Question,
    Comma,

    // Keywords
    Let,
    Const,
    Fn,
    Struct,
    If,
    Else,
    For,
    While,
}

impl TokenKind {
    /// Lookup table for regex expressions, matching all tokens of this kind.
    fn get_regex(self) -> Regex {
        use TokenKind::*;
        let pattern = match self {
            Number(_) => r"a^",
            String => r"a^",
            Identifier => r"a^",

            Eof => r"a^",
            WhiteSpace => r"\s+",

            Equals => r"a^",
            NotEquals => r"a^",
            Less => r"a^",
            LessOrEquals => r"a^",
            Greater => r"a^",
            GreaterOrEquals => r"a^",
            Or => r"a^",
            Not => r"a^",
            And => r"a^",

            Plus => r"a^",
            Minus => r"a^",
            Slash => r"a^",
            Asterisk => r"a^",
            Percent => r"a^",

            Dot => r"a^",
            Assign => r"a^",

            LeftBracket => r"\[",
            RightBracket => r"\]",
            LeftCurly => r"\{",
            RightCurly => r"\}",
            LeftParen => r"\(",
            RightParen => r"\)",

            Semicolon => r"a^",
            Colon => r"a^",
            Question => r"a^",
            Comma => r"a^",
            Let => r"a^",
            Const => r"a^",
            Fn => r"a^",
            Struct => r"a^",
            If => r"a^",
            Else => r"a^",
            For => r"a^",
            While => r"a^",
        };

        Regex::new(pattern).expect(&format!("WRONG REGEX LOOKUP TABLE FOR {:?}", self))
    }

    
    /// Lookup table for handler, that defines how this kind of token is in the input stream.
    fn get_handler(self) -> Box<MatchHandler> {
        use TokenKind::*;
        match self {
            WhiteSpace => skip_handler(),
            _ => default_handler(self),
        }
    }
}

mod handlers {
    use super::TokenKind;
    use crate::lexer::lexer::Lexer;

    pub type MatchHandler = dyn Fn(&mut Lexer, regex::Match);

    pub fn default_handler(kind: TokenKind) -> Box<MatchHandler> {
        Box::new(move |lexer, matched| lexer.consume(kind, matched))
    }
    pub fn skip_handler() -> Box<MatchHandler> {
        Box::new(move |lexer, matched| lexer.advance(matched.end()))
    }
}

pub mod patterns {
    use super::TokenKind;
    use crate::lexer::token::handlers::MatchHandler;

    #[fully_pub::fully_pub]
    struct RegexPattern {
        handler: Box<MatchHandler>,
        regex: regex::Regex,
    }

    impl RegexPattern {
        fn from_kind(kind: TokenKind) -> Self {
            RegexPattern {
                handler: kind.get_handler(),
                regex: kind.get_regex(),
            }
        }
    }

    #[inline]
    pub fn all() -> Vec<RegexPattern> {
        use strum::IntoEnumIterator;
        let patterns = TokenKind::iter().map(RegexPattern::from_kind);

        patterns.collect()
    }
}
