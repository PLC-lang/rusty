//! Hover content formatter — converts a `SymbolUnderCursor` into the
//! markdown body returned by `textDocument/hover`.
//!
//! Strategy: ask the `Index` for the resolved declaration's type / POU
//! signature / type definition and build a compact display from that.
//! Index-driven rather than source-sliced so hover works even when the
//! declaration's file isn't currently open in the editor.
//!
//! Trade-off flagged for post-phase-13:
//!
//! - The walk can be a longer chain than the prototype currently
//!   follows — e.g. a `Variable` of an array type whose element type
//!   is itself a struct whose fields each have their own types. We
//!   surface only the immediate type name here. A richer hover that
//!   follows the chain (and caches per-decl strings) is a follow-up.
//! - Struct / enum bodies are surfaced compactly; ARRAY / SUBRANGE /
//!   POINTER / STRING shapes show just the type name. Refining those
//!   is also post-phase-13.

use plc::index::{Index, PouIndexEntry, VariableIndexEntry, VariableType};
use plc::typesystem::DataTypeInformation;

use crate::position::{ResolvedSymbol, SymbolKind, SymbolUnderCursor};

/// Build the hover markdown body for what the cursor points at.
/// Returns `None` when there's nothing to show.
pub fn format_symbol(symbol: &SymbolUnderCursor, index: &Index) -> Option<String> {
    let resolved = symbol.resolved.as_ref()?;
    if resolved.declaration_location.is_undefined() {
        return None;
    }
    if matches!(resolved.declaration_location.get_file_name(), Some(name) if name == "<internal>") {
        return None;
    }

    let body = match resolved.kind {
        SymbolKind::Variable | SymbolKind::Argument => format_variable(resolved, index, true),
        // Struct fields / FB members: index stores them with
        // `VariableType::Input` to keep type-checking uniform with
        // POU parameters. For hover we'd rather not lie about the
        // section; suppress it for members.
        SymbolKind::Member => format_variable(resolved, index, false),
        SymbolKind::Pou => format_pou(resolved, index),
        SymbolKind::Type => format_type(resolved, index),
    }?;

    Some(format!("```st\n{body}\n```"))
}

fn format_variable(resolved: &ResolvedSymbol, index: &Index, show_section: bool) -> Option<String> {
    let entry = lookup_variable(index, &resolved.qualified_name)?;
    let type_name = effective_type_name(entry.get_type_name(), index);
    if show_section {
        let section = section_keyword(entry);
        Some(format!("{section} {} : {type_name}", entry.get_name()))
    } else {
        Some(format!("{} : {type_name}", entry.get_name()))
    }
}

/// Render `FUNCTION foo : DINT VAR_INPUT x : DINT END_VAR END_FUNCTION`
/// from the POU entry + its parameter list.
fn format_pou(resolved: &ResolvedSymbol, index: &Index) -> Option<String> {
    let pou = index.find_pou(&resolved.qualified_name)?;
    let kind = pou_keyword(pou);

    let mut out = String::new();
    out.push_str(kind);
    out.push(' ');
    out.push_str(pou.get_name());
    if let Some(return_type) = pou.get_return_type() {
        out.push_str(" : ");
        out.push_str(&effective_type_name(return_type, index));
    }

    // Group parameters by section so the signature reads naturally.
    let params = index.get_available_parameters(pou.get_name());
    if !params.is_empty() {
        let mut last_section: Option<&'static str> = None;
        for entry in params {
            let section = section_keyword(entry);
            if last_section != Some(section) {
                if last_section.is_some() {
                    out.push_str("\n    END_VAR");
                }
                out.push_str("\n    ");
                out.push_str(section);
                last_section = Some(section);
            }
            let type_name = effective_type_name(entry.get_type_name(), index);
            out.push_str(&format!("\n        {} : {type_name};", entry.get_name()));
        }
        if last_section.is_some() {
            out.push_str("\n    END_VAR");
        }
    }
    out.push('\n');
    out.push_str("END_");
    out.push_str(kind);
    Some(out)
}

/// Render `TYPE myType : STRUCT … END_STRUCT END_TYPE` from the
/// `DataType`. Structs and enums surface their bodies; other shapes
/// show the type name and let goto-def carry the user to the source.
fn format_type(resolved: &ResolvedSymbol, index: &Index) -> Option<String> {
    let ty = index
        .find_type(&resolved.qualified_name)
        .or_else(|| index.find_pou_type(&resolved.qualified_name))?;
    let detail = match ty.get_type_information() {
        DataTypeInformation::Struct { members, .. } => {
            let mut s = String::from("STRUCT");
            for m in members {
                let type_name = effective_type_name(m.get_type_name(), index);
                s.push_str(&format!("\n    {} : {type_name};", m.get_name()));
            }
            s.push_str("\nEND_STRUCT");
            s
        }
        DataTypeInformation::Enum { variants, .. } => {
            let names: Vec<&str> = variants.iter().map(|v| v.get_name()).collect();
            format!("({})", names.join(", "))
        }
        DataTypeInformation::Alias { referenced_type, .. } => referenced_type.to_string(),
        _ => return Some(format!("TYPE {}\nEND_TYPE", ty.get_name())),
    };
    Some(format!("TYPE {} : {detail}\nEND_TYPE", ty.get_name()))
}

fn lookup_variable<'a>(index: &'a Index, qualified_name: &str) -> Option<&'a VariableIndexEntry> {
    match qualified_name.rsplit_once('.') {
        Some((container, member)) => index.find_member(container, member),
        None => index.find_global_variable(qualified_name),
    }
}

fn section_keyword(entry: &VariableIndexEntry) -> &'static str {
    match entry.get_variable_type() {
        VariableType::Input => "VAR_INPUT",
        VariableType::Output => "VAR_OUTPUT",
        VariableType::InOut => "VAR_IN_OUT",
        VariableType::Local => "VAR",
        VariableType::Temp => "VAR_TEMP",
        VariableType::Global => "VAR_GLOBAL",
        VariableType::External => "VAR_EXTERNAL",
        VariableType::Return => "(return)",
        VariableType::Property => "PROPERTY",
    }
}

fn pou_keyword(pou: &PouIndexEntry) -> &'static str {
    match pou {
        PouIndexEntry::Program { .. } => "PROGRAM",
        PouIndexEntry::Function { .. } => "FUNCTION",
        PouIndexEntry::FunctionBlock { .. } => "FUNCTION_BLOCK",
        PouIndexEntry::Class { .. } => "CLASS",
        PouIndexEntry::Method { .. } => "METHOD",
        PouIndexEntry::Action { .. } => "ACTION",
    }
}

/// Strip preprocessor-inferred literal types (`__INT_LITERAL_TYPE`,
/// `__STRING_TYPE_n`, …) by chasing the alias chain to the user-facing
/// canonical type name.
fn effective_type_name(name: &str, index: &Index) -> String {
    index
        .find_effective_type_by_name(name)
        .map(|t| t.get_name().to_string())
        .unwrap_or_else(|| name.to_string())
}
