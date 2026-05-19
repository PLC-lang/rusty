//! Project discovery helpers.
//!
//! Locates `plc.json` either via an explicit override (`--config` CLI arg
//! or `initializationOptions.plcConfigPath` from the client) or by
//! breadth-first search downward from the workspace root, skipping
//! well-known ignore directories. Per the prototype's project memory,
//! `plc.json` lives in a generated subfolder in real downstream projects
//! — upward search would miss it.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use lsp_types::InitializeParams;
use serde::Deserialize;

const IGNORED_DIRS: &[&str] = &[".git", "target", "node_modules", ".baseline"];
const PROJECT_FILE: &str = "plc.json";

/// Parsed shape of the LSP client's `initializationOptions` payload.
/// All fields are optional; unknown keys are tolerated.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct InitializationOptions {
    pub plc_config_path: Option<String>,
}

/// First workspace folder URI in initialize params, or the legacy
/// root_uri, mapped to a filesystem path. Returns None for non-`file://`
/// URIs or for clients that supply neither.
pub fn extract_workspace_root(params: &InitializeParams) -> Option<PathBuf> {
    #[allow(deprecated)] // root_uri is deprecated but still needed for older clients
    let legacy_root = params.root_uri.clone();
    params
        .workspace_folders
        .as_ref()
        .and_then(|folders| folders.first())
        .map(|f| f.uri.clone())
        .or(legacy_root)
        .and_then(|uri| file_uri_to_path(&uri))
}

fn file_uri_to_path(uri: &lsp_types::Uri) -> Option<PathBuf> {
    let s = uri.as_str();
    s.strip_prefix("file://").map(PathBuf::from)
}

/// Resolve the project's `plc.json` location. Honours an explicit
/// override when present, otherwise BFS-discovers downward from the
/// workspace root.
pub fn resolve_plc_config_path(
    workspace_root: Option<&Path>,
    override_path: Option<&Path>,
) -> Option<PathBuf> {
    if let Some(p) = override_path {
        return Some(p.to_path_buf());
    }
    workspace_root.and_then(discover_plc_json)
}

/// BFS for `plc.json` under `root`, returning the shallowest match.
/// Skips `.git`, `target`, `node_modules`, and `.baseline`.
pub fn discover_plc_json(root: &Path) -> Option<PathBuf> {
    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    queue.push_back(root.to_path_buf());

    while let Some(dir) = queue.pop_front() {
        let candidate = dir.join(PROJECT_FILE);
        if candidate.is_file() {
            return Some(candidate);
        }

        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if path.file_name().and_then(|n| n.to_str()).is_some_and(|n| IGNORED_DIRS.contains(&n)) {
                continue;
            }
            queue.push_back(path);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn discover_returns_none_when_no_plc_json() {
        let dir = tempdir().unwrap();
        assert!(discover_plc_json(dir.path()).is_none());
    }

    #[test]
    fn discover_finds_plc_json_at_root() {
        let dir = tempdir().unwrap();
        let plc = dir.path().join("plc.json");
        fs::write(&plc, "{}").unwrap();
        assert_eq!(discover_plc_json(dir.path()).unwrap(), plc);
    }

    #[test]
    fn discover_finds_plc_json_in_subdir() {
        let dir = tempdir().unwrap();
        let sub = dir.path().join("generated");
        fs::create_dir(&sub).unwrap();
        let plc = sub.join("plc.json");
        fs::write(&plc, "{}").unwrap();
        assert_eq!(discover_plc_json(dir.path()).unwrap(), plc);
    }

    #[test]
    fn discover_prefers_shallower_match() {
        let dir = tempdir().unwrap();
        let shallow = dir.path().join("plc.json");
        let sub = dir.path().join("nested");
        fs::create_dir(&sub).unwrap();
        let deep = sub.join("plc.json");
        fs::write(&shallow, "{}").unwrap();
        fs::write(&deep, "{}").unwrap();
        assert_eq!(discover_plc_json(dir.path()).unwrap(), shallow);
    }

    #[test]
    fn discover_skips_ignored_dirs() {
        let dir = tempdir().unwrap();
        for ignored in IGNORED_DIRS {
            let sub = dir.path().join(ignored);
            fs::create_dir(&sub).unwrap();
            fs::write(sub.join("plc.json"), "{}").unwrap();
        }
        assert!(discover_plc_json(dir.path()).is_none(), "ignored dirs must not be searched");
    }

    #[test]
    fn resolve_uses_override_when_provided() {
        let dir = tempdir().unwrap();
        let override_path = dir.path().join("custom-plc.json");
        fs::write(&override_path, "{}").unwrap();
        let resolved = resolve_plc_config_path(Some(dir.path()), Some(&override_path));
        assert_eq!(resolved.unwrap(), override_path);
    }

    #[test]
    fn resolve_falls_back_to_discovery() {
        let dir = tempdir().unwrap();
        let plc = dir.path().join("plc.json");
        fs::write(&plc, "{}").unwrap();
        let resolved = resolve_plc_config_path(Some(dir.path()), None);
        assert_eq!(resolved.unwrap(), plc);
    }

    #[test]
    fn resolve_returns_none_with_no_root_and_no_override() {
        assert!(resolve_plc_config_path(None, None).is_none());
    }
}
