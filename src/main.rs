use std::env;
use std::fs;

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Cannot read file");

    //Start lexing
    let lexer = lexer::lex(&contents);
    print_tokens(&contents);

    //Parse
    parser::parse(lexer);
}

fn print_tokens(content: &str) {
    let mut lexer = lexer::lex(content);

    while lexer.token != lexer::Token::End && lexer.token != lexer::Token::Error {
        println!(
            "{word} -> {token:?}",
            word = lexer.slice(),
            token = lexer.token
        );
        lexer.advance();
    }
}
