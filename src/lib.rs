use std::path::Path;

use inkwell::targets::{Target, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::context::Context;

use crate::index::Index;

mod ast;
mod codegen;
mod lexer;
mod parser;
pub mod index;
#[macro_use]
extern crate pretty_assertions;

///
/// Compiles the given source into an object file and saves it in output
/// 
pub fn compile(source : String, output : &str) {
    let context = Context::create();
    let mut index = Index::new();
    let path = Path::new(output);
    
    let code_generator = compile_module(&context, &mut index, source);

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).unwrap();
    let machine = target.create_target_machine(
        &triple, 
        "", 
        "", 
        inkwell::OptimizationLevel::Default, 
        RelocMode::Default, 
        CodeModel::Default
    ).unwrap();
    machine.write_to_file(&code_generator.module, FileType::Object, path).unwrap();
}

pub fn create_index<'ctx>() -> Index<'ctx>{
    Index::new()
}
 
///
/// Compiles the given source into LLVM IR and returns it 
/// 
pub fn compile_to_ir(source : String) -> String {
    let context = Context::create();
    let mut index = Index::new();
    let code_gen = compile_module(&context, &mut index, source);
    get_ir(&code_gen)
}

pub fn get_ir(codegen : &codegen::CodeGen) -> String {
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

fn parse(source : String) -> ast::CompilationUnit {
    //Start lexing
    let lexer = lexer::lex(&source);
    //Parse
    parser::parse(lexer).unwrap()
}