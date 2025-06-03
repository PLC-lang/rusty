use crate::test_utils::tests::parse_and_validate_buffered;

#[test]
fn pointer_to_ignores_types_when_assigned_with_adr_call_in_initializer() {
    let source = r#"
    TYPE Position1D:
        STRUCT
            x : INT;
        END_STRUCT
    END_TYPE

    TYPE Position2D:
        STRUCT
            x : INT;
            y : INT;
        END_STRUCT
    END_TYPE

    TYPE Position3D:
        STRUCT
            x : INT;
            y : INT;
            z : INT;
        END_STRUCT
    END_TYPE

    FUNCTION main
        VAR
            value_sint  : SINT;
            value_int   : INT;
            value_dint  : DINT;
            value_lint  : LINT;
            value_real  : REAL;
            value_wstr  : WSTRING;
            value_time  : TIME;
            value_tod   : TIME_OF_DAY;
            value_date  : DATE;
            value_dt    : DATE_AND_TIME;
            value_bool  : BOOL;
            value_byte  : BYTE;
            value_word  : WORD;
            value_dword : DWORD;
            value_lword : LWORD;

            value_pos1d : Position1D;
            value_pos2d : Position2D;
            value_pos3d : Position3D;

            // All of these, while not correct per-se, are valid because `POINTER TO` ignores type checks so 
            // long as the right-hand side is an integer value
            ptr_sint    : POINTER TO STRING := ADR(value_sint);
            ptr_int     : POINTER TO STRING := ADR(value_int);
            ptr_dint    : POINTER TO STRING := ADR(value_dint);
            ptr_lint    : POINTER TO STRING := ADR(value_lint);
            ptr_real    : POINTER TO STRING := ADR(value_real);
            ptr_wstr    : POINTER TO STRING := ADR(value_wstr);
            ptr_time    : POINTER TO STRING := ADR(value_time);
            ptr_tod     : POINTER TO STRING := ADR(value_tod);
            ptr_date    : POINTER TO STRING := ADR(value_date);
            ptr_dt      : POINTER TO STRING := ADR(value_dt);
            ptr_bool    : POINTER TO STRING := ADR(value_bool);
            ptr_byte    : POINTER TO STRING := ADR(value_byte);
            ptr_word    : POINTER TO STRING := ADR(value_word);
            ptr_dword   : POINTER TO STRING := ADR(value_dword);
            ptr_lword   : POINTER TO STRING := ADR(value_lword);
            ptr_pos1d   : POINTER TO STRING := ADR(value_pos1d);
            ptr_pos2d   : POINTER TO STRING := ADR(value_pos2d);
            ptr_pos3d   : POINTER TO STRING := ADR(value_pos3d);
        END_VAR
    END_FUNCTION
    "#;

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:47:27
       │
    47 │             ptr_sint    : POINTER TO STRING := ADR(value_sint);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:48:27
       │
    48 │             ptr_int     : POINTER TO STRING := ADR(value_int);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:49:27
       │
    49 │             ptr_dint    : POINTER TO STRING := ADR(value_dint);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:50:27
       │
    50 │             ptr_lint    : POINTER TO STRING := ADR(value_lint);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:51:27
       │
    51 │             ptr_real    : POINTER TO STRING := ADR(value_real);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:52:27
       │
    52 │             ptr_wstr    : POINTER TO STRING := ADR(value_wstr);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:53:27
       │
    53 │             ptr_time    : POINTER TO STRING := ADR(value_time);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:54:27
       │
    54 │             ptr_tod     : POINTER TO STRING := ADR(value_tod);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:55:27
       │
    55 │             ptr_date    : POINTER TO STRING := ADR(value_date);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:56:27
       │
    56 │             ptr_dt      : POINTER TO STRING := ADR(value_dt);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:57:27
       │
    57 │             ptr_bool    : POINTER TO STRING := ADR(value_bool);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:58:27
       │
    58 │             ptr_byte    : POINTER TO STRING := ADR(value_byte);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:59:27
       │
    59 │             ptr_word    : POINTER TO STRING := ADR(value_word);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:60:27
       │
    60 │             ptr_dword   : POINTER TO STRING := ADR(value_dword);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:61:27
       │
    61 │             ptr_lword   : POINTER TO STRING := ADR(value_lword);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:62:27
       │
    62 │             ptr_pos1d   : POINTER TO STRING := ADR(value_pos1d);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:63:27
       │
    63 │             ptr_pos2d   : POINTER TO STRING := ADR(value_pos2d);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:64:27
       │
    64 │             ptr_pos3d   : POINTER TO STRING := ADR(value_pos3d);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
    ");
}

#[test]
fn pointer_to_ignores_types_when_assigned_with_adr_call_in_body() {
    let source = r#"
    TYPE Position1D:
        STRUCT
            x : INT;
        END_STRUCT
    END_TYPE

    TYPE Position2D:
        STRUCT
            x : INT;
            y : INT;
        END_STRUCT
    END_TYPE

    TYPE Position3D:
        STRUCT
            x : INT;
            y : INT;
            z : INT;
        END_STRUCT
    END_TYPE

    FUNCTION main
        VAR
            ptr         : POINTER TO STRING;

            value_sint  : SINT;
            value_int   : INT;
            value_dint  : DINT;
            value_lint  : LINT;
            value_real  : REAL;
            value_wstr  : WSTRING;
            value_time  : TIME;
            value_tod   : TIME_OF_DAY;
            value_date  : DATE;
            value_dt    : DATE_AND_TIME;
            value_bool  : BOOL;
            value_byte  : BYTE;
            value_word  : WORD;
            value_dword : DWORD;
            value_lword : LWORD;

            value_pos1d : Position1D;
            value_pos2d : Position2D;
            value_pos3d : Position3D;
        END_VAR

        // All of these, while not correct per-se, are valid because `POINTER TO` ignores type checks so long
        // as the right-hand side is an integer value
        ptr := ADR(value_sint);
        ptr := ADR(value_int);
        ptr := ADR(value_dint);
        ptr := ADR(value_lint);
        ptr := ADR(value_real);
        ptr := ADR(value_wstr);
        ptr := ADR(value_time);
        ptr := ADR(value_tod);
        ptr := ADR(value_date);
        ptr := ADR(value_dt);
        ptr := ADR(value_bool);
        ptr := ADR(value_byte);
        ptr := ADR(value_word);
        ptr := ADR(value_dword);
        ptr := ADR(value_lword);
        ptr := ADR(value_pos1d);
        ptr := ADR(value_pos2d);
        ptr := ADR(value_pos3d);
    END_FUNCTION
    "#;

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:25:27
       │
    25 │             ptr         : POINTER TO STRING;
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
    ");
}

#[test]
fn pointer_to_validates_types_when_assigned_with_ref_call_in_initializer() {
    let source = r#"
    TYPE Position1D:
        STRUCT
            x : INT;
        END_STRUCT
    END_TYPE

    TYPE Position2D:
        STRUCT
            x : INT;
            y : INT;
        END_STRUCT
    END_TYPE

    TYPE Position3D:
        STRUCT
            x : INT;
            y : INT;
            z : INT;
        END_STRUCT
    END_TYPE

    FUNCTION main
        VAR
            value_sint  : SINT;
            value_int   : INT;
            value_dint  : DINT;
            value_lint  : LINT;
            value_real  : REAL;
            value_wstr  : WSTRING;
            value_time  : TIME;
            value_tod   : TIME_OF_DAY;
            value_date  : DATE;
            value_dt    : DATE_AND_TIME;
            value_bool  : BOOL;
            value_byte  : BYTE;
            value_word  : WORD;
            value_dword : DWORD;
            value_lword : LWORD;
            value_pos1d : Position1D;
            value_pos2d : Position2D;
            value_pos3d : Position3D;

            // Valid assignments
            ptr_sint    : POINTER TO DINT := REF(value_sint);
            ptr_int     : POINTER TO DINT := REF(value_int);
            ptr_dint    : POINTER TO DINT := REF(value_dint);
            ptr_lint    : POINTER TO DINT := REF(value_lint);
            ptr_time    : POINTER TO DINT := REF(value_time);
            ptr_tod     : POINTER TO DINT := REF(value_tod);
            ptr_date    : POINTER TO DINT := REF(value_date);
            ptr_dt      : POINTER TO DINT := REF(value_dt);
            ptr_bool    : POINTER TO DINT := REF(value_bool);
            ptr_byte    : POINTER TO DINT := REF(value_byte);
            ptr_word    : POINTER TO DINT := REF(value_word);
            ptr_dword   : POINTER TO DINT := REF(value_dword);
            ptr_lword   : POINTER TO DINT := REF(value_lword);
            ptr_real    : POINTER TO DINT := REF(value_real);
            ptr_wstr    : POINTER TO DINT := REF(value_wstr);
            ptr_pos1d   : POINTER TO DINT := REF(value_pos1d);
            ptr_pos2d   : POINTER TO DINT := REF(value_pos2d);
            ptr_pos3d   : POINTER TO DINT := REF(value_pos3d);

        END_VAR
        
    END_FUNCTION
    "#;

    // TODO(vosa): How strict / annoying do we want to be here? Obviously both smaller and bigger types than
    //             `DINT` could result in funky behavior (under- and overflows), hence a warning if lhs != rhs
    //             but that can become very annoying very quickly?
    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:45:27
       │
    45 │             ptr_sint    : POINTER TO DINT := REF(value_sint);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:46:27
       │
    46 │             ptr_int     : POINTER TO DINT := REF(value_int);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:47:27
       │
    47 │             ptr_dint    : POINTER TO DINT := REF(value_dint);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:48:27
       │
    48 │             ptr_lint    : POINTER TO DINT := REF(value_lint);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:49:27
       │
    49 │             ptr_time    : POINTER TO DINT := REF(value_time);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:50:27
       │
    50 │             ptr_tod     : POINTER TO DINT := REF(value_tod);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:51:27
       │
    51 │             ptr_date    : POINTER TO DINT := REF(value_date);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:52:27
       │
    52 │             ptr_dt      : POINTER TO DINT := REF(value_dt);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:53:27
       │
    53 │             ptr_bool    : POINTER TO DINT := REF(value_bool);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:54:27
       │
    54 │             ptr_byte    : POINTER TO DINT := REF(value_byte);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:55:27
       │
    55 │             ptr_word    : POINTER TO DINT := REF(value_word);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:56:27
       │
    56 │             ptr_dword   : POINTER TO DINT := REF(value_dword);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:57:27
       │
    57 │             ptr_lword   : POINTER TO DINT := REF(value_lword);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:58:27
       │
    58 │             ptr_real    : POINTER TO DINT := REF(value_real);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:59:27
       │
    59 │             ptr_wstr    : POINTER TO DINT := REF(value_wstr);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:60:27
       │
    60 │             ptr_pos1d   : POINTER TO DINT := REF(value_pos1d);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:61:27
       │
    61 │             ptr_pos2d   : POINTER TO DINT := REF(value_pos2d);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:62:27
       │
    62 │             ptr_pos3d   : POINTER TO DINT := REF(value_pos3d);
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
    ");
}

#[test]
fn pointer_to_validates_types_when_assigned_with_ref_call_body() {
    let source = r#"
    TYPE Position1D:
        STRUCT
            x : INT;
        END_STRUCT
    END_TYPE

    TYPE Position2D:
        STRUCT
            x : INT;
            y : INT;
        END_STRUCT
    END_TYPE

    TYPE Position3D:
        STRUCT
            x : INT;
            y : INT;
            z : INT;
        END_STRUCT
    END_TYPE

    FUNCTION main
        VAR
            ptr         : POINTER TO DINT;

            value_sint  : SINT;
            value_int   : INT;
            value_dint  : DINT;

            value_lint  : LINT;
            value_real  : REAL;
            value_wstr  : WSTRING;
            value_time  : TIME;
            value_tod   : TIME_OF_DAY;
            value_date  : DATE;
            value_dt    : DATE_AND_TIME;
            value_bool  : BOOL;
            value_byte  : BYTE;
            value_word  : WORD;
            value_dword : DWORD;
            value_lword : LWORD;
            value_pos1d : Position1D;
            value_pos2d : Position2D;
            value_pos3d : Position3D;
        END_VAR

        // Valid assignments
        ptr := REF(value_sint);
        ptr := REF(value_int);
        ptr := REF(value_dint);
        ptr := REF(value_lint);
        ptr := REF(value_time);
        ptr := REF(value_tod);
        ptr := REF(value_date);
        ptr := REF(value_dt);
        ptr := REF(value_bool);
        ptr := REF(value_byte);
        ptr := REF(value_word);
        ptr := REF(value_dword);
        ptr := REF(value_lword);

        // Invalid assignments
        ptr := REF(value_real);
        ptr := REF(value_wstr);
        ptr := REF(value_pos1d);
        ptr := REF(value_pos2d);
        ptr := REF(value_pos3d);
    END_FUNCTION
    "#;

    // TODO(vosa): How strict / annoying do we want to be here? Obviously both smaller and bigger types than
    //             `DINT` could result in funky behavior (under- and overflows), hence a warning if lhs != rhs
    //             but that can become very annoying very quickly?
    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
       ┌─ <internal>:25:27
       │
    25 │             ptr         : POINTER TO DINT;
       │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
    ");
}
