// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;
use inkwell::targets::{InitializationConfig, Target};

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    val: i32,
}

extern "C" fn times_two(param: &MainType) -> i32 {
    param.val * 2
}

#[test]
fn test_external_function_called() {
    //Given some external function.

    let prog = "
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
    let context: Context = Context::create();
    let source = SourceCode {
        path: "external_test.st".to_string(),
        source: prog.to_string(),
    };
    let code_gen = compile_module(&context, vec![source], None, Diagnostician::default()).unwrap();
    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let fn_value = code_gen.module.get_function("times_two").unwrap();

    exec_engine.add_global_mapping(&fn_value, times_two as usize);
    let res: i32 = run(&exec_engine, "main", &mut MainType { val: 0 });
    assert_eq!(res, 200)

    //Call that function
    //Test the function's result is executed
}
