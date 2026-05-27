use crate::test_utils::tests::codegen_debug_without_unwrap_oc_with_build_info;
use crate::DebugLevel;
use crate::OnlineChange;

const TRIVIAL_SOURCE: &str = r"
FUNCTION main : DINT
    main := 1;
END_FUNCTION
";

#[test]
fn codegen_emits_llvm_ident_when_build_info_is_set() {
    let ir = codegen_debug_without_unwrap_oc_with_build_info(
        TRIVIAL_SOURCE,
        DebugLevel::None,
        OnlineChange::Disabled,
        Some("plc version test-placeholder"),
    )
    .expect("codegen");

    assert!(ir.contains("!llvm.ident"), "expected !llvm.ident in IR; got:\n{ir}");
    assert!(
        ir.contains(r#"!"plc version test-placeholder""#),
        "expected build_info string operand in IR; got:\n{ir}",
    );
}

#[test]
fn codegen_omits_llvm_ident_when_build_info_is_none() {
    let ir = codegen_debug_without_unwrap_oc_with_build_info(
        TRIVIAL_SOURCE,
        DebugLevel::None,
        OnlineChange::Disabled,
        None,
    )
    .expect("codegen");

    assert!(!ir.contains("!llvm.ident"), "expected NO !llvm.ident when build_info=None; got:\n{ir}");
}

#[test]
fn codegen_emits_exactly_one_llvm_ident_entry_per_module() {
    let ir = codegen_debug_without_unwrap_oc_with_build_info(
        TRIVIAL_SOURCE,
        DebugLevel::None,
        OnlineChange::Disabled,
        Some("plc version single-entry-check"),
    )
    .expect("codegen");

    let occurrences = ir.matches("!llvm.ident = ").count();
    assert_eq!(occurrences, 1, "expected exactly one !llvm.ident declaration; got {occurrences} in:\n{ir}");
}
