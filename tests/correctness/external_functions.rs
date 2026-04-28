// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::*;
use inkwell::targets::{InitializationConfig, Target};
use rusty::codegen::CodegenContext;

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
    let source = SourceCode::new(prog, "external_test.st");
    let context = CodegenContext::create();
    let module = compile(&context, source);
    module.add_global_function_mapping("times_two", times_two as *const () as usize);

    let res: i32 = module.run_no_param("main");
    assert_eq!(res, 200)

    //Call that function
    //Test the function's result is executed
}

extern "C" fn add_local(size: i32, ptr: *const i32) -> i32 {
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
        FUNCTION add_local : DINT
        VAR_INPUT
            args : {sized} DINT...;
        END_VAR
        END_FUNCTION

        FUNCTION main : DINT
            main := add_local(1, 2, 3);
        END_FUNCTION
        ";

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let source = SourceCode::new(src, "external_test.st");
    let context = CodegenContext::create();
    let module = compile(&context, source);
    module.add_global_function_mapping("add_local", add_local as *const () as usize);

    let res: i32 = module.run_no_param("main");
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
    let source = SourceCode::new(src, "external_test.st");
    let context = CodegenContext::create();
    let module = compile(&context, source);
    module.add_global_function_mapping("add_ref", add_ref as *const () as usize);

    let res: i32 = module.run_no_param("main");
    assert_eq!(res, 6)
}

fn verify_string(size: i32, ptr: *const i8) -> bool {
    let mut result = vec![];
    unsafe {
        let mut ptr = ptr;
        for _ in 0..size {
            let string = std::ffi::CStr::from_ptr(ptr as *const _).to_str().unwrap();
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
        let s = new_cstr(*ptr).to_str().unwrap();
        assert_eq!("abc", s);

        let s = new_cstr(*(ptr.add(1))).to_str().unwrap();
        assert_eq!("sample text", s);

        let s = new_cstr(*(ptr.add(2))).to_str().unwrap();
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
    let source = SourceCode::new(src, "external_test.st");
    let context = CodegenContext::create();
    let module = compile(&context, source);
    module.add_global_function_mapping("verify_string", verify_string as *const () as usize);

    let res: bool = module.run_no_param("main");
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
    let source = SourceCode::new(src, "external_test.st");
    let context = CodegenContext::create();
    let module = compile(&context, source);
    module.add_global_function_mapping("verify_string_ref", verify_string_ref as *const () as usize);

    let res: bool = module.run_no_param("main");
    assert!(res)
}

#[no_mangle]
extern "C" fn echo__DINT(val: i32) -> i32 {
    val
}

#[test]
fn generic_external_function_having_same_name_as_local_variable() {
    let src = "
        {external}
        FUNCTION echo <T: ANY_INT> : DINT
            VAR_INPUT
                val : T;
            END_VAR
        END_FUNCTION

        FUNCTION main : DINT
            VAR
                echo : DINT;
            END_VAR
            echo := echo(12345);
            main := echo;
        END_FUNCTION
    ";

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let source = SourceCode::new(src, "external_test.st");
    let context = CodegenContext::create();
    let module = compile(&context, source);
    module.add_global_function_mapping("echo__DINT", echo__DINT as *const () as usize);

    let res: i32 = module.run_no_param("main");
    assert_eq!(res, 12345)
}
