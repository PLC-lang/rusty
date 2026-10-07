//! Naming of the intermediate build artifacts (object, bitcode and IR files).
//!
//! All artifacts of a build land in one flat directory. A layout that mirrors the
//! source tree instead carries the full path of every source file into the build
//! directory, and for sources outside of the project (libraries, for example) that
//! path goes past the limits of the Windows file system.

use std::{ffi::OsStr, path::Path};

use plc_util::path::path_digest;

/// Upper limit for the stem in the readable part of an artifact name. The digest keeps
/// names unique, so a longer stem has nothing to add.
const STEM_LIMIT: usize = 32;

/// Upper limit for the extension of the source in the readable part. It is cut apart
/// from the stem, so `.st` and `.pli` units stay apart in the readable part too.
const EXTENSION_LIMIT: usize = 16;

/// Used when the unit has no file name, for example because it comes from a source
/// container that is not a file.
const FALLBACK_NAME: &str = "unit";

/// Builds the flat artifact name for the unit that `key` identifies.
///
/// The name keeps the file name of the unit, which makes artifacts and linker
/// messages readable, and adds a digest of the full `key`. The digest is what makes
/// the name unique: two units with the same file name in different directories share
/// one flat directory and must not overwrite each other. The full 64 bits of the
/// digest are used because a collision would let one unit overwrite the artifact of
/// another.
pub fn file_name(key: &Path, extension: &str) -> String {
    format!("{}-{:016x}.{extension}", readable_name(key), path_digest(key))
}

/// The file name of the unit with its stem and its extension each cut to their limit.
fn readable_name(key: &Path) -> String {
    let stem = sanitize(key.file_stem().unwrap_or_default(), STEM_LIMIT);
    if stem.is_empty() {
        return FALLBACK_NAME.to_string();
    }

    match key.extension() {
        Some(extension) => format!("{stem}.{}", sanitize(extension, EXTENSION_LIMIT)),
        None => stem,
    }
}

/// Keeps the characters that are safe in a file name on every platform and replaces
/// all others, so a key that holds separators or a drive letter cannot escape the
/// artifact directory. The result holds at most `limit` characters.
fn sanitize(name: &OsStr, limit: usize) -> String {
    name.to_string_lossy()
        .chars()
        .map(|it| if it.is_ascii_alphanumeric() || matches!(it, '.' | '-' | '_') { it } else { '_' })
        .take(limit)
        .collect()
}
