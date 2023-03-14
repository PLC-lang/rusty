use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

#[test]
fn function_no_return_unsupported() {
    // GIVEN FUNCTION with no return type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate("FUNCTION foo VAR_INPUT END_VAR END_FUNCTION");
    // THEN there should be one diagnostic -> missing return type
    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn actions_container_no_name() {
    // GIVEN ACTIONS without a name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate("ACTIONS ACTION myAction END_ACTION END_ACTIONS");
    // THEN there should be one diagnostic -> missing action container name
    assert_validation_snapshot!(&diagnostics);
}
