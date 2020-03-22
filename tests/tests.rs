use rusty::*;
use inkwell::context::Context;
use inkwell::execution_engine::{JitFunction, ExecutionEngine};

type MainFunction = unsafe extern "C" fn() -> i32;

mod correctness {
    mod sums;
    mod global_variables;
    mod control_flow;
}


///
/// Compiles and runs the given source
/// Returns the std result as String
/// Source must define a main function that takes no arguments and returns an int and string
/// The int is the return value which can be verified
/// The string will eventually be the Stdout of the function.
/// 
pub fn compile<'ctx>(context : &'ctx Context, source : String) -> ExecutionEngine<'ctx> {
    let code_gen = compile_module(context, source);
    println!("{}", get_ir(&code_gen));
    code_gen.module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap()
}
pub fn compile_and_run(source : String) ->  (i32, &'static str){
    let context : Context = Context::create();
    let exec_engine = compile(&context, source);
    run(&exec_engine, "main")
}

pub fn run(exec_engine : &ExecutionEngine, name : &str) -> (i32, &'static str) {

    unsafe {
        let main : JitFunction<MainFunction> = exec_engine.get_function(name).unwrap();
        let int_res = main.call();
        (int_res, "")
    }
}