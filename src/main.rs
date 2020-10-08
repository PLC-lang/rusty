/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use core::slice::Iter;
use rusty::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let parameters = read_params(args.as_slice());
    let contents = fs::read_to_string(parameters.input).expect("Cannot read file");
    match parameters.output_type {
        OutputType::IR => generate_ir(contents.to_string(), parameters.output.as_str(), parameters.debug),
        OutputType::ObjectCode => compile(contents.to_string(), parameters.output.as_str(), parameters.target, parameters.debug),
        OutputType::PicObject => compile_to_shared_object(contents.to_string(), parameters.output.as_str(), parameters.target, parameters.debug),
        OutputType::SharedObject => compile_to_shared_object(contents.to_string(), parameters.output.as_str(), parameters.target, parameters.debug),
        OutputType::Bitcode => compile_to_bitcode(contents.to_string(),parameters.output.as_str(), parameters.debug),
    }
}

fn generate_ir(content : String, output: &str, enable_debug : bool) {
    let ir = compile_to_ir(content, enable_debug);
    fs::write(output, ir).unwrap(); 
}

struct CompileParameters {
    input: String,
    output: String,
    output_type: OutputType,
    target : Option<String>,
    debug: bool,
}

enum OutputType {
    IR,
    SharedObject,
    PicObject,
    ObjectCode,
    Bitcode,
}

fn read_params(args: &[String]) -> CompileParameters {
    let mut result = CompileParameters {
        input: "".to_string(),
        output: "a.out".to_string(),
        output_type: OutputType::PicObject,
        target : None,
        debug : false,
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
        "--static" => parameters.output_type = OutputType::ObjectCode,
        "--shared" => parameters.output_type = OutputType::SharedObject,
        "--pic" => parameters.output_type = OutputType::PicObject,
        "-g" | "--debug" => parameters.debug = true,
        "--target" => {
            //Resolve the target here since the --target was specified
            parameters.target = Some(iterator.next()
                .expect("Target not specified")
                .to_string())
        }
        _ => panic!("Unkown parameter {}", option),
    }
}
