//! `textDocument/semanticTokens/full` — server-side semantic
//! highlighting refinement.
//!
//! Per the F6 grill we only emit semantic tokens for **identifiers**.
//! Keywords, comments, strings, numbers, operators stay under the
//! TextMate grammar (serhioromano's vscode-st). The LSP's job is to
//! resolve identifier *kind*: `foo` could be a variable, function call,
//! type name, or struct field; the annotator already knows, we just
//! surface that knowledge.
//!
//! Legend (advertised in the InitializeResult, must stay in sync with
//! the `TYPES` / `MODIFIERS` constants below):
//!
//! ```text
//!  TYPES:     function, method, class, struct, type, variable,
//!             parameter, property
//!  MODIFIERS: readonly, defaultLibrary
//! ```
//!
//! Wire format: a flat `Vec<u32>` of 5-tuples
//! `(deltaLine, deltaStart, length, tokenType, modifierBitmask)` per
//! token, delta-encoded relative to the previous token. First token's
//! `deltaLine` / `deltaStart` are absolute.

use std::path::Path;

use lsp_types::{Position, PositionEncodingKind, SemanticToken, SemanticTokens};
use plc::index::Index;
use plc::resolver::{AnnotationMap, StatementAnnotation};
use plc_ast::ast::{AstNode, CompilationUnit, DataType, Pou, UserTypeDeclaration, Variable, VariableBlock};
use plc_ast::visitor::{AstVisitor, Walker};
use plc_driver::pipelines::AnnotatedProject;

// ---------------------------------------------------------------------
// Legend
// ---------------------------------------------------------------------

/// Indices into the semantic-token type legend, mirroring `TYPES`.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TokenTypeIdx {
    Function = 0,
    Method = 1,
    Class = 2,
    Struct = 3,
    Type = 4,
    Variable = 5,
    Parameter = 6,
    Property = 7,
}

/// Indices into the modifier bitmask. Each modifier is one bit.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ModifierIdx {
    Readonly = 0,
    DefaultLibrary = 1,
}

pub const TYPES: &[&str] =
    &["function", "method", "class", "struct", "type", "variable", "parameter", "property"];

pub const MODIFIERS: &[&str] = &["readonly", "defaultLibrary"];

// ---------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------

/// Compute the semantic-token list for the file at `path` within
/// `annotated`. Returns an empty result if the file isn't part of the
/// project (`None` from `find_unit`).
pub fn semantic_tokens_for_file(
    annotated: &AnnotatedProject,
    path: &Path,
    source: &str,
    encoding: &PositionEncodingKind,
) -> SemanticTokens {
    let Some(unit) = find_unit(annotated, path) else {
        return SemanticTokens::default();
    };

    let mut collector = Collector { entries: Vec::new(), annotated, source };
    unit.walk(&mut collector);
    // The visitor produces entries in walk order, which is roughly
    // source order but not strictly; sort to be safe before delta-
    // encoding (LSP requires strictly non-decreasing positions).
    collector.entries.sort_by_key(|e| (e.line, e.character));
    encode(&collector.entries, encoding, source)
}

fn find_unit<'a>(annotated: &'a AnnotatedProject, path: &Path) -> Option<&'a CompilationUnit> {
    // Same shape as `position::find_unit` — match either a fully
    // matching path or a suffix match against the unit's filename.
    let needle = path.to_string_lossy();
    annotated.units.iter().map(|au| au.get_unit()).find(|unit| {
        unit.file.get_name().map(|file| file == needle.as_ref() || needle.ends_with(file)).unwrap_or(false)
    })
}

// ---------------------------------------------------------------------
// Collection
// ---------------------------------------------------------------------

#[derive(Debug)]
struct Entry {
    line: u32,
    character: u32, // byte column; encoder converts to utf-16 if needed
    length: u32,    // byte length; encoder converts to utf-16 if needed
    token_type: u32,
    modifiers: u32,
}

struct Collector<'a> {
    entries: Vec<Entry>,
    annotated: &'a AnnotatedProject,
    source: &'a str,
}

impl<'a> Collector<'a> {
    /// Push an entry covering `(line, byte_col)` and spanning `byte_len`.
    fn push(&mut self, line: usize, byte_col: usize, byte_len: usize, ty: TokenTypeIdx, mods: u32) {
        self.entries.push(Entry {
            line: line as u32,
            character: byte_col as u32,
            length: byte_len as u32,
            token_type: ty as u32,
            modifiers: mods,
        });
    }

    /// Push from a `SourceLocation`. Returns silently when the location
    /// is internal / undefined (no real span to tag).
    fn push_loc(&mut self, loc: &plc_source::source_location::SourceLocation, ty: TokenTypeIdx, mods: u32) {
        let plc_source::source_location::CodeSpan::Range(_) = loc.get_span() else {
            return;
        };
        let line = loc.get_span().get_line();
        let col = loc.get_span().get_column();
        let Some(end) = loc.to_range() else { return };
        let byte_len = end.end.saturating_sub(end.start);
        self.push(line, col, byte_len, ty, mods);
    }

    /// Resolve an annotated AstNode's annotation into a semantic token
    /// kind. Mirrors `position::translate`, but produces a token-type
    /// instead of a `ResolvedSymbol`.
    fn classify_annotation(&self, annot: &StatementAnnotation) -> Option<(TokenTypeIdx, u32)> {
        match annot {
            StatementAnnotation::Variable { qualified_name, .. } => {
                let kind = classify_variable_kind(qualified_name, &self.annotated.index);
                let mods = variable_modifiers(qualified_name, &self.annotated.index);
                Some((kind, mods))
            }
            StatementAnnotation::Argument { pou, position, .. } => {
                let params = self.annotated.index.get_available_parameters(pou);
                let mods = params.get(*position).map(|e| modifier_bits_for_var(e)).unwrap_or(0);
                Some((TokenTypeIdx::Parameter, mods))
            }
            StatementAnnotation::Function { .. } | StatementAnnotation::Program { .. } => {
                // Distinguish methods from free functions via the
                // qualified_name shape (`Parent.method` for methods).
                if let StatementAnnotation::Function { qualified_name, .. } = annot {
                    if let Some(pou) = self.annotated.index.find_pou(qualified_name) {
                        let kind = match pou {
                            plc::index::PouIndexEntry::Method { .. } => TokenTypeIdx::Method,
                            plc::index::PouIndexEntry::FunctionBlock { .. } => TokenTypeIdx::Class,
                            plc::index::PouIndexEntry::Class { .. } => TokenTypeIdx::Class,
                            plc::index::PouIndexEntry::Action { .. } => TokenTypeIdx::Method,
                            _ => TokenTypeIdx::Function,
                        };
                        return Some((kind, 0));
                    }
                }
                Some((TokenTypeIdx::Function, 0))
            }
            StatementAnnotation::Type { type_name } => {
                let kind = classify_type_kind(type_name, &self.annotated.index);
                Some((kind, 0))
            }
            _ => None,
        }
    }
}

impl<'a> AstVisitor for Collector<'a> {
    fn visit_pou(&mut self, pou: &Pou) {
        // Declaration site: the POU's name. Pick the token kind by the
        // POU's index entry so `FUNCTION_BLOCK Foo` is `class` and
        // `METHOD m` is `method`.
        let kind = match self.annotated.index.find_pou(&pou.name) {
            Some(plc::index::PouIndexEntry::Method { .. }) => TokenTypeIdx::Method,
            Some(plc::index::PouIndexEntry::FunctionBlock { .. }) => TokenTypeIdx::Class,
            Some(plc::index::PouIndexEntry::Class { .. }) => TokenTypeIdx::Class,
            Some(plc::index::PouIndexEntry::Action { .. }) => TokenTypeIdx::Method,
            _ => TokenTypeIdx::Function,
        };
        self.push_loc(&pou.name_location, kind, 0);
        Walker::walk(pou, self);
    }

    fn visit_user_type_declaration(&mut self, ut: &UserTypeDeclaration) {
        // Type declaration name. UserTypeDeclaration.data_type.get_name()
        // gives us the name, but the parser doesn't expose a name
        // location separately from the whole declaration. Tag the
        // overall location with the appropriate kind — tightest-wins
        // means inner AstNode hits (e.g. references to field types)
        // still take precedence at narrower spans.
        if let Some(name) = ut.data_type.get_name() {
            let kind = classify_data_type_kind(&ut.data_type);
            // Use the first identifier-shaped slice of the location.
            // Without a precise name-location we fall back to the full
            // declaration span; this overpaints slightly but the
            // grammar's keyword colouring still wins inside it because
            // semantic tokens layer on top of the grammar palette.
            if let Some(range) = ut.location.to_range() {
                // Find the first token-position in the source that
                // matches the name; gives a precise span without
                // needing parser changes.
                if let Some(rel) = self.source[range.clone()].find(name) {
                    let abs_start = range.start + rel;
                    if let Some((line, col)) = byte_to_line_col(self.source, abs_start) {
                        self.push(line, col, name.len(), kind, 0);
                    }
                }
            }
        }
        Walker::walk(ut, self);
    }

    fn visit_variable_block(&mut self, block: &VariableBlock) {
        for var in &block.variables {
            self.visit_variable(var);
        }
    }

    fn visit_variable(&mut self, variable: &Variable) {
        // Look the variable up in the index. We don't have the
        // enclosing container threaded through the visitor in the POC,
        // so try both find_member (across all containers) and
        // find_global_variable. First match wins.
        let entry = self
            .annotated
            .index
            .find_global_variable(variable.name.as_str())
            .or_else(|| self.find_member_anywhere(variable.name.as_str()));
        let kind = entry.map(token_type_for_variable_entry).unwrap_or(TokenTypeIdx::Variable);
        let mods = entry.map(modifier_bits_for_var).unwrap_or(0);
        self.push_loc(&variable.location, kind, mods);
        Walker::walk(variable, self);
    }

    fn visit(&mut self, node: &AstNode) {
        // Reference usage sites: ask the annotation map what this node
        // resolves to. We only tag the LEAF reference (the identifier
        // span), so inner `ReferenceExpr::Member` nodes whose `base`
        // recurses get tagged by recursion, not by the outer wrapper.
        if let Some(annot) = self.annotated.annotations.get_with_id(node.id) {
            if let Some((kind, mods)) = self.classify_annotation(annot) {
                if let Some(range) = node.location.to_range() {
                    // Only tag if the location is a single-line span and
                    // looks like a bare identifier (no `.` / `[` inside).
                    // ReferenceExpr wrappers around `foo.bar` have a
                    // wider span; we want to tag `foo` and `bar`
                    // separately, which happens when we recurse into
                    // each member.
                    let slice = &self.source[range.clone()];
                    if slice.chars().all(is_ident_char) {
                        if let Some((line, col)) = byte_to_line_col(self.source, range.start) {
                            self.push(line, col, slice.len(), kind, mods);
                        }
                    }
                }
            }
        }
        node.walk(self);
    }
}

impl<'a> Collector<'a> {
    /// Walk every POU and every type's members looking for an entry
    /// whose name matches. Used by `visit_variable` when we don't have
    /// the enclosing container threaded through. Returns the first
    /// match — names are project-globally unique inside their
    /// container, and the var-vs-parameter classification is the
    /// same for any container the entry lives in.
    fn find_member_anywhere(&self, name: &str) -> Option<&'a plc::index::VariableIndexEntry> {
        for pou in self.annotated.index.get_pous().values() {
            if let Some(entry) = self.annotated.index.find_member(pou.get_name(), name) {
                return Some(entry);
            }
        }
        for ty in self.annotated.index.get_types().elements() {
            if let Some(entry) = self.annotated.index.find_member(ty.1.get_name(), name) {
                return Some(entry);
            }
        }
        None
    }
}

// ---------------------------------------------------------------------
// Classification helpers
// ---------------------------------------------------------------------

fn classify_variable_kind(qualified_name: &str, _index: &Index) -> TokenTypeIdx {
    if qualified_name.contains('.') {
        TokenTypeIdx::Property
    } else {
        TokenTypeIdx::Variable
    }
}

fn variable_modifiers(qualified_name: &str, index: &Index) -> u32 {
    let entry = match qualified_name.rsplit_once('.') {
        Some((container, member)) => index.find_member(container, member),
        None => index.find_global_variable(qualified_name),
    };
    entry.map(modifier_bits_for_var).unwrap_or(0)
}

fn modifier_bits_for_var(entry: &plc::index::VariableIndexEntry) -> u32 {
    let mut bits = 0;
    if entry.is_constant() {
        bits |= 1 << ModifierIdx::Readonly as u32;
    }
    bits
}

fn token_type_for_variable_entry(entry: &plc::index::VariableIndexEntry) -> TokenTypeIdx {
    use plc::index::VariableType;
    match entry.get_variable_type() {
        VariableType::Input | VariableType::Output | VariableType::InOut => TokenTypeIdx::Parameter,
        _ => TokenTypeIdx::Variable,
    }
}

fn classify_type_kind(type_name: &str, index: &Index) -> TokenTypeIdx {
    use plc::typesystem::DataTypeInformation;
    if let Some(t) = index.find_type(type_name) {
        return match t.get_type_information() {
            DataTypeInformation::Struct { .. } => TokenTypeIdx::Struct,
            _ => TokenTypeIdx::Type,
        };
    }
    if let Some(pou) = index.find_pou(type_name) {
        return match pou {
            plc::index::PouIndexEntry::FunctionBlock { .. } | plc::index::PouIndexEntry::Class { .. } => {
                TokenTypeIdx::Class
            }
            _ => TokenTypeIdx::Type,
        };
    }
    TokenTypeIdx::Type
}

fn classify_data_type_kind(data_type: &DataType) -> TokenTypeIdx {
    match data_type {
        DataType::StructType { .. } => TokenTypeIdx::Struct,
        _ => TokenTypeIdx::Type,
    }
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Convert a byte offset to (line, byte_column). Both 0-based.
fn byte_to_line_col(source: &str, byte_offset: usize) -> Option<(usize, usize)> {
    if byte_offset > source.len() {
        return None;
    }
    let mut line = 0;
    let mut last_line_start = 0;
    for (i, b) in source.as_bytes()[..byte_offset].iter().enumerate() {
        if *b == b'\n' {
            line += 1;
            last_line_start = i + 1;
        }
    }
    Some((line, byte_offset - last_line_start))
}

// ---------------------------------------------------------------------
// Encoding
// ---------------------------------------------------------------------

fn encode(entries: &[Entry], encoding: &PositionEncodingKind, source: &str) -> SemanticTokens {
    let mut data: Vec<SemanticToken> = Vec::with_capacity(entries.len());
    let mut prev_line = 0u32;
    let mut prev_start = 0u32;
    for entry in entries {
        let (character, length) = if encoding == &PositionEncodingKind::UTF16 {
            (
                byte_col_to_utf16(source, entry.line as usize, entry.character as usize) as u32,
                byte_len_to_utf16(
                    source,
                    entry.line as usize,
                    entry.character as usize,
                    entry.length as usize,
                ) as u32,
            )
        } else {
            (entry.character, entry.length)
        };
        let delta_line = entry.line - prev_line;
        let delta_start = if delta_line == 0 { character - prev_start } else { character };
        data.push(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type: entry.token_type,
            token_modifiers_bitset: entry.modifiers,
        });
        prev_line = entry.line;
        prev_start = character;
    }
    SemanticTokens { result_id: None, data }
}

fn byte_col_to_utf16(source: &str, line: usize, byte_col: usize) -> usize {
    source
        .lines()
        .nth(line)
        .and_then(|l| l.get(..byte_col))
        .map(|prefix| prefix.encode_utf16().count())
        .unwrap_or(byte_col)
}

fn byte_len_to_utf16(source: &str, line: usize, byte_col: usize, byte_len: usize) -> usize {
    source
        .lines()
        .nth(line)
        .and_then(|l| l.get(byte_col..byte_col + byte_len))
        .map(|slice| slice.encode_utf16().count())
        .unwrap_or(byte_len)
}

// ---------------------------------------------------------------------
// _silence_unused
// ---------------------------------------------------------------------

#[allow(dead_code)]
fn _typed_to_position(_p: Position) {
    // Position is imported for callers; keep it referenced so the
    // module compiles cleanly when neither the encoder nor the test
    // suite mention it directly.
}
