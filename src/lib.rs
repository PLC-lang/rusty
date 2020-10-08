/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::path::Path;

use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};

use crate::index::Index;
use crate::codegen::debugger::DebugManager;

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
fn compile_to_obj(
    source: String,
    output: &str,
    reloc: RelocMode,
    triple: Option<String>,
    enable_debug: bool,
) {
    let context = Context::create();
    let mut index = Index::new();
    let path = Path::new(output);

    let code_generator = compile_module(&context, &mut index, source, enable_debug);
    let initialization_config = &InitializationConfig::default();
    Target::initialize_all(initialization_config);

    //TODO get triple as parameter.

    let triple = triple
        .map(|it| TargetTriple::create(it.as_str()))
        .or(Some(TargetMachine::get_default_triple()))
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
}

pub fn compile(source: String, output: &str, target: Option<String>, enable_debug: bool) {
    compile_to_obj(source, output, RelocMode::Default, target, enable_debug);
}

pub fn compile_to_shared_pic_object(source: String, output: &str, target: Option<String>, enable_debug: bool) {
    compile_to_obj(source, output, RelocMode::PIC, target, enable_debug);
}

pub fn compile_to_shared_object(source: String, output: &str, target: Option<String>, enable_debug: bool) {
    compile_to_obj(source, output, RelocMode::DynamicNoPic, target, enable_debug);
}

///
/// Compiles the given source into a bitcode file
///
pub fn compile_to_bitcode(source: String, output: &str, enable_debug: bool) {
    let context = Context::create();
    let mut index = Index::new();
    let path = Path::new(output);

    let code_generator = compile_module(&context, &mut index, source, enable_debug);
    code_generator.module.write_bitcode_to_path(path);
}

pub fn create_index<'ctx>() -> Index<'ctx> {
    Index::new()
}

///
/// Compiles the given source into LLVM IR and returns it
///
pub fn compile_to_ir(source: String, enable_debug: bool) -> String {
    let context = Context::create();
    let mut index = Index::new();
    let code_gen = compile_module(&context, &mut index, source, enable_debug);
    get_ir(&code_gen)
}

pub fn get_ir(codegen: &codegen::CodeGen) -> String {
    codegen.module.print_to_string().to_string()
}

pub fn compile_module<'ctx>(
    context: &'ctx Context,
    index: &'ctx mut Index<'ctx>,
    //path: &str,
    source: String,
    enable_debug: bool,
) -> codegen::CodeGen<'ctx> {
    let mut parse_result = parse(source);
    //first pre-process the AST
    index.pre_process(&mut parse_result);
    //then index the AST
    index.visit(&mut parse_result);
    //and finally codegen
    let mut debugger = DebugManager::create_inactive();
    let mut code_generator = codegen::CodeGen::create_codegen(context, index);
    if enable_debug {
        let debug_metadata_version = context.i32_type().const_int(3, false);
        let dwarf_version = context.i32_type().const_int(4, false);
        &code_generator.module.add_basic_value_flag(
            "Debug Info Version",
            inkwell::module::FlagBehavior::Warning,
            debug_metadata_version,
        );
        &code_generator.module.add_basic_value_flag(
            "Dwarf Version",
            inkwell::module::FlagBehavior::Warning,
            dwarf_version,
        );
        let (di_builder, compilation_unit) = code_generator.module.create_debug_info_builder(true, 
            inkwell::debug_info::DWARFSourceLanguage::C, 
            "examples/MainProg.st", 
            "/home/vagrant/ruSTy/", "RuSTy ST Compiler", 
            false,
            "", 
            0, 
            "", 
            inkwell::debug_info::DWARFEmissionKind::Full, 
            0,
            false, 
            false,
        );
        debugger.activate(&context, di_builder, compilation_unit);
        
    }
    code_generator.generate_compilation_unit(&mut debugger, parse_result);
    code_generator
}

fn parse(source: String) -> ast::CompilationUnit {
    //Start lexing
    let lexer = lexer::lex(&source);
    //Parse
    parser::parse(lexer).unwrap()
}
