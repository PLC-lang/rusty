use core::slice::Iter;
use rusty::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let parameters = read_params(args.as_slice());
    let contents = fs::read_to_string(parameters.input).expect("Cannot read file");
    match parameters.output_type {
        OutputType::IR => generate_ir(contents.to_string(), parameters.output.as_str()),
        OutputType::ObjectCode => compile(contents.to_string(), parameters.output.as_str()),
        OutputType::Bitcode => compile_to_bitcode(contents.to_string(),parameters.output.as_str()),
    }
}

fn generate_ir(content : String, output: &str) {
    let ir = compile_to_ir(content);
    fs::write(output, ir).unwrap(); 
}

struct CompileParameters {
    input: String,
    output: String,
    output_type: OutputType,
}

enum OutputType {
    IR,
    ObjectCode,
    Bitcode,
}

fn read_params(args: &[String]) -> CompileParameters {
    let mut result = CompileParameters {
        input: "".to_string(),
        output: "a.out".to_string(),
        output_type: OutputType::ObjectCode,
    };

    let mut args_iter = args.iter();
    let _application_name = args_iter.next();
    while let Some(arg) = args_iter.next() {
        if arg.starts_with("-") {
            parse_argument(&mut result, arg, &mut args_iter);
        } else {
            if !result.input.is_empty() {
                panic!("Input already defined");
            }
            result.input = arg.to_string();
        }
    }
    if result.input.is_empty() {
        panic!("Input not set");
    }
    result
}

fn parse_argument(
    parameters: &mut CompileParameters,
    option: &String,
    iterator: &mut Iter<String>,
) {
    match option.as_str() {
        "--output" | "-o" => {
            parameters.output = iterator
                .next()
                .expect("Output file not specified")
                .to_string()
        }
        "--bc" => parameters.output_type = OutputType::Bitcode,
        "--ir" => parameters.output_type = OutputType::IR,
        _ => panic!("Unkown parameter {}", option),
    }
}
