/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use rusty::{cli::{CompileParameters, ParameterError, parse_parameters}, compile, compile_error::CompileError, compile_to_bitcode, compile_to_ir, compile_to_shared_object};
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let compile_parameters: Result<CompileParameters, ParameterError>  = parse_parameters(args);
    match compile_parameters {
        Ok(cp) => main_compile(cp),
        Err(err) => err.exit() // prints the nice message to std-out
    }
}

fn main_compile(parameters: CompileParameters) {
    let contents = fs::read_to_string(parameters.input.as_str()).expect(format!("Cannot read input file {}", parameters.input.as_str()).as_str());

    if parameters.output_bit_code {
        compile_to_bitcode(contents.to_string(),parameters.output.as_str()).unwrap();
    }else if parameters.output_ir {   
        generate_ir(contents.to_string(), parameters.output.as_str()).unwrap();
    }else if parameters.output_pic_obj {   
        compile_to_shared_object(contents.to_string(), parameters.output.as_str(), parameters.target).unwrap();
    }else if parameters.output_shared_obj {   
        compile_to_shared_object(contents.to_string(), parameters.output.as_str(), parameters.target).unwrap()
    }else if parameters.output_obj_code {
        compile(contents.to_string(), parameters.output.as_str(), parameters.target).unwrap();
    }else{
        //none is set, so we use default
        panic!("no output format defined");
    }
}
fn generate_ir(content : String, output: &str) -> Result<(), CompileError> {
    let ir = compile_to_ir(content)?;
    fs::write(output, ir).unwrap(); 
    Ok(())
}
