// The build keeps its artifacts in one flat directory. A layout that mirrors the
// source tree carries the path of every source file into the build directory, and as
// soon as a source lives outside of the project (an installed library, for example)
// the result goes past the path limit of the Windows file system.

use std::{
    fs,
    path::{Path, PathBuf},
};

use driver::compile;

/// Longest artifact name the compiler produces: 32 characters of file name, the
/// separator, 16 characters of digest, the dot and the extension.
const ARTIFACT_NAME_LIMIT: usize = 32 + 1 + 16 + 1 + 2;

/// Path limit of the Windows file system for programs that do not opt into long paths.
const WINDOWS_PATH_LIMIT: usize = 260;

const TARGET: &str = "x86_64-linux-gnu";

fn write_source(path: &Path, source: &str) {
    fs::create_dir_all(path.parent().expect("a source file has a parent")).expect("source directory");
    fs::write(path, source).expect("source file");
}

/// Pushes directories onto `base` until the path is at least `length` characters long,
/// which is how the tests get the long build and source paths of a real workspace
/// without depending on where the temporary directory of the platform lives.
fn extend_to_length(base: &Path, length: usize) -> PathBuf {
    let mut path = base.to_path_buf();
    while path.to_string_lossy().chars().count() < length {
        path.push("padding");
    }
    path
}

fn artifacts_in(directory: &Path) -> Vec<PathBuf> {
    let mut artifacts = Vec::new();
    let Ok(entries) = fs::read_dir(directory) else {
        return artifacts;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            artifacts.extend(artifacts_in(&path));
        } else {
            artifacts.push(path);
        }
    }
    artifacts.sort();
    artifacts
}

fn length(path: &Path) -> usize {
    path.to_string_lossy().chars().count()
}

/// Compiles a unit inside the project and two units that live outside of it and share
/// a file name. Uses `--ir`, so no linker takes part and the test runs on every
/// platform.
fn compile_project_with_external_sources(build_directory: &Path, sources: &Path) {
    let project = sources.join("workspace/MyProject");
    let library = sources.join("libraries/iec61131std/include/generated/nested");

    let main = project.join("main.st");
    let first = library.join("first/util.st");
    let second = library.join("second/util.st");

    write_source(&main, "FUNCTION main : DINT END_FUNCTION");
    write_source(&first, "FUNCTION util_a : DINT END_FUNCTION");
    write_source(&second, "FUNCTION util_b : DINT END_FUNCTION");

    let output = sources.join("out.ll");

    compile(&[
        "plc",
        &main.to_string_lossy(),
        &first.to_string_lossy(),
        &second.to_string_lossy(),
        "--ir",
        "-o",
        &output.to_string_lossy(),
        "--build-location",
        &build_directory.to_string_lossy(),
        "--target",
        TARGET,
    ])
    .expect("compile succeeded");
}

#[test]
fn artifacts_of_external_sources_land_flat_in_the_target_directory() {
    let sources = tempfile::tempdir().unwrap();
    let build_directory = tempfile::tempdir().unwrap();

    compile_project_with_external_sources(build_directory.path(), sources.path());

    let target_directory = build_directory.path().join(TARGET);
    let artifacts = artifacts_in(build_directory.path());

    // One artifact per unit: the two units that share the file name `util.st` must not
    // claim the same artifact.
    assert_eq!(artifacts.len(), 3, "{artifacts:?}");
    for artifact in &artifacts {
        assert_eq!(artifact.parent(), Some(target_directory.as_path()), "{artifact:?} is not flat");
    }
}

#[test]
fn artifact_paths_do_not_grow_with_the_path_of_their_source() {
    let sources = tempfile::tempdir().unwrap();
    let build_directory = tempfile::tempdir().unwrap();

    compile_project_with_external_sources(build_directory.path(), sources.path());

    // Everything the compiler adds to the build directory: the directory of the target
    // and one artifact name, both with a separator.
    let limit = TARGET.len() + ARTIFACT_NAME_LIMIT + 2;
    for artifact in artifacts_in(build_directory.path()) {
        let added = length(&artifact) - length(build_directory.path());
        assert!(added <= limit, "{added} characters added by {artifact:?}, limit is {limit}");
    }
}

/// A build that renames its artifacts leaves the artifacts of the run before it
/// behind, and every rebuild grows the build directory.
#[test]
fn a_second_build_reuses_the_artifact_names_of_the_first() {
    let sources = tempfile::tempdir().unwrap();
    let build_directory = tempfile::tempdir().unwrap();

    compile_project_with_external_sources(build_directory.path(), sources.path());
    let first = artifacts_in(build_directory.path());

    compile_project_with_external_sources(build_directory.path(), sources.path());
    let second = artifacts_in(build_directory.path());

    assert_eq!(first, second);
}

/// A long build directory together with sources outside of the project is the case
/// that used to produce paths the Windows file system cannot handle.
#[test]
fn artifact_paths_stay_within_the_windows_path_limit() {
    let sources = tempfile::tempdir().unwrap();
    let build_root = tempfile::tempdir().unwrap();

    // The source tree stays well within the limit on its own, so that only the artifact
    // paths are under test here.
    let build_directory = extend_to_length(build_root.path(), 150);
    fs::create_dir_all(&build_directory).expect("build directory");
    let source_root = extend_to_length(sources.path(), 120);

    compile_project_with_external_sources(&build_directory, &source_root);

    let artifacts = artifacts_in(&build_directory);
    assert_eq!(artifacts.len(), 3, "{artifacts:?}");
    for artifact in artifacts {
        assert!(length(&artifact) < WINDOWS_PATH_LIMIT, "{} characters: {artifact:?}", length(&artifact));
    }
}
