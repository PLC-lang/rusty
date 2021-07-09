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

use ast::{NewLines, SourceRange};
use compile_error::CompileError;
use index::Index;
use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};

use crate::ast::CompilationUnit;
mod ast;
pub mod cli;
mod codegen;
pub mod compile_error;
pub mod index;
mod lexer;
mod parser;
mod typesystem;

#[macro_use]
extern crate pretty_assertions;

pub type Sources<'a> = [&'a dyn SourceContainer];

/// SourceContainers offer source-code to be compiled via the load_source function.
/// Furthermore it offers a location-String used when reporting diagnostics.
pub trait SourceContainer {
    /// loads and returns the SourceEntry that contains the SourceCode and the path it was loaded from
    fn load_source(&self) -> Result<SourceCode, String>;
    /// returns the location of this source-container. Used when reporting diagnostics.
    fn get_location(&self) -> &str;
}

/// The SourceCode unit is the smallest unit of compilation that can be passed to the compiler
#[derive(Clone)]
pub struct SourceCode {
    /// the source code to be compiled
    pub source: String,
    /// the location this code was loaded from
    pub path: String,
}

impl SourceCode {
    /// casts the SourceCode into a SourceContainer
    pub fn as_source_container(&self) -> &dyn SourceContainer {
        self
    }
}

/// tests can provide a SourceCode directly
impl SourceContainer for SourceCode {
    fn load_source(&self) -> Result<SourceCode, String> {
        Ok(self.clone())
    }

    fn get_location(&self) -> &str {
        self.path.as_str()
    }
}

///
/// Compiles the given source into an object file and saves it in output
///
fn compile_to_obj(
    sources: &Sources,
    output: &str,
    reloc: RelocMode,
    triple: Option<String>,
) -> Result<(), CompileError> {
    let initialization_config = &InitializationConfig::default();
    Target::initialize_all(initialization_config);

    let triple = triple
        .map(|it| TargetTriple::create(it.as_str()))
        .or_else(|| Some(TargetMachine::get_default_triple()))
        .unwrap();
    let target = Target::from_triple(&triple).unwrap();
    let machine = target
        .create_target_machine(
            &triple,
            //TODO : Add cpu features as optionals
            "generic", //TargetMachine::get_host_cpu_name().to_string().as_str(),
            "",        //TargetMachine::get_host_cpu_features().to_string().as_str(),
            //TODO Optimisation as parameter
            inkwell::OptimizationLevel::Default,
            reloc,
            CodeModel::Default,
        )
        .unwrap();

    let c = Context::create();
    let code_generator = compile_module(&c, sources)?;
    machine
        .write_to_file(&code_generator.module, FileType::Object, Path::new(output))
        .unwrap();

    Ok(())
}

/// Compiles a given source string to a static object and saves the output.
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_static_obj(
    sources: &Sources,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(sources, output, RelocMode::Default, target)
}

/// Compiles a given source string to a shared position independent object and saves the output.
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_shared_pic_object(
    sources: &Sources,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(sources, output, RelocMode::PIC, target)
}

/// Compiles a given source string to a dynamic non PIC object and saves the output.
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_shared_object(
    sources: &Sources,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(sources, output, RelocMode::DynamicNoPic, target)
}

///
/// Compiles the given source into a bitcode file
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `output` - the location on disk to save the output
pub fn compile_to_bitcode(sources: &Sources, output: &str) -> Result<(), CompileError> {
    let path = Path::new(output);
    let c = Context::create();
    let code_generator = compile_module(&c, sources)?;
    code_generator.module.write_bitcode_to_path(path);
    Ok(())
}

///
/// Compiles the given source into LLVM IR and returns it
///
/// # Arguments
///
/// * `sources` - the source to be compiled
pub fn compile_to_ir(sources: &Sources) -> Result<String, CompileError> {
    let c = Context::create();
    let code_gen = compile_module(&c, sources)?;
    Ok(code_gen.module.print_to_string().to_string())
}

///
/// Compiles the given source into a `codegen::CodeGen` using the provided context
///
/// # Arguments
///
/// * `context` - the LLVM Context to be used for the compilation
/// * `sources` - the source to be compiled
pub fn compile_module<'c>(
    context: &'c Context,
    sources: &Sources,
) -> Result<codegen::CodeGen<'c>, CompileError> {
    let mut full_index = Index::new();
    let mut unit = CompilationUnit::default();
    for container in sources {
        let e = container
            .load_source()
            .map_err(|err| CompileError::io_error(err, container.get_location().to_string()))?;
        let (mut parse_result, _) = parse(e.path.as_str(), e.source.as_str())?; //TODO dont clone the source!
                                                                                //first pre-process the AST
        ast::pre_process(&mut parse_result);
        //then index the AST
        full_index.import(index::visitor::visit(&parse_result));
        unit.import(parse_result);
    }

    //and finally codegen
    let code_generator = codegen::CodeGen::new(context, "main");
    code_generator.generate(unit, &full_index)?;
    Ok(code_generator)
}

fn parse(file_path: &str, source: &str) -> Result<(ast::CompilationUnit, NewLines), CompileError> {
    //Start lexing
    let lexer = lexer::lex(file_path, source);
    //Parse
    //TODO : Parser should also return compile errors with sane locations
    parser::parse(lexer).map_err(|err| CompileError::codegen_error(err, SourceRange::undefined()))
}
