use lang::lexer::{lexer::*, token::Token};

fn main() {
    env_logger::init();

    let tokens = tokenize("100 + 50");
    // "100 + 50", Ok(vec![Integer(100), Plus, Integer(50), Eof]))
    let res = lang::utils::print_difference(
        tokens.unwrap(),
        vec![
            Token::Integer(100),
            Token::Plus,
            Token::Integer(5),
            Token::Integer(0),
            Token::Eof,
        ],
    );
    println!("{}", res)
}
