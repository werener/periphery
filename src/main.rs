use lang::lexer::lexer::*;
use log::{debug, info};


fn main() {
    env_logger::init();

    let tokens = tokenize("[{ [) (");
    println!("{:?}", tokens)
}
