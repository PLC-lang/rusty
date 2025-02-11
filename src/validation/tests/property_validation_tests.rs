use plc_index::GlobalContext;
use plc_source::SourceCode;

use crate::{lowering::validator::ParticipantValidator, test_utils::tests::parse, ErrorFormat};

pub fn lower_and_validate(src: &str) -> String {
    let mut context = GlobalContext::new();
    context.with_error_fmt(plc_index::ErrorFormat::Null);
    context.insert(&SourceCode::from(src), None).unwrap();

    let (unit, mut diagnostics) = parse(src);

    let mut validator = ParticipantValidator::new(&context, ErrorFormat::None);
    validator.validate_properties(&unit.properties);

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
