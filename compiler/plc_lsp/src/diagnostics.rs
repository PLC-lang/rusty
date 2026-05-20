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
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString, Position,
    PositionEncodingKind, Range, Uri,
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
    encoding: &PositionEncodingKind,
    source_contents: &HashMap<String, String>,
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
        let main_source = source_contents.get(path).map(String::as_str);
        let Some(range) = code_span_to_range(&diag.main_location.span, encoding, main_source) else {
            log::warn!("diagnostic without a text range dropped: code={} file={}", diag.code, path);
            continue;
        };
        let Some(uri) = path_to_uri(path) else {
            log::warn!("could not build a URI from {path:?}; dropping diagnostic {}", diag.code);
            continue;
        };

        let related = diag.additional_locations.map(|locs| {
            locs.into_iter()
                .filter_map(|loc| related_info(&loc, file_paths, encoding, source_contents))
                .collect::<Vec<_>>()
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

/// Build an LSP `Range` from a `CodeSpan`, applying the utf-16 column
/// conversion when the negotiated encoding requires it. `source` is the
/// source content of the file the span refers to — only consulted on the
/// utf-16 path. When `source` is None on the utf-16 path (file content
/// wasn't available, e.g. read failure), the raw byte offsets are used
/// as a best-effort fallback — slightly off for non-ASCII content but
/// better than dropping the diagnostic.
pub(crate) fn code_span_to_range(
    span: &CodeSpan,
    encoding: &PositionEncodingKind,
    source: Option<&str>,
) -> Option<Range> {
    let CodeSpan::Range(_) = span else {
        return None;
    };
    let convert_col = |line: usize, byte_col: usize| -> u32 {
        if encoding == &PositionEncodingKind::UTF16 {
            source
                .and_then(|s| s.lines().nth(line))
                .map(|l| byte_offset_to_utf16_units(l, byte_col))
                .unwrap_or(byte_col as u32)
        } else {
            byte_col as u32
        }
    };

    Some(Range {
        start: Position {
            line: span.get_line() as u32,
            character: convert_col(span.get_line(), span.get_column()),
        },
        end: Position {
            line: span.get_line_end() as u32,
            character: convert_col(span.get_line_end(), span.get_column_end()),
        },
    })
}

/// Count utf-16 code units in the byte prefix `line[..byte_offset]`.
/// Used to convert rusty's byte-offset columns into LSP's utf-16
/// `character` positions when that encoding was negotiated. See D4.
fn byte_offset_to_utf16_units(line: &str, byte_offset: usize) -> u32 {
    line.get(..byte_offset)
        .map(|prefix| prefix.encode_utf16().count() as u32)
        // Byte offset not on a char boundary — shouldn't happen with
        // positions emitted by the rusty parser. Fall back to the raw
        // byte offset (correct for ASCII, off by a small amount for
        // non-ASCII).
        .unwrap_or(byte_offset as u32)
}

fn related_info(
    loc: &ResolvedLocation,
    file_paths: &FxHashMap<usize, String>,
    encoding: &PositionEncodingKind,
    source_contents: &HashMap<String, String>,
) -> Option<DiagnosticRelatedInformation> {
    let path = file_paths.get(&loc.file_handle).map(String::as_str)?;
    if path == INTERNAL_FILENAME {
        return None;
    }
    let source = source_contents.get(path).map(String::as_str);
    let range = code_span_to_range(&loc.span, encoding, source)?;
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
pub(crate) fn path_to_uri(path: &str) -> Option<Uri> {
    crate::project::path_to_file_uri(std::path::Path::new(path))
}

#[cfg(test)]
mod tests {
    use plc_source::source_location::TextLocation;

    use super::*;

    fn handle_paths(pairs: &[(usize, &str)]) -> FxHashMap<usize, String> {
        pairs.iter().map(|(h, p)| (*h, p.to_string())).collect()
    }

    /// Build a host-absolute path string from a relative-looking fixture.
    /// Linux/macOS: `"/a.st"`. Windows: `"C:/a.st"`. We need this because
    /// `url::Url::from_file_path` only accepts paths the host considers
    /// absolute — a hard-coded `"/a.st"` works on Unix but fails on
    /// Windows, where it isn't a valid drive-rooted path.
    fn test_path(name: &str) -> String {
        if cfg!(windows) {
            format!("C:/{name}")
        } else {
            format!("/{name}")
        }
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
        let a_path = test_path("a.st");
        let b_path = test_path("b.st");
        let paths = handle_paths(&[(1, &a_path), (2, &b_path)]);
        let diags = vec![
            diag("E001", Severity::Error, 1, 0, 0, 5),
            diag("E002", Severity::Error, 2, 1, 2, 4),
            diag("E003", Severity::Warning, 1, 3, 0, 1),
        ];
        let grouped = map_collected(diags, &paths, &PositionEncodingKind::UTF8, &HashMap::new());

        let a = grouped.get(&path_to_uri(&a_path).unwrap()).expect("URI a");
        let b = grouped.get(&path_to_uri(&b_path).unwrap()).expect("URI b");
        assert_eq!(a.len(), 2);
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn maps_severity_filtering_ignore() {
        let a_path = test_path("a.st");
        let paths = handle_paths(&[(1, &a_path)]);
        let diags = vec![
            diag("E001", Severity::Error, 1, 0, 0, 5),
            diag("W001", Severity::Warning, 1, 1, 0, 5),
            diag("I001", Severity::Info, 1, 2, 0, 5),
            diag("X001", Severity::Ignore, 1, 3, 0, 5), // filtered out
        ];
        let grouped = map_collected(diags, &paths, &PositionEncodingKind::UTF8, &HashMap::new());
        let entry = grouped.get(&path_to_uri(&a_path).unwrap()).unwrap();
        assert_eq!(entry.len(), 3);
        assert_eq!(entry[0].severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(entry[1].severity, Some(DiagnosticSeverity::WARNING));
        assert_eq!(entry[2].severity, Some(DiagnosticSeverity::INFORMATION));
    }

    #[test]
    fn drops_internal_diagnostics() {
        let paths = handle_paths(&[(1, INTERNAL_FILENAME)]);
        let diags = vec![diag("E001", Severity::Error, 1, 0, 0, 5)];
        let grouped = map_collected(diags, &paths, &PositionEncodingKind::UTF8, &HashMap::new());
        assert!(grouped.is_empty());
    }

    #[test]
    fn drops_diagnostics_with_unknown_file_handle() {
        let paths = handle_paths(&[(1, "/a.st")]);
        let diags = vec![diag("E001", Severity::Error, 99, 0, 0, 5)]; // handle 99 not in map
        let grouped = map_collected(diags, &paths, &PositionEncodingKind::UTF8, &HashMap::new());
        assert!(grouped.is_empty());
    }

    #[test]
    fn drops_diagnostics_with_no_text_range() {
        let paths = handle_paths(&[(1, "/a.st")]);
        let mut d = diag("E001", Severity::Error, 1, 0, 0, 5);
        d.main_location.span = CodeSpan::None;
        let grouped = map_collected(vec![d], &paths, &PositionEncodingKind::UTF8, &HashMap::new());
        assert!(grouped.is_empty());
    }

    #[test]
    fn byte_offset_to_utf16_units_ascii_is_identity() {
        assert_eq!(byte_offset_to_utf16_units("hello", 5), 5);
        assert_eq!(byte_offset_to_utf16_units("hello", 0), 0);
        assert_eq!(byte_offset_to_utf16_units("a := 1;", 4), 4);
    }

    #[test]
    fn byte_offset_to_utf16_units_handles_multibyte() {
        // `名` is 3 bytes in UTF-8 but a single UTF-16 code unit (BMP).
        assert_eq!(byte_offset_to_utf16_units("名", 3), 1);
        // ASCII `a` + `名` = 1 + 1 utf-16 units after 4 bytes (a is 1 byte, 名 is 3).
        assert_eq!(byte_offset_to_utf16_units("a名b", 4), 2);
    }

    #[test]
    fn byte_offset_to_utf16_units_handles_surrogate_pair() {
        // `🌍` is 4 bytes in UTF-8 and encodes as a UTF-16 surrogate pair
        // (2 code units). This is the case the prototype is most likely to
        // get wrong if it just counted bytes.
        assert_eq!(byte_offset_to_utf16_units("🌍", 4), 2);
    }

    #[test]
    fn utf16_encoding_converts_non_ascii_columns() {
        // Source line: `名 := 1;`. `名` is 3 bytes in UTF-8 and 1 utf-16
        // code unit. A diagnostic span starting at byte offset 3 sits
        // *after* the `名` character.
        //
        // Under utf-8 negotiation, LSP `character` is the byte offset
        // (3). Under utf-16, it should be the utf-16 code-unit count
        // (1). This test makes that conversion measurable.
        let a_path = test_path("a.st");
        let mut sources = HashMap::new();
        sources.insert(a_path.clone(), "名 := 1;".to_string());
        let paths = handle_paths(&[(1, &a_path)]);

        let d_utf8 = diag("E001", Severity::Error, 1, 0, 3, 5);
        let d_utf16 = d_utf8.clone();

        let grouped_utf8 = map_collected(vec![d_utf8], &paths, &PositionEncodingKind::UTF8, &HashMap::new());
        let grouped_utf16 = map_collected(vec![d_utf16], &paths, &PositionEncodingKind::UTF16, &sources);

        let utf8_diag = grouped_utf8.into_values().next().unwrap().pop().unwrap();
        let utf16_diag = grouped_utf16.into_values().next().unwrap().pop().unwrap();

        assert_eq!(utf8_diag.range.start.character, 3, "utf-8: raw byte offset");
        assert_eq!(utf16_diag.range.start.character, 1, "utf-16: code-unit count");
    }

    #[test]
    fn utf16_falls_back_to_byte_offset_without_source() {
        // If source content isn't available (e.g., file read failed in
        // the worker), we fall back to raw byte offsets. Correct for
        // ASCII content, slightly off for non-ASCII — better than
        // dropping the diagnostic.
        let a_path = test_path("a.st");
        let paths = handle_paths(&[(1, &a_path)]);
        let d = diag("E001", Severity::Error, 1, 0, 7, 9);
        let grouped = map_collected(vec![d], &paths, &PositionEncodingKind::UTF16, &HashMap::new());
        let diag_out = grouped.into_values().next().unwrap().pop().unwrap();
        assert_eq!(diag_out.range.start.character, 7);
    }

    #[test]
    fn fields_match_phase_4_decisions() {
        let a_path = test_path("a.st");
        let paths = handle_paths(&[(1, &a_path)]);
        let d = diag("E033", Severity::Error, 1, 2, 4, 8);
        let grouped = map_collected(vec![d], &paths, &PositionEncodingKind::UTF8, &HashMap::new());
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
