/// Token represents a single token.
///
/// The order, in which these variants are placed is the same order,
/// in which the lexer tries to match them to their corresponding regex pattern.
#[derive(Debug, Clone, PartialEq, strum_macros::EnumIter)]
pub enum Token {
    // Keywords
    Let,
    Const,
    Fn,
    Struct,
    If,
    Else,
    For,
    While,

    Integer(i64),
    String(String),
    Identifier(String),

    /// spaces, \n, \t, \r, \f, \v
    WhiteSpace,
    /// automatically inserted at the end of provided input
    Eof,

    /// ==
    Equals,
    /// !=
    NotEquals,
    /// <=
    LessOrEquals,
    /// <
    Less,
    /// >=
    GreaterOrEquals,
    /// >
    Greater,

    /// ||
    Or,
    /// !
    Not,
    /// &&
    And,

    /// +
    Plus,
    /// -
    Minus,
    /// /
    Slash,
    /// *
    Asterisk,
    /// %
    Percent,

    /// ..=
    ClosedInterval,
    // ..
    HalfInterval,
    /// .
    Dot,
    /// =
    Assign,

    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// {
    LeftCurly,
    /// }
    RightCurly,
    /// (
    LeftParen,
    /// )
    RightParen,

    /// ;
    Semicolon,
    /// :
    Colon,
    /// ?
    Question,
    /// ,
    Comma,
}

mod handlers {

    use super::Token;
    use crate::{error::LexicalError::InvalidIntegerLiteral, lexer::lexer::Lexer};

    pub type MatchHandler = dyn Fn(&mut Lexer, regex::Match) -> crate::lexer::Result<()>;

    fn default_handler(token: Token) -> Box<MatchHandler> {
        Box::new(move |lexer, matched| {
            log::debug!("Handling token '{:?}'", token);
            lexer.consume(token.clone(), matched);
            Ok(())
        })
    }

    fn skip_handler() -> Box<MatchHandler> {
        Box::new(move |lexer, matched| {
            lexer.advance(matched.len());
            Ok(())
        })
    }

    fn integer_handler() -> Box<MatchHandler> {
        Box::new(move |lexer, matched| {
            log::debug!("Handling integer: {}", matched.as_str());

            let mut s = matched.as_str();
            let is_negative = s.starts_with('-');
            s = if is_negative { &s[1..] } else { s };

            let radix = match s.get(0..2) {
                Some("0b") => 2,
                Some("0o") => 8,
                Some("0x") => 16,
                _ => 10,
            };
            if radix != 10 {
                s = &s[2..]
            }

            log::debug!(
                "Identified it as: {}, base {}, digits: {}",
                if is_negative { "negative" } else { "positive" },
                radix,
                s
            );

            let mut num: i64 = 0;
            for digit in s.chars() {
                if digit == '_' {
                    continue;
                }
                let digit = digit.to_digit(radix).ok_or_else(|| {
                    InvalidIntegerLiteral(matched.as_str().to_string(), digit, radix)
                })?;

                num = num.wrapping_mul(radix as i64).wrapping_add(digit as i64);
            }
            num = if is_negative { num.wrapping_neg() } else { num };

            lexer.consume(Token::Integer(num), matched);
            Ok(())
        })
    }

    fn identifier_handler() -> Box<MatchHandler> {
        Box::new(move |lexer, matched| {
            log::debug!("Handling identifier: {}", matched.as_str());
            lexer.consume(Token::Identifier(matched.as_str().to_string()), matched);
            Ok(())
        })
    }

    impl Token {
        /// Lookup table for handler, that defines how this kind of token is in the input stream.
        pub(crate) fn get_handler(self) -> Box<MatchHandler> {
            use Token::*;
            match self {
                WhiteSpace => skip_handler(),
                Integer(_) => integer_handler(),
                Identifier(_) => identifier_handler(),
                _ => default_handler(self),
            }
        }
    }
}

pub mod patterns {

    use super::Token;
    use crate::lexer::token::handlers::MatchHandler;
    use regex::Regex;

    #[fully_pub::fully_pub]
    struct RegexPattern {
        handler: Box<MatchHandler>,
        regex: regex::Regex,
    }

    impl RegexPattern {
        fn from_kind(kind: Token) -> Self {
            RegexPattern {
                handler: kind.clone().get_handler(),
                regex: kind.get_regex(),
            }
        }
    }

    #[inline]
    pub(crate) fn all() -> Vec<RegexPattern> {
        use strum::IntoEnumIterator;
        let patterns = Token::iter().map(RegexPattern::from_kind);

        patterns.collect()
    }

    impl Token {
        /// Lookup table for regex expressions, matching all tokens of this kind.
        pub fn get_regex(self) -> Regex {
            use Token::*;
            let pattern = match self {
                Integer(_) => r"-?(?:(?:0[box])[0-9a-zA-Z_]+|[0-9_]+)",
                String(_) => r#"a^"#,
                Identifier(_) => r"[a-zA-Z_]{1}\w+",

                Eof => r"a^",
                WhiteSpace => r"\s+",

                Equals => r"==",
                NotEquals => r"!=",
                Less => r"<",
                LessOrEquals => r"<=",
                Greater => r">",
                GreaterOrEquals => r">=",
                Or => r"\|\|",
                Not => r"!",
                And => r"&&",

                Plus => r"\+",
                Minus => r"-",
                Slash => r"/",
                Asterisk => r"\*",
                Percent => r"%",

                HalfInterval => r"\.\.",
                ClosedInterval => r"\.\.=",
                Dot => r"\.",
                Assign => r"=",

                LeftBracket => r"\[",
                RightBracket => r"\]",
                LeftCurly => r"\{",
                RightCurly => r"\}",
                LeftParen => r"\(",
                RightParen => r"\)",

                Semicolon => r";",
                Colon => r":",
                Question => r"\?",
                Comma => r",",

                Let => r"a^",
                Const => r"a^",
                Fn => r"a^",
                Struct => r"a^",
                If => r"a^",
                Else => r"a^",
                For => r"a^",
                While => r"a^",
            };

            Regex::new(pattern).expect(&format!("CANNOT COMPILE REGEX FOR {:?}", self))
        }
    }
}
