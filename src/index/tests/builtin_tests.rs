use plc_ast::provider::IdProvider;

use crate::{builtins, test_utils::tests::index};

#[test]
fn builtin_functions_added_to_index() {
    let provider = IdProvider::default();
    let builtins = builtins::parse_built_ins(provider);
    let index = crate::index::visitor::visit(&builtins);
    assert!(index.find_member("ADR", "in").is_some());
    assert!(index.find_member("REF", "in").is_some());
    assert!(index.find_member("MUX", "K").is_some());
    assert!(index.find_member("SEL", "G").is_some());
    assert!(index.find_member("MOVE", "in").is_some());
    assert!(index.find_implementation_by_name("ADR").is_some());
    assert!(index.find_implementation_by_name("REF").is_some());
    assert!(index.find_implementation_by_name("MUX").is_some());
    assert!(index.find_implementation_by_name("SEL").is_some());
    assert!(index.find_implementation_by_name("MOVE").is_some());
}

#[test]
fn test_indexer_has_builtins() {
    let (_, index) = index("");
    assert!(index.find_member("ADR", "in").is_some());
    assert!(index.find_member("REF", "in").is_some());
    assert!(index.find_member("MUX", "K").is_some());
    assert!(index.find_member("SEL", "G").is_some());
    assert!(index.find_member("MOVE", "in").is_some());
    assert!(index.find_implementation_by_name("ADR").is_some());
    assert!(index.find_implementation_by_name("REF").is_some());
    assert!(index.find_implementation_by_name("MUX").is_some());
    assert!(index.find_implementation_by_name("SEL").is_some());
    assert!(index.find_implementation_by_name("MOVE").is_some());
}
