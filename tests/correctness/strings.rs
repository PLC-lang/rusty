use inkwell::targets::{Target, InitializationConfig};
use pretty_assertions::assert_eq;

use super::super::*;
use std::ffi::CStr;

#[test]
fn string_assignment_from_smaller_literal() {
    let src = "
        PROGRAM main
            VAR x : STRING[6]; END_VAR
            x := 'hello';
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 7],
    }
    let mut main_type = MainType { x: [0; 7] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hello\0\0".as_bytes(), &main_type.x);
}

#[test]
fn string_assignment_from_bigger_literal() {
    let src = "
        PROGRAM main
            VAR x : STRING[4];END_VAR
            x := 'hello';
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
    }
    let mut main_type = MainType { x: [0; 5] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hell\0".as_bytes(), &main_type.x);
}
#[test]
fn string_assignment_from_smaller_string() {
    let src = "
        PROGRAM main 
            VAR x : STRING[6]; y : STRING[5]; END_VAR
            y := 'hello';
            x := y;
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 7],
        y: [u8; 6],
    }
    let mut main_type = MainType {
        x: [0; 7],
        y: [0; 6],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hello\0\0".as_bytes(), &main_type.x);
}

#[test]
fn string_assignment_from_bigger_string() {
    let src = "
        PROGRAM main
            VAR x : STRING[4]; y : STRING[5]; END_VAR
            y := 'hello';
            x := y;
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
        y: [u8; 6],
    }
    let mut main_type = MainType {
        x: [0; 5],
        y: [0; 6],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hell\0".as_bytes(), &main_type.x);
}

#[test]
fn string_assignment_from_smaller_function() {
    let src = "
        TYPE ReturnString : STRING[5] := ''; END_TYPE

        FUNCTION hello : ReturnString
        hello := 'hello';
        END_FUNCTION

        PROGRAM main
            VAR x : STRING[6]; END_VAR
            x := hello();
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 7],
    }
    let mut main_type = MainType { x: [0; 7] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hello\0\0".as_bytes(), &main_type.x);
}

#[test]
fn string_assignment_from_bigger_function() {
    let src = "
        FUNCTION hello : STRING[5]
        hello := 'hello';
        END_FUNCTION

        PROGRAM main
            VAR x : STRING[4]; END_VAR
            x := hello();
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
    }
    let mut main_type = MainType { x: [0; 5] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hell\0".as_bytes(), &main_type.x);
}

#[test]
fn string_assignment_from_bigger_literal_do_not_leak() {
    let src = "
        FUNCTION main : DINT
            VAR x,y : STRING[4]; END_VAR
            x := 'hello';
        END_FUNCTION
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
        y: [u8; 5],
    }
    let mut main_type = MainType {
        x: [0; 5],
        y: [0; 5],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(&[0; 5], &main_type.y);
}

#[test]
fn string_assignment_from_bigger_string_does_not_leak() {
    let src = "
        FUNCTION main : DINT
            VAR x,y : STRING[4]; z : STRING[10]; END_VAR
            z := 'hello foo';
            x := z;
        END_FUNCTION
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
        y: [u8; 5],
        z: [u8; 11],
    }
    let mut main_type = MainType {
        x: [0; 5],
        y: [0; 5],
        z: [0; 11],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(&[0; 5], &main_type.y);
}

#[test]
fn string_parameter_assignment_in_functions_with_multiple_size2() {
    let src = "
        FUNCTION small : STRING[10]
        VAR_INPUT
            str_param : STRING[5];
        END_VAR
        small := str_param;
        END_FUNCTION

        FUNCTION big : STRING[10]
        VAR_INPUT
            str_param : STRING[15];
        END_VAR
        big := str_param;
        END_FUNCTION


        PROGRAM main
            VAR x : STRING[20]; y : STRING[20]; END_VAR
            x := small('hello world');
            y := big('hello');
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 21],
        y: [u8; 21],
    }
    let mut main_type = MainType {
        x: [0; 21],
        y: [0; 21],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    // long string passed to short function and returned
    assert_eq!(
        format!("{:?}", "hello\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes()),
        format!("{:?}", &main_type.x)
    );
    // short string passed to long function and returned
    assert_eq!(
        format!("{:?}", "hello\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes()),
        format!("{:?}", &main_type.y)
    );
}

#[test]
fn string_assignment_from_bigger_function_does_not_leak() {
    let src = "
        FUNCTION hello : STRING[10]
        hello := 'hello foo';
        END_FUNCTION

        PROGRAM main
            VAR x,y : STRING[4]; END_VAR
            x := hello();
        END_PROGRAM
    ";
    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
        y: [u8; 5],
    }
    let mut main_type = MainType {
        x: [0; 5],
        y: [0; 5],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(&[0; 5], &main_type.y);
    assert_eq!("hell\0".as_bytes(), &main_type.x);
}

#[test]
fn initialization_of_string_arrays() {
    let src = "
        VAR_GLOBAL
            texts: ARRAY[0..2] OF STRING[10] := ['hello', 'world', 'ten chars!']
        END_VAR

        PROGRAM main
            VAR x,y,z : STRING[10]; END_VAR
        
            x := texts[0];
            y := texts[1];
            z := texts[2];
        
        END_PROGRAM
    ";

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: [u8; 11],
        y: [u8; 11],
        z: [u8; 11],
    }
    let mut main_type = MainType {
        x: [0; 11],
        y: [0; 11],
        z: [0; 11],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(main_type.x, "hello\0\0\0\0\0\0".as_bytes());
    assert_eq!(main_type.y, "world\0\0\0\0\0\0".as_bytes());
    assert_eq!(main_type.z, "ten chars!\0".as_bytes());
}

#[repr(C, align(1))]
#[derive(Debug)]
struct Wrapper<T> {
    inner : T,
}

/// .
///
/// # Safety
///
/// Unsafe by design, it dereferences a pointer
#[allow(dead_code)]
unsafe extern "C" fn string_id(input : *const i8) -> Wrapper<[u8; 81]> {
    let mut res = [0; 81];
    let bytes = CStr::from_ptr(input).to_bytes();
    for (index,val) in bytes.iter().enumerate() {
        res[index] = *val;
    }
    Wrapper { inner : res}
}

/// .
///
/// # Safety
///
/// Unsafe by design, it dereferences a pointer
#[allow(dead_code)]
unsafe extern "C" fn wstring_id(input : *const i16) -> Wrapper<[u16; 81]> {
    let mut res = [0; 81];
    let bytes = std::slice::from_raw_parts(input, 81);
    for (index,val) in bytes.iter().enumerate() {
        res[index] = *val as u16;
    }
    Wrapper { inner : res}
}


#[test]
fn string_as_function_parameters() {
    let src = "
    @EXTERNAL
    FUNCTION func : STRING
        VAR_INPUT
            in : STRING;
        END_VAR
    END_FUNCTION

    PROGRAM main
	VAR
		res : STRING;
	END_VAR
		res := func('hello');
    END_PROGRAM
    ";

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        res: [u8; 81],
    }

    let mut main_type = MainType {
        res: [0; 81],
    };

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context: Context = Context::create();
    let source = SourceCode {
        path: "string_test.st".to_string(),
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
    code_gen.module.print_to_stderr();
    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let fn_value = code_gen.module.get_function("func").unwrap();

    exec_engine.add_global_mapping(&fn_value, string_id as usize);

    let _: i32 = run(&exec_engine, "main", &mut main_type);
    let res = CStr::from_bytes_with_nul(&main_type.res[..6]).unwrap().to_str().unwrap();
    assert_eq!(res, "hello");
}

#[test]
fn wstring_as_function_parameters() {
    let src = r#"
    @EXTERNAL
    FUNCTION func : WSTRING
        VAR_INPUT
            in : WSTRING;
        END_VAR
    END_FUNCTION

    PROGRAM main
	VAR
		res : WSTRING;
	END_VAR
		res := func("hello");
    END_PROGRAM
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        res: [u16; 81],
    }

    let mut main_type = MainType {
        res: [0; 81],
    };

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context: Context = Context::create();
    let source = SourceCode {
        path: "string_test.st".to_string(),
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
    code_gen.module.print_to_stderr();
    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let fn_value = code_gen.module.get_function("func").unwrap();

    exec_engine.add_global_mapping(&fn_value, wstring_id as usize);

    let _: i32 = run(&exec_engine, "main", &mut main_type);

    let res = String::from_utf16_lossy(&main_type.res[..5]);
    assert_eq!(res, "hello");
}


#[test]
fn string_as_function_parameters_cast() {
    let src = "
    @EXTERNAL
    FUNCTION func : STRING
        VAR_INPUT
            in : STRING;
        END_VAR
    END_FUNCTION

    PROGRAM main
	VAR
		res : STRING;
	END_VAR
		res := func(STRING#'hello');
    END_PROGRAM
    ";

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        res: [u8; 81],
    }

    let mut main_type = MainType {
        res: [0; 81],
    };

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context: Context = Context::create();
    let source = SourceCode {
        path: "string_test.st".to_string(),
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
    code_gen.module.print_to_stderr();
    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let fn_value = code_gen.module.get_function("func").unwrap();

    exec_engine.add_global_mapping(&fn_value, string_id as usize);

    let _: i32 = run(&exec_engine, "main", &mut main_type);
    let res = CStr::from_bytes_with_nul(&main_type.res[..6]).unwrap().to_str().unwrap();
    assert_eq!(res, "hello");
}

#[test]
fn wstring_as_function_parameters_cast() {
    let src = r#"
    @EXTERNAL
    FUNCTION func : WSTRING
        VAR_INPUT
            in : WSTRING;
        END_VAR
    END_FUNCTION

    PROGRAM main
	VAR
		res : WSTRING;
	END_VAR
		res := func(WSTRING#"hello");
    END_PROGRAM
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        res: [u16; 81],
    }

    let mut main_type = MainType {
        res: [0; 81],
    };

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context: Context = Context::create();
    let source = SourceCode {
        path: "string_test.st".to_string(),
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
    code_gen.module.print_to_stderr();
    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let fn_value = code_gen.module.get_function("func").unwrap();

    exec_engine.add_global_mapping(&fn_value, wstring_id as usize);

    let _: i32 = run(&exec_engine, "main", &mut main_type);

    let res = String::from_utf16_lossy(&main_type.res[..5]);
    assert_eq!(res, "hello");
}

