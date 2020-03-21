use rusty::*;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;

type MainFunction = unsafe extern "C" fn() -> i32;

#[test]
fn adds_in_result() {
    let prog = 
    "
    FUNCTION main : INT
        main := 10 + 50;
    END_FUNCTION
    ";

    let (res, _) = compile_and_run(prog.to_string());
    assert_eq!(res,60)  
}


///
/// Compiles and runs the given source
/// Returns the std result as String
/// Source must define a main function that takes no arguments and returns an int and string
/// The int is the return value which can be verified
/// The string will eventually be the Stdout of the function.
/// 
pub fn compile_and_run(source : String) -> (i32, &'static str) {
    let context = Context::create();
    let code_gen = compile_module(&context, source);
    let exec_engine = code_gen.module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap();
    let result = unsafe {
        let main : JitFunction<MainFunction> = exec_engine.get_function("main").unwrap();
        println!("{:?}", main);
        let int_res = main.call();
        (int_res, "")
    };
    result
}