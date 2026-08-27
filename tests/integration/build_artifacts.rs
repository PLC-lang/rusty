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

/// Name that the test projects give to their final output. That output is not an
/// intermediate artifact: it keeps the name the project description asks for and lands
/// in the directory of the target next to the artifacts.
const PROJECT_OUTPUT: &str = "proj.out";

/// Writes a project with a `plc.json`. The `build` subcommand takes the root of the
/// project from the location of that file rather than from the current directory, and
/// that root is what an artifact name is relative to.
fn write_project(project: &Path, files: &[&str]) -> PathBuf {
    for (index, file) in files.iter().enumerate() {
        write_source(&project.join(file), &format!("FUNCTION unit_{index} : DINT END_FUNCTION"));
    }

    let names = files.iter().map(|it| format!("{it:?}")).collect::<Vec<_>>().join(", ");
    let config = project.join("plc.json");
    let description = format!(r#"{{"name": "proj", "files": [{names}], "output": "{PROJECT_OUTPUT}"}}"#);
    fs::write(&config, description).expect("plc.json");
    config
}

/// Builds through the `build` subcommand. Uses `--ir`, so no linker takes part and the
/// test runs on every platform.
fn build_project(config: &Path, build_directory: &Path) {
    compile(&[
        "plc",
        "build",
        &config.to_string_lossy(),
        "--ir",
        "--build-location",
        &build_directory.to_string_lossy(),
        "--target",
        TARGET,
    ])
    .expect("build succeeded");
}

/// The intermediate artifacts of a build, by name and without the final output of the
/// project, sorted.
fn artifact_names_in(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = intermediate_artifacts_in(directory)
        .iter()
        .map(|it| it.file_name().expect("an artifact has a file name").to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

fn intermediate_artifacts_in(directory: &Path) -> Vec<PathBuf> {
    artifacts_in(directory).into_iter().filter(|it| it.file_name() != Some(PROJECT_OUTPUT.as_ref())).collect()
}

/// The key of a unit inside the project is its path relative to the project, and the
/// name of its artifact follows from that key alone. So one project produces the same
/// artifact names wherever it is checked out and on whichever platform it is built.
///
/// On Windows this is what the canonicalization of the project root buys: `canonicalize`
/// returns a verbatim path (`\\?\C:\...`) for the unit, so a root that is not
/// canonicalized never prefixes it and the absolute path becomes the key.
#[test]
#[serial]
fn artifacts_of_a_project_build_are_named_after_the_path_relative_to_the_project() {
    let project = tempfile::tempdir().unwrap();
    let build_directory = tempfile::tempdir().unwrap();

    let config = write_project(project.path(), &["src/main.st", "src/nested/util.st"]);
    build_project(&config, build_directory.path());

    let mut expected = vec![
        driver::artifacts::file_name(Path::new("src/main.st"), "ll"),
        driver::artifacts::file_name(Path::new("src/nested/util.st"), "ll"),
    ];
    expected.sort();
    assert_eq!(artifact_names_in(build_directory.path()), expected);

    let target_directory = build_directory.path().join(TARGET);
    for artifact in intermediate_artifacts_in(build_directory.path()) {
        assert_eq!(artifact.parent(), Some(target_directory.as_path()), "{artifact:?} is not flat");
    }
}

/// A source file named like one of the device names Windows reserves (`con`, `nul`,
/// ...). The system resolves a bare device name to the device, so the artifact of such a
/// source must keep the digest and the extension that make it an ordinary file name.
#[test]
#[serial]
fn a_source_named_like_a_reserved_device_builds() {
    let project = tempfile::tempdir().unwrap();
    let build_directory = tempfile::tempdir().unwrap();

    let config = write_project(project.path(), &["src/con.st", "src/nul.st"]);
    build_project(&config, build_directory.path());

    let names = artifact_names_in(build_directory.path());
    assert_eq!(names.len(), 2, "{names:?}");
    assert!(names.iter().any(|it| it.starts_with("con.st-")), "{names:?}");
    assert!(names.iter().any(|it| it.starts_with("nul.st-")), "{names:?}");
}

/// A source file whose name is not ASCII. The artifact name is plain ASCII by
/// construction, so it does not depend on the code page of the workspace.
#[test]
#[serial]
fn a_source_with_a_non_ascii_name_builds() {
    let project = tempfile::tempdir().unwrap();
    let build_directory = tempfile::tempdir().unwrap();

    let config = write_project(project.path(), &["src/pr\u{fc}f.st"]);
    build_project(&config, build_directory.path());

    let names = artifact_names_in(build_directory.path());
    assert_eq!(names.len(), 1, "{names:?}");
    assert!(names[0].is_ascii(), "{:?}", names[0]);
    assert!(names[0].starts_with("pr_f.st-"), "{:?}", names[0]);
}

#[cfg(windows)]
mod windows {
    use std::fs;

    use super::{artifact_names_in, build_project, extend_to_length, write_project, WINDOWS_PATH_LIMIT};

    /// Paths are case insensitive on Windows, so a project that spells the name of a
    /// source differently must not get a second artifact for it. The casing that reaches
    /// the digest is the one on the file system, because the key of a unit is
    /// canonicalized first; `the_digest_does_not_normalize_the_casing_by_itself` in the
    /// driver tests shows that the naming alone would not do it.
    #[test]
    #[serial]
    fn a_source_referenced_with_another_casing_keeps_its_artifact() {
        let project = tempfile::tempdir().unwrap();
        let build_directory = tempfile::tempdir().unwrap();

        let config = write_project(project.path(), &["src/main.st"]);
        build_project(&config, build_directory.path());
        let first = artifact_names_in(build_directory.path());

        // The same source, spelled the way it is not spelled on disk.
        let config = write_project(project.path(), &["SRC/MAIN.ST"]);
        build_project(&config, build_directory.path());
        let second = artifact_names_in(build_directory.path());

        assert_eq!(first.len(), 1, "{first:?}");
        assert_eq!(first, second);
    }

    /// A build directory that is past the limit before the compiler adds anything. The
    /// standard library hands absolute paths to the file system in their verbatim form,
    /// which is not bound by the limit, so the build goes through and there is nothing
    /// for the compiler to report. What fails on such a path are the tools around the
    /// compiler, and that is the reason the artifacts have to stay short.
    #[test]
    #[serial]
    fn a_build_directory_past_the_path_limit_still_builds() {
        let project = tempfile::tempdir().unwrap();
        let build_root = tempfile::tempdir().unwrap();

        let build_directory = extend_to_length(build_root.path(), WINDOWS_PATH_LIMIT + 10);
        fs::create_dir_all(&build_directory).expect("build directory");

        let config = write_project(project.path(), &["src/main.st"]);
        build_project(&config, &build_directory);

        assert_eq!(artifact_names_in(&build_directory).len(), 1);
    }
}
