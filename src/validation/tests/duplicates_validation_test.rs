use crate::{ast::SourceRange, diagnostics::Diagnostic, test_utils::tests::parse_and_validate};

#[test]
fn duplicate_pous_validation() {
    // GIVEN two POUs witht he same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION        foo : INT  END_FUNCTION

        PROGRAM         foo  END_PROGRAM

        FUNCTION_BLOCK  foo  END_FUNCTION_BLOCK
    "#,
    );
    // THEN there should be 3 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict(
                "foo",
                SourceRange::without_file(25..28),
                vec![
                    SourceRange::without_file(74..77),
                    SourceRange::without_file(116..119)
                ]
            ),
            Diagnostic::global_name_conflict(
                "foo",
                SourceRange::without_file(74..77),
                vec![
                    SourceRange::without_file(25..28),
                    SourceRange::without_file(116..119)
                ]
            ),
            Diagnostic::global_name_conflict(
                "foo",
                SourceRange::without_file(116..119),
                vec![
                    SourceRange::without_file(25..28),
                    SourceRange::without_file(74..77),
                ]
            ),
        ]
    );
}

#[test]
fn duplicate_pous_and_types_validation() {
    // GIVEN a POU and a Type with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION_BLOCK  foo  END_FUNCTION_BLOCK
        TYPE foo : INT END_TYPE
    "#,
    );
    // THEN there should be 3 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict(
                "foo",
                SourceRange::without_file(25..28),
                vec![SourceRange::without_file(62..65),]
            ),
            Diagnostic::global_name_conflict(
                "foo",
                SourceRange::without_file(62..65),
                vec![SourceRange::without_file(25..28),]
            ),
        ]
    );
}

#[ignore = "technically this works, practically this adds sooo much complexity :-("]
#[test]
fn duplicate_function_and_type_is_no_issue() {
    // GIVEN a Function and a Type with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION  foo: INT  END_FUNCTION
        TYPE foo : INT END_TYPE
    "#,
    );
    // THEN there should be 0 duplication diagnostics
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn duplicate_global_variables() {
    // GIVEN some duplicate global variables
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        VAR_GLOBAL
            a: INT;
            b: INT;
            c: INT;
        END_VAR

        VAR_GLOBAL
            a: BOOL;
        END_VAR
    
        "#,
    );
    // THEN there should be 0 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict(
                "a",
                SourceRange::without_file(32..33),
                vec![SourceRange::without_file(128..129),]
            ),
            Diagnostic::global_name_conflict(
                "a",
                SourceRange::without_file(128..129),
                vec![SourceRange::without_file(32..33),]
            ),
        ]
    );
}

#[test]
fn duplicate_variables_in_same_pou() {
    // GIVEN a POU with a duplicate variable
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        PROGRAM prg
        VAR
            a: INT;
            b: INT;
            c: INT;
        END_VAR
        VAR
            b: BOOL;
        END_VAR
        END_PROGRAM
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict(
                "prg.b",
                SourceRange::without_file(65..66),
                vec![SourceRange::without_file(133..134),]
            ),
            Diagnostic::global_name_conflict(
                "prg.b",
                SourceRange::without_file(133..134),
                vec![SourceRange::without_file(65..66),]
            ),
        ]
    );
}

#[test]
fn duplicate_enum_members_in_different_types_is_no_issue() {
    // GIVEN a two enums with the same elements
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            TYPE enum1 : (red, green, yellow); END_TYPE
            TYPE enum2 : (red, green, yellow); END_TYPE
        "#,
    );
    // THEN there should be no issues
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn duplicate_enum_variables() {
    // GIVEN an enum with two identical elements
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            TYPE enum1 : (red, green, yellow, red); END_TYPE
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict(
                "enum1.red",
                SourceRange::without_file(27..30),
                vec![SourceRange::without_file(47..50),]
            ),
            Diagnostic::global_name_conflict(
                "enum1.red",
                SourceRange::without_file(47..50),
                vec![SourceRange::without_file(27..30),]
            ),
        ]
    );
}
