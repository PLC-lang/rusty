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

#[test]
fn pointer_to_validates_assignment_when_not_dealing_with_memory_address_in_initializer() {
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
            ptr_str     : POINTER TO STRING         := 'foo';
            ptr_int     : POINTER TO INT            := 'foo';
            ptr_real    : POINTER TO REAL           := 'foo';
            ptr_time    : POINTER TO TIME           := 'foo';
            ptr_tod     : POINTER TO TIME_OF_DAY    := 'foo';
            ptr_date    : POINTER TO DATE           := 'foo';
            ptr_pos1d   : POINTER TO Position1D     := 'foo';
            ptr_pos2d   : POINTER TO Position2D     := 'foo';
            ptr_pos3d   : POINTER TO Position3D     := 'foo';

            ref_str     : REF_TO STRING             := 'foo';
            ref_int     : REF_TO INT                := 'foo';
            ref_real    : REF_TO REAL               := 'foo';
            ref_time    : REF_TO TIME               := 'foo';
            ref_tod     : REF_TO TIME_OF_DAY        := 'foo';
            ref_date    : REF_TO DATE               := 'foo';
            ref_pos1d   : REF_TO Position1D         := 'foo';
            ref_pos2d   : REF_TO Position2D         := 'foo';
            ref_pos3d   : REF_TO Position3D         := 'foo';
        END_VAR
    END_FUNCTION
    "#;

    let diagnostics = parse_and_validate(source);
    let filtered_diagnostics =
        diagnostics.into_iter().filter(|diagnostic| !matches!(diagnostic.error_code, "E015" | "E065")).collect::<Vec<_>>();

    // TODO: Validation between variable initialization / assignment in the variable block versus body are handled differently, these
    //       must be unified at some point. Once done, this assertion MUST fail and be identical to the test 
    //       `pointer_to_validates_assignment_when_not_dealing_with_memory_address_in_body`. Furthermore, we should unify these tests by
    //       having two source code strings, one for the variable block and one for the implementation, run validation on both of them and
    //       assert that the diagnostics (with exception of the location) are identical with regards to their assignment validation.
    assert_eq!(filtered_diagnostics, Vec::new(), "these are empty for now, but eventually should be the same as the test below");
}

#[test]
fn pointer_to_validates_assignment_when_not_dealing_with_memory_address_in_body() {
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
            ptr_str     : POINTER TO STRING;
            ptr_int     : POINTER TO INT;
            ptr_real    : POINTER TO REAL;
            ptr_time    : POINTER TO TIME;
            ptr_tod     : POINTER TO TIME_OF_DAY;
            ptr_date    : POINTER TO DATE;
            ptr_pos1d   : POINTER TO Position1D;
            ptr_pos2d   : POINTER TO Position2D;
            ptr_pos3d   : POINTER TO Position3D;

            ref_str     : REF_TO STRING;
            ref_int     : REF_TO INT;
            ref_real    : REF_TO REAL;
            ref_time    : REF_TO TIME;
            ref_tod     : REF_TO TIME_OF_DAY;
            ref_date    : REF_TO DATE;
            ref_pos1d   : REF_TO Position1D;
            ref_pos2d   : REF_TO Position2D;
            ref_pos3d   : REF_TO Position3D;
        END_VAR

        // None of these should be valid
        ptr_str     := 'foo';
        ptr_int     := 'foo';
        ptr_real    := 'foo';
        ptr_time    := 'foo';
        ptr_tod     := 'foo';
        ptr_date    := 'foo';
        ptr_pos1d   := 'foo';
        ptr_pos2d   := 'foo';
        ptr_pos3d   := 'foo';

        ref_str     := 'foo';
        ref_int     := 'foo';
        ref_real    := 'foo';
        ref_time    := 'foo';
        ref_tod     := 'foo';
        ref_date    := 'foo';
        ref_pos1d   := 'foo';
        ref_pos2d   := 'foo';
        ref_pos3d   := 'foo';
    END_FUNCTION
    "#;

    let diagnostics = parse_and_validate(source);
    let filtered_diagnostics =
        diagnostics.into_iter().filter(|diagnostic| !matches!(diagnostic.error_code, "E015" | "E065")).collect::<Vec<_>>();

    insta::assert_debug_snapshot!(filtered_diagnostics, @r###"
    [
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'POINTER TO STRING'",
            primary_location: SourceLocation {
                span: Range(46:8 - 46:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'POINTER TO INT'",
            primary_location: SourceLocation {
                span: Range(47:8 - 47:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'POINTER TO REAL'",
            primary_location: SourceLocation {
                span: Range(48:8 - 48:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'POINTER TO TIME'",
            primary_location: SourceLocation {
                span: Range(49:8 - 49:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'POINTER TO TIME_OF_DAY'",
            primary_location: SourceLocation {
                span: Range(50:8 - 50:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'POINTER TO DATE'",
            primary_location: SourceLocation {
                span: Range(51:8 - 51:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'POINTER TO Position1D'",
            primary_location: SourceLocation {
                span: Range(52:8 - 52:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'POINTER TO Position2D'",
            primary_location: SourceLocation {
                span: Range(53:8 - 53:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'POINTER TO Position3D'",
            primary_location: SourceLocation {
                span: Range(54:8 - 54:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'REF_TO STRING'",
            primary_location: SourceLocation {
                span: Range(56:8 - 56:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'REF_TO INT'",
            primary_location: SourceLocation {
                span: Range(57:8 - 57:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'REF_TO REAL'",
            primary_location: SourceLocation {
                span: Range(58:8 - 58:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'REF_TO TIME'",
            primary_location: SourceLocation {
                span: Range(59:8 - 59:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'REF_TO TIME_OF_DAY'",
            primary_location: SourceLocation {
                span: Range(60:8 - 60:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'REF_TO DATE'",
            primary_location: SourceLocation {
                span: Range(61:8 - 61:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'REF_TO Position1D'",
            primary_location: SourceLocation {
                span: Range(62:8 - 62:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'REF_TO Position2D'",
            primary_location: SourceLocation {
                span: Range(63:8 - 63:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Invalid assignment: cannot assign 'STRING' to 'REF_TO Position3D'",
            primary_location: SourceLocation {
                span: Range(64:8 - 64:28),
                file: Some(
                    "<internal>",
                ),
            },
            secondary_locations: None,
            error_code: "E037",
            sub_diagnostics: [],
            internal_error: None,
        },
    ]
    "###);
}
