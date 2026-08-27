// The name of a build artifact must stay flat and short. A build directory that
// mirrors the source tree carries the full path of every source file into the build
// directory, and for sources outside of the project (libraries, for example) the
// result goes past the limits of the Windows file system.
//
// # Mirroring policy
//
// Like the `debug_paths` tests, the platform dependent scenarios live in two sibling
// modules: `linux` (compiled with `cfg(not(windows))`) and `windows` (compiled with
// `cfg(windows)`). `Path::file_name` splits on the separators of the host, so the
// readable part of a name depends on the platform. When you add a test to one side,
// add a mirror with the same name to the other side, or document why the scenario
// fits one platform only.

use std::path::Path;

use insta::assert_snapshot;

use crate::artifacts;

/// Longest name the scheme can produce: 32 characters of file name, the separator,
/// 16 characters of digest, the dot and the extension.
fn name_limit(extension: &str) -> usize {
    32 + 1 + 16 + 1 + extension.len()
}

/// Returns the digest of an artifact name, which is the part between the last `-` and
/// the extension.
fn digest_of(name: &str) -> &str {
    let (name, _) = name.rsplit_once('.').expect("an artifact name has an extension");
    let (_, digest) = name.rsplit_once('-').expect("an artifact name has a digest");
    digest
}

#[test]
fn name_keeps_the_file_name_of_the_unit() {
    let name = artifacts::file_name(Path::new("app/prg/main.st"), "o");

    assert!(name.starts_with("main.st-"), "{name}");
    assert!(name.ends_with(".o"), "{name}");
}

/// The extension of the source file is part of the readable name. Without it, a
/// `const.st` and a `const.dt` next to each other claim the same artifact.
#[test]
fn units_that_differ_only_in_their_extension_get_different_names() {
    let structured_text = artifacts::file_name(Path::new("app/const.st"), "o");
    let data_types = artifacts::file_name(Path::new("app/const.dt"), "o");

    assert_ne!(structured_text, data_types);
}

#[test]
fn units_with_the_same_file_name_get_different_names() {
    let first = artifacts::file_name(Path::new("a/util.st"), "o");
    let second = artifacts::file_name(Path::new("b/util.st"), "o");

    assert_ne!(first, second);
}

/// The digest covers the whole key, not the parent directory alone: `b/util.st` and
/// `a/b/util.st` are different units.
#[test]
fn a_deeper_unit_with_the_same_file_name_gets_a_different_name() {
    let shallow = artifacts::file_name(Path::new("b/util.st"), "o");
    let deep = artifacts::file_name(Path::new("a/b/util.st"), "o");

    assert_ne!(shallow, deep);
}

/// The name of an artifact must not change between runs or between platforms; a build
/// that renames its artifacts leaves the artifacts of the run before it behind.
#[test]
fn the_name_of_a_unit_stays_the_same() {
    assert_snapshot!(artifacts::file_name(Path::new("app/prg/main.st"), "o"), @"main.st-116b35a8828869ac.o");
}

#[test]
fn the_extension_of_the_artifact_follows_the_argument() {
    let key = Path::new("app/main.st");

    assert!(artifacts::file_name(key, "o").ends_with(".o"));
    assert!(artifacts::file_name(key, "ll").ends_with(".ll"));
    assert!(artifacts::file_name(key, "bc").ends_with(".bc"));
}

/// The name is a single file name in a flat directory, so nothing in it may be read as
/// a directory or as a drive.
#[test]
fn the_name_holds_no_path_separators() {
    for key in ["a/b/main.st", r"a\b\main.st", "C:/a/main.st", r"C:\a\main.st", "../a/main.st"] {
        let name = artifacts::file_name(Path::new(key), "o");

        assert!(!name.contains(['/', '\\', ':']), "{key} produced {name}");
    }
}

#[test]
fn characters_that_windows_reserves_are_replaced() {
    let name = artifacts::file_name(Path::new(r#"we*ird<na>me|?.st"#), "o");

    assert!(name.starts_with("we_ird_na_me__.st-"), "{name}");
}

/// Names on a file system can hold characters that no compiler should have to reason
/// about; the artifact name stays plain ASCII.
#[test]
fn characters_outside_of_ascii_are_replaced() {
    let name = artifacts::file_name(Path::new("m\u{fc}ll\u{e4}imer.st"), "o");

    assert!(name.starts_with("m_ll_imer.st-"), "{name}");
    assert!(name.is_ascii(), "{name}");
}

#[test]
fn long_file_names_are_cut_to_the_limit() {
    let key = format!("{}.st", "a".repeat(200));

    let name = artifacts::file_name(Path::new(&key), "o");

    assert!(name.len() <= name_limit("o"), "{} characters: {name}", name.len());
}

/// Two long file names can end up with the same readable part. The digest is what
/// keeps their artifacts apart.
#[test]
fn units_whose_file_name_is_cut_stay_apart() {
    let prefix = "a".repeat(64);
    let first = artifacts::file_name(Path::new(&format!("{prefix}_first.st")), "o");
    let second = artifacts::file_name(Path::new(&format!("{prefix}_second.st")), "o");

    assert_ne!(first, second);
}

#[test]
fn a_key_without_a_file_name_gets_a_fallback_name() {
    let name = artifacts::file_name(Path::new(""), "o");

    assert!(name.starts_with("unit-"), "{name}");
}

/// The same project is built on Linux and on Windows, where the callers hand in the
/// separator of their platform. The digest ignores that difference, so a unit keeps
/// the name of its artifact.
#[test]
fn the_digest_ignores_the_separator_style() {
    let unix = artifacts::file_name(Path::new("a/b/main.st"), "o");
    let windows = artifacts::file_name(Path::new(r"a\b\main.st"), "o");

    assert_eq!(digest_of(&unix), digest_of(&windows));
}

#[cfg(not(windows))]
mod linux {
    use super::{artifacts, Path};

    #[test]
    fn an_absolute_key_keeps_only_the_file_name() {
        let name = artifacts::file_name(Path::new("/opt/lib/iec61131std/include/string.st"), "o");

        assert!(name.starts_with("string.st-"), "{name}");
    }

    /// A Windows path is not a path here: the host does not split on `\`, so the whole
    /// key becomes the readable part and only the sanitizing keeps the name legal.
    /// Windows does split on `/`, so this scenario has no mirror there.
    #[test]
    fn a_windows_key_is_sanitized_into_a_single_name() {
        let name = artifacts::file_name(Path::new(r"C:\lib\include\string.st"), "o");

        assert!(name.starts_with("C__lib_include_string.st-"), "{name}");
    }
}

#[cfg(windows)]
mod windows {
    use super::{artifacts, Path};

    #[test]
    fn an_absolute_key_keeps_only_the_file_name() {
        let name = artifacts::file_name(Path::new(r"C:\lib\iec61131std\include\string.st"), "o");

        assert!(name.starts_with("string.st-"), "{name}");
    }

    /// `fs::canonicalize` returns paths with a verbatim prefix on Windows, and that
    /// prefix must not reach the name either.
    #[test]
    fn a_verbatim_key_keeps_only_the_file_name() {
        let name = artifacts::file_name(Path::new(r"\\?\C:\lib\include\string.st"), "o");

        assert!(name.starts_with("string.st-"), "{name}");
    }

    /// A drive relative key has no directory component, and the drive letter must not
    /// reach the name.
    #[test]
    fn a_drive_relative_key_keeps_only_the_file_name() {
        let name = artifacts::file_name(Path::new("C:string.st"), "o");

        assert!(name.starts_with("string.st-"), "{name}");
    }
}
