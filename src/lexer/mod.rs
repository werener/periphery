pub mod lexer;
pub mod token;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{error::LexicalError, lexer::token::*};
    use TokenKind::*;
    use lexer::*;
    use rstest::rstest;

    #[rstest]
    #[case("[ { [ ) (", Ok(vec![LeftBracket, LeftCurly, LeftBracket, RightParen, LeftParen]))]
    #[case("201", Ok(vec![Number(201)]))]
    fn test_pattern(#[case] input: &str, #[case] expected: Result<Vec<TokenKind>, LexicalError>) {
        let tokens = tokenize(input);

        assert_eq!(tokens, expected, "Wrong tokenization");
    }
}
