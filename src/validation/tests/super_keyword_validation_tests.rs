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
fn super_accessor_used_in_non_extended_function_block_is_an_error() {
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
    error[E119]: `SUPER` can only be used in POUs that extend another POU
      ┌─ <internal>:3:13
      │
    3 │             SUPER^.x := 2;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU

    error[E048]: Could not resolve reference to x
      ┌─ <internal>:3:20
      │
    3 │             SUPER^.x := 2;
      │                    ^ Could not resolve reference to x

    error[E119]: `SUPER` can only be used in POUs that extend another POU
      ┌─ <internal>:4:13
      │
    4 │             SUPER;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU

    error[E119]: `SUPER` can only be used in POUs that extend another POU
      ┌─ <internal>:5:13
      │
    5 │             SUPER^;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU
    ");
}

#[test]
fn super_keyword_used_in_non_extendable_pous_is_an_error() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION foo
            SUPER^.x := 2;
            SUPER;
            SUPER^;
        END_FUNCTION

        PROGRAM prog
            SUPER^.x := 2;
            SUPER;
            SUPER^;
        END_PROGRAM

        ACTION prog.act
            SUPER^.x := 2;
            SUPER;
            SUPER^;
        END_ACTION
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: `SUPER` can only be used in POUs that extend another POU
      ┌─ <internal>:3:13
      │
    3 │             SUPER^.x := 2;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU

    error[E048]: Could not resolve reference to x
      ┌─ <internal>:3:20
      │
    3 │             SUPER^.x := 2;
      │                    ^ Could not resolve reference to x

    error[E119]: `SUPER` can only be used in POUs that extend another POU
      ┌─ <internal>:4:13
      │
    4 │             SUPER;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU

    error[E119]: `SUPER` can only be used in POUs that extend another POU
      ┌─ <internal>:5:13
      │
    5 │             SUPER^;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU

    error[E119]: `SUPER` can only be used in POUs that extend another POU
      ┌─ <internal>:9:13
      │
    9 │             SUPER^.x := 2;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU

    error[E048]: Could not resolve reference to x
      ┌─ <internal>:9:20
      │
    9 │             SUPER^.x := 2;
      │                    ^ Could not resolve reference to x

    error[E119]: `SUPER` can only be used in POUs that extend another POU
       ┌─ <internal>:10:13
       │
    10 │             SUPER;
       │             ^^^^^ `SUPER` can only be used in POUs that extend another POU

    error[E119]: `SUPER` can only be used in POUs that extend another POU
       ┌─ <internal>:11:13
       │
    11 │             SUPER^;
       │             ^^^^^ `SUPER` can only be used in POUs that extend another POU

    error[E119]: `SUPER` can only be used in POUs that extend another POU
       ┌─ <internal>:15:13
       │
    15 │             SUPER^.x := 2;
       │             ^^^^^ `SUPER` can only be used in POUs that extend another POU

    error[E048]: Could not resolve reference to x
       ┌─ <internal>:15:20
       │
    15 │             SUPER^.x := 2;
       │                    ^ Could not resolve reference to x

    error[E119]: `SUPER` can only be used in POUs that extend another POU
       ┌─ <internal>:16:13
       │
    16 │             SUPER;
       │             ^^^^^ `SUPER` can only be used in POUs that extend another POU

    error[E119]: `SUPER` can only be used in POUs that extend another POU
       ┌─ <internal>:17:13
       │
    17 │             SUPER^;
       │             ^^^^^ `SUPER` can only be used in POUs that extend another POU
    ");
}

#[test]
fn super_keyword_is_not_assignable() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK child EXTENDS parent
        VAR
            super_inst: parent;
            super_ptr: REF_TO parent;
        END_VAR
            SUPER^ := super_inst;
            SUPER := super_ptr;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E050]: Expression SUPER is not assignable.
      ┌─ <internal>:9:13
      │
    9 │             SUPER^ := super_inst;
      │             ^^^^^ Expression SUPER is not assignable.

    error[E050]: Expression SUPER is not assignable.
       ┌─ <internal>:10:13
       │
    10 │             SUPER := super_ptr;
       │             ^^^^^ Expression SUPER is not assignable.
    ");
}
