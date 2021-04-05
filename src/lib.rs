/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::path::Path;

use ast::NewLines;
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
    source: String,
    output: &str,
    reloc: RelocMode,
    triple: Option<String>,
) -> Result<(), CompileError> {
    let path = Path::new(output);

    let context = Context::create();
    let code_generator = compile_module(&context, source)?;
    let initialization_config = &InitializationConfig::default();
    Target::initialize_all(initialization_config);

    //TODO get triple as parameter.

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

pub fn compile(source: String, output: &str, target: Option<String>) -> Result<(), CompileError> {
    compile_to_obj(source, output, RelocMode::Default, target)
}

pub fn compile_to_shared_pic_object(
    source: String,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(source, output, RelocMode::PIC, target)
}

pub fn compile_to_shared_object(
    source: String,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(source, output, RelocMode::DynamicNoPic, target)
}

///
/// Compiles the given source into a bitcode file
///
pub fn compile_to_bitcode(source: String, output: &str) -> Result<(), CompileError> {
    let path = Path::new(output);

    let context = Context::create();
    let code_generator = compile_module(&context, source)?;
    code_generator.module.write_bitcode_to_path(path);
    Ok(())
}

///
/// Compiles the given source into LLVM IR and returns it
///
pub fn compile_to_ir(source: String) -> Result<String, CompileError> {
    let context = Context::create();
    let code_gen = compile_module(&context, source)?;
    Ok(get_ir(&code_gen))
}

pub fn get_ir(codegen: &codegen::CodeGen) -> String {
    codegen.module.print_to_string().to_string()
}

pub fn compile_module(context: &Context, source: String) -> Result<codegen::CodeGen, CompileError> {
    let (mut parse_result, _) = parse(source);
    //first pre-process the AST
    ast::pre_process(&mut parse_result);
    //then index the AST
    let index = index::visitor::visit(&parse_result);
    //and finally codegen
    let code_generator = codegen::CodeGen::new(context, "main");
    code_generator.generate(parse_result, &index)?;
    Ok(code_generator)
}

fn parse(source: String) -> (ast::CompilationUnit, NewLines) {
    //Start lexing
    let lexer = lexer::lex(&source);
    //Parse
    parser::parse(lexer).unwrap()
}
