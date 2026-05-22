//! `textDocument/completion` handler.
//!
//! Detects the user's completion context from the cursor position and emits
//! a `CompletionList`. Context detection walks the `lex_with_trivia` token
//! stream (cached per file on `ServerState`), so comments and arbitrary
//! whitespace between the cursor and the trigger token are transparent.

use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionList, CompletionTriggerKind, InsertTextFormat,
};
use plc::index::{Index, PouIndexEntry, VariableIndexEntry};
use plc::lexer::{LspToken, Token};
use plc::typesystem::DataTypeInformation;
use plc_ast::ast::CompilationUnit;
use plc_driver::pipelines::AnnotatedProject;

use crate::docstring;
use crate::token_walk::TokenWalk;

/// Detected completion context at the user's cursor. Determines which
/// category of items the handler emits and how to rank them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionContextKind {
    /// Cursor immediately after a `.`. `base_text` is the qualified
    /// expression on the LHS of the dot (e.g. `foo`, `foo.bar`,
    /// `arr[1]`), reconstructed from the chain tokens so trivia
    /// between them (comments, whitespace) doesn't leak into the
    /// lookup string the enumerator passes to `Index::find_member`.
    Member { base_text: String },

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

    /// Generic statement position inside a POU body — all in-scope items
    /// (locals, globals, POU names) plus ST statement keywords.
    Statement,
}

/// Detect the completion context at the cursor.
///
/// Routes the entire decision through the `lex_with_trivia` tokens:
/// position lookup is O(log n) binary search; comments and whitespace
/// are skipped transparently. Source bytes are no longer consulted.
pub fn detect_context(tokens: &[LspToken], source: &str, cursor_byte_offset: usize) -> CompletionContextKind {
    let walk = TokenWalk::new(tokens);
    if let Some(member) = detect_member_via_tokens(&walk, source, cursor_byte_offset) {
        return member;
    }
    if let Some(call) = detect_call_via_tokens(&walk, cursor_byte_offset) {
        return call;
    }
    let Some(prev_idx) = walk.prev_real_before(cursor_byte_offset) else {
        return CompletionContextKind::TopLevel;
    };
    match walk.token_kind(prev_idx) {
        // `VAR x :|` — bare colon is type position. The `:=` assignment
        // is its own KeywordAssignment token, so this arm never fires for
        // assignment context.
        Some(Token::KeywordColon) => CompletionContextKind::TypePosition,
        // `a :=|` — cursor follows the assignment operator. Hint
        // refinement happens in `items_at`.
        Some(Token::KeywordAssignment) => CompletionContextKind::Expression { hint_type: None },
        // `a := 1;|` — end of a statement.
        Some(Token::KeywordSemicolon) => CompletionContextKind::Statement,
        _ => CompletionContextKind::Expression { hint_type: None },
    }
}

/// Token-walk path for Call detection. The last real token at-or-before
/// the cursor must be `(` (call site opener) or `,` (next positional or
/// named argument). On match, walk left through balanced parens to find
/// the unmatched `(`, then take the identifier immediately preceding it
/// as the operator.
///
/// Returns `None` when no Identifier sits in front of the opening paren
/// — that's a parenthesised expression (`(x + |`), not a call.
fn detect_call_via_tokens(walk: &TokenWalk, cursor: usize) -> Option<CompletionContextKind> {
    let prev_idx = walk.prev_real_before(cursor)?;
    let trigger = walk.token_kind(prev_idx)?;
    let open_paren_idx = match trigger {
        Token::KeywordParensOpen => prev_idx,
        Token::KeywordComma => find_unmatched_open_paren_via_tokens(walk, prev_idx)?,
        _ => return None,
    };
    let op_idx = walk.prev_real(open_paren_idx)?;
    if !matches!(walk.token_kind(op_idx), Some(Token::Identifier)) {
        return None;
    }
    let op_range = walk.range_at(op_idx)?;
    Some(CompletionContextKind::Call { operator_offset: op_range.start, operator_end: op_range.end })
}

/// Walk LEFT from `start_idx` through balanced parens until we find the
/// unmatched `(`. Used by Call detection when the cursor sits after a
/// comma — the enclosing call's open paren may be several args back.
fn find_unmatched_open_paren_via_tokens(walk: &TokenWalk, start_idx: usize) -> Option<usize> {
    let mut depth: i32 = 0;
    let mut current = start_idx;
    while let Some(prev) = walk.prev_real(current) {
        match walk.token_kind(prev) {
            Some(Token::KeywordParensClose) => depth += 1,
            Some(Token::KeywordParensOpen) => {
                if depth == 0 {
                    return Some(prev);
                }
                depth -= 1;
            }
            _ => {}
        }
        current = prev;
    }
    None
}

/// Token-walk path for Member detection. Returns `Some(Member { ... })`
/// when the last real token at-or-before the cursor is a `.`; otherwise
/// `None`, letting `detect_context` move on to Call or fall through.
fn detect_member_via_tokens(walk: &TokenWalk, source: &str, cursor: usize) -> Option<CompletionContextKind> {
    let prev_idx = walk.prev_real_before(cursor)?;
    if !matches!(walk.token_kind(prev_idx), Some(Token::KeywordDot)) {
        return None;
    }
    let base_text = scan_base_via_tokens(walk, source, prev_idx);
    Some(CompletionContextKind::Member { base_text })
}

/// Walk LEFT from the `.` token, collecting the qualified base expression
/// (`Identifier`, `.`, `^`, balanced `[ ... ]`). Returns the concatenated
/// source slices of the chain tokens — skipping trivia between them so
/// e.g. `foo (* mid *).` resolves to base `foo`, not `foo (* mid *)`.
fn scan_base_via_tokens(walk: &TokenWalk, source: &str, dot_idx: usize) -> String {
    let mut chain: Vec<usize> = Vec::new();
    let mut current = dot_idx;
    let mut bracket_depth: i32 = 0;
    while let Some(prev) = walk.prev_real(current) {
        let kind = walk.token_kind(prev);
        let advance = match kind {
            Some(Token::KeywordSquareParensClose) => {
                bracket_depth += 1;
                true
            }
            Some(Token::KeywordSquareParensOpen) => {
                if bracket_depth == 0 {
                    break;
                }
                bracket_depth -= 1;
                true
            }
            _ if bracket_depth > 0 => true,
            Some(Token::Identifier)
            | Some(Token::KeywordDot)
            | Some(Token::OperatorDeref)
            | Some(Token::KeywordThis)
            | Some(Token::KeywordSuper) => true,
            _ => false,
        };
        if !advance {
            break;
        }
        chain.push(prev);
        current = prev;
    }
    // Reverse to source order, then concatenate each token's source slice.
    chain.reverse();
    let mut out = String::new();
    for idx in chain {
        if let Some(range) = walk.range_at(idx) {
            out.push_str(&source[range.clone()]);
        }
    }
    out
}

/// Refine an `Expression { hint_type: None }` to carry a concrete hint
/// when the token context immediately before the cursor implies one:
///
///   - `[`           → array index slot → `DINT`
///   - `WHILE` / `UNTIL` / `IF` / `ELSIF`   → `BOOL`
///   - `TO` / `BY` inside a `FOR`           → counter's declared type
///
/// Returns `None` when no hint can be inferred (cursor is in a generic
/// expression slot).
fn refine_expression_hint(
    tokens: &[LspToken],
    source: &str,
    cursor: usize,
    enclosing_pou: Option<&str>,
    index: &Index,
) -> Option<String> {
    let walk = TokenWalk::new(tokens);
    let prev_idx = walk.prev_real_before(cursor)?;
    match walk.token_kind(prev_idx)? {
        Token::KeywordSquareParensOpen => Some("DINT".to_string()),
        Token::KeywordWhile | Token::KeywordUntil | Token::KeywordIf | Token::KeywordElseIf => {
            Some("BOOL".to_string())
        }
        Token::KeywordTo | Token::KeywordBy => {
            for_counter_type_via_tokens(&walk, prev_idx, enclosing_pou, index, source).map(String::from)
        }
        _ => None,
    }
}

/// `FOR <counter> := <start> TO <cursor>` — walk LEFT from the TO/BY
/// token to find the `:=`, then read the identifier immediately before
/// the `:=` and the FOR keyword before that as a guard. Returns the
/// counter's declared type from the Index, or `None` if the lookback
/// doesn't match the FOR header shape.
fn for_counter_type_via_tokens<'a>(
    walk: &TokenWalk,
    to_or_by_idx: usize,
    enclosing_pou: Option<&'a str>,
    index: &'a Index,
    source: &str,
) -> Option<&'a str> {
    let mut current = to_or_by_idx;
    while let Some(prev) = walk.prev_real(current) {
        if matches!(walk.token_kind(prev), Some(Token::KeywordAssignment)) {
            let counter_idx = walk.prev_real(prev)?;
            if !matches!(walk.token_kind(counter_idx), Some(Token::Identifier)) {
                return None;
            }
            let for_idx = walk.prev_real(counter_idx)?;
            if !matches!(walk.token_kind(for_idx), Some(Token::KeywordFor)) {
                return None;
            }
            let counter_range = walk.range_at(counter_idx)?;
            let counter_name = &source[counter_range.clone()];
            return resolve_simple_variable_type(counter_name, enclosing_pou, index);
        }
        current = prev;
    }
    None
}

// ============================================================================
// Per-context symbol enumeration. sortText tiers: 0=local, 1=member, 2=global,
// 3=pou, 4=type, 5=keyword. Items whose declared type matches the slot's hint
// get a leading `-` prefix to float to the top within their tier.
// ============================================================================

/// Find the qualified name of the POU whose body contains the given byte
/// offset in the given file. Returns the implementation's `type_name`
/// (e.g. `"main"`, `"FB.foo"`) which is also the key used to look up
/// members in the Index. Returns `None` when the cursor is between POUs
/// or outside any compiled unit.
fn enclosing_pou_qname<'a>(unit: &'a CompilationUnit, file_path: &str, cursor: usize) -> Option<&'a str> {
    for impl_ in &unit.implementations {
        let Some(start) = impl_.location.to_range() else { continue };
        let end = impl_.end_location.to_range().unwrap_or_else(|| start.clone());
        let in_file = impl_.location.get_file_name().map(|f| f == file_path).unwrap_or(false);
        if in_file && cursor >= start.start && cursor <= end.end {
            return Some(impl_.type_name.as_str());
        }
    }
    None
}

/// Build the `sortText` field for a completion item. Tier prefix keeps the
/// 5-category schema visible; the trailing name preserves alphabetic order
/// within the tier. When `type_match` is true the item gets a leading `-`
/// so it sorts ahead of non-matches in the same tier (the type-hint
/// discount from Q6/D18).
fn sort_text(tier: u8, name: &str, type_match: bool) -> String {
    if type_match {
        format!("-{tier}_{name}")
    } else {
        format!("{tier}_{name}")
    }
}

fn make_variable_item(entry: &VariableIndexEntry, tier: u8, hint_type: Option<&str>) -> CompletionItem {
    let type_match = hint_type.map(|h| h == entry.get_type_name()).unwrap_or(false);
    let detail = format!("{} : {}", entry.get_name(), entry.get_type_name());
    let kind = if entry.is_constant() { CompletionItemKind::CONSTANT } else { CompletionItemKind::VARIABLE };
    // Pick the resolve-tag flavour from how the variable lives in the
    // index. Members of a container resolve via `find_member`; globals
    // (no `.` in qname) resolve via `find_global_variable`. We treat
    // both as Variable for docstring widening (no prefix walk).
    let qname = entry.get_qualified_name();
    let tag = docstring::ResolveTag::for_variable(qname);
    CompletionItem {
        label: entry.get_name().to_string(),
        kind: Some(kind),
        detail: Some(detail),
        sort_text: Some(sort_text(tier, entry.get_name(), type_match)),
        filter_text: Some(entry.get_name().to_string()),
        insert_text: Some(entry.get_name().to_string()),
        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
        data: serde_json::to_value(&tag).ok(),
        ..Default::default()
    }
}

fn make_pou_item(entry: &PouIndexEntry, tier: u8, hint_type: Option<&str>) -> CompletionItem {
    let (label, kind, detail, type_for_hint) = match entry {
        PouIndexEntry::Function { name, return_type, .. } => (
            name.clone(),
            CompletionItemKind::FUNCTION,
            format!("FUNCTION {name} : {return_type}"),
            return_type.as_str(),
        ),
        PouIndexEntry::FunctionBlock { name, .. } => {
            (name.clone(), CompletionItemKind::CLASS, format!("FUNCTION_BLOCK {name}"), name.as_str())
        }
        PouIndexEntry::Program { name, .. } => {
            (name.clone(), CompletionItemKind::MODULE, format!("PROGRAM {name}"), name.as_str())
        }
        PouIndexEntry::Class { name, .. } => {
            (name.clone(), CompletionItemKind::CLASS, format!("CLASS {name}"), name.as_str())
        }
        PouIndexEntry::Method { name, return_type, parent_name, .. } => {
            // Methods are indexed as `Parent.method` — strip the parent
            // prefix from the label so completion in `w.|` shows `process`
            // (not `Worker.process`). Keep the qualified name in `detail`
            // so the user can still distinguish overloads / inherited
            // methods at a glance.
            let prefix = format!("{parent_name}.");
            let bare = name.strip_prefix(&prefix).unwrap_or(name).to_string();
            (bare, CompletionItemKind::METHOD, format!("METHOD {name} : {return_type}"), return_type.as_str())
        }
        PouIndexEntry::Action { name, .. } => {
            (name.clone(), CompletionItemKind::METHOD, format!("ACTION {name}"), name.as_str())
        }
    };
    let type_match = hint_type.map(|h| h == type_for_hint).unwrap_or(false);
    // POUs (and methods) are indexed by name; doc resolver looks up via
    // `find_pou(name)` regardless of variant. Methods carry the
    // `Parent.method` qualified name; the label is the bare suffix.
    let tag = docstring::ResolveTag::for_pou(entry.get_name());
    CompletionItem {
        label: label.clone(),
        kind: Some(kind),
        detail: Some(detail),
        sort_text: Some(sort_text(tier, &label, type_match)),
        filter_text: Some(label.clone()),
        insert_text: Some(label),
        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
        data: serde_json::to_value(&tag).ok(),
        ..Default::default()
    }
}

fn make_type_item(name: &str, tier: u8) -> CompletionItem {
    let tag = docstring::ResolveTag::for_type(name);
    CompletionItem {
        label: name.to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        detail: Some(format!("TYPE {name}")),
        sort_text: Some(sort_text(tier, name, false)),
        filter_text: Some(name.to_string()),
        insert_text: Some(name.to_string()),
        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
        data: serde_json::to_value(&tag).ok(),
        ..Default::default()
    }
}

fn make_keyword_item(keyword: &str, tier: u8) -> CompletionItem {
    CompletionItem {
        label: keyword.to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        sort_text: Some(sort_text(tier, keyword, false)),
        filter_text: Some(keyword.to_string()),
        insert_text: Some(keyword.to_string()),
        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
        ..Default::default()
    }
}

/// Resolve a base expression captured by the byte-heuristic detector
/// (`foo`, `foo.bar`, `THIS^`, `arr[1]`, `arr[1].field` …) to the type
/// name whose members we should enumerate. Bare identifiers, simple
/// `.` chains, `THIS^` / `SUPER^`, and arrays-indexed bases are all
/// handled. Member chains traverse via `Index::find_member`; array
/// indexing peels the element type from `DataTypeInformation::Array`.
fn resolve_base_to_type<'a>(
    base_text: &str,
    enclosing_pou: Option<&'a str>,
    index: &'a Index,
) -> Option<&'a str> {
    let base = base_text.trim_end_matches('^');
    if base == "THIS" || base == "SUPER" {
        return enclosing_pou;
    }
    // Peel one head segment at a time. A segment is either an identifier
    // (`foo`) or an identifier followed by `[..]` (`arr[1]`), separated
    // by `.`. We resolve `head` to a type, then walk the remaining
    // segments through `find_member` / array-element peeling.
    let mut segments = split_member_segments(base);
    let head = segments.next()?;
    let mut current_type = resolve_segment(head, enclosing_pou, index)?;
    for segment in segments {
        current_type = step_member(current_type, segment, index)?;
    }
    Some(current_type)
}

/// Split `foo.bar[1].baz` into `["foo", "bar[1]", "baz"]`. Honours
/// `.`-separation outside brackets so bounds expressions like
/// `arr[foo.bar]` don't get split mid-index.
fn split_member_segments(s: &str) -> impl Iterator<Item = &str> + '_ {
    let bytes = s.as_bytes();
    let mut segments = Vec::new();
    let mut start = 0;
    let mut depth: i32 = 0;
    for (i, &c) in bytes.iter().enumerate() {
        match c {
            b'[' => depth += 1,
            b']' => depth -= 1,
            b'.' if depth == 0 => {
                if i > start {
                    segments.push(&s[start..i]);
                }
                start = i + 1;
            }
            _ => {}
        }
    }
    if start < s.len() {
        segments.push(&s[start..]);
    }
    segments.into_iter()
}

/// Resolve the *first* segment of a base expression. Pure identifier →
/// variable lookup (POU member or global). Identifier-with-brackets
/// (`arr[1]`) → look up the array, then peel the element type from its
/// `DataTypeInformation::Array`.
fn resolve_segment<'a>(segment: &str, enclosing_pou: Option<&'a str>, index: &'a Index) -> Option<&'a str> {
    let (name, has_index) = match segment.find('[') {
        Some(i) => (&segment[..i], true),
        None => (segment, false),
    };
    let var_type = resolve_simple_variable_type(name, enclosing_pou, index)?;
    if has_index {
        array_element_type(var_type, index)
    } else {
        Some(var_type)
    }
}

/// Step one member into the chain. Honours `[..]` on the segment so
/// `field[3].next` properly walks to the indexed element's type.
fn step_member<'a>(current_type: &'a str, segment: &str, index: &'a Index) -> Option<&'a str> {
    let (name, has_index) = match segment.find('[') {
        Some(i) => (&segment[..i], true),
        None => (segment, false),
    };
    if name.is_empty() {
        return None;
    }
    let member_type = index.find_member(current_type, name)?.get_type_name();
    if has_index {
        array_element_type(member_type, index)
    } else {
        Some(member_type)
    }
}

/// Peel the element type from an array type. Returns `None` if the
/// type isn't an array (`arr[1].|` only makes sense if `arr` is an
/// `ARRAY[..] OF T`).
fn array_element_type<'a>(type_name: &str, index: &'a Index) -> Option<&'a str> {
    let ty = index.find_type(type_name)?;
    match ty.get_type_information() {
        DataTypeInformation::Array { inner_type_name, .. } => Some(inner_type_name.as_str()),
        _ => None,
    }
}

fn resolve_simple_variable_type<'a>(
    name: &str,
    enclosing_pou: Option<&'a str>,
    index: &'a Index,
) -> Option<&'a str> {
    if let Some(pou) = enclosing_pou {
        if let Some(member) = index.find_member(pou, name) {
            return Some(member.get_type_name());
        }
    }
    index.find_global_variable(name).map(|g| g.get_type_name())
}

// --- Per-context enumerators ---

fn enumerate_member(base_text: &str, enclosing_pou: Option<&str>, index: &Index) -> Vec<CompletionItem> {
    let Some(type_name) = resolve_base_to_type(base_text, enclosing_pou, index) else {
        return vec![];
    };

    let mut items: Vec<CompletionItem> = Vec::new();

    // POU containers (FB / Class / Program) — members via the POU table.
    // Skip lowering-synthesised members (`__vtable`, inheritance back-
    // pointers, etc.) — their `source_location.is_internal()` flags them.
    for entry in index.get_pou_members(type_name) {
        if is_synthetic_variable(entry) {
            continue;
        }
        items.push(make_variable_item(entry, 0, None));
    }

    // User-defined structs — fields off the type information.
    if let Some(ty) = index.find_type(type_name) {
        if let DataTypeInformation::Struct { members, .. } = ty.get_type_information() {
            for m in members {
                if is_synthetic_variable(m) {
                    continue;
                }
                items.push(make_variable_item(m, 0, None));
            }
        }
    }

    // Methods of an FB / Class are POU entries keyed `Parent.name`.
    let prefix = format!("{type_name}.");
    for entry in index.get_pous().values() {
        if let PouIndexEntry::Method { .. } = entry {
            if entry.get_name().starts_with(&prefix) && !is_synthetic_pou(entry) {
                items.push(make_pou_item(entry, 0, None));
            }
        }
    }

    items
}

fn enumerate_call(
    operator: &str,
    enclosing_pou: Option<&str>,
    index: &Index,
    hint_type: Option<&str>,
) -> Vec<CompletionItem> {
    let mut items: Vec<CompletionItem> = Vec::new();

    // Tier 0: callee parameters (named-arg candidates). The label shows
    // the named-arg separator (`:=` for IN/IN_OUT, `=>` for OUTPUT) so
    // the user can see the direction in the completion list before
    // accepting; filterText stays bare so fuzzy matching keys on the
    // parameter name only.
    if let Some(callee) = index.find_pou(operator) {
        for member in index.get_pou_members(callee.get_name()) {
            if is_synthetic_variable(member) {
                continue;
            }
            if member.is_input() || member.is_inout() || member.is_output() {
                let mut item = make_variable_item(member, 0, hint_type);
                let separator = if member.is_output() { "=>" } else { ":=" };
                item.label = format!("{} {separator}", member.get_name());
                item.insert_text = Some(format!("{} {separator} ", member.get_name()));
                items.push(item);
            }
        }
    }
    // Tier 1: in-scope locals (positional-argument candidates).
    if let Some(pou) = enclosing_pou {
        for member in index.get_pou_members(pou) {
            if is_synthetic_variable(member) {
                continue;
            }
            items.push(make_variable_item(member, 1, hint_type));
        }
    }
    // Tier 2: globals.
    for global in index.get_globals().values() {
        if is_synthetic_variable(global) {
            continue;
        }
        items.push(make_variable_item(global, 2, hint_type));
    }

    items
}

fn enumerate_type_position(index: &Index) -> Vec<CompletionItem> {
    let mut items: Vec<CompletionItem> = Vec::new();
    // Iterate values so we read the original-case name off `DataType.name`
    // rather than the lowercased SymbolMap key (which would show `dint` /
    // `point` instead of `DINT` / `Point` to the user).
    for (_key, datatype) in index.get_types().elements() {
        let label = datatype.get_name();
        if is_synthetic_data_type_name(label) {
            continue;
        }
        items.push(make_type_item(label, 0));
    }
    for (_key, datatype) in index.get_pou_types().elements() {
        let label = datatype.get_name();
        if is_synthetic_data_type_name(label) {
            continue;
        }
        items.push(make_type_item(label, 0));
    }
    items
}

/// Identify entities synthesised by lowering. Where possible we use
/// structural markers from the AST/Index rather than name-shape:
///
/// - `is_synthetic_variable` checks `entry.source_location.is_internal()`
///   — vtable members, inheritance back-pointers, and other synthesised
///   variables are created with `SourceLocation::internal()` /
///   `internal_in_unit(...)`, both of which have `span: None` and so
///   report `is_internal() == true`.
///
/// - `is_synthetic_pou` checks `pou.get_location().is_internal()` plus
///   the `is_generated` flag on `Function`. `*__ctor` and `__unit_*__ctor`
///   POUs are created via `new_constructor` / `new_unit_constructor`
///   with `SourceLocation::internal()`.
///
/// - `is_synthetic_data_type_name` is the name-based fallback for
///   types. `DataType` lacks a "compiler-generated" tag the way
///   `UserTypeDeclaration.scope` does in the AST, so completion still
///   has to discriminate `__main_points` / `__vtable_FB` from
///   user-declared types by name. Adding a structural marker to
///   `DataType` is logged as L13 in the phase-13 plan.
fn is_synthetic_variable(entry: &VariableIndexEntry) -> bool {
    entry.source_location.is_internal()
}

fn is_synthetic_pou(pou: &PouIndexEntry) -> bool {
    if pou.get_location().is_internal() {
        return true;
    }
    matches!(pou, PouIndexEntry::Function { is_generated: true, .. })
}

fn is_synthetic_data_type_name(name: &str) -> bool {
    name.starts_with("__") || name.ends_with("__ctor")
}

fn enumerate_expression_or_statement(
    enclosing_pou: Option<&str>,
    index: &Index,
    hint_type: Option<&str>,
    keywords: bool,
) -> Vec<CompletionItem> {
    let mut items: Vec<CompletionItem> = Vec::new();

    if let Some(pou) = enclosing_pou {
        for member in index.get_pou_members(pou) {
            if is_synthetic_variable(member) {
                continue;
            }
            items.push(make_variable_item(member, 0, hint_type));
        }
    }
    for global in index.get_globals().values() {
        if is_synthetic_variable(global) {
            continue;
        }
        items.push(make_variable_item(global, 2, hint_type));
    }
    // POU map can hold multiple entries per qualified name (the user's
    // POU + a lowering-synthesised wrapper share the same key in the
    // SymbolMap). Dedup on qualified name so `main` doesn't surface
    // twice — see L10. Filter methods / synthetic / generated before
    // the dedup so we don't accidentally keep a Method version of a
    // user POU.
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for pou in index.get_pous().values() {
        if matches!(pou, PouIndexEntry::Method { .. }) {
            continue;
        }
        if is_synthetic_pou(pou) {
            continue;
        }
        if !seen.insert(pou.get_name()) {
            continue;
        }
        items.push(make_pou_item(pou, 3, hint_type));
    }
    if keywords {
        for kw in STATEMENT_KEYWORDS {
            items.push(make_keyword_item(kw, 5));
        }
    }

    items
}

fn enumerate_top_level() -> Vec<CompletionItem> {
    TOP_LEVEL_KEYWORDS.iter().map(|kw| make_keyword_item(kw, 0)).collect()
}

const TOP_LEVEL_KEYWORDS: &[&str] = &[
    "PROGRAM",
    "FUNCTION",
    "FUNCTION_BLOCK",
    "CLASS",
    "INTERFACE",
    "TYPE",
    "VAR_GLOBAL",
    "ACTIONS",
    "ACTION",
];

const STATEMENT_KEYWORDS: &[&str] = &[
    "IF",
    "ELSIF",
    "ELSE",
    "THEN",
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
    "TRUE",
    "FALSE",
    "NULL",
    "AND",
    "OR",
    "NOT",
    "XOR",
];

// ============================================================================
// Main entry point
// ============================================================================

/// Build the `CompletionList` for the cursor at `cursor_byte_offset` in
/// `source`. Branches on the detected context category and routes to
/// the matching enumerator. `trigger` is consulted per Q5/D17: on
/// `TriggerCharacter` (the `.` auto-fire) we restrict the output to
/// member-access items so the editor doesn't get a flood of unrelated
/// suggestions immediately after a dot.
///
/// `project` is optional — before the first successful compile only
/// keyword-based enumerators (TopLevel, statement keywords) have data
/// to emit; everything else returns an empty list.
///
/// `tokens` is the cached `lex_with_trivia` output for the current
/// buffer; reused for context detection so completion costs O(log n)
/// position lookup rather than re-lexing per request.
pub fn items_at(
    tokens: &[LspToken],
    source: &str,
    cursor_byte_offset: usize,
    trigger: Option<CompletionTriggerKind>,
    file_path: Option<&str>,
    project: Option<&AnnotatedProject>,
) -> CompletionList {
    let context = detect_context(tokens, source, cursor_byte_offset);
    let is_dot_trigger = matches!(trigger, Some(CompletionTriggerKind::TRIGGER_CHARACTER));
    log::debug!("completion: trigger={trigger:?} cursor={cursor_byte_offset} context={context:?}");

    let enclosing_pou = match (project, file_path) {
        (Some(p), Some(path)) => {
            p.units.iter().find_map(|u| enclosing_pou_qname(u.get_unit(), path, cursor_byte_offset))
        }
        _ => None,
    };

    let items: Vec<CompletionItem> = match (&context, is_dot_trigger, project) {
        (CompletionContextKind::Member { base_text }, _, Some(p)) => {
            enumerate_member(base_text.as_str(), enclosing_pou, &p.index)
        }
        // Dot-trigger fired but no project yet → empty list (suppresses
        // the rest of the routing).
        (_, true, _) => vec![],

        (CompletionContextKind::Call { operator_offset, operator_end }, false, Some(p)) => {
            let operator = &source[*operator_offset..*operator_end];
            enumerate_call(operator, enclosing_pou, &p.index, None)
        }
        (CompletionContextKind::TypePosition, false, Some(p)) => enumerate_type_position(&p.index),
        (CompletionContextKind::Expression { hint_type }, false, Some(p)) => {
            // hint_type is `None` from the detector; refine via byte
            // patterns (WHILE → BOOL, `[` → DINT, FOR counter lookup …).
            let refined_owned = hint_type.clone().or_else(|| {
                refine_expression_hint(tokens, source, cursor_byte_offset, enclosing_pou, &p.index)
            });
            enumerate_expression_or_statement(enclosing_pou, &p.index, refined_owned.as_deref(), false)
        }
        (CompletionContextKind::Statement, false, Some(p)) => {
            enumerate_expression_or_statement(enclosing_pou, &p.index, None, true)
        }
        (CompletionContextKind::TopLevel, false, _) => enumerate_top_level(),

        // No project attached + non-keyword context → empty list.
        _ => vec![],
    };

    CompletionList { is_incomplete: false, items }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plc::lexer::lex_with_trivia;

    fn at(src: &str, marker: &str) -> usize {
        src.find(marker).expect("marker not found") + marker.len()
    }

    /// Convenience: lex the source and call `detect_context`. Lets tests
    /// stay focused on the context-detection contract without restating
    /// the cache integration on every call.
    fn ctx(src: &str, cursor: usize) -> CompletionContextKind {
        let tokens = lex_with_trivia(src);
        detect_context(&tokens, src, cursor)
    }

    #[test]
    fn detect_member_after_dot() {
        let src = "    foo.";
        let cursor = at(src, "foo.");
        assert!(matches!(ctx(src, cursor), CompletionContextKind::Member { .. }));
    }

    #[test]
    fn detect_member_captures_qualified_base() {
        let src = "    foo.bar.";
        let cursor = at(src, "foo.bar.");
        match ctx(src, cursor) {
            CompletionContextKind::Member { base_text } => {
                assert_eq!(base_text, "foo.bar");
            }
            other => panic!("expected Member, got {other:?}"),
        }
    }

    #[test]
    fn detect_member_handles_array_then_dot() {
        let src = "    arr[1].";
        let cursor = at(src, "arr[1].");
        match ctx(src, cursor) {
            CompletionContextKind::Member { base_text } => {
                assert_eq!(base_text, "arr[1]");
            }
            other => panic!("expected Member, got {other:?}"),
        }
    }

    #[test]
    fn detect_member_handles_deref() {
        let src = "    THIS^.";
        let cursor = at(src, "THIS^.");
        match ctx(src, cursor) {
            CompletionContextKind::Member { base_text } => {
                assert_eq!(base_text, "THIS^");
            }
            other => panic!("expected Member, got {other:?}"),
        }
    }

    #[test]
    fn detect_member_after_comment_between_base_and_dot() {
        // Comment between the base and the dot must neither hide the dot
        // (Member still classifies) NOR leak into the base text the
        // enumerator looks up. base_text comes from token concatenation
        // so trivia is excluded automatically.
        let src = "    foo (* mid *).";
        let cursor = at(src, "foo (* mid *).");
        match ctx(src, cursor) {
            CompletionContextKind::Member { base_text } => {
                assert_eq!(base_text, "foo", "comment leaked into base_text");
            }
            other => panic!("expected Member, got {other:?}"),
        }
    }

    #[test]
    fn detect_call_after_open_paren() {
        let src = "    other(";
        let cursor = at(src, "other(");
        match ctx(src, cursor) {
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
        match ctx(src, cursor) {
            CompletionContextKind::Call { operator_offset, operator_end } => {
                assert_eq!(&src[operator_offset..operator_end], "other");
            }
            other => panic!("expected Call, got {other:?}"),
        }
    }

    #[test]
    fn detect_call_through_inline_comment() {
        // `other(/* doc */ x,|` — byte-walk used to land on `*`/`/` and lose
        // track of the call. TokenWalk skips the BlockComment trivia.
        let src = "    other(/* arg1 */ x,";
        let cursor = at(src, "other(/* arg1 */ x,");
        match ctx(src, cursor) {
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
        assert_eq!(ctx(src, cursor), CompletionContextKind::TypePosition);
    }

    #[test]
    fn detect_expression_after_assignment() {
        let src = "a := ";
        let cursor = at(src, "a := ");
        match ctx(src, cursor) {
            CompletionContextKind::Expression { .. } => {}
            other => panic!("expected Expression, got {other:?}"),
        }
    }

    #[test]
    fn detect_statement_after_semicolon() {
        let src = "a := 1;";
        let cursor = at(src, "a := 1;");
        assert_eq!(ctx(src, cursor), CompletionContextKind::Statement);
    }

    #[test]
    fn detect_statement_through_trailing_comment() {
        // `a := 1; // note` — byte-walk used to land on `e` and pick
        // Expression. TokenWalk skips the LineComment and lands on `;`.
        let src = "a := 1; // note\n";
        let cursor = src.len();
        assert_eq!(ctx(src, cursor), CompletionContextKind::Statement);
    }

    #[test]
    fn detect_type_position_through_block_comment() {
        // `VAR x : (* hint *)|` — cursor after a comment that follows the
        // colon. Token-walk still sees the bare `:` as the last real token.
        let src = "VAR x : (* hint *)";
        let cursor = src.len();
        assert_eq!(ctx(src, cursor), CompletionContextKind::TypePosition);
    }

    #[test]
    fn detect_top_level_at_file_start() {
        let src = "";
        assert_eq!(ctx(src, 0), CompletionContextKind::TopLevel);
    }

    #[test]
    fn detect_top_level_with_leading_whitespace() {
        let src = "   \n  ";
        assert_eq!(ctx(src, src.len()), CompletionContextKind::TopLevel);
    }

    #[test]
    fn items_at_with_no_project_returns_top_level_keywords_only() {
        // No `project` attached — items_at should still emit the top-level
        // keyword list when the cursor is at file scope, because that
        // enumerator doesn't depend on the Index.
        let src = "";
        let tokens = lex_with_trivia(src);
        let list = items_at(&tokens, src, 0, None, None, None);
        assert!(!list.is_incomplete);
        assert!(!list.items.is_empty(), "top-level keywords should surface without a project");
        let labels: Vec<&str> = list.items.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"PROGRAM"));
        assert!(labels.contains(&"FUNCTION"));
        assert!(labels.contains(&"TYPE"));
    }

    #[test]
    fn items_at_dot_trigger_without_project_returns_empty() {
        // Dot-trigger with no project → no member info available → empty
        // list. We don't fall back to keyword spam after a `.`.
        let src = "foo.";
        let tokens = lex_with_trivia(src);
        let list =
            items_at(&tokens, src, src.len(), Some(CompletionTriggerKind::TRIGGER_CHARACTER), None, None);
        assert!(list.items.is_empty());
    }
}
