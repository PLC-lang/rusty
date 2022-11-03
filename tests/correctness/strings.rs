use inkwell::targets::{InitializationConfig, Target};
use pretty_assertions::assert_eq;

use super::super::*;
use std::ffi::CStr;

#[test]
fn string_assignment_from_smaller_literal() {
    let src = r#"
        PROGRAM main
            VAR 
                x : STRING[6]; 
                y : WSTRING[6]; 
            END_VAR
            x := 'hello';
            y := "hello";
        END_PROGRAM
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: [u8; 7],
        y: [u16; 7],
    }
    let mut main_type = MainType {
        x: [0; 7],
        y: [0; 7],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hello\0\0".as_bytes(), &main_type.x);
    assert_eq!("hello", String::from_utf16_lossy(&main_type.y[..5]));
}

#[test]
fn string_assignment_from_bigger_literal() {
    let src = r#"
        PROGRAM main
            VAR 
                x : STRING[4];
                y : WSTRING[4];
            END_VAR
            x := 'hello';
            y := "hello";
        END_PROGRAM
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: [u8; 5],
        y: [u16; 5],
    }
    let mut main_type = MainType {
        x: [0; 5],
        y: [0; 5],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hell\0".as_bytes(), &main_type.x);
    assert_eq!("hell", String::from_utf16_lossy(&main_type.y[..4]));
}

#[test]
fn string_assignment_from_smaller_string() {
    let src = r#"
        PROGRAM main 
            VAR 
                x : STRING[6]; y : STRING[5]; 
                u : WSTRING[6]; v : WSTRING[5]; 
            END_VAR
            y := 'hello';
            x := y;
            v := "hello";
            u := v;
        END_PROGRAM
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: [u8; 7],
        y: [u8; 6],
        u: [u16; 7],
        v: [u16; 6],
    }
    let mut main_type = MainType {
        x: [0; 7],
        y: [0; 6],
        u: [0; 7],
        v: [0; 6],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hello\0\0".as_bytes(), &main_type.x);
    assert_eq!("hello", String::from_utf16_lossy(&main_type.u[..5]));
}

#[test]
fn string_assignment_from_bigger_string() {
    let src = r#"
        PROGRAM main
            VAR 
                x : STRING[4]; y : STRING[5];
                u : WSTRING[4]; v : WSTRING[5]; 
            END_VAR
            y := 'hello';
            x := y;
            v := "hello";
            u := v;
        END_PROGRAM
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: [u8; 5],
        y: [u8; 6],
        u: [u16; 5],
        v: [u16; 6],
    }
    let mut main_type = MainType {
        x: [0; 5],
        y: [0; 6],
        u: [0; 5],
        v: [0; 6],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hell\0".as_bytes(), &main_type.x);
    assert_eq!("hell", String::from_utf16_lossy(&main_type.u[..4]));
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
    #[repr(C)]
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
    #[repr(C)]
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
    #[repr(C)]
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
    #[repr(C)]
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
    #[repr(C)]
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
    #[repr(C)]
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
    inner: T,
}

/// .
///
/// # Safety
///
/// Unsafe by design, it dereferences a pointer
#[allow(dead_code)]
unsafe extern "C" fn string_id(input: *const i8) -> Wrapper<[u8; 81]> {
    let mut res = [0; 81];
    let bytes = CStr::from_ptr(input).to_bytes();
    for (index, val) in bytes.iter().enumerate() {
        res[index] = *val;
    }
    Wrapper { inner: res }
}

/// .
///
/// # Safety
///
/// Unsafe by design, it dereferences a pointer
#[allow(dead_code)]
unsafe extern "C" fn wstring_id(input: *const i16) -> Wrapper<[u16; 81]> {
    let mut res = [0; 81];
    let bytes = std::slice::from_raw_parts(input, 81);
    for (index, val) in bytes.iter().enumerate() {
        res[index] = *val as u16;
    }
    Wrapper { inner: res }
}

#[test]
fn string_as_function_parameters() {
    let src = "
    @EXTERNAL
    FUNCTION func : STRING
        VAR_INPUT {ref}
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

    let mut main_type = MainType { res: [0; 81] };

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
        OptimizationLevel::None,
        DebugLevel::None,
    )
    .unwrap();
    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let fn_value = code_gen.module.get_function("func").unwrap();

    exec_engine.add_global_mapping(&fn_value, string_id as usize);

    let _: i32 = run(&exec_engine, "main", &mut main_type);
    let res = CStr::from_bytes_with_nul(&main_type.res[..6])
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(res, "hello");
}

#[test]
fn wstring_as_function_parameters() {
    let src = r#"
    @EXTERNAL
    FUNCTION func : WSTRING
        VAR_INPUT {ref}
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

    let mut main_type = MainType { res: [0; 81] };

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
        OptimizationLevel::None,
        DebugLevel::None,
    )
    .unwrap();
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
        VAR_INPUT {ref}
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

    let mut main_type = MainType { res: [0; 81] };

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
        OptimizationLevel::None,
        DebugLevel::None,
    )
    .unwrap();
    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let fn_value = code_gen.module.get_function("func").unwrap();

    exec_engine.add_global_mapping(&fn_value, string_id as usize);

    let _: i32 = run(&exec_engine, "main", &mut main_type);
    let res = CStr::from_bytes_with_nul(&main_type.res[..6])
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(res, "hello");
}

#[test]
fn wstring_as_function_parameters_cast() {
    let src = r#"
    @EXTERNAL
    FUNCTION func : WSTRING
        VAR_INPUT {ref}
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

    let mut main_type = MainType { res: [0; 81] };

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
        OptimizationLevel::None,
        DebugLevel::None,
    )
    .unwrap();
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
fn string_as_function_return_type_does_not_truncate() {
    let src = "
        FUNCTION foo : STRING[100]
        VAR_INPUT 
            str_param : STRING[100];
        END_VAR
            foo := str_param;
        END_FUNCTION

        PROGRAM main
        VAR 
            x : STRING[100]
        END_VAR
            x := foo('     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.')
        END_PROGRAM
    ";

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: [u8; 101],
    }
    let mut main_type = MainType { x: [0; 101] };

    let _: i32 = compile_and_run(src, &mut main_type);
    // long string passed to short function and returned
    assert_eq!(
        format!("{:?}", "     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.\0".as_bytes()),
        format!("{:?}", &main_type.x)
    );
}

#[test]
fn string_ref_returned_from_wrapper_function_does_not_truncate() {
    let src = "
        FUNCTION foo : STRING[100]
        VAR_INPUT 
            str_param : STRING[100];
        END_VAR
            bar(str_param, foo);
        END_FUNCTION

        FUNCTION bar : DINT
        VAR_INPUT {ref}
            in : STRING[100];
        END_VAR
        VAR_IN_OUT
            out: STRING[100];
        END_VAR
            out := in;
        END_FUNCTION

        PROGRAM main
        VAR 
            x : STRING[100]
        END_VAR
            x := foo('     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.')
        END_PROGRAM
    ";

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: [u8; 101],
    }
    let mut main_type = MainType { x: [0; 101] };

    let _: i32 = compile_and_run(src, &mut main_type);
    // long string passed to short function and returned
    assert_eq!(
        format!("{:?}", "     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.\0".as_bytes()),
        format!("{:?}", &main_type.x)
    );
}

#[test]
fn string_returned_from_generic_wrapper_function_does_not_truncate() {
    let src = "
        FUNCTION foo<T: ANY_STRING> : T
        VAR_INPUT {ref}
            in : T;
        END_VAR        
        END_FUNCTION

        FUNCTION foo__STRING : STRING[100]
        VAR_INPUT {ref}
            param : STRING[100];
        END_VAR
            bar(param, foo__STRING);
        END_FUNCTION

        FUNCTION bar : DINT
        VAR_INPUT {ref}
            in : STRING[100];
        END_VAR
        VAR_IN_OUT
            out: STRING[100];
        END_VAR
            out := in;
        END_FUNCTION

        PROGRAM main 
        VAR 
            param : STRING[100];
            x : STRING[100];
        END_VAR
            param := '     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.';
            x := foo(param);
        END_PROGRAM
    ";

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: [u8; 101],
    }
    let mut main_type = MainType { x: [0; 101] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(
        format!("{:?}", "     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.\0".as_bytes()),
        format!("{:?}", &main_type.x)
    );
}

#[test]
fn string_returned_from_main_does_not_truncate() {
    let src = "
        PROGRAM main : STRING[100]
        VAR 
            param : STRING[100];
        END_VAR
            param := '     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.';
            main := param;
        END_PROGRAM
    ";
    let res: [u8; 101] = compile_and_run(src, &mut MainType::default());

    assert_eq!(
        format!("{:?}",res), 
        format!(
            "{:?}", 
            "     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.\0".as_bytes()
        )
    )
}

#[test]
fn when_function_returns_value_from_generic_function_call_then_string_does_not_truncate() {
    let src = "
        FUNCTION foo<T: ANY_STRING> : T
        VAR_INPUT {ref}
            in : T;
        END_VAR        
        END_FUNCTION

        FUNCTION foo__STRING : STRING[100]
        VAR_INPUT {ref}
            param : STRING[100];
        END_VAR
            bar(param, foo__STRING);
        END_FUNCTION

        FUNCTION bar : DINT
        VAR_INPUT {ref}
            in : STRING[100];
        END_VAR
        VAR_IN_OUT
            out: STRING[100];
        END_VAR
            out := in;
        END_FUNCTION

        FUNCTION main : STRING[100]
        VAR 
            param : STRING[100];
        END_VAR
            param := '     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.';
            main := foo(param);           
        END_FUNCTION
    ";
    let res: [u8; 101] = compile_and_run(src, &mut MainType::default());

    assert_eq!(
        format!("{:?}",res), 
        format!(
            "{:?}", 
            "     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.\0".as_bytes()
        )
    )
}

#[test]
fn when_function_returns_value_from_function_call_string_does_not_truncate() {
    let src = "
        FUNCTION foo : STRING[100]
        VAR_INPUT {ref}
            param : STRING[100];
        END_VAR
            bar(param, foo);
        END_FUNCTION

        FUNCTION bar : DINT
        VAR_INPUT {ref}
            in : STRING[100];
        END_VAR
        VAR_IN_OUT
            out: STRING[100];
        END_VAR
            out := in;
        END_FUNCTION

        FUNCTION main : STRING[100]
        VAR 
            param : STRING[100];
        END_VAR
            param := '     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.';
            main := foo(param);            
        END_FUNCTION
    ";
    let res: [u8; 101] = compile_and_run(src, &mut MainType::default());

    assert_eq!(
        format!("{:?}",res), 
        format!(
            "{:?}", 
            "     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.\0".as_bytes()
        )
    )
}

#[test]
fn program_string_output() {
    let src = r#"
		PROGRAM prog
		VAR_OUTPUT
			output1 : STRING;
			output2 : WSTRING;
		END_VAR
			output1 := 'string';
			output2 := "wstring";
		END_PROGRAM

        PROGRAM main
		VAR 
			x : STRING[6]; 
			y : WSTRING[7]; 
		END_VAR
			prog(x, y);
        END_PROGRAM
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: [u8; 7],
        y: [u16; 8],
    }
    let mut main_type = MainType {
        x: [0; 7],
        y: [0; 8],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("string\0".as_bytes(), &main_type.x);
    assert_eq!("wstring", String::from_utf16_lossy(&main_type.y[..7]));
}
