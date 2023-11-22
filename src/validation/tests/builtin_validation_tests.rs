use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

#[test]
fn non_variadic_overload_of_variadic_add_function_called_with_too_many_params() {
    let diagnostics = parse_and_validate(
        "
        FUNCTION main : LINT
        VAR
            x1 : ARRAY[0..3] OF DATE := [DATE#1970-01-01, DATE#2000-01-02, DATE#2023-05-30];
            x2 : DATE := DATE#1999-12-31;
        END_VAR
            main := ADD(x1[0], x1[1], x1[2], x1[3], x2);
        END_FUNCTION
       ",
    );

    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn overloaded_generic_function_with_different_type_nature_does_not_err() {
    let diagnostics = parse_and_validate(
        "
        FUNCTION main : LINT
        VAR
            x1 : DATE := DATE#1970-01-01;
            x2 : DATE := DATE#1999-12-31;
        END_VAR
            main := ADD(x1, x2);
        END_FUNCTION
       ",
    );

    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn arithmetic_builtins_called_with_incompatible_types() {
    let diagnostics = parse_and_validate(
        "
        FUNCTION main : DINT
        VAR
            x1 : ARRAY[0..2] OF TOD;
            x2 : STRING;
        END_VAR
            ADD(x1, x1);
            ADD(x1, x2);
            ADD(x2, x2);
        END_FUNCTION
       ",
    );

    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn comparison_builtins_called_with_incompatible_types() {
    let diagnostics = parse_and_validate(
        "
        FUNCTION main : DINT
        VAR
            x1 : ARRAY[0..2] OF TOD;
            x2 : STRING;
        END_VAR
            EQ(x1, x1);
            GT(x1, x2);
            NE(x2, x2);
        END_FUNCTION
       ",
    );

    assert_validation_snapshot!(&diagnostics);
}
