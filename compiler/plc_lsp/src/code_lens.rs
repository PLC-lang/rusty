//! `textDocument/codeLens` — interface implementation lenses (H3).
//!
//! Two flavours, both rendered as a clickable annotation above the
//! declaration line:
//!
//!   - Above an INTERFACE method:  `N implementations`
//!     Clicking jumps via the editor's `editor.action.showReferences`
//!     command to a quickpick of the FB/Class method bodies that
//!     implement it.
//!
//!   - Above an FB/Class METHOD that implements an interface:
//!     `implements Interface.method`
//!     Clicking jumps to the interface's method signature via the
//!     same goto-style command.
//!
//! No per-extension configurability: vscode (and helix) gate
//! codelenses with a master "show/hide all codelenses" toggle, which
//! is the conventional escape hatch. If a user wants ours off, they
//! flip the editor-wide setting.

use std::path::Path;

use lsp_types::{CodeLens, Command, PositionEncodingKind};
use plc::index::PouIndexEntry;
use plc_driver::pipelines::AnnotatedProject;
use plc_source::source_location::SourceLocation;

use crate::diagnostics::{code_span_to_range, path_to_uri};
use crate::interfaces;

/// Compute the code lenses for `path` against the cached project.
pub fn code_lenses_for_file(
    annotated: &AnnotatedProject,
    path: &Path,
    encoding: &PositionEncodingKind,
) -> Vec<CodeLens> {
    let mut out: Vec<CodeLens> = Vec::new();
    let path_str = path.to_string_lossy();

    for pou in annotated.index.get_pous().values() {
        let Some(method) = method_info(pou) else { continue };
        let location = pou.get_location();
        let Some(file) = location.get_file_name() else { continue };
        if file != path_str.as_ref() && !path_str.ends_with(file) {
            continue;
        }
        let parent = method.parent_name;
        let parent_pou = annotated.index.find_pou(parent);

        // Case 1: the container is an INTERFACE → emit "N implementations".
        if annotated.index.get_interfaces().get(parent).is_some() {
            let impls = interfaces::implementations_of(&annotated.index, pou.get_name());
            if !impls.is_empty() {
                if let Some(lens) = make_n_implementations_lens(pou, &impls, encoding) {
                    out.push(lens);
                }
            }
            continue;
        }

        // Case 2: the container is an FB or Class implementing some
        // interface → emit "implements Interface.method".
        let impls_iface = parent_pou.and_then(|p| match p {
            PouIndexEntry::FunctionBlock { interfaces, .. } | PouIndexEntry::Class { interfaces, .. } => {
                Some(interfaces.as_slice())
            }
            _ => None,
        });
        let Some(impls_iface) = impls_iface else { continue };
        if impls_iface.is_empty() {
            continue;
        }
        if let Some(decl_loc) = interfaces::interface_method_decl_for(&annotated.index, pou.get_name()) {
            if let Some(lens) = make_implements_lens(pou, &decl_loc, encoding) {
                out.push(lens);
            }
        }
    }

    out
}

struct MethodInfo<'a> {
    parent_name: &'a str,
}

fn method_info(pou: &PouIndexEntry) -> Option<MethodInfo<'_>> {
    match pou {
        PouIndexEntry::Method { parent_name, .. } => Some(MethodInfo { parent_name }),
        _ => None,
    }
}

fn make_n_implementations_lens(
    pou: &PouIndexEntry,
    impls: &[SourceLocation],
    encoding: &PositionEncodingKind,
) -> Option<CodeLens> {
    let range = code_span_to_range(pou.get_location().get_span(), encoding, None)?;
    let path = pou.get_location().get_file_name()?;
    let title = format!("{} implementations", impls.len());
    // `editor.action.showReferences` is vscode's built-in command for
    // popping the references quickpick — same one that goto-references
    // uses. Other editors recognise it too.
    let uri = path_to_uri(path)?;
    let positions: Vec<lsp_types::Location> = impls
        .iter()
        .filter_map(|loc| {
            let r = code_span_to_range(loc.get_span(), encoding, None)?;
            let p = loc.get_file_name()?;
            Some(lsp_types::Location { uri: path_to_uri(p)?, range: r })
        })
        .collect();
    Some(CodeLens {
        range,
        command: Some(Command {
            title,
            command: "editor.action.showReferences".to_string(),
            arguments: Some(vec![
                serde_json::to_value(uri).ok()?,
                serde_json::to_value(range.start).ok()?,
                serde_json::to_value(positions).ok()?,
            ]),
        }),
        data: None,
    })
}

fn make_implements_lens(
    pou: &PouIndexEntry,
    decl_loc: &SourceLocation,
    encoding: &PositionEncodingKind,
) -> Option<CodeLens> {
    let range = code_span_to_range(pou.get_location().get_span(), encoding, None)?;
    let decl_range = code_span_to_range(decl_loc.get_span(), encoding, None)?;
    let decl_path = decl_loc.get_file_name()?;
    let decl_uri = path_to_uri(decl_path)?;
    // The "implements Interface.method" lens uses the same
    // showReferences command but with a single target — clicking
    // jumps straight to the interface method.
    let title = {
        // Pull the qualified name from the decl_loc isn't directly
        // possible; reconstruct as best-effort from the FB method's
        // qualified name. e.g. Worker.aaa implementing I_Worker.aaa.
        let qualified = pou.get_name();
        match qualified.rsplit_once('.') {
            Some((_, short)) => format!("implements {short}"),
            None => "implements".to_string(),
        }
    };
    let self_uri = path_to_uri(pou.get_location().get_file_name()?)?;
    Some(CodeLens {
        range,
        command: Some(Command {
            title,
            command: "editor.action.showReferences".to_string(),
            arguments: Some(vec![
                serde_json::to_value(self_uri).ok()?,
                serde_json::to_value(range.start).ok()?,
                serde_json::to_value(vec![lsp_types::Location { uri: decl_uri, range: decl_range }]).ok()?,
            ]),
        }),
        data: None,
    })
}
