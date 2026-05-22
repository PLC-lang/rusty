//! `textDocument/inlayHint` — parameter-name hints for positional
//! call arguments.
//!
//! When a `CallStatement` uses positional arguments (e.g. `foo(1, 2)`
//! rather than `foo(x := 1, y := 2)`), the server emits one inlay hint
//! per argument showing the parameter name. The hint sits inline at
//! the start of the argument's source span and reads as e.g.
//! `foo(│x:│ 1, │y:│ 2)`.
//!
//! Calls that already use named arguments (any `:=` / `=>`) are
//! skipped — the user clearly knows the names and the hints would be
//! redundant noise.

use std::path::Path;

use lsp_types::{InlayHint, InlayHintKind, InlayHintLabel, Position, PositionEncodingKind};
use plc::resolver::{AnnotationMap, StatementAnnotation};
use plc_ast::ast::{flatten_expression_list, AstNode, AstStatement, CallStatement, CompilationUnit};
use plc_ast::visitor::{AstVisitor, Walker};
use plc_driver::pipelines::AnnotatedProject;

/// Top-level entry point. Walks the compilation unit for `path` and
/// returns one `InlayHint` per positional call argument.
pub fn inlay_hints_for_file(
    annotated: &AnnotatedProject,
    path: &Path,
    source: &str,
    encoding: &PositionEncodingKind,
) -> Vec<InlayHint> {
    let Some(unit) = find_unit(annotated, path) else {
        return Vec::new();
    };
    let mut collector = Collector { hints: Vec::new(), annotated, source, encoding };
    unit.walk(&mut collector);
    collector.hints
}

fn find_unit<'a>(annotated: &'a AnnotatedProject, path: &Path) -> Option<&'a CompilationUnit> {
    let needle = path.to_string_lossy();
    annotated.units.iter().map(|au| au.get_unit()).find(|unit| {
        unit.file.get_name().map(|file| file == needle.as_ref() || needle.ends_with(file)).unwrap_or(false)
    })
}

struct Collector<'a> {
    hints: Vec<InlayHint>,
    annotated: &'a AnnotatedProject,
    source: &'a str,
    encoding: &'a PositionEncodingKind,
}

impl<'a> Collector<'a> {
    /// Inspect a CallStatement. When the args are all positional and
    /// the callee resolves to a known POU, emit one hint per arg.
    fn handle_call(&mut self, stmt: &CallStatement) {
        let Some(params_node) = stmt.parameters.as_deref() else {
            return; // call with no parameters: nothing to hint
        };
        let args = flatten_expression_list(params_node);
        if args.is_empty() {
            return;
        }
        // Any named-arg shape (`Assignment` / `OutputAssignment`)
        // suppresses hints for the whole call: the user has already
        // committed to the named-arg style.
        if args.iter().any(|arg| {
            matches!(arg.get_stmt(), AstStatement::Assignment(..) | AstStatement::OutputAssignment(..))
        }) {
            return;
        }

        // Resolve the callee through its annotation. Free functions and
        // FB instance calls both surface as `Function` annotations on
        // the operator node.
        let qualified_name = match self.annotated.annotations.get_with_id(stmt.operator.id) {
            Some(StatementAnnotation::Function { qualified_name, .. }) => qualified_name.clone(),
            Some(StatementAnnotation::Program { qualified_name }) => qualified_name.clone(),
            _ => return,
        };
        let params = self.annotated.index.get_available_parameters(&qualified_name);
        if params.is_empty() {
            return;
        }

        for (arg, param) in args.iter().zip(params.iter()) {
            // Hint sits at the argument's start. Convert the byte
            // offset into LSP `(line, character)` via the encoding-
            // aware helper that mirrors diagnostics.rs.
            let Some(range) = arg.location.to_range() else { continue };
            let Some(position) = byte_offset_to_position(self.source, range.start, self.encoding) else {
                continue;
            };
            self.hints.push(InlayHint {
                position,
                label: InlayHintLabel::String(format!("{}:", param.get_name())),
                kind: Some(InlayHintKind::PARAMETER),
                text_edits: None,
                tooltip: None,
                padding_left: None,
                padding_right: Some(true),
                data: None,
            });
        }
    }
}

impl<'a> AstVisitor for Collector<'a> {
    fn visit_call_statement(&mut self, stmt: &CallStatement, node: &AstNode) {
        self.handle_call(stmt);
        // Descend — nested calls inside arguments get their own hints.
        let _ = node;
        stmt.walk(self);
    }
}

/// Convert a byte offset in `source` into an LSP `Position`. Mirrors
/// the encoding logic in `diagnostics.rs`: utf-8 keeps byte columns,
/// utf-16 counts code units in the line prefix.
fn byte_offset_to_position(
    source: &str,
    byte_offset: usize,
    encoding: &PositionEncodingKind,
) -> Option<Position> {
    if byte_offset > source.len() {
        return None;
    }
    let mut line: u32 = 0;
    let mut last_line_start = 0usize;
    for (i, b) in source.as_bytes()[..byte_offset].iter().enumerate() {
        if *b == b'\n' {
            line += 1;
            last_line_start = i + 1;
        }
    }
    let byte_col = byte_offset - last_line_start;
    let character = if encoding == &PositionEncodingKind::UTF16 {
        source
            .lines()
            .nth(line as usize)
            .and_then(|l| l.get(..byte_col))
            .map(|p| p.encode_utf16().count() as u32)
            .unwrap_or(byte_col as u32)
    } else {
        byte_col as u32
    };
    Some(Position { line, character })
}
