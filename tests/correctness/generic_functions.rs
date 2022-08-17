use super::super::*;
use inkwell::targets::{InitializationConfig, Target};

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    a: i16,
    b: f32,
}

extern "C" fn times_two_int(val: i16) -> i16 {
    val * 2
}

extern "C" fn times_two_real(val: f32) -> f32 {
    val * 2.0f32
}

#[test]
fn test_external_function_called() {
    //Given some external function.
    let prog = "
    @EXTERNAL FUNCTION times_two<T: ANY_NUM> : T
    VAR_INPUT
        val : T;
    END_VAR
    END_FUNCTION

    PROGRAM main
    VAR
        a : INT;
        b : REAL;
    END_VAR
        a := times_two(INT#100);
        b := times_two(2.5);
    END_PROGRAM
    ";

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context: Context = Context::create();
    let source = SourceCode {
        path: "external_test.st".to_string(),
        source: prog.to_string(),
    };
    let (_, code_gen) = compile_module(
        &context,
        vec![source],
        vec![],
        None,
        Diagnostician::default(),
        OptimizationLevel::None,
        DebugLevel::None,
    )
    .unwrap();
    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let fn_value = code_gen.module.get_function("times_two__INT").unwrap();
    exec_engine.add_global_mapping(&fn_value, times_two_int as usize);
    let fn_value = code_gen.module.get_function("times_two__REAL").unwrap();
    exec_engine.add_global_mapping(&fn_value, times_two_real as usize);

    let mut main_type = MainType { a: 0, b: 0.0f32 };
    let _: i32 = run(&exec_engine, "main", &mut main_type);
    assert_eq!(main_type.a, 200);
    assert_eq!(main_type.b, 5.0f32);
}

#[test]
fn test_generic_function_implemented_in_st_called() {
    //Given some external function.
    let prog = "
    FUNCTION times_two<T: ANY_NUM> : T
    VAR_INPUT
        val : T;
    END_VAR
    END_FUNCTION

    FUNCTION times_two__INT : INT
    VAR_INPUT
        val : INT;
    END_VAR
        times_two__INT := val * 2;
    END_FUNCTION

    FUNCTION times_two__REAL : REAL
    VAR_INPUT
        val : REAL;
    END_VAR
        times_two__REAL := val * 2.0;
    END_FUNCTION

    PROGRAM main 
    VAR
        a : INT;
        b : REAL;
    END_VAR
        a := times_two(INT#100);
        b := times_two(2.5);
    END_PROGRAM
    ";

    let mut main_type = MainType { a: 0, b: 0.0f32 };
    let _: i32 = compile_and_run(prog.to_string(), &mut main_type);
    assert_eq!(main_type.a, 200);
    assert_eq!(main_type.b, 5.0f32);
}
