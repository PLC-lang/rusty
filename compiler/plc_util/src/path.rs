use std::path::{Component, Path, PathBuf};

/// Normalize a path lexically without touching the filesystem: drop `.`
/// components, collapse `..` against preceding normal components, preserve
/// `..` segments that would escape an unrooted path, and keep root/prefix
/// components. Returns `.` for the empty path.
pub fn normalize_lexical_path(path: &Path) -> PathBuf {
    let mut stack: Vec<Component> = Vec::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => match stack.last() {
                Some(Component::Normal(_)) => {
                    stack.pop();
                }
                Some(Component::RootDir) | Some(Component::Prefix(_)) => {
                    // Cannot climb above the root — discard the `..`.
                }
                Some(Component::ParentDir) | None => {
                    // No earlier component to cancel against, keep the `..`.
                    stack.push(component);
                }
                Some(Component::CurDir) => unreachable!("CurDir is filtered out above"),
            },
            Component::RootDir | Component::Prefix(_) | Component::Normal(_) => {
                stack.push(component);
            }
        }
    }

    let mut normalized = PathBuf::new();
    for component in stack {
        normalized.push(component.as_os_str());
    }

    if normalized.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        normalized
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_lexical_path;
    use std::path::{Path, PathBuf};

    #[test]
    fn empty_path_becomes_dot() {
        assert_eq!(normalize_lexical_path(Path::new("")), PathBuf::from("."));
    }

    #[test]
    fn current_dir_components_are_dropped() {
        assert_eq!(normalize_lexical_path(Path::new("./foo/./bar")), PathBuf::from("foo/bar"));
        assert_eq!(normalize_lexical_path(Path::new(".")), PathBuf::from("."));
    }

    #[test]
    fn parent_dir_pops_when_possible() {
        assert_eq!(normalize_lexical_path(Path::new("foo/bar/../baz")), PathBuf::from("foo/baz"));
        assert_eq!(normalize_lexical_path(Path::new("foo/../bar")), PathBuf::from("bar"));
    }

    #[test]
    fn parent_dir_at_start_is_preserved() {
        // We don't have a root to pop above, so leading `..` segments stay in the result.
        assert_eq!(normalize_lexical_path(Path::new("../foo")), PathBuf::from("../foo"));
        assert_eq!(normalize_lexical_path(Path::new("../../foo")), PathBuf::from("../../foo"));
    }

    #[test]
    fn parent_dir_does_not_climb_past_root() {
        // POSIX root: `pop()` clears the path, so `..` after root pushes literal `..`.
        // The exact result is platform-specific but must not silently lose the leading root.
        let normalized = normalize_lexical_path(Path::new("/foo/../bar"));
        assert_eq!(normalized, PathBuf::from("/bar"));
    }

    #[test]
    fn absolute_path_is_kept_absolute() {
        assert_eq!(normalize_lexical_path(Path::new("/foo/bar")), PathBuf::from("/foo/bar"));
        assert_eq!(normalize_lexical_path(Path::new("/foo/./bar")), PathBuf::from("/foo/bar"));
    }

    #[test]
    fn redundant_separators_and_dots_collapse() {
        assert_eq!(normalize_lexical_path(Path::new("foo/./././bar")), PathBuf::from("foo/bar"));
    }
}
