//! `textDocument/completion` handler — phase 13 (P13.5 skeleton).
//!
//! Detects the user's completion context from the cursor position and emits
//! a `CompletionList`. The context detector here is a byte-level heuristic:
//! we walk back from the cursor through whitespace and inspect the
//! preceding non-whitespace character (`.` → member access, `(` → call
//! site, `:` → type position, etc.).
//!
//! The heuristic is deliberately not AST-based for P13.5. The lenient
//! parser preserves enough structure that an AST-walk would also work for
//! most cases (see the dot-path fix that emits `Member(EmptyStatement)`),
//! but the A6 merge case (`foo.\na := 1;` parses cleanly as `foo.a := 1;`
//! with no diagnostic) is genuinely invisible to the AST. Source bytes are
//! the only signal that survives that case, so we make them the primary
//! input here. P13.6 layers type-hint enrichment on top by consulting the
//! AnnotationMap once the context is known.
//!
//! The handler returns an empty `CompletionList` for now — symbol
//! enumeration is P13.6.

use lsp_types::{CompletionList, CompletionTriggerKind};

/// Detected completion context at the user's cursor. Determines which
/// category of items the handler emits and how to rank them.
///
/// Each variant carries the offsets / spans the enumeration step (P13.6)
/// will need; the detector populates them but doesn't dereference yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionContextKind {
    /// Cursor immediately after a `.` — member-access completion against
    /// the resolved type of the base expression preceding the dot. The
    /// `base_offset..base_end` range captures the base token (e.g. `foo`
    /// in `foo.|` or `arr[1]` in `arr[1].|`); the enumerator resolves it
    /// through the AnnotationMap to get the type whose members to offer.
    Member { base_offset: usize, base_end: usize },

    /// Cursor inside the argument list of a call (`other(|`,
    /// `other(x,|`). The `operator_offset..operator_end` range captures
    /// the callee identifier; the enumerator emits the callee's
    /// parameters as named-arg candidates, plus in-scope expression
    /// items for positional arguments.
    Call { operator_offset: usize, operator_end: usize },

    /// Cursor immediately after `:` in a declaration context (`VAR x :|`,
    /// `FUNCTION foo :|`, `FUNCTION_BLOCK B EXTENDS|`). Only type names
    /// are offered.
    TypePosition,

    /// Cursor in expression position. `hint_type` is the expected type
    /// at the slot when known (e.g. DINT for `FOR i := 1 TO |`), used by
    /// the ranking pass; `None` falls back to alphabetic-within-tier.
    Expression { hint_type: Option<String> },

    /// Cursor outside any POU at the file's top level. Offers POU-start
    /// keywords (`PROGRAM`, `FUNCTION`, ...) plus type / global keywords.
    TopLevel,

    /// Cursor inside a `VAR_GLOBAL` block. Offers `CONSTANT` modifier,
    /// then type names (after `:`), then identifier-name patterns.
    VarGlobalBlock,

    /// Generic statement position inside a POU body — all in-scope items
    /// (locals, globals, POU names) plus ST statement keywords.
    Statement,
}

/// Detect the completion context at the given cursor position by walking
/// back from the cursor through whitespace and inspecting the preceding
/// non-whitespace character (and a few characters of lookback for
/// multi-char operators like `:=`).
///
/// `cursor_byte_offset` is clamped to `source.len()`.
pub fn detect_context(source: &str, cursor_byte_offset: usize) -> CompletionContextKind {
    let cursor = cursor_byte_offset.min(source.len());
    let bytes = source.as_bytes();

    // Walk back through whitespace (spaces, tabs, newlines).
    let mut idx = cursor;
    while idx > 0 && bytes[idx - 1].is_ascii_whitespace() {
        idx -= 1;
    }

    if idx == 0 {
        return CompletionContextKind::TopLevel;
    }

    let preceding = bytes[idx - 1];
    match preceding {
        b'.' => {
            let base_end = idx - 1;
            let base_offset = scan_back_qualified_token(source, base_end);
            CompletionContextKind::Member { base_offset, base_end }
        }
        b'(' | b',' => {
            // Both `other(|` and `other(x,|` are call-context positions.
            // Walk back through the arg list to find the operator that
            // opened the parens. For now, naively scan back past balanced
            // commas/expressions and look for the operator identifier
            // before `(`. The naive heuristic: walk back to the matching
            // `(` and capture the identifier preceding it.
            if let Some((op_offset, op_end)) = scan_back_call_operator(source, idx - 1) {
                CompletionContextKind::Call { operator_offset: op_offset, operator_end: op_end }
            } else {
                CompletionContextKind::Expression { hint_type: None }
            }
        }
        b':' => {
            // Disambiguate type-position (`:`) from assignment (`:=`).
            // `:=` is two consecutive bytes; if the byte AFTER our `:` is
            // `=`, this isn't a type position. But we walked back through
            // whitespace, so the `:` we're looking at is the last
            // non-whitespace character. The byte at `cursor` (or anything
            // between) is whitespace, not `=`. So a bare `:` here is type
            // position. CASE labels (`1:`) are followed by a body, not
            // by cursor-at-end-of-line, so they don't trigger here.
            CompletionContextKind::TypePosition
        }
        b'=' if idx >= 2 && bytes[idx - 2] == b':' => {
            // Cursor follows `:=` — expression position. P13.6 may enrich
            // with the LHS's type as a hint via AST walk.
            CompletionContextKind::Expression { hint_type: None }
        }
        b';' => CompletionContextKind::Statement,
        _ => CompletionContextKind::Expression { hint_type: None },
    }
}

/// Scan back from `end` through identifier/qualified-access characters
/// (alphanumerics, `_`, `^`, `.`, `[`, `]`) to find the start of the base
/// expression. Stops at the first non-token character. Used by the member
/// detector to capture `foo.bar` or `arr[1]` or `THIS^` as the base.
fn scan_back_qualified_token(source: &str, end: usize) -> usize {
    let bytes = source.as_bytes();
    let mut start = end;
    let mut bracket_depth: i32 = 0;
    while start > 0 {
        let c = bytes[start - 1];
        match c {
            b']' => {
                bracket_depth += 1;
                start -= 1;
            }
            b'[' => {
                if bracket_depth == 0 {
                    break;
                }
                bracket_depth -= 1;
                start -= 1;
            }
            c if bracket_depth > 0 => {
                // inside brackets, accept any char (including whitespace
                // and operators inside index expressions)
                start -= 1;
                let _ = c;
            }
            c if c.is_ascii_alphanumeric() || c == b'_' || c == b'^' || c == b'.' => {
                start -= 1;
            }
            _ => break,
        }
    }
    start
}

/// Scan back from a `(` or `,` to find the operator (callee identifier)
/// at the head of the call. When `paren_or_comma_idx` points directly at
/// `(`, that's the opening paren. When it points at `,`, walk back
/// through balanced parens until we find the unmatched `(`. Returns
/// `(operator_offset, operator_end)`, or `None` when no operator can be
/// identified (e.g. cursor is in a bare parenthesised expression with no
/// callee identifier in front of it).
fn scan_back_call_operator(source: &str, paren_or_comma_idx: usize) -> Option<(usize, usize)> {
    let bytes = source.as_bytes();

    let opening_paren_idx = match bytes.get(paren_or_comma_idx) {
        Some(&b'(') => paren_or_comma_idx,
        Some(&b',') => {
            let mut paren_depth: i32 = 0;
            let mut probe = paren_or_comma_idx;
            loop {
                if probe == 0 {
                    return None;
                }
                probe -= 1;
                match bytes[probe] {
                    b')' => paren_depth += 1,
                    b'(' => {
                        if paren_depth == 0 {
                            break probe;
                        }
                        paren_depth -= 1;
                    }
                    _ => {}
                }
            }
        }
        _ => return None,
    };

    // Operator is immediately before the opening `(`, skipping whitespace.
    let mut op_end = opening_paren_idx;
    while op_end > 0 && bytes[op_end - 1].is_ascii_whitespace() {
        op_end -= 1;
    }
    let op_offset = scan_back_qualified_token(source, op_end);
    if op_offset == op_end {
        return None;
    }
    Some((op_offset, op_end))
}

/// Build the `CompletionList` for the given trigger context. Phase 13.5
/// is the skeleton — returns an empty list. The detector runs and its
/// output influences logging (so we can verify routing in tests) but no
/// items are emitted yet. P13.6 fills in enumeration per category.
///
/// `trigger` distinguishes auto-fire (`.` typed) from explicit ctrl-space.
/// On `.` trigger we'll later restrict to member-access items only; on
/// `Invoked` we offer the broader context-detected set.
pub fn items_at(
    source: &str,
    cursor_byte_offset: usize,
    trigger: Option<CompletionTriggerKind>,
) -> CompletionList {
    let context = detect_context(source, cursor_byte_offset);
    log::debug!("completion: trigger={trigger:?} cursor={cursor_byte_offset} context={context:?}");

    CompletionList { is_incomplete: false, items: vec![] }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(src: &str, marker: &str) -> usize {
        src.find(marker).expect("marker not found") + marker.len()
    }

    #[test]
    fn detect_member_after_dot() {
        let src = "    foo.";
        let cursor = at(src, "foo.");
        assert!(matches!(detect_context(src, cursor), CompletionContextKind::Member { .. }));
    }

    #[test]
    fn detect_member_captures_qualified_base() {
        let src = "    foo.bar.";
        let cursor = at(src, "foo.bar.");
        match detect_context(src, cursor) {
            CompletionContextKind::Member { base_offset, base_end } => {
                assert_eq!(&src[base_offset..base_end], "foo.bar");
            }
            other => panic!("expected Member, got {other:?}"),
        }
    }

    #[test]
    fn detect_member_handles_array_then_dot() {
        let src = "    arr[1].";
        let cursor = at(src, "arr[1].");
        match detect_context(src, cursor) {
            CompletionContextKind::Member { base_offset, base_end } => {
                assert_eq!(&src[base_offset..base_end], "arr[1]");
            }
            other => panic!("expected Member, got {other:?}"),
        }
    }

    #[test]
    fn detect_member_handles_deref() {
        let src = "    THIS^.";
        let cursor = at(src, "THIS^.");
        match detect_context(src, cursor) {
            CompletionContextKind::Member { base_offset, base_end } => {
                assert_eq!(&src[base_offset..base_end], "THIS^");
            }
            other => panic!("expected Member, got {other:?}"),
        }
    }

    #[test]
    fn detect_call_after_open_paren() {
        let src = "    other(";
        let cursor = at(src, "other(");
        match detect_context(src, cursor) {
            CompletionContextKind::Call { operator_offset, operator_end } => {
                assert_eq!(&src[operator_offset..operator_end], "other");
            }
            other => panic!("expected Call, got {other:?}"),
        }
    }

    #[test]
    fn detect_call_after_comma() {
        let src = "    other(x,";
        let cursor = at(src, "other(x,");
        match detect_context(src, cursor) {
            CompletionContextKind::Call { operator_offset, operator_end } => {
                assert_eq!(&src[operator_offset..operator_end], "other");
            }
            other => panic!("expected Call, got {other:?}"),
        }
    }

    #[test]
    fn detect_type_position_after_colon() {
        let src = "VAR x :";
        let cursor = at(src, "VAR x :");
        assert_eq!(detect_context(src, cursor), CompletionContextKind::TypePosition);
    }

    #[test]
    fn detect_expression_after_assignment() {
        let src = "a := ";
        let cursor = at(src, "a := ");
        match detect_context(src, cursor) {
            CompletionContextKind::Expression { .. } => {}
            other => panic!("expected Expression, got {other:?}"),
        }
    }

    #[test]
    fn detect_statement_after_semicolon() {
        let src = "a := 1;";
        let cursor = at(src, "a := 1;");
        assert_eq!(detect_context(src, cursor), CompletionContextKind::Statement);
    }

    #[test]
    fn detect_top_level_at_file_start() {
        let src = "";
        assert_eq!(detect_context(src, 0), CompletionContextKind::TopLevel);
    }

    #[test]
    fn detect_top_level_with_leading_whitespace() {
        let src = "   \n  ";
        assert_eq!(detect_context(src, src.len()), CompletionContextKind::TopLevel);
    }

    #[test]
    fn items_at_returns_empty_list() {
        let src = "foo.";
        let list = items_at(src, src.len(), Some(CompletionTriggerKind::TRIGGER_CHARACTER));
        assert!(list.items.is_empty());
        assert!(!list.is_incomplete);
    }
}
