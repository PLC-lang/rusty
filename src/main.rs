// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
//! A Structured Text LLVM Frontent
//!
//! RuSTy is an [`ST`] Compiler using LLVM
//!
//! # Features
//! ## Standard language support
//! Most of the [`IEC61131-3`] standard for ST and general programing is supported.
//! ## Native compilation
//! A (currently) single ST files into object code using LLVM.
//! A compiled object can be linked statically or dynamically linked
//!     with other programs using standard compiler linkers (ld, clang, gcc)
//! ## IR Output
//! An [`IR`] file can be generated from any given ST file in order to examin the generated LLVM IR code.
//! For a usage guide refer to the [User Documentation](../../)
//!
//! [`ST`]: https://en.wikipedia.org/wiki/Structured_text
//! [`IEC61131-3`]: https://en.wikipedia.org/wiki/IEC_61131-3
//! [`IR`]: https://llvm.org/docs/LangRef.html
use rusty::{
    cli::{parse_parameters, CompileParameters, ParameterError},
    compile_error::CompileError,
    compile_to_bitcode, compile_to_ir, compile_to_shared_object, compile_to_static_obj,
};
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let compile_parameters: Result<CompileParameters, ParameterError> = parse_parameters(args);
    match compile_parameters {
        Ok(cp) => main_compile(cp),
        Err(err) => err.exit(), // prints the nice message to std-out
    }
}

fn main_compile(parameters: CompileParameters) {
    let contents = fs::read_to_string(parameters.input.as_str())
        .unwrap_or_else(|_| panic!("Cannot read input file {}", parameters.input.as_str()));

    if parameters.output_bit_code {
        compile_to_bitcode(contents, parameters.output.as_str()).unwrap();
    } else if parameters.output_ir {
        generate_ir(contents, parameters.output.as_str()).unwrap();
    } else if parameters.output_pic_obj {
        compile_to_shared_object(contents, parameters.output.as_str(), parameters.target).unwrap();
    } else if parameters.output_shared_obj {
        compile_to_shared_object(contents, parameters.output.as_str(), parameters.target).unwrap()
    } else if parameters.output_obj_code {
        compile_to_static_obj(contents, parameters.output.as_str(), parameters.target).unwrap();
    } else {
        //none is set, so we use default
        panic!("no output format defined");
    }
}
fn generate_ir(content: String, output: &str) -> Result<(), CompileError> {
    let ir = compile_to_ir(content)?;
    fs::write(output, ir).unwrap();
    Ok(())
}
