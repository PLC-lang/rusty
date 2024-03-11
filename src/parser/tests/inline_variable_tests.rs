use crate::test_utils::tests::parse;

#[test]
fn inline_variable_declaration_on_new_line_with_def_pragma() {
    let (result, ..) = parse(
        r#"
            PROGRAM main 
            {def} VAR x : DINT;
            VAR y := 1;
            END_PROGRAM
        "#,
    );
    insta::assert_debug_snapshot!(result);
}

#[test]
fn variable_block_still_parsed_without_pragma() {
    let (result, ..) = parse(
        r#"
            PROGRAM main 
            VAR x : DINT; END_VAR;
            {def} VAR y := 1;
            END_PROGRAM
        "#,
    );
    insta::assert_debug_snapshot!(result);
}

#[test]
fn when_already_in_body_pragma_optional() {
    let (result, ..) = parse(
        r#"
            PROGRAM main 
            VAR x : DINT; END_VAR
            x := 10;
            VAR y := 1;
            y := 10;
            END_PROGRAM
        "#,
    );
    insta::assert_debug_snapshot!(result);
}

#[test]
fn variable_declared_in_for_loop() {
    let (result, ..) = parse(
        r#"
            PROGRAM main 
            FOR VAR x := 0 TO 10 DO
            END_FOR
            END_PROGRAM
        "#,
    );
    insta::assert_debug_snapshot!(result);
}

#[test]
fn variable_declared_in_inner_scope() {
    let (result, ..) = parse(
        r#"
            PROGRAM main 
            FOR VAR x := 0 TO 10 DO
                VAR y := 5;
            END_FOR

            IF true THEN
                VAR y := 10;
            END_IF
            END_PROGRAM
        "#,
    );
    insta::assert_debug_snapshot!(result);
}
