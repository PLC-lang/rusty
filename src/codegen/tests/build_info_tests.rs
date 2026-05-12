use crate::test_utils::tests::codegen_debug_without_unwrap_oc_with_build_info;
use crate::DebugLevel;
use crate::OnlineChange;

const TRIVIAL_SOURCE: &str = r"
FUNCTION main : DINT
    main := 1;
END_FUNCTION
";

#[test]
fn codegen_emits_module_asm_ident_directive_when_build_info_is_set() {
    let ir = codegen_debug_without_unwrap_oc_with_build_info(
        TRIVIAL_SOURCE,
        DebugLevel::None,
        OnlineChange::Disabled,
        Some("plc version test-placeholder"),
    )
    .expect("codegen");

    assert!(
        ir.contains(r#"module asm ".ident"#),
        "expected `module asm \".ident...\"` directive in IR; got:\n{ir}",
    );
    assert!(
        ir.contains("plc version test-placeholder"),
        "expected the build_info text to appear in the IR; got:\n{ir}",
    );
}

#[test]
fn codegen_omits_ident_directive_when_build_info_is_none() {
    let ir = codegen_debug_without_unwrap_oc_with_build_info(
        TRIVIAL_SOURCE,
        DebugLevel::None,
        OnlineChange::Disabled,
        None,
    )
    .expect("codegen");

    assert!(!ir.contains("module asm"), "expected NO `module asm` line when build_info=None; got:\n{ir}");
    assert!(!ir.contains(".ident"), "expected NO `.ident` directive when build_info=None; got:\n{ir}");
}

#[test]
fn codegen_emits_exactly_one_ident_directive_per_module() {
    let ir = codegen_debug_without_unwrap_oc_with_build_info(
        TRIVIAL_SOURCE,
        DebugLevel::None,
        OnlineChange::Disabled,
        Some("plc version single-entry-check"),
    )
    .expect("codegen");

    let occurrences = ir.matches(r#"module asm ".ident"#).count();
    assert_eq!(occurrences, 1, "expected exactly one `.ident` directive; got {occurrences} in:\n{ir}");
}
