use rusty::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let gen_source = compile_to_ir(contents);    
    println!("{}", gen_source);
}

//fn _print_tokens(content: &str) {
//    let mut lexer = lexer::lex(content);
//
//    while lexer.token != lexer::Token::End && lexer.token != lexer::Token::Error {
//        println!(
//            "{word} -> {token:?}",
//            word = lexer.slice(),
//            token = lexer.token
//        );
//        lexer.advance();
//    }
//}
