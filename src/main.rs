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
use glob::glob;
use rusty::{
    cli::{CompileParameters, FormatOption, ParameterError},
    compile_error::CompileError,
    compile_to_bitcode, compile_to_ir, compile_to_shared_object, compile_to_static_obj, SourceCode,
    SourceContainer,
};
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let compile_parameters: Result<CompileParameters, ParameterError> =
        CompileParameters::parse(args);
    match compile_parameters {
        Ok(cp) => main_compile(cp),
        Err(err) => err.exit(), // prints the nice message to std-out
    }
}

struct FilePath {
    path: String,
}

impl SourceContainer for FilePath {
    fn load_source(&self) -> Result<SourceCode, String> {
        //why do I need to clone here :-( ???
        let path = self.get_location().to_string();
        fs::read_to_string(self.path.to_string())
            .map(move |source| SourceCode { source, path })
            .map_err(|err| format!("{:}", err))
    }

    fn get_location(&self) -> &str {
        self.path.as_str()
    }
}

fn create_file_paths(inputs: &[String]) -> Result<Vec<FilePath>, String> {
    let mut sources = Vec::new();
    for input in inputs {
        let paths =
            glob(input).map_err(|e| format!("Failed to read glob pattern: {}, ({})", input, e))?;

        for p in paths {
            let path = p.map_err(|err| format!("Illegal path: {:}", err))?;
            sources.push(FilePath {
                path: path.to_string_lossy().to_string(),
            });
        }
    }
    Ok(sources)
}

fn main_compile(parameters: CompileParameters) {
    let file_paths = create_file_paths(&parameters.input).unwrap();
    let sources: Vec<_> = file_paths
        .iter()
        .map(|it| it as &dyn SourceContainer)
        .collect::<Vec<_>>();

    let sources = sources.as_slice();
    let output_filename = parameters.output_name();

    println!("output_filename: {}", output_filename);

    match parameters.output_format_or_default() {
        FormatOption::Static => {
            compile_to_static_obj(sources, output_filename.as_str(), parameters.target).unwrap();
        }
        FormatOption::Shared | FormatOption::PIC => {
            compile_to_shared_object(sources, output_filename.as_str(), parameters.target).unwrap();
        }
        FormatOption::Bitcode => {
            compile_to_bitcode(sources, output_filename.as_str()).unwrap();
        }
        FormatOption::IR => {
            generate_ir(sources, output_filename.as_str()).unwrap();
        }
        _ => panic!("output_format_or_default() should not return None!"),
    }
}
fn generate_ir(sources: &[&dyn SourceContainer], output: &str) -> Result<(), CompileError> {
    let ir = compile_to_ir(sources)?;
    fs::write(output, ir).unwrap();
    Ok(())
}
