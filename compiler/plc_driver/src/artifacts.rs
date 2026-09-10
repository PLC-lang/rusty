//! Naming of the intermediate build artifacts (object, bitcode and IR files).
//!
//! All artifacts of a build land in one flat directory. A layout that mirrors the
//! source tree instead carries the full path of every source file into the build
//! directory, and for sources outside of the project (libraries, for example) that
//! path goes past the limits of the Windows file system.

use std::{hash::Hasher, path::Path};

/// Upper limit for the readable part of an artifact name. The digest keeps names
/// unique, so a longer file name has nothing to add.
const NAME_LIMIT: usize = 32;

/// Used when the unit has no file name, for example because it comes from a source
/// container that is not a file.
const FALLBACK_NAME: &str = "unit";

/// Builds the flat artifact name for the unit that `key` identifies.
///
/// The name keeps the file name of the unit, which makes artifacts and linker
/// messages readable, and adds a digest of the full `key`. The digest is what makes
/// the name unique: two units with the same file name in different directories share
/// one flat directory and must not overwrite each other.
pub fn file_name(key: &Path, extension: &str) -> String {
    format!("{}-{:016x}.{extension}", readable_name(key), digest(key))
}

/// Keeps the characters that are safe in a file name on every platform and replaces
/// all others, so a key that holds separators or a drive letter cannot escape the
/// artifact directory.
fn readable_name(key: &Path) -> String {
    let name: String = key
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .chars()
        .map(|it| if it.is_ascii_alphanumeric() || matches!(it, '.' | '-' | '_') { it } else { '_' })
        .take(NAME_LIMIT)
        .collect();

    if name.is_empty() {
        FALLBACK_NAME.to_string()
    } else {
        name
    }
}

/// SipHash-1-3 with a fixed zero key, so the name of an artifact is the same on every
/// run, process and platform. The full 64 bits are used because a collision would let
/// one unit overwrite the artifact of another.
///
/// The separators are normalized first, so the same unit keeps its name no matter
/// which separator the caller used.
fn digest(key: &Path) -> u64 {
    let normalized = key.to_string_lossy().replace('\\', "/");
    let mut hasher = siphasher::sip::SipHasher13::new_with_keys(0, 0);
    hasher.write(normalized.as_bytes());
    hasher.finish()
}
