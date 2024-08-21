use crate::{compile, compile_and_run, new_cstr};
use inkwell::{
    context::Context,
    targets::{InitializationConfig, Target},
};
use plc_source::SourceCode;
use pretty_assertions::assert_eq;
use rusty::codegen::CodegenContext;

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
    let mut main_type = MainType { x: [0; 7], y: [0; 7] };

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
    let mut main_type = MainType { x: [0; 5], y: [0; 5] };

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
    let mut main_type = MainType { x: [0; 7], y: [0; 6], u: [0; 7], v: [0; 6] };

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
    let mut main_type = MainType { x: [0; 5], y: [0; 6], u: [0; 5], v: [0; 6] };

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
    let mut main_type = MainType { x: [0; 5], y: [0; 5] };

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
    let mut main_type = MainType { x: [0; 5], y: [0; 5], z: [0; 11] };

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
    let mut main_type = MainType { x: [0; 21], y: [0; 21] };

    let _: i32 = compile_and_run(src, &mut main_type);
    let long = CStr::from_bytes_until_nul(&main_type.x).unwrap().to_string_lossy();
    let short = CStr::from_bytes_until_nul(&main_type.y).unwrap().to_string_lossy();

    // long string passed to short function and returned
    assert_eq!("hello", &long);
    // short string passed to long function and returned
    assert_eq!("hello", &short);
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
    let mut main_type = MainType { x: [0; 5], y: [0; 5] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(&[0; 5], &main_type.y);
    assert_eq!("hell\0".as_bytes(), &main_type.x);
}

#[test]
fn initialization_of_string_arrays() {
    let src = "
        VAR_GLOBAL
            texts: ARRAY[0..2] OF STRING[10] := ['hello', 'world', 'ten chars!'];
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
    let mut main_type = MainType { x: [0; 11], y: [0; 11], z: [0; 11] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(main_type.x, "hello\0\0\0\0\0\0".as_bytes());
    assert_eq!(main_type.y, "world\0\0\0\0\0\0".as_bytes());
    assert_eq!(main_type.z, "ten chars!\0".as_bytes());
}

/// .
///
/// # Safety
///
/// Unsafe by design, it dereferences a pointer
#[allow(dead_code)]
unsafe extern "C" fn string_id(res: *mut u8, input: *const i8) {
    let bytes = new_cstr(input).to_bytes();
    let mut res = res;
    for val in bytes.iter() {
        *res = *val;
        res = res.add(1);
    }

    *res = 0;
}

/// .
///
/// # Safety
///
/// Unsafe by design, it dereferences a pointer
#[allow(dead_code)]
unsafe extern "C" fn wstring_id(mut res: *mut u16, input: *const i16) {
    let bytes = std::slice::from_raw_parts(input, 81);

    for val in bytes.iter() {
        *res = *val as u16;
        res = res.add(1);
    }
}

#[test]
fn string_as_function_parameters_internal() {
    let src = "
    FUNCTION func : STRING
        VAR_INPUT {ref}
            in : STRING;
        END_VAR

        func := in;
    END_FUNCTION

    PROGRAM main
    VAR
        res : STRING;
    END_VAR
        res := func('hello');
    END_PROGRAM
    ";

    let mut main_type: [u8; 81] = [0; 81];

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let _context: Context = Context::create();
    let source = SourceCode::new(src, "string_test.st");
    let _: i32 = compile_and_run(source, &mut main_type);
    let res = CStr::from_bytes_with_nul(&main_type[..6]).unwrap().to_str().unwrap();
    assert_eq!(res, "hello");
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

    let mut main_type: [u8; 81] = [0; 81];

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let source = SourceCode::new(src, "string_test.st");
    let context = CodegenContext::create();
    let module = compile(&context, source);
    module.add_global_function_mapping("func", string_id as usize);

    let _: i32 = module.run("main", &mut main_type);
    let res = CStr::from_bytes_with_nul(&main_type[..6]).unwrap().to_str().unwrap();
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
    let source = SourceCode::new(src, "string_test.st");
    let context = CodegenContext::create();
    let module = compile(&context, source);
    module.add_global_function_mapping("func", wstring_id as usize);

    let _: i32 = module.run("main", &mut main_type);

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
    let source = SourceCode::new(src, "string_test.st");
    let context = CodegenContext::create();
    let module = compile(&context, source);
    module.add_global_function_mapping("func", string_id as usize);
    let _: i32 = module.run("main", &mut main_type);
    let res = CStr::from_bytes_with_nul(&main_type.res[..6]).unwrap().to_str().unwrap();
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
    let source = SourceCode::new(src, "string_test.st");
    let context = CodegenContext::create();
    let module = compile(&context, source);
    module.add_global_function_mapping("func", wstring_id as usize);
    let _: i32 = module.run("main", &mut main_type);

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
            x : STRING[100];
        END_VAR
            x := foo('     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.');
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
            x : STRING[100];
        END_VAR
            x := foo('     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.');
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
        x : STRING[100];
        END_VAR
        VAR_TEMP
            param : STRING[100];
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
    let expected = "     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.\0".as_bytes();
    assert_eq!(format!("{:?}", main_type.x), format!("{:?}", expected))
}

#[test]
fn string_returned_from_main_does_not_truncate() {
    let src = "
        FUNCTION main : STRING[100]
        VAR
            param : STRING[100];
        END_VAR
            param := '     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.';
            main := param;
        END_FUNCTION
    ";
    let mut res: [u8; 101] = [0; 101];
    let _: u32 = compile_and_run(src, &mut res);

    assert_eq!(
        format!("{res:?}"),
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
    let mut res: [u8; 101] = [0; 101];
    let _: () = compile_and_run(src, &mut res);

    assert_eq!(
        format!("{res:?}"),
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
    let mut res: [u8; 101] = [0; 101];
    let _: () = compile_and_run(src, &mut res);

    assert_eq!(
        format!("{res:?}"),
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
    let mut main_type = MainType { x: [0; 7], y: [0; 8] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("string\0".as_bytes(), &main_type.x);
    assert_eq!("wstring", String::from_utf16_lossy(&main_type.y[..7]));
}

#[test]
fn assigning_global_strings_in_function_by_passing_references() {
    let src = r#"
        FUNCTION foo : DINT
        VAR_INPUT
            in : STRING;
        END_VAR
        VAR_INPUT {ref}
            in_ref : STRING;
        END_VAR
        VAR_IN_OUT
            inout : STRING;
        END_VAR
            glob_str_in     := in;
            glob_str_in_ref := in_ref;
            glob_str_inout  := inout;
        END_FUNCTION

        PROGRAM main
        VAR
            str_in      : STRING;
            str_in_ref  : STRING;
            str_inout   : STRING;
        END_VAR
        VAR_TEMP
            a : STRING := 'input';
            b : STRING := 'input ref';
            c : STRING := 'input inout';
        END_VAR
            foo(a, b, c);

            str_in      := glob_str_in;
            str_in_ref  := glob_str_in_ref;
            str_inout   := glob_str_inout;
        END_PROGRAM

        VAR_GLOBAL
            glob_str_in     : STRING;
            glob_str_in_ref : STRING;
            glob_str_inout  : STRING;
        END_VAR
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        str_in: [u8; 81],
        str_in_ref: [u8; 81],
        str_inout: [u8; 81],
    }
    let mut main_type = MainType { str_in: [0; 81], str_in_ref: [0; 81], str_inout: [0; 81] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("input\0".as_bytes(), &main_type.str_in[0..6]);
    assert_eq!("input ref\0".as_bytes(), &main_type.str_in_ref[0..10]);
    assert_eq!("input inout\0".as_bytes(), &main_type.str_inout[0..12]);
}

#[test]
fn assigning_global_strings_in_function_by_passing_sized_strings() {
    let src = r#"
        FUNCTION foo : DINT
        VAR_INPUT
            in : STRING;
        END_VAR
        VAR_INPUT {ref}
            in_ref : STRING;
        END_VAR
        VAR_IN_OUT
            inout : STRING;
        END_VAR
            glob_str_in     := in;
            glob_str_in_ref := in_ref;
            glob_str_inout  := inout;
        END_FUNCTION

        PROGRAM main
        VAR
            str_in      : STRING;
            str_in_ref  : STRING;
            str_inout   : STRING;
        END_VAR
        VAR_TEMP
            a : STRING[5] := 'input';
            b : STRING[9] := 'input ref';
            c : STRING[11] := 'input inout';
        END_VAR
            foo(a, b, c);

            str_in      := glob_str_in;
            str_in_ref  := glob_str_in_ref;
            str_inout   := glob_str_inout;
        END_PROGRAM

        VAR_GLOBAL
            glob_str_in     : STRING;
            glob_str_in_ref : STRING;
            glob_str_inout  : STRING;
        END_VAR
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        str_in: [u8; 81],
        str_in_ref: [u8; 81],
        str_inout: [u8; 81],
    }
    let mut main_type = MainType { str_in: [0; 81], str_in_ref: [0; 81], str_inout: [0; 81] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("input\0".as_bytes(), &main_type.str_in[0..6]);
    assert_eq!("input ref\0".as_bytes(), &main_type.str_in_ref[0..10]);
    assert_eq!("input inout\0".as_bytes(), &main_type.str_inout[0..12]);
}

#[test]
fn assigning_global_strings_in_function_by_passing_literals() {
    let src = r#"
        FUNCTION foo : DINT
        VAR_INPUT
            in : STRING;
        END_VAR
        VAR_INPUT {ref}
            in_ref : STRING;
        END_VAR
            glob_str_in     := in;
            glob_str_in_ref := in_ref;
        END_FUNCTION

        PROGRAM main
        VAR
            str_in      : STRING;
            str_in_ref  : STRING;
        END_VAR
            foo('in literal', STRING#'in ref literal');

            str_in      := glob_str_in;
            str_in_ref  := glob_str_in_ref;
        END_PROGRAM

        VAR_GLOBAL
            glob_str_in     : STRING;
            glob_str_in_ref : STRING;
        END_VAR
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        str_in: [u8; 81],
        str_in_ref: [u8; 81],
    }
    let mut main_type = MainType { str_in: [0; 81], str_in_ref: [0; 81] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("in literal\0".as_bytes(), &main_type.str_in[0..11]);
    assert_eq!("in ref literal\0".as_bytes(), &main_type.str_in_ref[0..15]);
}

#[test]
fn assigning_by_ref_string_parameters_in_function() {
    let src = r#"
        FUNCTION foo : DINT
        VAR_INPUT
            in : STRING;
        END_VAR
        VAR_INPUT {ref}
            in_ref : STRING;
        END_VAR
        VAR_IN_OUT
            inout : STRING;
        END_VAR
            in      := 'in assigned in function';
            in_ref  := 'in ref assigned in function';
            inout   := 'inout assigned in function';
        END_FUNCTION

        PROGRAM main
        VAR
            str_in      : STRING;
            str_in_ref  : STRING;
            str_inout   : STRING;
        END_VAR
        VAR_TEMP
            a : STRING := 'input';
            b : STRING := 'input ref';
            c : STRING := 'input inout';
        END_VAR
            foo(a, b, c);

            str_in      := a;
            str_in_ref  := b;
            str_inout   := c;
        END_PROGRAM
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        str_in: [u8; 81],
        str_in_ref: [u8; 81],
        str_inout: [u8; 81],
    }
    let mut main_type = MainType { str_in: [0; 81], str_in_ref: [0; 81], str_inout: [0; 81] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("input\0".as_bytes(), &main_type.str_in[0..6]);
    assert_eq!("in ref assigned in function\0".as_bytes(), &main_type.str_in_ref[0..28]);
    assert_eq!("inout assigned in function\0".as_bytes(), &main_type.str_inout[0..27]);
}

#[test]
fn reassign_strings_after_function_call() {
    let src = r#"
        FUNCTION foo : DINT
        VAR_INPUT
            in : STRING;
        END_VAR
        VAR_INPUT {ref}
            in_ref : STRING;
        END_VAR
        VAR_IN_OUT
            inout : STRING;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            str_in      : STRING;
            str_in_ref  : STRING;
            str_inout   : STRING;
        END_VAR
        VAR_TEMP
            a : STRING := 'input';
            b : STRING := 'input ref';
            c : STRING := 'input inout';
        END_VAR
            foo(a, b, c);
            a := 'a assigned after function call';
            b := 'b assigned after function call';
            c := 'c assigned after function call';

            str_in      := a;
            str_in_ref  := b;
            str_inout   := c;
        END_PROGRAM
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        str_in: [u8; 81],
        str_in_ref: [u8; 81],
        str_inout: [u8; 81],
    }
    let mut main_type = MainType { str_in: [0; 81], str_in_ref: [0; 81], str_inout: [0; 81] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("a assigned after function call\0".as_bytes(), &main_type.str_in[0..31]);
    assert_eq!("b assigned after function call\0".as_bytes(), &main_type.str_in_ref[0..31]);
    assert_eq!("c assigned after function call\0".as_bytes(), &main_type.str_inout[0..31]);
}

#[test]
fn assigning_global_strings_in_program_by_passing_references() {
    let src = r#"
        PROGRAM prog
        VAR_INPUT
            in : STRING;
        END_VAR
        VAR_INPUT {ref}
            in_ref : STRING;
        END_VAR
        VAR_IN_OUT
            inout : STRING;
        END_VAR
            glob_str_in     := in;
            glob_str_in_ref := in_ref;
            glob_str_inout  := inout;
        END_PROGRAM

        PROGRAM main
        VAR
            str_in      : STRING;
            str_in_ref  : STRING;
            str_inout   : STRING;
        END_VAR
        VAR_TEMP
            a : STRING := 'input';
            b : STRING := 'input ref';
            c : STRING := 'input inout';
        END_VAR
            prog(a, b, c);

            str_in      := glob_str_in;
            str_in_ref  := glob_str_in_ref;
            str_inout   := glob_str_inout;
        END_PROGRAM

        VAR_GLOBAL
            glob_str_in     : STRING;
            glob_str_in_ref : STRING;
            glob_str_inout  : STRING;
        END_VAR
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        str_in: [u8; 81],
        str_in_ref: [u8; 81],
        str_inout: [u8; 81],
    }
    let mut main_type = MainType { str_in: [0; 81], str_in_ref: [0; 81], str_inout: [0; 81] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("input\0".as_bytes(), &main_type.str_in[0..6]);
    assert_eq!("input ref\0".as_bytes(), &main_type.str_in_ref[0..10]);
    assert_eq!("input inout\0".as_bytes(), &main_type.str_inout[0..12]);
}

// TODO: module.verify() will fail
// "Stored value type does not match pointer operand type!"
#[test]
fn assigning_global_strings_in_program_by_passing_sized_strigs() {
    let src = r#"
        PROGRAM prog
        VAR_INPUT
            in : STRING;
        END_VAR
        VAR_INPUT {ref}
            in_ref : STRING;
        END_VAR
        VAR_IN_OUT
            inout : STRING;
        END_VAR
            glob_str_in     := in;
            glob_str_in_ref := in_ref;
            glob_str_inout  := inout;
        END_PROGRAM

        PROGRAM main
        VAR
            str_in      : STRING;
            str_in_ref  : STRING;
            str_inout   : STRING;
        END_VAR
        VAR_TEMP
            a : STRING[5] := 'input';
            b : STRING[9] := 'input ref';
            c : STRING[11] := 'input inout';
        END_VAR
            prog(a, b, c);

            str_in      := glob_str_in;
            str_in_ref  := glob_str_in_ref;
            str_inout   := glob_str_inout;
        END_PROGRAM

        VAR_GLOBAL
            glob_str_in     : STRING;
            glob_str_in_ref : STRING;
            glob_str_inout  : STRING;
        END_VAR
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        str_in: [u8; 81],
        str_in_ref: [u8; 81],
        str_inout: [u8; 81],
    }
    let mut main_type = MainType { str_in: [0; 81], str_in_ref: [0; 81], str_inout: [0; 81] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("input\0".as_bytes(), &main_type.str_in[0..6]);
    assert_eq!("input ref\0".as_bytes(), &main_type.str_in_ref[0..10]);
    assert_eq!("input inout\0".as_bytes(), &main_type.str_inout[0..12]);
}

#[ignore = "Cannot generate a LValue for CastStatement fix in new issue"]
#[test]
fn assigning_global_strings_in_program_by_passing_literals() {
    let src = r#"
        PROGRAM prog
        VAR_INPUT
            in : STRING;
        END_VAR
        VAR_INPUT {ref}
            in_ref : STRING;
        END_VAR
            glob_str_in     := in;
            glob_str_in_ref := in_ref;
        END_PROGRAM

        PROGRAM main
        VAR
            str_in      : STRING;
            str_in_ref  : STRING;
        END_VAR
            prog('in literal', STRING#'in ref literal');

            str_in      := glob_str_in;
            str_in_ref  := glob_str_in_ref;
        END_PROGRAM

        VAR_GLOBAL
            glob_str_in     : STRING;
            glob_str_in_ref : STRING;
        END_VAR
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        str_in: [u8; 81],
        str_in_ref: [u8; 81],
    }
    let mut main_type = MainType { str_in: [0; 81], str_in_ref: [0; 81] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("in literal\0".as_bytes(), &main_type.str_in[0..11]);
    assert_eq!("in ref literal\0".as_bytes(), &main_type.str_in_ref[0..15]);
}

#[test]
fn function_wstring_memcopies_the_right_amount_of_bytes() {
    let src = r#"
        FUNCTION foo : WSTRING[7]
        VAR_INPUT
            s : WSTRING[7];
        END_VAR
            foo := s;
        END_FUNCTION

        PROGRAM main
        VAR
            y : WSTRING[7];
        END_VAR
            y := foo("wstring cutoff");
        END_PROGRAM
    "#;

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        y: [u16; 8],
    }
    let mut main_type = MainType { y: [0; 8] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("wstring", String::from_utf16_lossy(&main_type.y[..7]));
}
