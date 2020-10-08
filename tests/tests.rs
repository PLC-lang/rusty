/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use rusty::*;
use rusty::index::Index;
use inkwell::context::Context;
use inkwell::execution_engine::{JitFunction, ExecutionEngine};

type MainFunction<T> = unsafe extern "C" fn(*mut T) -> i32;

mod correctness {
    mod sums;
    mod global_variables;
    mod control_flow;
    mod functions;
    mod datatypes ;
}


///
/// Compiles and runs the given source
/// Returns the std result as String
/// Source must define a main function that takes no arguments and returns an int and string
/// The int is the return value which can be verified
/// The string will eventually be the Stdout of the function.
/// 
pub fn compile<'ctx>(context : &'ctx Context, index: &'ctx mut Index<'ctx>, source : String) -> ExecutionEngine<'ctx> {
    let code_gen = compile_module(context, index, source, false);
    println!("{}", get_ir(&code_gen));
    code_gen.module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap()
}
pub fn compile_and_run<T>(source : String, params : &mut T) ->  (i32, &'static str){
    let context : Context = Context::create();
    let mut index = rusty::create_index();
    let exec_engine = compile(&context, &mut index, source);
    run::<T>(&exec_engine, "main", params)
}

pub fn run<T>(exec_engine : &ExecutionEngine, name : &str, params : &mut T) -> (i32, &'static str) {
    unsafe {
        let main : JitFunction<MainFunction<T>> = exec_engine.get_function(name).unwrap();
        let main_t_ptr = &mut *params as *mut _; 
        let int_res = main.call(main_t_ptr);
        (int_res, "")
    }
}
