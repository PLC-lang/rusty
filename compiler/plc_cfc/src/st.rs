//! Parses embedded structured-text fragments with the main compiler's parser.

use plc::lexer::{self, Token, TokenClass};
use plc::parser::{self, expressions_parser};
use plc_ast::ast::{AstNode, CompilationUnit, LinkageType};
use plc_ast::mut_visitor::{AstVisitorMut, WalkerMut};
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};
use plc_source::{SourceCode, SourceContainer};

use crate::model::{Pou, PouKind};

// Moves every node of a parsed tree to one location.
struct Relocate(SourceLocation);

impl AstVisitorMut for Relocate {
    fn visit(&mut self, node: &mut AstNode) {
        node.location = self.0.clone();
        node.walk(self)
    }
}

pub fn parse_expression(text: &str, ids: IdProvider) -> AstNode {
    let factory = SourceLocationFactory::internal(text);
    let mut session = lexer::lex_with_ids(text, ids, factory);

    expressions_parser::parse_expression(&mut session)
}

// Whether `text` is a single operator keyword that names a builtin function, e.g. `AND`.
pub fn is_operator_function(text: &str) -> bool {
    let session = lexer::lex_with_ids(text, IdProvider::default(), SourceLocationFactory::internal(text));
    session.token.classes().contains(&TokenClass::OperatorFunction) && session.peek() == Token::End
}

// A parsed expression relocated to where the network places it, so
// diagnostics point at the diagram element rather than the parsed text.
// Every node is relocated, not only the root: the parser locates nested nodes
// within `text`, which no file contains, so diagnostics on them (e.g. an
// unresolved `foo`) would print no location at all.
pub fn parse_expression_at(text: &str, ids: IdProvider, location: &SourceLocation) -> AstNode {
    let mut node = parse_expression(text, ids);
    Relocate(location.clone()).visit(&mut node);
    node
}

// The declaration text as the IDE's text editor shows it, completed by the
// closing keyword the export omits; text locations count lines within it.
pub fn declaration(pou: &Pou) -> String {
    let end_keyword = match pou.kind() {
        PouKind::Function => "END_FUNCTION",
        PouKind::FunctionBlock => "END_FUNCTION_BLOCK",
        PouKind::Program => "END_PROGRAM",
    };

    format!("{}\n{end_keyword}", pou.content().declaration().unwrap_or_default())
}

pub fn parse_interface(
    pou: &Pou,
    source: &SourceCode,
    ids: IdProvider,
) -> (CompilationUnit, Vec<Diagnostic>) {
    // Parse the completed declaration like any ST source.
    let declaration = SourceCode { source: declaration(pou), path: source.path.clone() };
    let factory = SourceLocationFactory::for_source(&declaration);
    let session = lexer::lex_with_ids(&declaration.source, ids, factory);

    parser::parse(session, LinkageType::Internal, source.get_location_str())
}
