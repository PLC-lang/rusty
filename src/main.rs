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

use rusty::build;
use rusty::cli::{CompileParameters, ParameterError};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let compile_parameters: Result<CompileParameters, ParameterError> =
        CompileParameters::parse(args);
    match compile_parameters {
        Ok(cp) => {
            if let Err(msg) = build(cp) {
                println!("Error: {:?}", msg);
                std::process::exit(1);
            }
        }
        Err(err) => err.exit(), // prints the nice message to std-out
    }
}
