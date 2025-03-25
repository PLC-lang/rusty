use insta::assert_snapshot;
use test_utils::parse_and_validate_buffered;

#[test]
fn accessing_derived_member_through_dereferenced_super_is_no_illegal_access() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK greatgrandparent
        VAR
            x : INT;
            y : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK grandparent EXTENDS greatgrandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            SUPER^.x := 2;
            SUPER^.y := SUPER^.x + 1;
        END_FUNCTION_BLOCK
    ",
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn chaining_super_is_invalid() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK greatgrandparent
        VAR
            x : INT;
            y : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK grandparent EXTENDS greatgrandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: Chaining multiple `SUPER` accessors is not allowed
       ┌─ <internal>:16:13
       │
    16 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │             ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed

    error[E119]: Chaining multiple `SUPER` accessors is not allowed
       ┌─ <internal>:16:32
       │
    16 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                                ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed

    error[E119]: Chaining multiple `SUPER` accessors is not allowed
       ┌─ <internal>:16:39
       │
    16 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                                       ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed
    ");
}

#[test]
fn chained_super_references_still_report_unresolved_references() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK greatgrandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK grandparent EXTENDS greatgrandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: Chaining multiple `SUPER` accessors is not allowed
       ┌─ <internal>:12:13
       │
    12 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │             ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed

    error[E048]: Could not resolve reference to x
       ┌─ <internal>:12:27
       │
    12 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                           ^ Could not resolve reference to x

    error[E119]: Chaining multiple `SUPER` accessors is not allowed
       ┌─ <internal>:12:32
       │
    12 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                                ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed

    error[E119]: Chaining multiple `SUPER` accessors is not allowed
       ┌─ <internal>:12:39
       │
    12 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                                       ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed

    error[E048]: Could not resolve reference to y
       ┌─ <internal>:12:53
       │
    12 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                                                     ^ Could not resolve reference to y
    ");
}

#[test]
fn super_accessor_used_in_non_extended_function_block() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb
            SUPER^.x := 2;
            SUPER;
            SUPER^;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: `SUPER` can only be used in function blocks that extend another function block
      ┌─ <internal>:3:13
      │
    3 │             SUPER^.x := 2;
      │             ^^^^^ `SUPER` can only be used in function blocks that extend another function block

    error[E048]: Could not resolve reference to x
      ┌─ <internal>:3:20
      │
    3 │             SUPER^.x := 2;
      │                    ^ Could not resolve reference to x

    error[E119]: `SUPER` can only be used in function blocks that extend another function block
      ┌─ <internal>:4:13
      │
    4 │             SUPER;
      │             ^^^^^ `SUPER` can only be used in function blocks that extend another function block

    error[E119]: `SUPER` can only be used in function blocks that extend another function block
      ┌─ <internal>:5:13
      │
    5 │             SUPER^;
      │             ^^^^^ `SUPER` can only be used in function blocks that extend another function block
    ");
}
