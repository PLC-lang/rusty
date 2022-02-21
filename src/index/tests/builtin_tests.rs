use crate::{index::Index, lexer::IdProvider, test_utils::tests::index};

#[test]
fn builtin_functions_added_to_index() {
    let index = Index::create_with_builtins(IdProvider::default());
    assert!(index.find_member("ADR", "in").is_some());
    assert!(index.find_member("REF", "in").is_some());
    assert!(index.find_implementation("ADR").is_some());
    assert!(index.find_implementation("REF").is_some());
}

#[test]
fn default_visitor_creates_builtins() {
    let (_, index) = index("");
    assert!(index.find_member("ADR", "in").is_some());
    assert!(index.find_member("REF", "in").is_some());
    assert!(index.find_implementation("ADR").is_some());
    assert!(index.find_implementation("REF").is_some());
}
