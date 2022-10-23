use crate::{diagnostics::Diagnostic, test_utils::tests::parse_and_validate};

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
            Diagnostic::global_name_conflict("foo", vec!["foo".into()]),
        ]
    );
}
