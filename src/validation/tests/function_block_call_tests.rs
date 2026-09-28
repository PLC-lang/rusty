use crate::test_utils::tests::{parse_and_validate, parse_and_validate_buffered};

#[test]
fn function_block_type_alias_calls_require_instances() {
    let diagnostics = parse_and_validate(
        "
        FUNCTION_BLOCK MyFb
        END_FUNCTION_BLOCK
        TYPE FbAlias : MyFb; END_TYPE
        TYPE SecondAlias : FbAlias; END_TYPE
        FUNCTION main : DINT
            FbAlias();
            SecondAlias();
        END_FUNCTION
        ",
    );

    assert_eq!(diagnostics.len(), 2, "{diagnostics:#?}");
    for (diagnostic, name) in diagnostics.iter().zip(["FbAlias", "SecondAlias"]) {
        assert_eq!(diagnostic.get_error_code(), "E156");
        assert_eq!(
            diagnostic.get_message(),
            format!("`{name}` is a FUNCTION_BLOCK. Declare an instance and call that instance")
        );
    }
}

#[test]
fn function_block_type_named_call_reports_only_missing_instance() {
    let diagnostics = parse_and_validate_buffered(
        "FUNCTION_BLOCK MyFb
    VAR_INPUT x : DINT; END_VAR
    VAR_OUTPUT y : DINT; END_VAR
    VAR_IN_OUT z : DINT; END_VAR
END_FUNCTION_BLOCK
FUNCTION main : DINT
    VAR value : DINT; END_VAR
    MyFb(x := 1, y => value, z := value);
END_FUNCTION",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E156]: `MyFb` is a FUNCTION_BLOCK. Declare an instance and call that instance
      ┌─ <internal>:8:5
      │
    8 │     MyFb(x := 1, y => value, z := value);
      │     ^^^^ `MyFb` is a FUNCTION_BLOCK. Declare an instance and call that instance
    ");
}

#[test]
fn function_block_type_call_preserves_argument_errors() {
    for argument in ["unknown", "x := unknown", "y => unknown"] {
        let source = format!(
            "FUNCTION_BLOCK MyFb
                VAR_INPUT x : DINT; END_VAR
                VAR_OUTPUT y : DINT; END_VAR
            END_FUNCTION_BLOCK
            FUNCTION main : DINT
                MyFb({argument});
            END_FUNCTION"
        );
        let diagnostics = parse_and_validate(&source);

        assert_eq!(diagnostics.len(), 2, "{argument}: {diagnostics:#?}");
        assert_eq!(diagnostics[0].get_error_code(), "E156");
        assert_eq!(diagnostics[1].get_error_code(), "E048");
        assert_eq!(diagnostics[1].get_message(), "Could not resolve reference to unknown");
    }
}

#[test]
fn nested_function_block_type_calls_are_each_reported_once() {
    let diagnostics = parse_and_validate(
        "FUNCTION_BLOCK MyFb
            VAR_INPUT x : DINT; END_VAR
        END_FUNCTION_BLOCK
        TYPE FbAlias : MyFb; END_TYPE
        FUNCTION main : DINT
            FbAlias(x := MyFb(x := 1));
        END_FUNCTION",
    );

    assert_eq!(diagnostics.len(), 2, "{diagnostics:#?}");
    assert!(diagnostics.iter().all(|diagnostic| diagnostic.get_error_code() == "E156"));
    assert_ne!(diagnostics[0].get_location(), diagnostics[1].get_location());
}

#[test]
fn aliased_and_qualified_function_block_instances_are_callable() {
    let diagnostics = parse_and_validate_buffered(
        "FUNCTION_BLOCK MyFb
            VAR_INPUT x : DINT; END_VAR
        END_FUNCTION_BLOCK
        TYPE FbAlias : MyFb; END_TYPE
        TYPE SecondAlias : FbAlias; END_TYPE
        PROGRAM holder
            VAR_INPUT inst : SecondAlias; END_VAR
        END_PROGRAM
        FUNCTION main : DINT
            VAR inst : FbAlias; END_VAR
            inst(x := 1);
            holder.inst(x := 2);
        END_FUNCTION",
    );

    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn global_variable_shadows_function_block_type() {
    for variable_type in ["DINT", "MyFb"] {
        let source = format!(
            "FUNCTION_BLOCK MyFb
            END_FUNCTION_BLOCK
            VAR_GLOBAL MyFb : {variable_type}; END_VAR
            FUNCTION main : DINT
                MyFb();
                .MyFb();
            END_FUNCTION"
        );
        let diagnostics = parse_and_validate(&source);

        assert!(diagnostics.is_empty(), "{variable_type}: {diagnostics:#?}");
    }
}

#[test]
fn global_operator_does_not_resolve_function_block_types() {
    let diagnostics = parse_and_validate(
        "FUNCTION_BLOCK MyFb
        END_FUNCTION_BLOCK
        FUNCTION main : DINT
            main := 0;
            .MyFb();
        END_FUNCTION",
    );

    assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
    assert_eq!(diagnostics[0].get_error_code(), "E048");
}
