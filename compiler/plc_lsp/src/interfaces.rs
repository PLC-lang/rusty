//! Interface ↔ implementation linkage queries.
//!
//! H1 split: a method on a FUNCTION_BLOCK or CLASS that implements an
//! interface has *two* meaningful sites — its **definition** (the
//! concrete FB method body) and its **declaration** (the interface's
//! method signature). LSP exposes them via separate requests
//! (`textDocument/definition` vs `textDocument/declaration`); this
//! module supplies the lookups both handlers need.
//!
//! Linkage source of truth: the index already tracks both directions
//! — `PouIndexEntry::FunctionBlock { interfaces: Vec<String>, ... }`
//! lists which interfaces an FB implements, and
//! `InterfaceIndexEntry::get_declared_methods(...)` enumerates the
//! method signatures on the interface.

use plc::index::{Index, PouIndexEntry};
use plc_source::source_location::SourceLocation;

/// For a fully-qualified method name `Container.method`, return the
/// location of the interface's declaration of that method if the
/// container is an FB/Class that implements an interface declaring a
/// method with the same name. Returns `None` for free functions,
/// classes that don't implement any interface, or methods that
/// aren't overrides.
pub fn interface_method_decl_for(index: &Index, qualified_method_name: &str) -> Option<SourceLocation> {
    let (container_name, method_short_name) = qualified_method_name.rsplit_once('.')?;
    let container = index.find_pou(container_name)?;
    let interface_names = match container {
        PouIndexEntry::FunctionBlock { interfaces, .. } | PouIndexEntry::Class { interfaces, .. } => {
            interfaces.iter().map(|s| s.as_str()).collect::<Vec<_>>()
        }
        _ => return None,
    };
    for interface_name in interface_names {
        let Some(interface) = index.get_interfaces().get(interface_name) else {
            continue;
        };
        for declared in interface.get_declared_methods(index) {
            // declared.get_name() returns the qualified `Interface.method`
            // form; split and compare the short name.
            if let Some((_, decl_short)) = declared.get_name().rsplit_once('.') {
                if decl_short.eq_ignore_ascii_case(method_short_name) {
                    return Some(declared.get_location().clone());
                }
            }
        }
    }
    None
}

/// For an interface method `Interface.method`, walk every FB/Class
/// that implements `Interface` and return the locations of their
/// concrete method bodies. Used by `find-references` so the user
/// gets jump targets to every implementing class.
pub fn implementations_of(index: &Index, qualified_method_name: &str) -> Vec<SourceLocation> {
    let Some((interface_name, method_short_name)) = qualified_method_name.rsplit_once('.') else {
        return Vec::new();
    };
    if index.get_interfaces().get(interface_name).is_none() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for pou in index.get_pous().values() {
        let interfaces = match pou {
            PouIndexEntry::FunctionBlock { interfaces, .. } | PouIndexEntry::Class { interfaces, .. } => {
                interfaces
            }
            _ => continue,
        };
        if !interfaces.iter().any(|i| i.eq_ignore_ascii_case(interface_name)) {
            continue;
        }
        // Find this container's method with the matching short name.
        for entry in index.get_pous().values() {
            if let PouIndexEntry::Method { name, parent_name, .. } = entry {
                if parent_name.eq_ignore_ascii_case(pou.get_name()) {
                    if let Some((_, short)) = name.rsplit_once('.') {
                        if short.eq_ignore_ascii_case(method_short_name) {
                            out.push(entry.get_location().clone());
                        }
                    }
                }
            }
        }
    }
    out
}
