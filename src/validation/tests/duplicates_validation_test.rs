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
