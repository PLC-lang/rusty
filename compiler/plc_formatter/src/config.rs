//! `FormatConfig`: the YAML-configurable style options, loaded from a `.plcformat.yaml` file
//! discovered by walking up from the input file, or from built-in defaults.

use std::path::{Path, PathBuf};

use plc_diagnostics::diagnostics::Diagnostic;
use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &str = ".plcformat.yaml";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum KeywordCase {
    Upper,
    Lower,
    Preserve,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct FormatConfig {
    pub indent_width: u32,
    pub use_tab: bool,
    pub keyword_case: KeywordCase,
    pub max_consecutive_empty_lines: u32,
    pub blank_line_after_block_open: bool,
    pub blank_line_before_block_close: bool,
    pub space_around_assignment: bool,
    pub space_around_binary_operators: bool,
    pub space_after_comma: bool,
    pub space_before_colon: bool,
    pub space_after_colon: bool,
    pub trailing_comment_min_spacing: u32,
    pub align_consecutive_declarations: bool,
    pub align_consecutive_assignments: bool,
    pub align_trailing_comments: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        FormatConfig {
            indent_width: 4,
            use_tab: false,
            keyword_case: KeywordCase::Upper,
            max_consecutive_empty_lines: 1,
            blank_line_after_block_open: false,
            blank_line_before_block_close: false,
            space_around_assignment: true,
            space_around_binary_operators: true,
            space_after_comma: true,
            space_before_colon: false,
            space_after_colon: true,
            trailing_comment_min_spacing: 2,
            align_consecutive_declarations: false,
            align_consecutive_assignments: false,
            align_trailing_comments: false,
        }
    }
}

impl FormatConfig {
    pub fn from_yaml_str(source: &str) -> Result<Self, Diagnostic> {
        serde_yaml::from_str(source).map_err(|err| {
            Diagnostic::new(format!("invalid {CONFIG_FILE_NAME}: {err}")).with_error_code("E090")
        })
    }

    pub fn from_file(path: &Path) -> Result<Self, Diagnostic> {
        let source = std::fs::read_to_string(path).map_err(|err| {
            Diagnostic::new(format!("could not read {}: {err}", path.display())).with_error_code("E002")
        })?;
        Self::from_yaml_str(&source)
    }

    pub fn to_yaml_string(&self) -> String {
        serde_yaml::to_string(self).expect("FormatConfig always serializes")
    }
}

/// Walks `start_dir` and its ancestors looking for `.plcformat.yaml`, like `.clang-format`.
/// Returns the first hit, or `None` if none of the ancestors have one.
pub fn discover(start_dir: &Path) -> Option<PathBuf> {
    for dir in start_dir.ancestors() {
        let candidate = dir.join(CONFIG_FILE_NAME);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

/// Resolves the effective config for formatting `input_file`: an explicit `--config` path wins
/// outright, otherwise discovery is attempted from the input file's directory, falling back to
/// built-in defaults if nothing is found.
pub fn resolve(
    input_file: Option<&Path>,
    explicit_config: Option<&Path>,
) -> Result<FormatConfig, Diagnostic> {
    if let Some(path) = explicit_config {
        return FormatConfig::from_file(path);
    }
    let start_dir = input_file.and_then(Path::parent).unwrap_or_else(|| Path::new("."));
    match discover(start_dir) {
        Some(path) => FormatConfig::from_file(&path),
        None => Ok(FormatConfig::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_round_trips_through_yaml() {
        let yaml = FormatConfig::default().to_yaml_string();
        assert_eq!(FormatConfig::from_yaml_str(&yaml).unwrap(), FormatConfig::default());
    }

    #[test]
    fn partial_config_falls_back_to_defaults_for_missing_fields() {
        let config = FormatConfig::from_yaml_str("indent_width: 2\n").unwrap();
        assert_eq!(config.indent_width, 2);
        assert_eq!(config.keyword_case, KeywordCase::Upper);
    }

    #[test]
    fn unknown_field_is_rejected() {
        let result = FormatConfig::from_yaml_str("not_a_real_option: true\n");
        assert!(result.is_err());
    }

    #[test]
    fn discover_walks_up_to_an_ancestor() {
        let dir = tempfile::tempdir().unwrap();
        let nested = dir.path().join("a/b/c");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(dir.path().join(CONFIG_FILE_NAME), "indent_width: 2\n").unwrap();

        let found = discover(&nested).unwrap();
        assert_eq!(found, dir.path().join(CONFIG_FILE_NAME));
    }

    #[test]
    fn discover_returns_none_without_a_config_file() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(discover(dir.path()), None);
    }
}
