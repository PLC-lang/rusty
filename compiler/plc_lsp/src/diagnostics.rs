//! Map `ResolvedDiagnostics` from the compile worker into
//! `lsp_types::Diagnostic`s grouped per URI for `publishDiagnostics`.
//!
//! Filters out `<internal>` diagnostics (Q16: compiler-implementation
//! artifacts the user can't act on; logged for telemetry) and any
//! diagnostic whose `CodeSpan` isn't a real text range
//! (`Block`/`Combined`/`None` come from FBD/CFC or synthetic nodes —
//! out of scope here).

// See top of `lib.rs` for the rationale.
#![allow(clippy::mutable_key_type)]

use std::collections::HashMap;

use lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString, Position, Range,
    Uri,
};
use plc_diagnostics::diagnostics::Severity;
use plc_diagnostics::reporter::{ResolvedDiagnostics, ResolvedLocation};
use plc_source::source_location::CodeSpan;
use rustc_hash::FxHashMap;

pub const DIAGNOSTIC_SOURCE: &str = "plc";
const INTERNAL_FILENAME: &str = "<internal>";

pub fn map_collected(
    collected: Vec<ResolvedDiagnostics>,
    file_paths: &FxHashMap<usize, String>,
) -> HashMap<Uri, Vec<Diagnostic>> {
    let mut grouped: HashMap<Uri, Vec<Diagnostic>> = HashMap::new();

    for diag in collected {
        let Some(severity) = map_severity(diag.severity) else {
            continue; // Ignore severity → never publish
        };

        let path = file_paths.get(&diag.main_location.file_handle).map(String::as_str);
        let Some(path) = path else {
            log::warn!(
                "diagnostic with unknown file_handle {}: dropping ({})",
                diag.main_location.file_handle,
                diag.code
            );
            continue;
        };
        if path == INTERNAL_FILENAME {
            log::warn!("internal-source diagnostic dropped: code={} message={:?}", diag.code, diag.message);
            continue;
        }
        let Some(range) = code_span_to_range(&diag.main_location.span) else {
            log::warn!("diagnostic without a text range dropped: code={} file={}", diag.code, path);
            continue;
        };
        let Some(uri) = path_to_uri(path) else {
            log::warn!("could not build a URI from {path:?}; dropping diagnostic {}", diag.code);
            continue;
        };

        let related = diag.additional_locations.map(|locs| {
            locs.into_iter().filter_map(|loc| related_info(&loc, file_paths)).collect::<Vec<_>>()
        });

        let lsp_diag = Diagnostic {
            range,
            severity: Some(severity),
            code: Some(NumberOrString::String(diag.code)),
            code_description: None,
            source: Some(DIAGNOSTIC_SOURCE.to_string()),
            message: diag.message,
            related_information: related.filter(|v| !v.is_empty()),
            tags: None,
            data: None,
        };
        grouped.entry(uri).or_default().push(lsp_diag);
    }

    grouped
}

fn map_severity(sev: Severity) -> Option<DiagnosticSeverity> {
    match sev {
        Severity::Error => Some(DiagnosticSeverity::ERROR),
        Severity::Warning => Some(DiagnosticSeverity::WARNING),
        Severity::Info => Some(DiagnosticSeverity::INFORMATION),
        Severity::Ignore => None,
    }
}

fn code_span_to_range(span: &CodeSpan) -> Option<Range> {
    let CodeSpan::Range(_) = span else {
        return None;
    };
    Some(Range {
        start: Position { line: span.get_line() as u32, character: span.get_column() as u32 },
        end: Position { line: span.get_line_end() as u32, character: span.get_column_end() as u32 },
    })
}

fn related_info(
    loc: &ResolvedLocation,
    file_paths: &FxHashMap<usize, String>,
) -> Option<DiagnosticRelatedInformation> {
    let path = file_paths.get(&loc.file_handle).map(String::as_str)?;
    if path == INTERNAL_FILENAME {
        return None;
    }
    let range = code_span_to_range(&loc.span)?;
    let uri = path_to_uri(path)?;
    Some(DiagnosticRelatedInformation {
        location: Location { uri, range },
        // Per Q decision, related-info message is empty; the location
        // does the talking.
        message: String::new(),
    })
}

/// Build an `lsp_types::Uri` from a file path string. The diagnostician
/// stores paths as `String`; we delegate to `project::path_to_file_uri`
/// (backed by the `url` crate) which handles Linux, Windows drive
/// letters, and UNC paths uniformly.
fn path_to_uri(path: &str) -> Option<Uri> {
    crate::project::path_to_file_uri(std::path::Path::new(path))
}

#[cfg(test)]
mod tests {
    use plc_source::source_location::TextLocation;

    use super::*;

    fn handle_paths(pairs: &[(usize, &str)]) -> FxHashMap<usize, String> {
        pairs.iter().map(|(h, p)| (*h, p.to_string())).collect()
    }

    fn diag(
        code: &str,
        severity: Severity,
        handle: usize,
        line: usize,
        col_start: usize,
        col_end: usize,
    ) -> ResolvedDiagnostics {
        let start = TextLocation::new(line, col_start, 0);
        let end = TextLocation::new(line, col_end, 0);
        ResolvedDiagnostics {
            code: code.to_string(),
            message: format!("{code} message"),
            severity,
            main_location: ResolvedLocation {
                file_handle: handle,
                span: CodeSpan::from_text_info(start, end),
            },
            additional_locations: None,
        }
    }

    #[test]
    fn groups_per_uri() {
        let paths = handle_paths(&[(1, "/a.st"), (2, "/b.st")]);
        let diags = vec![
            diag("E001", Severity::Error, 1, 0, 0, 5),
            diag("E002", Severity::Error, 2, 1, 2, 4),
            diag("E003", Severity::Warning, 1, 3, 0, 1),
        ];
        let grouped = map_collected(diags, &paths);

        let a = grouped.get(&path_to_uri("/a.st").unwrap()).expect("URI a");
        let b = grouped.get(&path_to_uri("/b.st").unwrap()).expect("URI b");
        assert_eq!(a.len(), 2);
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn maps_severity_filtering_ignore() {
        let paths = handle_paths(&[(1, "/a.st")]);
        let diags = vec![
            diag("E001", Severity::Error, 1, 0, 0, 5),
            diag("W001", Severity::Warning, 1, 1, 0, 5),
            diag("I001", Severity::Info, 1, 2, 0, 5),
            diag("X001", Severity::Ignore, 1, 3, 0, 5), // filtered out
        ];
        let grouped = map_collected(diags, &paths);
        let entry = grouped.get(&path_to_uri("/a.st").unwrap()).unwrap();
        assert_eq!(entry.len(), 3);
        assert_eq!(entry[0].severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(entry[1].severity, Some(DiagnosticSeverity::WARNING));
        assert_eq!(entry[2].severity, Some(DiagnosticSeverity::INFORMATION));
    }

    #[test]
    fn drops_internal_diagnostics() {
        let paths = handle_paths(&[(1, INTERNAL_FILENAME)]);
        let diags = vec![diag("E001", Severity::Error, 1, 0, 0, 5)];
        let grouped = map_collected(diags, &paths);
        assert!(grouped.is_empty());
    }

    #[test]
    fn drops_diagnostics_with_unknown_file_handle() {
        let paths = handle_paths(&[(1, "/a.st")]);
        let diags = vec![diag("E001", Severity::Error, 99, 0, 0, 5)]; // handle 99 not in map
        let grouped = map_collected(diags, &paths);
        assert!(grouped.is_empty());
    }

    #[test]
    fn drops_diagnostics_with_no_text_range() {
        let paths = handle_paths(&[(1, "/a.st")]);
        let mut d = diag("E001", Severity::Error, 1, 0, 0, 5);
        d.main_location.span = CodeSpan::None;
        let grouped = map_collected(vec![d], &paths);
        assert!(grouped.is_empty());
    }

    #[test]
    fn fields_match_phase_4_decisions() {
        let paths = handle_paths(&[(1, "/a.st")]);
        let d = diag("E033", Severity::Error, 1, 2, 4, 8);
        let grouped = map_collected(vec![d], &paths);
        let mut entries: Vec<_> = grouped.into_iter().collect();
        let (_uri, mut diags) = entries.pop().unwrap();
        let diag = diags.pop().unwrap();
        assert_eq!(diag.source.as_deref(), Some("plc"));
        assert_eq!(diag.code, Some(NumberOrString::String("E033".to_string())));
        assert_eq!(diag.range.start, Position { line: 2, character: 4 });
        assert_eq!(diag.range.end, Position { line: 2, character: 8 });
        assert!(diag.related_information.is_none()); // none provided
    }
}
