use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[test]
fn wstring_to_string_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u8; 6],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : STRING[5];
    END_VAR
        res := WSTRING_TO_STRING(WSTRING#"hello");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(&maintype.res, "hello\0".as_bytes());
}

#[test]
fn empty_wstring_to_string_conversion() {
    #[repr(C)]
    struct MainType {
        res: [u8; 81],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : STRING;
    END_VAR
        res := WSTRING_TO_STRING("");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType { res: [0; 81] };
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, [0; 81]);
}

#[test]
fn wstring_to_string_extra_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u8; 15],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : STRING[14];
    END_VAR
        res := WSTRING_TO_STRING(WSTRING#"hèßlo👽️");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(&String::from_utf8_lossy(&maintype.res), "hèßlo👽️\0");
    assert_eq!(&maintype.res, "hèßlo👽️\0".as_bytes());
}

#[test]
fn wstring_to_string_conversion_long() {
    #[repr(C)]
    struct MainType {
        res: [u8; 81],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : STRING;
    END_VAR
        res := WSTRING_TO_STRING("111111111122222222223333333333444444444455555555556666666666777777777788888888889999999999");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType { res: [0; 81] };
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        &String::from_utf8_lossy(&maintype.res),
        "11111111112222222222333333333344444444445555555555666666666677777777778888888888\0"
    );
    assert_eq!(
        &maintype.res,
        "11111111112222222222333333333344444444445555555555666666666677777777778888888888\0".as_bytes()
    );
}

#[test]
fn wstring_to_wchar_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u16; 2],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : WCHAR;
    END_VAR
        res := WSTRING_TO_WCHAR(WSTRING#"ABC");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, [65u16, 0u16]);
}

#[test]
fn string_to_wstring_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u16; 6],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : WSTRING[5];
    END_VAR
        res := STRING_TO_WSTRING(STRING#'Hello');
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, [72u16, 101u16, 108u16, 108u16, 111u16, 0u16]);
}

#[test]
fn empty_string_to_wstring_conversion() {
    #[repr(C)]
    struct MainType {
        res: [u16; 81],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : WSTRING;
    END_VAR
        res := STRING_TO_WSTRING('');
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType { res: [0; 81] };
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, [0; 81]);
}

#[test]
fn string_to_wstring_extra_conversion() {
    struct MainType {
        res: [u16; 8],
    }
    let src = r#"
    PROGRAM main
    VAR
        res : WSTRING[7];
    END_VAR
        res := STRING_TO_WSTRING('Hèßlo😀');
    END_PROGRAM
        "#;

    let mut exp = [0; 8];
    for (i, c) in "Hèßlo😀\0".encode_utf16().enumerate() {
        exp[i] = c;
    }
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType { res: [0; 8] };
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(&String::from_utf16_lossy(&maintype.res), "Hèßlo😀\0");
    assert_eq!(&maintype.res, &exp);
}

#[test]
fn string_to_wstring_long_conversion() {
    struct MainType {
        res: [u16; 81],
    }
    let src = r#"
    PROGRAM main
    VAR
        res : WSTRING;
    END_VAR
        res := STRING_TO_WSTRING('111111111122222222223333333333444444444455555555556666666666777777777788888888889999999999');
    END_PROGRAM
        "#;

    let mut exp = [0; 81];
    for (i, c) in "11111111112222222222333333333344444444445555555555666666666677777777778888888888"
        .encode_utf16()
        .enumerate()
    {
        exp[i] = c;
    }
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType { res: [0; 81] };
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        &String::from_utf16_lossy(&maintype.res),
        "11111111112222222222333333333344444444445555555555666666666677777777778888888888\0"
    );
    assert_eq!(&maintype.res, &exp);
}

#[test]
fn string_to_char_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u8; 2],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : CHAR;
    END_VAR
        res := STRING_TO_CHAR(STRING#'BCD');
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, "B\0".as_bytes());
}

#[test]
fn wchar_to_wstring_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u16; 2],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : WSTRING[1];
    END_VAR
        res := WCHAR_TO_WSTRING(WCHAR#"A");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(&maintype.res, &[65u16, 0u16]);
}

#[test]
fn wchar_to_char_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u8; 2],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : CHAR;
    END_VAR
        res := WCHAR_TO_CHAR(WCHAR#"A");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, "A\0".as_bytes());
}

#[test]
fn char_to_string_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u8; 2],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : STRING[1];
    END_VAR
        res := CHAR_TO_STRING(CHAR#'B');
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, "B\0".as_bytes());
}

#[test]
fn char_to_wchar_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u16; 2],
    }

    let src = r#"
    PROGRAM main
    VAR
        res : WCHAR;
    END_VAR
        res := CHAR_TO_WCHAR(CHAR#'B');
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st", "string_functions.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, [66u16, 0u16]);
}
