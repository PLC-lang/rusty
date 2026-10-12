use std::path::Path;

use plc_ast::provider::IdProvider;

use crate::{builtins, test_utils::tests::index};

#[test]
fn builtin_functions_added_to_index() {
    let provider = IdProvider::default();
    let builtins = builtins::parse_built_ins(provider);
    let index = crate::index::indexer::index(&builtins);

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

#[test]
fn every_declaration_file_declares_the_builtin_it_is_named_after() {
    fn declaration_file_names(dir: &Path) -> Vec<String> {
        let mut names = vec![];
        for entry in std::fs::read_dir(dir).unwrap().map(Result::unwrap) {
            let path = entry.path();
            if path.is_dir() {
                names.extend(declaration_file_names(&path));
            } else if path.extension().is_some_and(|it| it == "pli") {
                names.push(path.file_stem().unwrap().to_string_lossy().into_owned());
            }
        }
        names
    }

    let mut files = declaration_file_names(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src/builtins"));
    files.sort();

    let unit = builtins::parse_built_ins(IdProvider::default());
    let mut declared = unit.pous.iter().map(|it| it.name.clone()).collect::<Vec<_>>();
    declared.sort();

    assert_eq!(files, declared);
    assert!(files.iter().all(|it| builtins::get_builtin(it).is_some()));
}
