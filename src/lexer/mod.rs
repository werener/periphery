
pub mod lexer;
pub mod token;

use crate::error::LexicalError;
pub type Result<T> = std::result::Result<T, LexicalError>;

#[cfg(test)]
mod test {
    use super::*;
    use Token::*;
    use rstest::rstest;
    use token::Token;

    fn assert_tokenization(input: &str, expected: Result<Vec<Token>>) {
        let tokens = lexer::tokenize(input);

        let mut debug_message = "".to_string();
        if tokens != expected && tokens.is_ok() && expected.is_ok() {
            let (tokens, expected) = (tokens.as_ref().unwrap(), expected.as_ref().unwrap());
            let mut i = 0;
            while i < tokens.len() && i < expected.len() {
                if tokens[i] != expected[i] {
                    debug_message = format!("Expected {:?} at position {i}. Got: {:?}.\n",  expected[i], tokens[i]);
                    break;
                }
                i += 1;
            }
        }
        assert_eq!(tokens, expected, "\nLexer error. {debug_message}");
    }

    #[rstest]
    #[case::trivial("hello_world", vec!["hello_world"])]
    fn test_identifiers(#[case] input: &str, #[case] expected_names: Vec<&str>) {
        let mut expected = expected_names.iter().map(|name| Identifier(name.to_string())).collect::<Vec<Token>>();
        expected.push(Eof);
        
        assert_tokenization(input, Ok(expected));
    }

    #[rstest]
    #[case::empty_input("", Ok(vec![Eof]))]
    #[case::whitespace("\r {     }\t\t[ ]\x0C", Ok(vec![LeftCurly, RightCurly, LeftBracket, RightBracket, Eof]))]
    fn test_skipping(#[case] input: &str, #[case] expected: Result<Vec<Token>>) {
        assert_tokenization(input, expected);
    }

    #[rstest]
    #[case::base2_number("0b110", Ok(vec![Integer(6), Eof]))]
    #[case::base2_negative_number("-0b110", Ok(vec![Integer(-6), Eof]))]
    #[case::base8_number("0o105", Ok(vec![Integer(69), Eof]))]
    #[case::base8_negative_number("-0o110", Ok(vec![Integer(-72), Eof]))]
    #[case::base10_number("201", Ok(vec![Integer(201), Eof]))]
    #[case::base10_negative_number("-201", Ok(vec![Integer(-201), Eof]))]
    #[case::base16_number("0xF0FF", Ok(vec![Integer(61695), Eof]))]
    #[case::base16_negative_number("-0x2A3", Ok(vec![Integer(-675), Eof]))]
    #[case::base10_separator("1_250_000", Ok(vec![Integer(1_250_000), Eof]))]
    fn test_number(#[case] input: &str, #[case] expected: Result<Vec<Token>>) {
        assert_tokenization(input, expected);
    }

    #[rstest]
    #[case::plus("100 + 50", Ok(vec![Integer(100), Plus, Integer(50), Eof]))]
    #[case::minus("100 - 50", Ok(vec![Integer(100), Minus, Integer(50), Eof]))]
    #[case::slash("100 / 50", Ok(vec![Integer(100), Slash, Integer(50), Eof]))]
    #[case::asterisk("100 * 50", Ok(vec![Integer(100), Asterisk, Integer(50), Eof]))]
    #[case::percent("100 % 50", Ok(vec![Integer(100), Percent, Integer(50), Eof]))]
     #[case::parenthesis("[] {} ()", Ok(vec![LeftBracket, RightBracket, LeftCurly, RightCurly, LeftParen, RightParen, Eof]))]
    #[case::all_binary_operators(
        "|| && > >= == = 
        <= < ! ! != ! =",
        Ok(vec![Or, And, Greater, GreaterOrEquals, Equals, Assign, 
        LessOrEquals, Less, Not, Not, NotEquals, Not, Assign, Eof])
    )]
    #[case::common_lexemes(
        "= <= >= = == ! = != ..= . .. . = ..=",
        Ok(vec![Assign, LessOrEquals, GreaterOrEquals, Assign, Equals, Not, Assign, NotEquals,
        ClosedInterval, Dot, HalfInterval, Dot, Assign, ClosedInterval, Eof]))]
    #[case::trivial_lexemes(
        "1 ? (100, 100) : (50, 50);", 
        Ok(vec![Integer(1), Question, LeftParen, Integer(100), Comma, Integer(100), RightParen, 
        Colon, LeftParen, Integer(50), Comma, Integer(50), RightParen, Semicolon, Eof]))]
    fn test_operations(#[case] input: &str, #[case] expected: Result<Vec<Token>>) {
        assert_tokenization(input, expected);
    }

   
}
