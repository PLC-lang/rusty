use crate::{builtins, lexer::IdProvider, test_utils::tests::index};

#[test]
fn builtin_functions_added_to_index() {
    let provider = IdProvider::default();
    let builtins = builtins::parse_built_ins(provider.clone());
    let index = crate::index::visitor::visit(&builtins, provider);

    assert!(index.find_member("ADR", "in").is_some());
    assert!(index.find_member("REF", "in").is_some());
    assert!(index.find_implementation_by_name("ADR").is_some());
    assert!(index.find_implementation_by_name("REF").is_some());
}

#[test]
fn test_indexer_has_builtins() {
    let (_, index) = index("");
    assert!(index.find_member("ADR", "in").is_some());
    assert!(index.find_member("REF", "in").is_some());
    assert!(index.find_implementation_by_name("ADR").is_some());
    assert!(index.find_implementation_by_name("REF").is_some());
}
