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
use std::path::Path;

use glob::glob;
use rusty::{
    cli::{CompileParameters, FormatOption, ParameterError},
    compile_to_bitcode, compile_to_ir, compile_to_shared_object, compile_to_shared_pic_object,
    compile_to_static_obj, FilePath, get_target_triple,
};
mod linker;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let compile_parameters: Result<CompileParameters, ParameterError> =
        CompileParameters::parse(args);
    match compile_parameters {
        Ok(cp) => {
            if let Err(msg) = main_compile(cp) {
                println!("Error: {}", msg);
                std::process::exit(1);
            }
        }
        Err(err) => err.exit(), // prints the nice message to std-out
    }
}

fn create_file_paths(inputs: &[String]) -> Result<Vec<FilePath>, String> {
    let mut sources = Vec::new();
    for input in inputs {
        let paths =
            glob(input).map_err(|e| format!("Failed to read glob pattern: {}, ({})", input, e))?;

        let source_count_before = sources.len();
        for p in paths {
            let path = p.map_err(|err| format!("Illegal path: {:}", err))?;
            sources.push(FilePath {
                path: path.to_string_lossy().to_string(),
            });
        }

        if sources.len() <= source_count_before {
            return Err(format!("No such file(s): {}", input));
        }
    }
    Ok(sources)
}

fn main_compile(parameters: CompileParameters) -> Result<(), String> {
    let sources = create_file_paths(&parameters.input)?;

    let output_filename = parameters.output_name(parameters.skip_linking).unwrap();
    let encoding = parameters.encoding;

    let out_format = parameters.output_format_or_default(); 
    match out_format {
        FormatOption::Static => {
            compile_to_static_obj(
                sources,
                encoding,
                output_filename.as_str(),
                parameters.target.clone(),
            )
            .unwrap();
        }
        FormatOption::Shared => {
            compile_to_shared_object(
                sources,
                encoding,
                output_filename.as_str(),
                parameters.target.clone(),
            )
            .unwrap();
        }
        FormatOption::PIC => {
            compile_to_shared_pic_object(
                sources,
                encoding,
                output_filename.as_str(),
                parameters.target.clone(),
            )
            .unwrap();
        }
        FormatOption::Bitcode => {
            compile_to_bitcode(sources, encoding, output_filename.as_str()).unwrap();
        }
        FormatOption::IR => {
            compile_to_ir(sources, encoding, &output_filename).unwrap();
        }
    }

    let linkable_formats = vec![
        FormatOption::Static,
        FormatOption::Shared,
        FormatOption::PIC
    ];
    if linkable_formats.contains(&out_format) && !parameters.skip_linking {
        let triple = get_target_triple(parameters.target);
        let mut linker = linker::create_with_target(triple.as_str().to_str().unwrap())?;
        linker.add_lib();
        linker.add_obj(Path::new(&output_filename));
        if out_format == FormatOption::Static {
            linker.build_exectuable(Path::new(&output_filename));
        } else {
            linker.build_shared_object(Path::new(&output_filename));
        }
        linker.finalize()?;
    }

    Ok(())
}
