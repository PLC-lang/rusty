use insta::assert_snapshot;

use crate::test_utils::tests::parse_and_validate_buffered;

#[test]
fn typedef_of_unknown_type_is_reported_at_the_typedef_site() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE myType : undeclaredType; END_TYPE
        VAR_GLOBAL
            a : myType;
        END_VAR
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E052]: Unknown type: undeclaredType
      ┌─ <internal>:2:23
      │
    2 │         TYPE myType : undeclaredType; END_TYPE
      │                       ^^^^^^^^^^^^^^ Unknown type: undeclaredType

    error[E052]: Type 'myType' references an unknown type
      ┌─ <internal>:4:17
      │
    4 │             a : myType;
      │                 ^^^^^^ Type 'myType' references an unknown type
    ");
}

#[test]
fn typedef_chain_flags_only_the_failing_typedef_then_each_use_site() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE alias_a : alias_b; END_TYPE
        TYPE alias_b : undeclaredType; END_TYPE
        VAR_GLOBAL
            a : alias_a;
        END_VAR
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E052]: Unknown type: undeclaredType
      ┌─ <internal>:3:24
      │
    3 │         TYPE alias_b : undeclaredType; END_TYPE
      │                        ^^^^^^^^^^^^^^ Unknown type: undeclaredType

    error[E052]: Type 'alias_a' references an unknown type
      ┌─ <internal>:5:17
      │
    5 │             a : alias_a;
      │                 ^^^^^^^ Type 'alias_a' references an unknown type
    ");
}

#[test]
fn typedef_used_in_struct_member_still_reports_typedef_site() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE myType : undeclaredType; END_TYPE
        TYPE myStruct : STRUCT
            m : myType;
        END_STRUCT END_TYPE
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E052]: Unknown type: undeclaredType
      ┌─ <internal>:2:23
      │
    2 │         TYPE myType : undeclaredType; END_TYPE
      │                       ^^^^^^^^^^^^^^ Unknown type: undeclaredType

    error[E052]: Type 'myType' references an unknown type
      ┌─ <internal>:4:17
      │
    4 │             m : myType;
      │                 ^^^^^^ Type 'myType' references an unknown type
    ");
}

#[test]
fn self_referential_typedef_is_caught() {
    // `TYPE a : a;` — direct cycle. Pin the diagnostic shape so cycle handling
    // doesn't silently regress as the typedef-site check evolves.
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE my_alias : my_alias; END_TYPE
        VAR_GLOBAL
            a : my_alias;
        END_VAR
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E052]: Type 'my_alias' references an unknown type
      ┌─ <internal>:4:17
      │
    4 │             a : my_alias;
      │                 ^^^^^^^^ Type 'my_alias' references an unknown type

    error[E121]: Recursive type alias `my_alias -> my_alias`
      ┌─ <internal>:2:14
      │
    2 │         TYPE my_alias : my_alias; END_TYPE
      │              ^^^^^^^^ Recursive type alias `my_alias -> my_alias`
    ");
}

#[test]
fn mutually_referential_typedef_chain_is_caught() {
    // Indirect cycle: `TYPE a : b; TYPE b : a;`. Same anchor purpose as the
    // self-referential case — confirm cycle resolution doesn't loop or hide
    // the diagnostic when the chain re-enters itself.
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE alias_a : alias_b; END_TYPE
        TYPE alias_b : alias_a; END_TYPE
        VAR_GLOBAL
            a : alias_a;
        END_VAR
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E052]: Type 'alias_a' references an unknown type
      ┌─ <internal>:5:17
      │
    5 │             a : alias_a;
      │                 ^^^^^^^ Type 'alias_a' references an unknown type

    error[E121]: Recursive type alias `alias_a -> alias_b -> alias_a`
      ┌─ <internal>:2:14
      │
    2 │         TYPE alias_a : alias_b; END_TYPE
      │              ^^^^^^^ Recursive type alias `alias_a -> alias_b -> alias_a`
    3 │         TYPE alias_b : alias_a; END_TYPE
      │              ------- see also

    error[E121]: Recursive type alias `alias_b -> alias_a -> alias_b`
      ┌─ <internal>:3:14
      │
    2 │         TYPE alias_a : alias_b; END_TYPE
      │              ------- see also
    3 │         TYPE alias_b : alias_a; END_TYPE
      │              ^^^^^^^ Recursive type alias `alias_b -> alias_a -> alias_b`
    ");
}

#[test]
fn typedef_of_known_type_is_clean() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE myAlias : DINT; END_TYPE
        VAR_GLOBAL
            a : myAlias;
        END_VAR
        ",
    );

    assert!(diagnostics.is_empty(), "expected clean diagnostics, got:\n{diagnostics}");
}
