use std::path::{Component, Path, PathBuf};

/// Normalize a path lexically without touching the filesystem: drop `.`
/// components, pop on `..` (or keep them if they would escape the root), and
/// preserve root/prefix components. Returns `.` for the empty path.
pub fn normalize_lexical_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() {
                    normalized.push(component.as_os_str());
                }
            }
            Component::RootDir | Component::Prefix(_) | Component::Normal(_) => {
                normalized.push(component.as_os_str())
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        normalized
    }
}
