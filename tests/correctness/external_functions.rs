// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;
use inkwell::targets::{InitializationConfig, Target};
use rusty::runner::run_no_param;

extern "C" fn times_two(val: i32) -> i32 {
    val * 2
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

    let fn_value = code_gen.module.get_function("times_two").unwrap();

    exec_engine.add_global_mapping(&fn_value, times_two as usize);

    let res: i32 = run_no_param(&exec_engine, "main");
    assert_eq!(res, 200)

    //Call that function
    //Test the function's result is executed
}

extern "C" fn add(size: i32, ptr: *const i32) -> i32 {
    let mut result = 0;
    let mut ptr = ptr;
    for _ in 0..size {
        unsafe {
            result += *ptr;
            ptr = ptr.add(1);
        };
    }
    result
}

extern "C" fn add_ref(size: i32, ptr: *const *const i32) -> i32 {
    let mut result = 0;
    let mut ptr = ptr;
    for _ in 0..size {
        unsafe {
            result += **ptr;
            ptr = ptr.add(1);
        };
    }
    result
}

#[test]
fn sized_variadic_call() {
    let src = "
        {external}
        FUNCTION add : DINT
        VAR_INPUT
            args : {sized} DINT...;
        END_VAR
        END_FUNCTION

        FUNCTION main : DINT
            main := add(1, 2, 3);
        END_FUNCTION
        ";

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context: Context = Context::create();
    let source = SourceCode {
        path: "external_test.st".to_string(),
        source: src.to_string(),
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

    let fn_value = code_gen.module.get_function("add").unwrap();

    exec_engine.add_global_mapping(&fn_value, add as usize);

    let res: i32 = run_no_param(&exec_engine, "main");
    assert_eq!(res, 6)
}

#[test]
fn sized_pointer_variadic_call() {
    let src = "
        {external}
        FUNCTION add_ref : DINT
        VAR_INPUT {ref}
            args : {sized} DINT...;
        END_VAR
        END_FUNCTION

        FUNCTION main : DINT
            main := add_ref(1, 2, 3);
        END_FUNCTION
        ";

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context: Context = Context::create();
    let source = SourceCode {
        path: "external_test.st".to_string(),
        source: src.to_string(),
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

    let fn_value = code_gen.module.get_function("add_ref").unwrap();

    exec_engine.add_global_mapping(&fn_value, add_ref as usize);

    let res: i32 = run_no_param(&exec_engine, "main");
    assert_eq!(res, 6)
}

fn verify_string(size: i32, ptr: *const i8) -> bool {
    let mut result = vec![];
    unsafe {
        let mut ptr = ptr;
        for _ in 0..size {
            let string = std::ffi::CStr::from_ptr(ptr).to_str().unwrap();
            result.push(string.to_owned());
            ptr = ptr.add(81);
        }
    }

    assert_eq!("abc", result[0]);

    assert_eq!("sample text", result[1]);

    assert_eq!("test string", result[2]);

    true
}

fn verify_string_ref(_: i32, ptr: *const *const i8) -> bool {
    unsafe {
        let s = std::ffi::CStr::from_ptr(*ptr).to_str().unwrap();
        assert_eq!("abc", s);

        let s = std::ffi::CStr::from_ptr(*(ptr.add(1))).to_str().unwrap();
        assert_eq!("sample text", s);

        let s = std::ffi::CStr::from_ptr(*(ptr.add(2))).to_str().unwrap();
        assert_eq!("test string", s);
    }
    true
}

#[test]
fn string_sized_variadic_call() {
    let src = "
        {external}
        FUNCTION verify_string : BOOL
        VAR_INPUT
            args : {sized} STRING...;
        END_VAR
        END_FUNCTION

        FUNCTION main : BOOL
            main := verify_string('abc', 'sample text', 'test string');
        END_FUNCTION
        ";

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context: Context = Context::create();
    let source = SourceCode {
        path: "external_test.st".to_string(),
        source: src.to_string(),
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

    let fn_value = code_gen.module.get_function("verify_string").unwrap();

    exec_engine.add_global_mapping(&fn_value, verify_string as usize);

    let res: bool = run_no_param(&exec_engine, "main");
    assert!(res)
}

#[test]
fn string_sized_pointer_variadic_call() {
    let src = "
        {external}
        FUNCTION verify_string_ref : BOOL
        VAR_INPUT {ref}
            args : {sized} STRING...;
        END_VAR
        END_FUNCTION

        FUNCTION main : BOOL
            main := verify_string_ref('abc', 'sample text', 'test string');
        END_FUNCTION
        ";

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context: Context = Context::create();
    let source = SourceCode {
        path: "external_test.st".to_string(),
        source: src.to_string(),
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

    let fn_value = code_gen.module.get_function("verify_string_ref").unwrap();

    exec_engine.add_global_mapping(&fn_value, verify_string_ref as usize);

    let res: bool = run_no_param(&exec_engine, "main");
    assert!(res)
}
