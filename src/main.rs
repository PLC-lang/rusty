use inkwell::context::Context;
use std::env;
use std::fs;
mod ast;
mod codegen;
mod lexer;
mod parser;
#[macro_use]
extern crate pretty_assertions;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Cannot read file");
    //Start lexing
    let lexer = lexer::lex(&contents);
    //print_tokens(&contents);

    //Parse
    let parse_result = parser::parse(lexer).unwrap();
    println!("{:#?}", parse_result.units[0]);
    //generate code
    let context = Context::create();
    let mut code_generator = codegen::CodeGen::new(&context);
    println!("{}", code_generator.generate(&parse_result));
}

fn _print_tokens(content: &str) {
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
