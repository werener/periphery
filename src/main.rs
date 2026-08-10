use lang::ast::lexer::*;

fn main() {
    let mut lexer = Lexer::new("12 + 3 * 4");
    while let Some(token) = lexer.next_token() {
        println!("{:?}",token.kind);
    }
}
