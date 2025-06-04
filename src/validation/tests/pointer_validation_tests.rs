use crate::test_utils::tests::parse_and_validate;

#[test]
fn pointer_to_ignores_type_checks_in_initializer() {
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

            // All of these are valid because `POINTER TO` is type-unsafe
            adr_call_sint    : POINTER TO STRING := ADR(value_sint);
            adr_call_int     : POINTER TO STRING := ADR(value_int);
            adr_call_dint    : POINTER TO STRING := ADR(value_dint);
            adr_call_lint    : POINTER TO STRING := ADR(value_lint);
            adr_call_real    : POINTER TO STRING := ADR(value_real);
            adr_call_wstr    : POINTER TO STRING := ADR(value_wstr);
            adr_call_time    : POINTER TO STRING := ADR(value_time);
            adr_call_tod     : POINTER TO STRING := ADR(value_tod);
            adr_call_date    : POINTER TO STRING := ADR(value_date);
            adr_call_dt      : POINTER TO STRING := ADR(value_dt);
            adr_call_bool    : POINTER TO STRING := ADR(value_bool);
            adr_call_byte    : POINTER TO STRING := ADR(value_byte);
            adr_call_word    : POINTER TO STRING := ADR(value_word);
            adr_call_dword   : POINTER TO STRING := ADR(value_dword);
            adr_call_lword   : POINTER TO STRING := ADR(value_lword);
            adr_call_pos1d   : POINTER TO STRING := ADR(value_pos1d);
            adr_call_pos2d   : POINTER TO STRING := ADR(value_pos2d);
            adr_call_pos3d   : POINTER TO STRING := ADR(value_pos3d);

            // same here, even though a `REF` call returns a `REF_TO <datatype>`, i.e. type-information
            ref_call_sint    : POINTER TO STRING := REF(value_sint);
            ref_call_int     : POINTER TO STRING := REF(value_int);
            ref_call_dint    : POINTER TO STRING := REF(value_dint);
            ref_call_lint    : POINTER TO STRING := REF(value_lint);
            ref_call_real    : POINTER TO STRING := REF(value_real);
            ref_call_wstr    : POINTER TO STRING := REF(value_wstr);
            ref_call_time    : POINTER TO STRING := REF(value_time);
            ref_call_tod     : POINTER TO STRING := REF(value_tod);
            ref_call_date    : POINTER TO STRING := REF(value_date);
            ref_call_dt      : POINTER TO STRING := REF(value_dt);
            ref_call_bool    : POINTER TO STRING := REF(value_bool);
            ref_call_byte    : POINTER TO STRING := REF(value_byte);
            ref_call_word    : POINTER TO STRING := REF(value_word);
            ref_call_dword   : POINTER TO STRING := REF(value_dword);
            ref_call_lword   : POINTER TO STRING := REF(value_lword);
            ref_call_pos1d   : POINTER TO STRING := REF(value_pos1d);
            ref_call_pos2d   : POINTER TO STRING := REF(value_pos2d);
            ref_call_pos3d   : POINTER TO STRING := REF(value_pos3d);
        END_VAR
    END_FUNCTION
    "#;

    let diagnostics = parse_and_validate(source);
    let filtered_diagnostics =
        diagnostics.into_iter().filter(|diagnostic| diagnostic.error_code != "E015").collect::<Vec<_>>();
    assert_eq!(filtered_diagnostics, Vec::new());
}

#[test]
fn pointer_to_ignores_type_checks_in_body() {
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

        // All of these are valid because `POINTER TO` is type-unsafe
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

        // same here, even though a `REF` call returns a `REF_TO <datatype>`, i.e. type-information
        ptr := REF(value_sint);
        ptr := REF(value_int);
        ptr := REF(value_dint);
        ptr := REF(value_lint);
        ptr := REF(value_real);
        ptr := REF(value_wstr);
        ptr := REF(value_time);
        ptr := REF(value_tod);
        ptr := REF(value_date);
        ptr := REF(value_dt);
        ptr := REF(value_bool);
        ptr := REF(value_byte);
        ptr := REF(value_word);
        ptr := REF(value_dword);
        ptr := REF(value_lword);
        ptr := REF(value_pos1d);
        ptr := REF(value_pos2d);
        ptr := REF(value_pos3d);
    END_FUNCTION
    "#;

    let diagnostics = parse_and_validate(source);
    let filtered_diagnostics =
        diagnostics.into_iter().filter(|diagnostic| diagnostic.error_code != "E015").collect::<Vec<_>>();
    assert_eq!(filtered_diagnostics, Vec::new());
}
