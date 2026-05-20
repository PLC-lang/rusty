//! Filesystem watcher registration helpers.
//!
//! Builds the `RegistrationParams` for `workspace/didChangeWatchedFiles`
//! (one `FileSystemWatcher` per project source glob from `plc.json`, plus
//! the `plc.json` file itself), and extracts the source globs in
//! absolute form so the client can match them straight against URIs.

use std::path::Path;

use lsp_types::{
    DidChangeWatchedFilesRegistrationOptions, FileSystemWatcher, GlobPattern, Registration,
    RegistrationParams, WatchKind,
};

/// Fixed identifier used when registering the file watcher with the
/// client. We use a single stable ID (rather than generating one per
/// registration) because we need to unregister + re-register the same
/// capability when `plc.json` changes — and unregistration matches by
/// id + method.
pub const WATCHER_REGISTRATION_ID: &str = "rusty.fileWatcher";

/// The LSP method name we're registering for. Promoted to a constant so
/// the register and unregister call sites can't drift apart.
pub const DID_CHANGE_WATCHED_FILES_METHOD: &str = "workspace/didChangeWatchedFiles";

/// Build a RegistrationParams for the project's source globs + plc.json
/// itself. Source globs are watched for all event types; plc.json gets
/// Change + Delete only (Create doesn't apply — it already existed when
/// the server resolved it on startup, and a re-create would surface as a
/// Change event in practice).
///
/// We use `GlobPattern::String` (a single absolute glob string) rather
/// than `GlobPattern::Relative` (base URI + pattern, introduced in LSP
/// 3.17). Both work for our case; the string form avoids a client-side
/// capability check and keeps the snippet copy-pasteable into config
/// dumps for debugging.
pub fn build_registration(plc_config_path: &Path, source_globs: &[String]) -> RegistrationParams {
    let mut watchers: Vec<FileSystemWatcher> = source_globs
        .iter()
        .map(|glob| FileSystemWatcher {
            glob_pattern: GlobPattern::String(glob.clone()),
            kind: None, // None = client default = Create | Change | Delete
        })
        .collect();

    watchers.push(FileSystemWatcher {
        glob_pattern: GlobPattern::String(plc_config_path.to_string_lossy().into_owned()),
        kind: Some(WatchKind::Change | WatchKind::Delete),
    });

    RegistrationParams {
        registrations: vec![Registration {
            id: WATCHER_REGISTRATION_ID.to_string(),
            method: DID_CHANGE_WATCHED_FILES_METHOD.to_string(),
            register_options: Some(
                serde_json::to_value(DidChangeWatchedFilesRegistrationOptions { watchers })
                    .expect("DidChangeWatchedFilesRegistrationOptions must serialise"),
            ),
        }],
    }
}

/// Read plc.json's `files` array directly via serde_json and join each
/// entry with the plc.json's parent directory so the globs are absolute
/// and usable as LSP watcher patterns. We avoid going through
/// `plc_project::ProjectConfig::from_file` because that's `pub(crate)`;
/// we only need the `files` field anyway.
pub fn extract_source_globs(plc_config_path: &Path) -> Result<Vec<String>, anyhow::Error> {
    let content = std::fs::read_to_string(plc_config_path)?;
    let value: serde_json::Value = serde_json::from_str(&content)?;
    let files = value
        .get("files")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("plc.json missing a 'files' array"))?;
    let parent = plc_config_path.parent().unwrap_or(Path::new("."));
    Ok(files.iter().filter_map(|v| v.as_str()).map(|s| absolute_glob(parent, Path::new(s))).collect())
}

/// Join a glob with `base` only if the glob isn't already absolute.
/// LSP watcher patterns are matched against absolute URIs, so any
/// `plc.json`-relative glob needs to be made absolute first.
fn absolute_glob(base: &Path, glob: &Path) -> String {
    let joined = if glob.is_absolute() { glob.to_path_buf() } else { base.join(glob) };
    joined.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn build_includes_a_watcher_per_source_glob_plus_plc_json() {
        let plc_path = PathBuf::from("/proj/plc.json");
        let globs = vec!["/proj/*.st".to_string(), "/proj/lib/**/*.iecst".to_string()];
        let params = build_registration(&plc_path, &globs);
        assert_eq!(params.registrations.len(), 1);

        let reg = &params.registrations[0];
        assert_eq!(reg.id, WATCHER_REGISTRATION_ID);
        assert_eq!(reg.method, DID_CHANGE_WATCHED_FILES_METHOD);

        let opts: DidChangeWatchedFilesRegistrationOptions =
            serde_json::from_value(reg.register_options.clone().unwrap()).unwrap();
        assert_eq!(opts.watchers.len(), 3); // 2 globs + plc.json
    }

    #[test]
    fn plc_json_watcher_kind_is_change_plus_delete() {
        let params = build_registration(&PathBuf::from("/proj/plc.json"), &[]);
        let opts: DidChangeWatchedFilesRegistrationOptions =
            serde_json::from_value(params.registrations[0].register_options.clone().unwrap()).unwrap();
        let plc_watcher = opts.watchers.last().unwrap();
        let expected = WatchKind::Change | WatchKind::Delete;
        assert_eq!(plc_watcher.kind, Some(expected));
    }

    #[test]
    fn extract_globs_makes_relative_patterns_absolute() {
        let dir = tempdir().unwrap();
        let plc = dir.path().join("plc.json");
        fs::write(&plc, r#"{ "name": "p", "files": ["src/*.st", "*.iecst"] }"#).unwrap();
        let globs = extract_source_globs(&plc).unwrap();
        assert_eq!(globs.len(), 2);
        assert!(globs[0].starts_with(dir.path().to_string_lossy().as_ref()));
        assert!(globs[0].ends_with("src/*.st"));
        assert!(globs[1].ends_with("*.iecst"));
    }

    #[test]
    fn extract_globs_leaves_absolute_patterns_alone() {
        // Host-absolute pattern: `/abs/path/*.st` on Unix is absolute and
        // gets passed through verbatim; on Windows it is *not* absolute
        // (Windows wants a drive letter), so to test the
        // already-absolute branch on Windows we use `C:/abs/...`.
        let dir = tempdir().unwrap();
        let plc = dir.path().join("plc.json");
        let absolute_glob = if cfg!(windows) { "C:/abs/path/*.st" } else { "/abs/path/*.st" };
        let json = format!(r#"{{ "name": "p", "files": ["{absolute_glob}"] }}"#);
        fs::write(&plc, &json).unwrap();
        let globs = extract_source_globs(&plc).unwrap();
        assert_eq!(globs[0], absolute_glob);
    }
}
