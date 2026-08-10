#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Number(i64),

    Plus,
    Minus,
    Slash,
    Asterisk,

    LeftParen,
    RightParen,

    Eof,
    Bad,
}

impl TokenKind {
    pub fn trivial_token(c: char) -> Self {
        use TokenKind::*;

        match c {
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '/' => Slash,
            '(' => LeftParen,
            ')' => RightParen,
            _ => Bad,
        }
    }
}

#[fully_pub::fully_pub]
#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[fully_pub::fully_pub]
#[derive(Debug)]
struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self {
            kind,
            span: Span { start, end },
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    current_index: usize,
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            current_index: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.current_index == self.input.len() {
            let token = Some(Token::new(
                TokenKind::Eof,
                self.current_index,
                self.current_index,
            ));
            self.current_index += 1;
            return token;
        }
        if self.current_index > self.input.len() {
            return None;
        }

        let c = self.peek().unwrap();
        if c.is_digit(10) {
            return Some(self.consume_number());
        }

        let token = Some(Token::new(
            TokenKind::trivial_token(c),
            self.current_index,
            self.current_index,
        ));
        self.current_index += 1;
        return token;
    }

    fn consume_number(&mut self) -> Token {
        let start = self.current_index;

        let mut num = 0;
        while let Some(c) = self.peek()
            && c.is_digit(10)
        {
            num = 10 * num + c.to_digit(10).unwrap() as i64;
            self.current_index += 1;
        }
        Token::new(TokenKind::Number(num), start, self.current_index - 1)
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.current_index)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek()
            && c.is_whitespace()
        {
            self.current_index += 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_number() {
        let mut lexer = Lexer::new("233");

        let token = lexer.next_token().expect("Expected to get a token");
        assert_eq!(token.kind, TokenKind::Number(233))
    }

    #[test]
    fn bad_token() {
        let mut lexer = Lexer::new("233");

        let token = lexer.next_token().expect("Expected to get a token");
        assert_eq!(token.kind, TokenKind::Number(233))
    }
    #[test]
    fn number_and_operators() {
        let mut lexer = Lexer::new("12 + (3 * 4)");

        use TokenKind::*;
        let expected_kinds = vec![
            Number(12),
            Plus,
            LeftParen,
            Number(3),
            Asterisk,
            Number(4),
            RightParen,
            Eof,
        ];
        for expected in expected_kinds.into_iter() {
            let token = lexer.next_token();

            let token = token.expect(&format!("Missing expected token {:?}", expected));
            assert_eq!(token.kind, expected, "token kind '{:?}' doesn't match the expected one ({:?})", token.kind, expected);
        }
        assert!(lexer.next_token().is_none(), "Getting unexpected tokens")
    }
}
