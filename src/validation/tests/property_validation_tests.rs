use plc_index::GlobalContext;
use plc_source::SourceCode;

use crate::{lowering::validator::ParticipantValidator, test_utils::tests::parse, ErrorFormat};

fn lower_and_validate(src: &str) -> String {
    let mut context = GlobalContext::new();
    context.with_error_fmt(plc_index::ErrorFormat::Null);
    context.insert(&SourceCode::from(src), None).unwrap();

    let (unit, mut diagnostics) = parse(src);

    let mut validator = ParticipantValidator::new(&context, ErrorFormat::None);
    for pou in &unit.units {
        validator.validate_properties(pou);
    }

    diagnostics.extend(validator.diagnostics);

    let mut results = Vec::new();
    for diagnostic in diagnostics {
        results.push(context.handle_as_str(&diagnostic));
    }

    results.join("\n").to_string()
}

#[test]
fn property_within_function_pou() {
    let diagnostics = lower_and_validate(
        r"
        FUNCTION foo : DINT
            PROPERTY prop : DINT
                GET
                    prop := 5;
                END_GET
            END_PROPERTY
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error: Methods cannot be declared in a POU of type 'Function'.
     --> <internal>:2:24
      |
    2 |         FUNCTION foo : DINT
      |                        ^^^^ Methods cannot be declared in a POU of type 'Function'.
      |
    error: Property `prop` must be defined in a stateful POU type (PROGRAM, CLASS or FUNCTION_BLOCK)
     --> <internal>:2:18
      |
    2 |         FUNCTION foo : DINT
      |                  ^^^ Property `prop` must be defined in a stateful POU type (PROGRAM, CLASS or FUNCTION_BLOCK)
      |
    ");
}

#[test]
fn property_with_more_than_one_get_block() {
    let diagnostics = lower_and_validate(
        r"
        FUNCTION_BLOCK foo
            PROPERTY prop : DINT
                GET END_GET
                GET END_GET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );
    insta::assert_snapshot!(diagnostics, @r"
    error: Property has more than one GET block
     --> <internal>:4:22
      |
    4 |             PROPERTY prop : DINT
      |                      ^^^^ Property has more than one GET block
    5 |                 GET END_GET
      |                 ^^^ see also
    6 |                 GET END_GET
      |                 ^^^ see also
      |
    ");
}

#[test]
fn property_with_var_output_in_get_block() {
    let diagnostics = lower_and_validate(
        r"
        FUNCTION_BLOCK foo
            PROPERTY prop : DINT
              GET
                  VAR_OUTPUT
                    out : DINT;
                  END_VAR
              END_Get
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error: Properties only allow variable blocks of type VAR
     --> <internal>:4:22
      |
    4 |             PROPERTY prop : DINT
      |                      ^^^^ Properties only allow variable blocks of type VAR
    5 |               GET
    6 |                   VAR_OUTPUT
      |                   ^^^^^^^^^^ see also
      |
    ");
}

#[test]
fn property_with_same_name_as_member_variable() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb
        VAR
            foo: DINT;
        END_VAR
        PROPERTY foo : DINT
            GET
                foo := 42;
            END_GET
            SET
                foo := 3;
            END_SET
        END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E004]: foo: Duplicate symbol.
      ┌─ <internal>:4:13
      │
    4 │             foo: DINT;
      │             ^^^ foo: Duplicate symbol.
    5 │         END_VAR
    6 │         PROPERTY foo : DINT
      │                  --- see also

    error[E004]: foo: Duplicate symbol.
      ┌─ <internal>:6:18
      │
    4 │             foo: DINT;
      │             --- see also
    5 │         END_VAR
    6 │         PROPERTY foo : DINT
      │                  ^^^ foo: Duplicate symbol.
    ");
}

#[test]
fn property_name_conflict_with_variable_in_parent() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            VAR
                foo: DINT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb1
            PROPERTY foo : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E021]: Variable `foo` is already declared in parent POU `fb1`
      ┌─ <internal>:9:22
      │
    4 │                 foo: DINT;
      │                 --- see also
      ·
    9 │             PROPERTY foo : DINT
      │                      ^^^ Variable `foo` is already declared in parent POU `fb1`
    ");
}

#[test]
fn property_name_conflict_with_variable_in_child() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            PROPERTY foo : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb1
            VAR
                foo: DINT;
            END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E021]: Variable `foo` is already declared in parent POU `foo`
       ┌─ <internal>:11:17
       │
     3 │             PROPERTY foo : DINT
       │                      --- see also
       ·
    11 │                 foo: DINT;
       │                 ^^^ Variable `foo` is already declared in parent POU `foo`
    ");
}

#[test]
fn property_name_conflict_with_variable_in_parent_chained() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            PROPERTY foo : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb1
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb3 EXTENDS fb2
            VAR
                foo: DINT;
            END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E021]: Variable `foo` is already declared in parent POU `foo`
       ┌─ <internal>:14:17
       │
     3 │             PROPERTY foo : DINT
       │                      --- see also
       ·
    14 │                 foo: DINT;
       │                 ^^^ Variable `foo` is already declared in parent POU `foo`
    ");
}

#[test]
fn property_clashing_with_parent_property_name_is_ok() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            PROPERTY foo : DINT
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb1
            PROPERTY foo : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    // Essentially we're overriding the property in the child, which is OK
    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn undefined_property_accessor_in_parent_yields_error() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            PROPERTY myProp : DINT
                GET END_GET
            END_PROPERTY

            myProp;         // Ok, this represents GET
            myProp := 5;    // Error, this represents a SET which is not defined in here
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            PROPERTY myProp : DINT
                SET END_SET
            END_PROPERTY

            myProp := 5;            // Ok, this represents a GET which is inherited from the parent
            myProp := myProp  + 1;  // Ok, this represents a SET that is overriden here
        END_FUNCTION_BLOCK

        FUNCTION main : DINT
            VAR
                parent_fb: parent;
                child_fb: child;
            END_VAR

            parent_fb.myProp := 5;                  // Error, the `parent` FB does not define a SET
            child_fb.myProp := 5;                   // Ok, the `child` FB does define a SET
            child_fb.myProp := parent_fb.myProp;    // Ok, the `child` FB does define a SET, the `parent` a GET
            child_fb.myProp := child_fb.myProp + 1; // Ok, the `child` FB does define a SET, inherits the GET from the parent
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r###"
    error[E048]: Could not resolve reference to myProp
      ┌─ <internal>:8:13
      │
    8 │             myProp := 5;    // Error, this represents a SET which is not defined in here
      │             ^^^^^^ Could not resolve reference to myProp

    error[E048]: Could not resolve reference to myProp
       ┌─ <internal>:26:23
       │
    26 │             parent_fb.myProp := 5;                  // Error, the `parent` FB does not define a SET
       │                       ^^^^^^ Could not resolve reference to myProp
    "###);
}
