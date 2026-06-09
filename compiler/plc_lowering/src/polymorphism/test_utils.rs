//! Test helpers mirroring `test_utils::tests` in the `plc` crate, which is private and only
//! compiled for the `plc` crate's own test builds.

use plc::{
    builtins,
    index::{self, Index},
    lexer, parser,
    typesystem::get_builtin_types,
};
use plc_ast::{
    ast::{pre_process, CompilationUnit, LinkageType},
    provider::IdProvider,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::{source_location::SourceLocationFactory, SourceCode, SourceContainer};

fn do_index<T: Into<SourceCode>>(
    src: T,
    id_provider: IdProvider,
) -> (CompilationUnit, Index, Vec<Diagnostic>) {
    let source = src.into();
    let source_str = &source.source;
    let source_path = source.get_location_str();
    let mut index = Index::default();
    //Import builtins
    let builtins = builtins::parse_built_ins(id_provider.clone());

    index.import(index::indexer::index(&builtins));
    // import built-in types like INT, BOOL, etc.
    for data_type in get_builtin_types() {
        index.register_type(data_type);
    }

    let range_factory = SourceLocationFactory::for_source(&source);
    let (mut unit, diagnostics) = parser::parse(
        lexer::lex_with_ids(source_str, id_provider.clone(), range_factory),
        LinkageType::Internal,
        source_path,
    );

    pre_process(&mut unit, id_provider);
    index.import(index::indexer::index(&unit));
    (unit, index, diagnostics)
}

pub fn index_with_ids<T: Into<SourceCode>>(src: T, id_provider: IdProvider) -> (CompilationUnit, Index) {
    let (unit, index, _) = do_index(src, id_provider);
    (unit, index)
}

pub fn index_unit_with_id(unit: &CompilationUnit, id_provider: IdProvider) -> Index {
    let mut index = Index::default();
    //Import builtins
    let builtins = builtins::parse_built_ins(id_provider.clone());

    index.import(index::indexer::index(&builtins));
    // import built-in types like INT, BOOL, etc.
    for data_type in get_builtin_types() {
        index.register_type(data_type);
    }

    index.import(index::indexer::index(unit));
    index
}
