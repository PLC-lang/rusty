use std::ffi::CStr;

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

#[allow(dead_code)]
#[repr(C)]
struct MainType2 {
    s: [u8; 6],
}

#[allow(non_snake_case, dead_code)]
unsafe extern "C" fn left_ext__string(in_param: *const u8, out: *mut u8) -> i32 {
    let mut in_param = in_param;
    let mut out = out;
    while *in_param != 0 {
        *out = *in_param;
        out = out.add(1);
        in_param = in_param.add(1)
    }
    0
}

#[test]
fn test_generic_function_with_param_by_ref_called() {
    //Given some external function.
    let prog = "
    FUNCTION LEFT <T: ANY_STRING> : T
    VAR_INPUT {ref}
        IN : T;
    END_VAR
    END_FUNCTION

    FUNCTION LEFT_EXT<T: ANY_STRING> : DINT
    VAR_INPUT {ref}
        IN : T;
    END_VAR
    END_FUNCTION

    FUNCTION LEFT__STRING : STRING 
    VAR_INPUT
        IN : STRING;
    END_VAR
        LEFT_EXT(IN);
    END_FUNCTION

    PROGRAM main 
    VAR
    END_VAR
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
    )
    .unwrap();
    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let fn_value = code_gen.module.get_function("LEFT_EXT__STRING").unwrap();
    exec_engine.add_global_mapping(&fn_value, left_ext__string as usize);

    let mut main_type = MainType2 { s: *b"hello\0" };
    let _: i32 = run(&exec_engine, "main", &mut main_type);
    let result = CStr::from_bytes_with_nul(&main_type.s)
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_any_real_called_with_ints() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        a: f32,
        b: f64,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
    }

    let prog = "
    FUNCTION times_two<T: ANY_REAL> : T
    VAR_INPUT
        val : T;
    END_VAR
    END_FUNCTION

    FUNCTION times_two__REAL : REAL
    VAR_INPUT
        val : REAL;
    END_VAR
		times_two__REAL := val * REAL#2.0;
    END_FUNCTION

    FUNCTION times_two__LREAL : LREAL
    VAR_INPUT
        val : LREAL;
    END_VAR
		times_two__LREAL := val * LREAL#2.0;
    END_FUNCTION

    PROGRAM main 
    VAR
        a : REAL;
        b : LREAL;
		c : REAL;
		d : REAL;
		e : REAL;
		f : REAL;
    END_VAR
	VAR_TEMP
		v_dint : DINT := -6;
	END_VAR
        a := times_two(REAL#2);
        b := times_two(LREAL#3);
		c := times_two(SINT#-4);
		d := times_two(UINT#5);
		e := times_two(v_dint);
		f := times_two(ULINT#7);
    END_PROGRAM
    ";

    let mut main_type = MainType::default();
    let _: i32 = compile_and_run(prog.to_string(), &mut main_type);
    assert_eq!(main_type.a, 4.0f32);
    assert_eq!(main_type.b, 6.0f64);
    assert_eq!(main_type.c, -8.0f32);
    assert_eq!(main_type.d, 10.0f32);
    assert_eq!(main_type.e, -12.0f32);
    assert_eq!(main_type.f, 14.0f32);
}
