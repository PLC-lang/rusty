use common::{compile_and_run, get_includes};

// Import common functionality into the integration tests
mod common;

#[derive(Default)]
struct F64Type {
    zero: f64,
    max: f64,
}

#[derive(Default)]
struct F32Type {
    zero: f32,
    max: f32,
}

#[derive(Default)]
struct I64Type {
    zero: i64,
    max: i64,
}

#[derive(Default)]
struct I32Type {
    zero: i32,
    max: i32,
    max_overflow: i32,
    negative: i32,
}

#[derive(Default)]
struct I16Type {
    zero: i16,
    max: i16,
}

#[derive(Default)]
struct I8Type {
    zero: i8,
    max: i8,
}

#[derive(Default)]
struct U64Type {
    zero: u64,
    max: u64,
}

#[derive(Default)]
struct U32Type {
    zero: u32,
    max: u32,
}

#[derive(Default)]
struct U16Type {
    zero: u16,
    max: u16,
}

#[derive(Default)]
struct U8Type {
    zero: u8,
    max: u8,
}

#[test]
fn lword_to_lreal_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LREAL; max : LREAL;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LWORD_TO_LREAL(LWORD#0);
        // bit transfer for conversion 4611686018427387904 should be the first bit from the exponent resulting in decimal 2
        ret.max := LWORD_TO_LREAL(LWORD#4611686018427387904);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = F64Type::default();
    let _res: f64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0.0f64);
    assert_eq!(maintype.max, 2f64);
}

#[test]
fn lword_to_lint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LINT; max : LINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LWORD_TO_LINT(LWORD#0);
        ret.max := LWORD_TO_LINT(LWORD#9223372036854775807);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.max, 9223372036854775807i64);
}

#[test]
fn lword_to_dint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DINT; max : DINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LWORD_TO_DINT(LWORD#0);
        ret.max := LWORD_TO_DINT(LWORD#2147483647);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.max, 2147483647i32);
}

#[test]
fn lword_to_int_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : INT; max : INT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LWORD_TO_INT(LWORD#0);
        ret.max := LWORD_TO_INT(LWORD#32767);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.max, 32767i16);
}

#[test]
fn lword_to_sint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : SINT; max : SINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LWORD_TO_SINT(LWORD#0);
        ret.max := LWORD_TO_SINT(LWORD#127);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.max, 127i8);
}

#[test]
fn lword_to_ulint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : ULINT; max : ULINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LWORD_TO_ULINT(LWORD#0);
        ret.max := LWORD_TO_ULINT(LWORD#18446744073709551615);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 18446744073709551615u64);
}

#[test]
fn lword_to_udint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : UDINT; max : UDINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LWORD_TO_UDINT(LWORD#0);
        ret.max := LWORD_TO_UDINT(LWORD#4294967295);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 4294967295u32);
}

#[test]
fn lword_to_uint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : UINT; max : UINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LWORD_TO_UINT(LWORD#0);
        ret.max := LWORD_TO_UINT(LWORD#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 65535u16);
}

#[test]
fn lword_to_usint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : USINT; max : USINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LWORD_TO_USINT(LWORD#0);
        ret.max := LWORD_TO_USINT(LWORD#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}

#[test]
fn dword_to_real_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : REAL; max : REAL;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DWORD_TO_REAL(DWORD#0);
        // bit transfer for conversion 1073741824 should be the first bit from the exponent resulting in decimal 2
        ret.max := DWORD_TO_REAL(DWORD#1073741824);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = F32Type::default();
    let _res: f32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0.0f32);
    assert_eq!(maintype.max, 2f32);
}

#[test]
fn dword_to_lint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LINT; max : LINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DWORD_TO_LINT(DWORD#0);
        ret.max := DWORD_TO_LINT(DWORD#4294967295);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.max, 4294967295i64);
}

#[test]
fn dword_to_dint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DINT; max : DINT; max_overflow : DINT; negative : DINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DWORD_TO_DINT(DWORD#0);
        ret.max := DWORD_TO_DINT(DWORD#2147483647);
        ret.max_overflow := DWORD_TO_DINT(DWORD#4294967295);
        ret.negative := DWORD_TO_DINT(-1);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.max, 2147483647i32);
    assert_eq!(maintype.max_overflow, -1i32);
    assert_eq!(maintype.negative, -1i32);
}

#[test]
fn dword_to_int_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : INT; max : INT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DWORD_TO_INT(DWORD#0);
        ret.max := DWORD_TO_INT(DWORD#32767);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.max, 32767i16);
}

#[test]
fn dword_to_sint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : SINT; max : SINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DWORD_TO_SINT(DWORD#0);
        ret.max := DWORD_TO_SINT(DWORD#127);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.max, 127i8);
}

#[test]
fn dword_to_ulint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : ULINT; max : ULINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DWORD_TO_ULINT(DWORD#0);
        ret.max := DWORD_TO_ULINT(DWORD#4294967295);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 4294967295u64);
}

#[test]
fn dword_to_udint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : UDINT; max : UDINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DWORD_TO_UDINT(DWORD#0);
        ret.max := DWORD_TO_UDINT(DWORD#4294967295);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 4294967295u32);
}

#[test]
fn dword_to_uint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : UINT; max : UINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DWORD_TO_UINT(DWORD#0);
        ret.max := DWORD_TO_UINT(DWORD#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 65535u16);
}

#[test]
fn dword_to_usint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : USINT; max : USINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DWORD_TO_USINT(DWORD#0);
        ret.max := DWORD_TO_USINT(DWORD#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}

#[test]
fn word_to_lint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LINT; max : LINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := WORD_TO_LINT(WORD#0);
        ret.max := WORD_TO_LINT(WORD#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.max, 65535i64);
}

#[test]
fn word_to_dint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DINT; max : DINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := WORD_TO_DINT(WORD#0);
        ret.max := WORD_TO_DINT(WORD#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.max, 65535i32);
}

#[test]
fn word_to_int_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : INT; max : INT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := WORD_TO_INT(WORD#0);
        ret.max := WORD_TO_INT(WORD#32767);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.max, 32767i16);
}

#[test]
fn word_to_sint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : SINT; max : SINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := WORD_TO_SINT(WORD#0);
        ret.max := WORD_TO_SINT(WORD#127);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.max, 127i8);
}

#[test]
fn word_to_ulint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : ULINT; max : ULINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := WORD_TO_ULINT(WORD#0);
        ret.max := WORD_TO_ULINT(WORD#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 65535u64);
}

#[test]
fn word_to_udint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : UDINT; max : UDINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := WORD_TO_UDINT(WORD#0);
        ret.max := WORD_TO_UDINT(WORD#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 65535u32);
}

#[test]
fn word_to_uint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : UINT; max : UINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := WORD_TO_UINT(WORD#0);
        ret.max := WORD_TO_UINT(WORD#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 65535u16);
}

#[test]
fn word_to_usint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : USINT; max : USINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := WORD_TO_USINT(WORD#0);
        ret.max := WORD_TO_USINT(WORD#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}

#[test]
fn byte_to_lint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LINT; max : LINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BYTE_TO_LINT(BYTE#0);
        ret.max := BYTE_TO_LINT(BYTE#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.max, 255i64);
}

#[test]
fn byte_to_dint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DINT; max : DINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BYTE_TO_DINT(BYTE#0);
        ret.max := BYTE_TO_DINT(BYTE#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.max, 255i32);
}

#[test]
fn byte_to_int_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : INT; max : INT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BYTE_TO_INT(BYTE#0);
        ret.max := BYTE_TO_INT(BYTE#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.max, 255i16);
}

#[test]
fn byte_to_sint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : SINT; max : SINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BYTE_TO_SINT(BYTE#0);
        ret.max := BYTE_TO_SINT(BYTE#127);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.max, 127i8);
}

#[test]
fn byte_to_ulint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : ULINT; max : ULINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BYTE_TO_ULINT(BYTE#0);
        ret.max := BYTE_TO_ULINT(BYTE#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 255u64);
}

#[test]
fn byte_to_udint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : UDINT; max : UDINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BYTE_TO_UDINT(BYTE#0);
        ret.max := BYTE_TO_UDINT(BYTE#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 255u32);
}

#[test]
fn byte_to_uint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : UINT; max : UINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BYTE_TO_UINT(BYTE#0);
        ret.max := BYTE_TO_UINT(BYTE#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 255u16);
}

#[test]
fn byte_to_usint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : USINT; max : USINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BYTE_TO_USINT(BYTE#0);
        ret.max := BYTE_TO_USINT(BYTE#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}

#[test]
fn bool_to_lint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LINT; max : LINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BOOL_TO_LINT(BOOL#0);
        ret.max := BOOL_TO_LINT(BOOL#1);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.max, 1i64);
}

#[test]
fn bool_to_dint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DINT; max : DINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BOOL_TO_DINT(BOOL#0);
        ret.max := BOOL_TO_DINT(BOOL#1);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.max, 1i32);
}

#[test]
fn bool_to_int_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : INT; max : INT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BOOL_TO_INT(BOOL#0);
        ret.max := BOOL_TO_INT(BOOL#1);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.max, 1i16);
}

#[test]
fn bool_to_sint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : SINT; max : SINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BOOL_TO_SINT(BOOL#0);
        ret.max := BOOL_TO_SINT(BOOL#1);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.max, 1i8);
}

#[test]
fn bool_to_ulint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : ULINT; max : ULINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BOOL_TO_ULINT(BOOL#0);
        ret.max := BOOL_TO_ULINT(BOOL#1);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 1u64);
}

#[test]
fn bool_to_udint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : UDINT; max : UDINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BOOL_TO_UDINT(BOOL#0);
        ret.max := BOOL_TO_UDINT(BOOL#1);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 1u32);
}

#[test]
fn bool_to_uint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : UINT; max : UINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BOOL_TO_UINT(BOOL#0);
        ret.max := BOOL_TO_UINT(BOOL#1);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 1u16);
}

#[test]
fn bool_to_usint_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : USINT; max : USINT;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := BOOL_TO_USINT(BOOL#0);
        ret.max := BOOL_TO_USINT(BOOL#1);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 1u8);
}

#[test]
fn lreal_to_lword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LWORD; max : LWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LREAL_TO_LWORD(LREAL#0);
        // counter test LWORD_TO_LREAL
        // 2 in LREAL is the first bit from exponent 2^62 = 4611686018427387904
        ret.max := LREAL_TO_LWORD(LREAL#2);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 4611686018427387904u64);
}

#[test]
fn real_to_dword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DWORD; max : DWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := REAL_TO_DWORD(REAL#0);
        // counter test DWORD_TO_REAL
        // 2 in REAL is the first bit from exponent 30 = 1073741824
        ret.max := REAL_TO_DWORD(REAL#2);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 1073741824u32);
}

#[test]
fn lint_to_lword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LWORD; max : LWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LINT_TO_LWORD(LINT#0);
        ret.max := LINT_TO_LWORD(LINT#9223372036854775807);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 9223372036854775807u64);
}

#[test]
fn lint_to_dword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DWORD; max : DWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LINT_TO_DWORD(LINT#0);
        ret.max := LINT_TO_DWORD(LINT#4294967295);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 4294967295u32);
}

#[test]
fn lint_to_word_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : WORD; max : WORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LINT_TO_WORD(LINT#0);
        ret.max := LINT_TO_WORD(LINT#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 65535u16);
}

#[test]
fn lint_to_byte_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : BYTE; max : BYTE;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := LINT_TO_BYTE(LINT#0);
        ret.max := LINT_TO_BYTE(LINT#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}

#[test]
fn dint_to_lword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LWORD; max : LWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DINT_TO_LWORD(DINT#0);
        ret.max := DINT_TO_LWORD(DINT#2147483647);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 2147483647u64);
}

#[test]
fn dint_to_dword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DWORD; max : DWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DINT_TO_DWORD(DINT#0);
        ret.max := DINT_TO_DWORD(DINT#2147483647);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 2147483647u32);
}

#[test]
fn dint_to_word_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : WORD; max : WORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DINT_TO_WORD(DINT#0);
        ret.max := DINT_TO_WORD(DINT#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 65535u16);
}

#[test]
fn dint_to_byte_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : BYTE; max : BYTE;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := DINT_TO_BYTE(DINT#0);
        ret.max := DINT_TO_BYTE(DINT#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}

#[test]
fn int_to_lword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LWORD; max : LWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := INT_TO_LWORD(INT#0);
        ret.max := INT_TO_LWORD(INT#32767);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 32767u64);
}

#[test]
fn int_to_dword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DWORD; max : DWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := INT_TO_DWORD(INT#0);
        ret.max := INT_TO_DWORD(INT#32767);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 32767u32);
}

#[test]
fn int_to_word_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : WORD; max : WORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := INT_TO_WORD(INT#0);
        ret.max := INT_TO_WORD(INT#32767);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 32767u16);
}

#[test]
fn int_to_byte_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : BYTE; max : BYTE;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := INT_TO_BYTE(INT#0);
        ret.max := INT_TO_BYTE(INT#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}

#[test]
fn sint_to_lword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LWORD; max : LWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := SINT_TO_LWORD(SINT#0);
        ret.max := SINT_TO_LWORD(SINT#127);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 127u64);
}

#[test]
fn sint_to_dword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DWORD; max :DWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := SINT_TO_DWORD(SINT#0);
        ret.max := SINT_TO_DWORD(SINT#127);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 127u32);
}

#[test]
fn sint_to_word_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : WORD; max : WORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := SINT_TO_WORD(SINT#0);
        ret.max := SINT_TO_WORD(SINT#127);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 127u16);
}

#[test]
fn sint_to_byte_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : BYTE; max : BYTE;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := SINT_TO_BYTE(SINT#0);
        ret.max := SINT_TO_BYTE(SINT#127);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 127u8);
}

#[test]
fn ulint_to_lword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LWORD; max : LWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := ULINT_TO_LWORD(ULINT#0);
        ret.max := ULINT_TO_LWORD(ULINT#18446744073709551615);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 18446744073709551615u64);
}

#[test]
fn ulint_to_dword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DWORD; max : DWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := ULINT_TO_DWORD(ULINT#0);
        ret.max := ULINT_TO_DWORD(ULINT#4294967295);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 4294967295u32);
}

#[test]
fn ulint_to_word_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : WORD; max : WORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := ULINT_TO_WORD(ULINT#0);
        ret.max := ULINT_TO_WORD(ULINT#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 65535u16);
}

#[test]
fn ulint_to_byte_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : BYTE; max : BYTE;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := ULINT_TO_BYTE(ULINT#0);
        ret.max := ULINT_TO_BYTE(ULINT#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}

#[test]
fn udint_to_lword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LWORD; max : LWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := UDINT_TO_LWORD(UDINT#0);
        ret.max := UDINT_TO_LWORD(UDINT#4294967295);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 4294967295u64);
}

#[test]
fn udint_to_dword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DWORD; max : DWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := UDINT_TO_DWORD(UDINT#0);
        ret.max := UDINT_TO_DWORD(UDINT#4294967295);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 4294967295u32);
}

#[test]
fn udint_to_word_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : WORD; max : WORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := UDINT_TO_WORD(UDINT#0);
        ret.max := UDINT_TO_WORD(UDINT#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 65535u16);
}

#[test]
fn udint_to_byte_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : BYTE; max : BYTE;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := UDINT_TO_BYTE(UDINT#0);
        ret.max := UDINT_TO_BYTE(UDINT#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}

#[test]
fn uint_to_lword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LWORD; max : LWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := UINT_TO_LWORD(UINT#0);
        ret.max := UINT_TO_LWORD(UINT#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 65535u64);
}

#[test]
fn uint_to_dword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DWORD; max : DWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := UINT_TO_DWORD(UINT#0);
        ret.max := UINT_TO_DWORD(UINT#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 65535u32);
}

#[test]
fn uint_to_word_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : WORD; max : WORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := UINT_TO_WORD(UINT#0);
        ret.max := UINT_TO_WORD(UINT#65535);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 65535u16);
}

#[test]
fn uint_to_byte_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : BYTE; max : BYTE;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := UINT_TO_BYTE(UINT#0);
        ret.max := UINT_TO_BYTE(UINT#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}

#[test]
fn usint_to_lword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : LWORD; max : LWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := USINT_TO_LWORD(USINT#0);
        ret.max := USINT_TO_LWORD(USINT#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.max, 255u64);
}

#[test]
fn usint_to_dword_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : DWORD; max : DWORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := USINT_TO_DWORD(USINT#0);
        ret.max := USINT_TO_DWORD(USINT#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.max, 255u32);
}

#[test]
fn usint_to_word_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : WORD; max : WORD;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := USINT_TO_WORD(USINT#0);
        ret.max := USINT_TO_WORD(USINT#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.max, 255u16);
}

#[test]
fn usint_to_byte_conversion() {
    let src = r"
    TYPE myType : STRUCT
        zero : BYTE; max : BYTE;
    END_STRUCT END_TYPE

    PROGRAM main
    VAR
        ret : myType;
    END_VAR
        ret.zero := USINT_TO_BYTE(USINT#0);
        ret.max := USINT_TO_BYTE(USINT#255);
    END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["bit_num_conversion.st"]);
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.max, 255u8);
}
