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
use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};

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

///
/// Compiles the given source into an object file and saves it in output
///
fn compile_to_obj(
    file_path: &str,
    source: String,
    output: &str,
    reloc: RelocMode,
    triple: Option<String>,
) -> Result<(), CompileError> {
    let path = Path::new(output);

    let context = Context::create();
    let code_generator = compile_module(&context, file_path, source)?;
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

    machine
        .write_to_file(&code_generator.module, FileType::Object, path)
        .unwrap();

    Ok(())
}

/// Compiles a given source string to a static object and saves the output.
///
/// # Arguments
///
/// * `source` - the source to be compiled
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_static_obj(
    file_path: &str,
    source: String,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(file_path, source, output, RelocMode::Default, target)
}

/// Compiles a given source string to a shared position independent object and saves the output.
///
/// # Arguments
///
/// * `source` - the source to be compiled
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_shared_pic_object(
    file_path: &str,
    source: String,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(file_path, source, output, RelocMode::PIC, target)
}

/// Compiles a given source string to a dynamic non PIC object and saves the output.
///
/// # Arguments
///
/// * `source` - the source to be compiled
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_shared_object(
    file_path: &str,
    source: String,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(file_path, source, output, RelocMode::DynamicNoPic, target)
}

///
/// Compiles the given source into a bitcode file
///
/// # Arguments
///
/// * `source` - the source to be compiled
/// * `output` - the location on disk to save the output
pub fn compile_to_bitcode(
    file_path: &str,
    source: String,
    output: &str,
) -> Result<(), CompileError> {
    let path = Path::new(output);

    let context = Context::create();
    let code_generator = compile_module(&context, file_path, source)?;
    code_generator.module.write_bitcode_to_path(path);
    Ok(())
}

///
/// Compiles the given source into LLVM IR and returns it
///
/// # Arguments
///
/// * `source` - the source to be compiled
pub fn compile_to_ir(file_path: &str, source: String) -> Result<String, CompileError> {
    let context = Context::create();
    let code_gen = compile_module(&context, file_path, source)?;
    Ok(get_ir(&code_gen))
}

fn get_ir(codegen: &codegen::CodeGen) -> String {
    codegen.module.print_to_string().to_string()
}

///
/// Compiles the given source into a `codegen::CodeGen` using the provided context
///
/// # Arguments
///
/// * `context` - the LLVM Context to be used for the compilation
/// * `source` - the source to be compiled
pub fn compile_module<'ink, 'file>(
    context: &'ink Context,
    file_path: &'file str,
    source: String,
) -> Result<codegen::CodeGen<'ink>, CompileError> {
    let (mut parse_result, _) = parse(file_path, source)?;
    //first pre-process the AST
    ast::pre_process(&mut parse_result);
    //then index the AST
    let index = index::visitor::visit(&parse_result);
    //and finally codegen
    let code_generator = codegen::CodeGen::new(context, "main");
    code_generator.generate(parse_result, &index)?;
    Ok(code_generator)
}

fn parse(
    file_path: &str,
    source: String,
) -> Result<(ast::CompilationUnit, NewLines), CompileError> {
    //Start lexing
    let lexer = lexer::lex(file_path, &source);
    //Parse
    //TODO : Parser should also return compile errors with sane locations
    parser::parse(lexer).map_err(|err| CompileError::codegen_error(err, SourceRange::undefined()))
}
