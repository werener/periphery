use lang::lexer::lexer::*;

fn main() {
    env_logger::init();

    let tokens = tokenize("hello_world");
    println!("{:?}", tokens)
}
