//! Parses embedded structured-text fragments with the main compiler's parser.

use plc::lexer;
use plc::parser::{self, expressions_parser};
use plc_ast::ast::{AstNode, CompilationUnit, LinkageType};
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};
use plc_source::{SourceCode, SourceContainer};

use crate::model::{Pou, PouKind};

pub fn parse_expression(text: &str, ids: IdProvider) -> AstNode {
    let factory = SourceLocationFactory::internal(text);
    let mut session = lexer::lex_with_ids(text, ids, factory);

    expressions_parser::parse_expression(&mut session)
}

// A parsed expression relocated to where the network places it, so
// diagnostics point at the diagram element rather than the parsed text.
pub fn parse_expression_at(text: &str, ids: IdProvider, location: &SourceLocation) -> AstNode {
    let mut node = parse_expression(text, ids);
    node.location = location.clone();
    node
}

pub fn parse_interface(
    pou: &Pou,
    source: &SourceCode,
    ids: IdProvider,
) -> (CompilationUnit, Vec<Diagnostic>) {
    // The declaration omits its closing keyword; re-attach it.
    let end_keyword = match pou.kind() {
        PouKind::Function => "END_FUNCTION",
        PouKind::FunctionBlock => "END_FUNCTION_BLOCK",
        PouKind::Program => "END_PROGRAM",
    };
    let declaration = format!("{}\n{end_keyword}", pou.content().declaration().unwrap_or_default());

    // Parse the completed declaration like any ST source.
    let declaration = SourceCode { source: declaration, path: source.path.clone() };
    let factory = SourceLocationFactory::for_source(&declaration);
    let session = lexer::lex_with_ids(&declaration.source, ids, factory);

    parser::parse(session, LinkageType::Internal, source.get_location_str())
}
