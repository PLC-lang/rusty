//! Rename: prepare + apply.
//!
//! Two handlers:
//!
//! - **`prepareRename`** — position → `Option<{ range, placeholder }>`.
//!   Strict per Q6: returns `None` when no symbol resolves at the
//!   cursor or the resolution points at a synthesised (`<internal>`)
//!   declaration.
//! - **`rename`** — position + new_name → `Option<WorkspaceEdit>`.
//!   Validates the new name (identifier syntax, reserved keyword,
//!   same-scope collision) and emits `TextEdit`s for the declaration
//!   site + every entry in the reverse index, bucketed by URI.
//!
//! Validation policy (Q4):
//!
//! - Allow case-only changes (`a` → `A`). ST is case-insensitive but
//!   users may want consistent casing.
//! - Reject empty / non-identifier names (would fail to lex).
//! - Reject reserved ST keywords (would change semantics or fail to
//!   parse).
//! - Reject same-scope collisions (would clash with an existing decl
//!   of the same shape).
//! - Accept everything else, including potential cross-inheritance
//!   breakage of method overrides — *loud-fail validation*. If
//!   renaming `Base.foo` leaves `Derived.foo` orphaned, the next
//!   compile will surface the error and the user can fix it. The
//!   probe will tell us whether lowering already propagates enough
//!   for this case to "just work."

use std::collections::HashMap;

use lsp_types::{PositionEncodingKind, PrepareRenameResponse, TextEdit, Uri, WorkspaceEdit};

use crate::diagnostics::{code_span_to_range, path_to_uri};
use crate::position::{ResolvedSymbol, SymbolKind, SymbolUnderCursor};
use crate::reverse_index::ReverseIndex;
use plc::index::Index;
use plc_driver::pipelines::AnnotatedProject;
use plc_source::source_location::SourceLocation;

/// `prepareRename`: return the renamable range + placeholder for the
/// cursor, or `None` if no rename is possible at this position.
pub fn prepare_rename(
    symbol: &SymbolUnderCursor,
    encoding: &PositionEncodingKind,
) -> Option<PrepareRenameResponse> {
    let resolved = symbol.resolved.as_ref()?;
    if !is_renamable(resolved) {
        return None;
    }
    let range = code_span_to_range(symbol.usage_location.get_span(), encoding, None)?;
    Some(PrepareRenameResponse::RangeWithPlaceholder { range, placeholder: symbol.usage_text.clone() })
}

/// `rename`: emit a `WorkspaceEdit` replacing the symbol's declaration
/// site and every recorded use with `new_name`.
///
/// Returns `Err(message)` on validation failure (handler maps to
/// `InvalidParams`) and `Ok(None)` when the rename is a no-op
/// (target name is byte-for-byte identical to current).
pub fn rename_symbol(
    annotated: &AnnotatedProject,
    reverse_index: &ReverseIndex,
    symbol: &SymbolUnderCursor,
    new_name: &str,
    encoding: &PositionEncodingKind,
) -> Result<Option<WorkspaceEdit>, String> {
    let resolved = symbol.resolved.as_ref().ok_or("no symbol resolves at this position")?;
    if !is_renamable(resolved) {
        return Err("this symbol cannot be renamed (synthesised or undefined declaration)".into());
    }
    validate_new_name(new_name)?;
    validate_no_collision(&annotated.index, resolved, new_name)?;

    // Case-only or identical-name rename → empty edit (no-op).
    if eq_ignore_ascii_case_trimmed(&symbol.usage_text, new_name) && symbol.usage_text == new_name {
        return Ok(None);
    }

    let mut by_uri: HashMap<Uri, Vec<TextEdit>> = HashMap::new();

    // Declaration site itself.
    push_edit(&mut by_uri, &resolved.declaration_location, new_name, encoding);

    // Every recorded use.
    for entry in reverse_index.lookup(&resolved.declaration_location) {
        push_edit(&mut by_uri, &entry.location, new_name, encoding);
    }

    if by_uri.is_empty() {
        return Ok(None);
    }
    Ok(Some(WorkspaceEdit { changes: Some(by_uri), document_changes: None, change_annotations: None }))
}

fn push_edit(
    by_uri: &mut HashMap<Uri, Vec<TextEdit>>,
    location: &SourceLocation,
    new_name: &str,
    encoding: &PositionEncodingKind,
) {
    let Some(path) = location.get_file_name() else {
        return;
    };
    if path == "<internal>" {
        return;
    }
    let Some(uri) = path_to_uri(path) else {
        return;
    };
    let Some(range) = code_span_to_range(location.get_span(), encoding, None) else {
        return;
    };
    // Defensive: only emit edits for ranges that look like identifier
    // spans (single line, ≤ 128 chars). Some annotator paths produce
    // entries with whole-POU spans for self-references — a rename
    // with those would destructively rewrite the entire declaration.
    // Filtering here is a prototype-grade safety net; the real fix is
    // to identify which synthesised entries the reverse index should
    // skip during collection. Recorded as a post-phase-13 follow-up.
    if range.start.line != range.end.line {
        return;
    }
    let width = range.end.character.saturating_sub(range.start.character);
    if width == 0 || width > 128 {
        return;
    }
    by_uri.entry(uri).or_default().push(TextEdit { range, new_text: new_name.to_string() });
}

fn is_renamable(resolved: &ResolvedSymbol) -> bool {
    if resolved.declaration_location.is_undefined() {
        return false;
    }
    if matches!(resolved.declaration_location.get_file_name(), Some(name) if name == "<internal>") {
        return false;
    }
    // All resolved kinds are renamable in principle. Per-kind
    // validation (collision, etc.) is in `validate_no_collision`.
    matches!(
        resolved.kind,
        SymbolKind::Pou | SymbolKind::Variable | SymbolKind::Member | SymbolKind::Type | SymbolKind::Argument
    )
}

fn validate_new_name(new_name: &str) -> Result<(), String> {
    let trimmed = new_name.trim();
    if trimmed.is_empty() {
        return Err("new name must not be empty".into());
    }
    if trimmed != new_name {
        return Err("new name must not contain leading or trailing whitespace".into());
    }
    if !is_valid_identifier(trimmed) {
        return Err(format!("'{trimmed}' is not a valid IEC 61131-3 identifier"));
    }
    if is_reserved_keyword(trimmed) {
        return Err(format!("'{trimmed}' is a reserved ST keyword"));
    }
    Ok(())
}

fn is_valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn validate_no_collision(index: &Index, resolved: &ResolvedSymbol, new_name: &str) -> Result<(), String> {
    // Renaming to the same name (case-insensitive) is a no-op, not a
    // collision — handled in rename_symbol's empty-edit branch.
    if resolved.qualified_name.eq_ignore_ascii_case(new_name) {
        return Ok(());
    }

    // Compute the candidate qualified name to check.
    let candidate = match resolved.qualified_name.rsplit_once('.') {
        Some((container, _old)) => format!("{container}.{new_name}"),
        None => new_name.to_string(),
    };

    match resolved.kind {
        SymbolKind::Variable | SymbolKind::Argument | SymbolKind::Member => {
            // Variable / member / argument: collision if another decl
            // with the candidate qualified name exists in the same
            // scope.
            let collision = match candidate.rsplit_once('.') {
                Some((container, member)) => index.find_member(container, member).is_some(),
                None => index.find_global_variable(&candidate).is_some(),
            };
            if collision {
                return Err(format!("'{new_name}' already exists in this scope"));
            }
        }
        SymbolKind::Pou => {
            if index.find_pou(new_name).is_some() {
                return Err(format!("a POU named '{new_name}' already exists"));
            }
        }
        SymbolKind::Type => {
            if index.find_type(new_name).is_some() || index.find_pou_type(new_name).is_some() {
                return Err(format!("a type named '{new_name}' already exists"));
            }
        }
    }
    Ok(())
}

fn eq_ignore_ascii_case_trimmed(a: &str, b: &str) -> bool {
    a.trim().eq_ignore_ascii_case(b.trim())
}

/// Reserved IEC 61131-3 / ST keywords. The lexer encodes these inline
/// via `#[token(...)]` attributes, so we mirror the set here rather
/// than introduce a public list. Worth refactoring into a shared
/// constant post-phase-13 (also noted in
/// `[[lsp-post-phase13-followups]]` re: AST-serializer keyword reuse).
fn is_reserved_keyword(name: &str) -> bool {
    const KEYWORDS: &[&str] = &[
        "PROGRAM",
        "END_PROGRAM",
        "FUNCTION",
        "END_FUNCTION",
        "FUNCTION_BLOCK",
        "END_FUNCTION_BLOCK",
        "CLASS",
        "END_CLASS",
        "METHOD",
        "END_METHOD",
        "ACTION",
        "END_ACTION",
        "ACTIONS",
        "END_ACTIONS",
        "INTERFACE",
        "END_INTERFACE",
        "TYPE",
        "END_TYPE",
        "STRUCT",
        "END_STRUCT",
        "VAR",
        "VAR_INPUT",
        "VAR_OUTPUT",
        "VAR_IN_OUT",
        "VAR_GLOBAL",
        "VAR_TEMP",
        "VAR_EXTERNAL",
        "VAR_CONFIG",
        "END_VAR",
        "IF",
        "THEN",
        "ELSE",
        "ELSIF",
        "END_IF",
        "FOR",
        "TO",
        "BY",
        "DO",
        "END_FOR",
        "WHILE",
        "END_WHILE",
        "REPEAT",
        "UNTIL",
        "END_REPEAT",
        "CASE",
        "OF",
        "END_CASE",
        "RETURN",
        "EXIT",
        "CONTINUE",
        "AND",
        "OR",
        "XOR",
        "NOT",
        "MOD",
        "TRUE",
        "FALSE",
        "EXTENDS",
        "IMPLEMENTS",
        "SUPER",
        "THIS",
        "REF",
        "REF_TO",
        "POINTER",
        "PROPERTY",
        "GET",
        "SET",
        "END_PROPERTY",
        "END_GET",
        "END_SET",
        "ARRAY",
        "STRING",
        "WSTRING",
        // Type names — refusing these prevents user confusion, even
        // though some parsers technically allow shadowing.
        "BOOL",
        "BYTE",
        "WORD",
        "DWORD",
        "LWORD",
        "INT",
        "SINT",
        "DINT",
        "LINT",
        "USINT",
        "UINT",
        "UDINT",
        "ULINT",
        "REAL",
        "LREAL",
        "CHAR",
        "WCHAR",
        "TIME",
        "DATE",
        "DT",
        "TOD",
        "TIME_OF_DAY",
        "DATE_AND_TIME",
    ];
    KEYWORDS.iter().any(|kw| kw.eq_ignore_ascii_case(name))
}
