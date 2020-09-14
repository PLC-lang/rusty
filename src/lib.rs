/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::path::Path;

use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};

use crate::index::Index;

mod ast;
mod codegen;
pub mod index;
mod lexer;
mod parser;
#[macro_use]
extern crate pretty_assertions;

///
/// Compiles the given source into an object file and saves it in output
///
pub fn compile(source: String, output: &str) {
    let context = Context::create();
    let mut index = Index::new();
    let path = Path::new(output);

    let code_generator = compile_module(&context, &mut index, source);
    let initialization_config = &InitializationConfig::default();
    Target::initialize_all(initialization_config);

    //TODO get triple as parameter.
    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).unwrap();
    let machine = target
        .create_target_machine(
            &triple,
            TargetMachine::get_host_cpu_name().to_string().as_str(),
            TargetMachine::get_host_cpu_features().to_string().as_str(),
            //TODO Optimisation as parameter
            inkwell::OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    machine
        .write_to_file(&code_generator.module, FileType::Object, path)
        .unwrap();
}

///
/// Compiles the given source into a bitcode file
///
pub fn compile_to_bitcode(source : String, output: &str) {
    let context = Context::create();
    let mut index = Index::new();
    let path = Path::new(output);

    let code_generator = compile_module(&context, &mut index, source);
    code_generator.module.write_bitcode_to_path(path);
}

pub fn create_index<'ctx>() -> Index<'ctx> {
    Index::new()
}

///
/// Compiles the given source into LLVM IR and returns it
///
pub fn compile_to_ir(source: String) -> String {
    let context = Context::create();
    let mut index = Index::new();
    let code_gen = compile_module(&context, &mut index, source);
    get_ir(&code_gen)
}

pub fn get_ir(codegen: &codegen::CodeGen) -> String {
    codegen.module.print_to_string().to_string()
}

pub fn compile_module<'ctx>(context : &'ctx Context, index: &'ctx mut Index<'ctx>, source : String) -> codegen::CodeGen<'ctx> {

    let mut parse_result = parse(source);
    //first pre-process the AST
    index.pre_process(&mut parse_result);
    //then index the AST
    index.visit(&mut parse_result);
    //and finally codegen
    let mut code_generator = codegen::CodeGen::new(context, index);
    code_generator.generate_compilation_unit(parse_result);
    code_generator
}

fn parse(source: String) -> ast::CompilationUnit {
    //Start lexing
    let lexer = lexer::lex(&source);
    //Parse
    parser::parse(lexer).unwrap()
}
