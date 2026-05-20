//! Call hierarchy: prepare + incoming + outgoing.
//!
//! Three handlers, all read-only:
//!
//! - **`prepareCallHierarchy`** — position → `Option<CallHierarchyItem>`.
//!   Strict per Q6: returns `None` unless the position resolves to a
//!   callable POU.
//! - **`incomingCalls`** — item → list of callers. Uses
//!   `ReverseIndex.lookup` filtered to `is_call == true`, grouped by
//!   `container_pou`.
//! - **`outgoingCalls`** — item → list of POUs this item calls.
//!   Re-walks the item's `Implementation` body with an
//!   `OutgoingCallCollector` (the reverse index doesn't give us this
//!   direction; the forward walk is cheap per query).
//!
//! Item identity round-trip: `CallHierarchyItem.data` carries the
//! POU's qualified name + declaration location as JSON, so subsequent
//! incoming/outgoing requests don't re-run position lookup.

use lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyItem, CallHierarchyOutgoingCall, PositionEncodingKind, SymbolKind,
};
use plc::index::PouIndexEntry;
use plc::resolver::{AnnotationMap, StatementAnnotation};
use plc_ast::ast::{AstNode, CallStatement};
use plc_ast::visitor::AstVisitor;
use plc_driver::pipelines::AnnotatedProject;
use plc_source::source_location::SourceLocation;
use rustc_hash::FxHashMap;
use serde_json::json;

use crate::diagnostics::{code_span_to_range, path_to_uri};
use crate::position::{ResolvedSymbol, SymbolKind as ResolvedKind};
use crate::reverse_index::ReverseIndex;

// Payload we round-trip through `CallHierarchyItem.data` so the
// client's subsequent `incomingCalls` / `outgoingCalls` requests don't
// have to re-run position lookup. Serialised as an untyped JSON
// object: `SourceLocation`'s `'de: 'static` bound makes a wire struct
// with derived `Deserialize` resist the `DeserializeOwned` requirement
// that `serde_json::from_value` needs. Storing as raw `Value` and
// pulling fields by name sidesteps the lifetime soup.

/// Build a `CallHierarchyItem` for a resolved symbol IF it points at a
/// callable POU. Returns `None` otherwise — caller maps to `null`.
pub fn item_for_symbol(
    annotated: &AnnotatedProject,
    resolved: &ResolvedSymbol,
    encoding: &PositionEncodingKind,
) -> Option<CallHierarchyItem> {
    if !matches!(resolved.kind, ResolvedKind::Pou) {
        return None;
    }
    item_for_qualified_name(annotated, &resolved.qualified_name, encoding)
}

/// Build a `CallHierarchyItem` for a POU by qualified name. Used by
/// `incomingCalls` (to wrap each caller) and `outgoingCalls` (to wrap
/// each callee).
pub fn item_for_qualified_name(
    annotated: &AnnotatedProject,
    qualified_name: &str,
    encoding: &PositionEncodingKind,
) -> Option<CallHierarchyItem> {
    let pou = annotated.index.find_pou(qualified_name)?;
    item_from_pou(pou, encoding)
}

fn item_from_pou(pou: &PouIndexEntry, encoding: &PositionEncodingKind) -> Option<CallHierarchyItem> {
    let decl_location = pou.get_location();
    let path = decl_location.get_file_name()?;
    let uri = path_to_uri(path)?;
    let range = code_span_to_range(decl_location.get_span(), encoding, None)?;
    // We only need the qualified name to round-trip the item. The
    // declaration location is recoverable via `Index::find_pou`
    // on the way back, which sidesteps `SourceLocation`'s
    // `'de: 'static` deserialize bound (incompatible with
    // `serde_json::from_value`).
    let data = json!({ "qualified_name": pou.get_name() });
    Some(CallHierarchyItem {
        name: pou.get_name().to_string(),
        kind: pou_to_lsp_symbol_kind(pou),
        tags: None,
        detail: None,
        uri,
        range,
        selection_range: range,
        data: Some(data),
    })
}

fn pou_to_lsp_symbol_kind(pou: &PouIndexEntry) -> SymbolKind {
    match pou {
        PouIndexEntry::Program { .. } => SymbolKind::FUNCTION,
        PouIndexEntry::Function { .. } => SymbolKind::FUNCTION,
        PouIndexEntry::FunctionBlock { .. } => SymbolKind::CLASS,
        PouIndexEntry::Class { .. } => SymbolKind::CLASS,
        PouIndexEntry::Method { .. } => SymbolKind::METHOD,
        PouIndexEntry::Action { .. } => SymbolKind::FUNCTION,
    }
}

/// Decode the qualified name from the item's data payload. Use it to
/// re-find the POU + its declaration location via `Index::find_pou`.
/// Returns `None` if the payload is missing / malformed.
pub fn decode_item(item: &CallHierarchyItem) -> Option<String> {
    let data = item.data.as_ref()?;
    data.get("qualified_name")?.as_str().map(str::to_string)
}

/// `incomingCalls`: who calls this POU?
///
/// Filter the reverse-index entries for the POU's declaration down to
/// call sites, group them by `container_pou`, and wrap each group as
/// a `CallHierarchyIncomingCall`.
pub fn incoming_calls(
    annotated: &AnnotatedProject,
    reverse_index: &ReverseIndex,
    decl_location: &SourceLocation,
    encoding: &PositionEncodingKind,
) -> Vec<CallHierarchyIncomingCall> {
    let mut grouped: FxHashMap<SourceLocation, Vec<lsp_types::Range>> = FxHashMap::default();

    for entry in reverse_index.lookup(decl_location) {
        if !entry.is_call {
            continue;
        }
        let Some(container) = &entry.container_pou else {
            // Defensive — call sites should always be inside *some*
            // POU body in well-formed ST.
            continue;
        };
        let Some(range) = code_span_to_range(entry.location.get_span(), encoding, None) else {
            continue;
        };
        grouped.entry(container.clone()).or_default().push(range);
    }

    let mut out = Vec::with_capacity(grouped.len());
    for (container_loc, ranges) in grouped {
        // Look up the containing POU. `container_pou` is the POU /
        // implementation's `name_location`.
        if let Some(item) = item_for_container_pou(annotated, &container_loc, encoding) {
            out.push(CallHierarchyIncomingCall { from: item, from_ranges: ranges });
        }
    }
    out
}

/// Find the POU whose `name_location` matches the given location, then
/// wrap it as a `CallHierarchyItem`. Used by incoming calls to wrap
/// each caller.
fn item_for_container_pou(
    annotated: &AnnotatedProject,
    name_location: &SourceLocation,
    encoding: &PositionEncodingKind,
) -> Option<CallHierarchyItem> {
    for unit in annotated.units.iter().map(|au| au.get_unit()) {
        for pou in &unit.pous {
            if &pou.name_location == name_location {
                return item_for_qualified_name(annotated, &pou.name, encoding);
            }
        }
    }
    None
}

/// `outgoingCalls`: what does this POU call?
///
/// Walks the POU's `Implementation` body, collects each
/// `CallStatement.operator` that resolves to a known POU, groups by
/// callee.
pub fn outgoing_calls(
    annotated: &AnnotatedProject,
    qualified_name: &str,
    encoding: &PositionEncodingKind,
) -> Vec<CallHierarchyOutgoingCall> {
    let Some(implementation) = find_implementation(annotated, qualified_name) else {
        return Vec::new();
    };

    let mut collector = OutgoingCallCollector { annotated, callees: FxHashMap::default() };
    for stmt in &implementation.statements {
        collector.visit(stmt);
    }

    let mut out = Vec::with_capacity(collector.callees.len());
    for (callee_qualified, ranges) in collector.callees {
        let Some(item) = item_for_qualified_name(annotated, &callee_qualified, encoding) else {
            continue;
        };
        let from_ranges: Vec<_> =
            ranges.into_iter().filter_map(|loc| code_span_to_range(loc.get_span(), encoding, None)).collect();
        out.push(CallHierarchyOutgoingCall { to: item, from_ranges });
    }
    out
}

fn find_implementation<'a>(
    annotated: &'a AnnotatedProject,
    qualified_name: &str,
) -> Option<&'a plc_ast::ast::Implementation> {
    let needle = qualified_name.to_lowercase();
    for unit in annotated.units.iter().map(|au| au.get_unit()) {
        for impl_ in &unit.implementations {
            if impl_.name.to_lowercase() == needle {
                return Some(impl_);
            }
        }
    }
    None
}

struct OutgoingCallCollector<'a> {
    annotated: &'a AnnotatedProject,
    /// callee qualified name → list of call-site locations (operator
    /// span) within the current implementation body.
    callees: FxHashMap<String, Vec<SourceLocation>>,
}

impl OutgoingCallCollector<'_> {
    fn record_call(&mut self, operator: &AstNode) {
        let annot = self.annotated.annotations.get_with_id(operator.id);
        let callee = match annot {
            Some(StatementAnnotation::Function { qualified_name, call_name, .. }) => {
                call_name.clone().unwrap_or_else(|| qualified_name.clone())
            }
            Some(StatementAnnotation::Program { qualified_name }) => qualified_name.clone(),
            // FB-instance invocation: operator is a Variable whose
            // resulting_type is a POU.
            Some(StatementAnnotation::Variable { resulting_type, .. })
                if self.annotated.index.find_pou(resulting_type).is_some() =>
            {
                resulting_type.clone()
            }
            // SUPER^() resolves through the supertype chain — emerges
            // as a Variable annotation on the dereferenced operator
            // pointing at the parent FB's instance struct. The
            // Variable branch above handles it via resulting_type.
            // THIS^.method() resolves to a Function annotation on the
            // outer ReferenceExpr; the Function branch handles it.
            _ => return,
        };
        self.callees.entry(callee).or_default().push(operator.location.clone());
    }
}

impl AstVisitor for OutgoingCallCollector<'_> {
    fn visit_call_statement(&mut self, stmt: &CallStatement, _node: &AstNode) {
        // Record this call, then descend so nested calls inside the
        // operator subtree or the params list also get recorded
        // through their own `visit_call_statement` dispatch.
        self.record_call(&stmt.operator);
        self.visit(&stmt.operator);
        if let Some(params) = &stmt.parameters {
            self.visit(params);
        }
    }
}
