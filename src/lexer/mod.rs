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

    fn assert_tokenization(input: &str, mut expected: Result<Vec<Token>>) {
        let tokens = lexer::tokenize(input);
        if expected.is_ok() && expected.as_ref().unwrap().last().expect("Empty expect vector") != &Eof {
            expected.as_mut().unwrap().push(Eof);
        }
        if tokens != expected && tokens.is_ok() && expected.is_ok() {
            panic!("\nLexer error. {}", crate::utils::print_difference(tokens.unwrap(), expected.unwrap()))
        }
        assert_eq!(tokens, expected, "Lexer error.\n");
    }

    fn deserialize_testcase(path: std::path::PathBuf) -> (String, Vec<Token>) {
        let content = std::fs::read_to_string(path).unwrap();
        let content = content.split("EXPECTED:\n").collect::<Vec<&str>>();

        let input = content.get(0).expect("Wrong case format").to_string();
        let expected = content.get(1).expect("Wrong case format: No \"EXPECTED:\"").replace("\n", " ");
        
        let expected_tokens: Vec<Token> = serde_json::from_str(expected.as_str()).expect("WRONG FORMAT: expected array of tokens isn't serializable");
        (input, expected_tokens)
    }
    
    /// Test grammatically consistent snippets from tests/lexer/cases
    #[rstest]
    fn test_real_grammar(#[files("tests/lexer/cases/*.case")] path: std::path::PathBuf) {
        let (input, expected) = deserialize_testcase(path);

        assert_tokenization(input.as_str(), Ok(expected));
    }

    /// Test skippable characters
    #[rstest]
    #[case::empty_input("", Ok(vec![Eof]))]
    #[case::whitespace("\r {     }\t\t[ ]\x0C", Ok(vec![LeftCurly, RightCurly, LeftBracket, RightBracket, Eof]))]
    fn test_skipping(#[case] input: &str, #[case] expected: Result<Vec<Token>>) {
        assert_tokenization(input, expected);
    }
    
    /// Test identifiers
    #[rstest]
    #[case::trivial("hello_world", vec!["hello_world"])]
    #[case::underscored("_12 a_1 __a _", vec!["_12", "a_1", "__a", "_"])]
    fn test_identifiers(#[case] input: &str, #[case] expected_names: Vec<&str>) {
        let mut expected = expected_names.iter().map(|name| Identifier(name.to_string())).collect::<Vec<Token>>();
        expected.push(Eof);
        assert_tokenization(input, Ok(expected));
    }

    /// Test keywords
    #[rstest]
    #[case::keyword_matching(
        "let const fn struct if else for while", 
        Ok(vec![Let, Const, Fn, Struct, If, Else, For, While, Eof])
    )]
    #[case::indentifiers_containing_keywords(
        "letter 2+xconstx 
        fn fnfn ;construct;
        elif {} else for while{}", 
        Ok(vec![Identifier("letter".to_string()), Integer(2), Plus, Identifier("xconstx".to_string()), 
        Fn, Identifier("fnfn".to_string()), Semicolon, Identifier("construct".to_string()), Semicolon,
        Identifier("elif".to_string()), LeftCurly, RightCurly, Else, For, While, LeftCurly, RightCurly, Eof])
    )]
    #[case::identifiers_separated(
        "let;const+elif-if/else", 
        Ok(vec![Let, Semicolon, Const, Plus, Identifier("elif".to_string()), Minus, If, Slash, Else, Eof])
    )]
    fn test_keywords(#[case] input: &str, #[case] expected: Result<Vec<Token>>) {
        assert_tokenization(input, expected);
    }

    /// Test integers
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
    fn test_integer(#[case] input: &str, #[case] expected: Result<Vec<Token>>) {
        assert_tokenization(input, expected);
    }

    /// Test most other lexemes
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
        ClosedInterval, Dot, HalfInterval, Dot, Assign, ClosedInterval, Eof])
    )]
    #[case::trivial_lexemes(
        "1 ? (100, 100) : (50, 50);", 
        Ok(vec![Integer(1), Question, LeftParen, Integer(100), Comma, Integer(100), RightParen, 
        Colon, LeftParen, Integer(50), Comma, Integer(50), RightParen, Semicolon, Eof])
    )]
    fn test_operations(#[case] input: &str, #[case] expected: Result<Vec<Token>>) {
        assert_tokenization(input, expected);
    } 
}
