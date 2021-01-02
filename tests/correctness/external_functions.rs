/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use super::super::*;
use inkwell::targets::{InitializationConfig, Target};

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    ret : i32,
}

extern fn times_two(param : &MainType) -> i32 {
    param.ret * 2
}

#[test]
fn test_external_function_called() {
    //Given some external function.

    let prog = 
    "
    @EXTERNAL FUNCTION times_two : DINT
    VAR_INPUT
        val : DINT;
    END_VAR
    END_FUNCTION

    FUNCTION main : DINT
        main := times_two(100);
    END_FUNCTION
    ";

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context : Context = Context::create();
    let mut index = rusty::create_index();
    let code_gen = compile_module(&context, &mut index, prog.to_string());
    let exec_engine = code_gen.module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap();

    let fn_value = code_gen.module.get_function("times_two").unwrap();


    exec_engine.add_global_mapping(&fn_value, times_two as usize);
    let (res, _ ) = run(&exec_engine, "main",  &mut MainType {ret : 0});
    assert_eq!(res,200)  

    //Call that function
    //Test the function's result is executed
}
