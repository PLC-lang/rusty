use std::{
    cmp::Reverse,
    env, fs,
    path::{Path, PathBuf},
};

use insta::assert_snapshot;
use plc_source::SourceCode;
use tempfile::Builder;

use crate::{test_utils::tests::codegen_multi_with_options, DebugLevel, DEFAULT_DWARF_VERSION};

fn write_test_source(path: &Path, source: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create source dir");
    }
    fs::write(path, source).expect("failed to write source");
}

fn relevant_debug_lines(ir: &str) -> String {
    ir.lines()
        .filter(|line| line.contains("!DICompileUnit(") || line.contains("!DIFile("))
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_snapshot_paths(value: &str) -> String {
    let mut normalized = value.replace("\\\\?\\", "").replace('\\', "/");

    while normalized.contains("//") {
        normalized = normalized.replace("//", "/");
    }

    for drive in 'A'..='Z' {
        normalized = normalized.replace(&format!("/{drive}:/"), &format!("{drive}:/"));

        let lower = drive.to_ascii_lowercase();
        normalized = normalized.replace(&format!("/{lower}:/"), &format!("{lower}:/"));
    }

    normalized
}

fn sanitize_debug_snapshot(ir: &str, replacements: &[(&str, &str)]) -> String {
    let mut sanitized = normalize_snapshot_paths(&relevant_debug_lines(ir));
    let mut replacements = replacements.to_vec();
    replacements.sort_by_key(|(from, _)| Reverse(from.len()));
    for (from, to) in replacements {
        sanitized = sanitized.replace(from, to);
    }
    sanitized
}

fn canonical_path(path: &Path) -> PathBuf {
    path.canonicalize().expect("canonical path")
}

fn canonical_cwd() -> PathBuf {
    canonical_path(&env::current_dir().expect("cwd available"))
}

fn case_dir_name(case_dir: &tempfile::TempDir) -> String {
    case_dir.path().file_name().expect("case dir has a name").to_string_lossy().to_string()
}

fn virtual_root(cwd: &Path) -> PathBuf {
    cwd.parent().unwrap_or(cwd).join("__virtual__/TestProject")
}

#[cfg(not(windows))]
#[test]
fn compile_unit_name_is_relative_to_root_in_codegen_debug_tests() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let case_dir = Builder::new().prefix("codegen-debug-paths-default-").tempdir_in(&cwd).expect("tempdir");
    let source = case_dir.path().join("src/main.st");
    write_test_source(&source, source_code);

    let ir = codegen_multi_with_options(
        vec![SourceCode::new(source_code, canonical_path(&source))],
        Some(cwd.as_path()),
        DebugLevel::Full(DEFAULT_DWARF_VERSION),
        &[],
        None,
    )
    .join("\n");

    let case_name = case_dir_name(&case_dir);
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[(&normalize_snapshot_paths(&cwd.to_string_lossy()), "<CWD>"), (&case_name, "<CASE_ROOT>")],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "<CASE_ROOT>/src/main.st", directory: "<CWD>")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !10, splitDebugInlining: false)
"#);
}

#[cfg(not(windows))]
#[test]
fn prefix_map_and_debug_compilation_dir_are_applied_in_codegen_debug_tests() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let case_dir = Builder::new().prefix("codegen-debug-paths-prefix-").tempdir_in(&cwd).expect("tempdir");
    let source = case_dir.path().join("src/main.st");
    write_test_source(&source, source_code);

    let virtual_root = virtual_root(&cwd);
    let ir = codegen_multi_with_options(
        vec![SourceCode::new(source_code, canonical_path(&source))],
        None,
        DebugLevel::Full(DEFAULT_DWARF_VERSION),
        &[(cwd.clone(), virtual_root.clone())],
        Some(virtual_root.as_path()),
    )
    .join("\n");

    let case_name = case_dir_name(&case_dir);
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&virtual_root.to_string_lossy()), "/src/TestProject"),
            (&case_name, "<CASE_ROOT>"),
            (&normalize_snapshot_paths(&cwd.to_string_lossy()), "<CWD>"),
        ],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "<CASE_ROOT>/src/main.st", directory: "/src/TestProject")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !10, splitDebugInlining: false)
"#);

    assert!(!snapshot.contains("<CWD>"), "expected no local path leakage, got:\n{snapshot}");
}

#[cfg(not(windows))]
#[test]
fn mapped_source_root_outside_mapped_build_root_stays_absolute_in_codegen_debug_tests() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let case_dir =
        Builder::new().prefix("codegen-debug-paths-virtual-roots-").tempdir_in(&cwd).expect("tempdir");
    let source = case_dir.path().join("src/main.st");
    write_test_source(&source, source_code);

    let ir = codegen_multi_with_options(
        vec![SourceCode::new(source_code, canonical_path(&source))],
        None,
        DebugLevel::Full(DEFAULT_DWARF_VERSION),
        &[(cwd.clone(), PathBuf::from("/SOURCE_ROOT"))],
        Some(Path::new("/BUILD_ROOT")),
    )
    .join("\n");

    let case_name = case_dir_name(&case_dir);
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[(&case_name, "<CASE_ROOT>"), (&normalize_snapshot_paths(&cwd.to_string_lossy()), "<CWD>")],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "main.st", directory: "/SOURCE_ROOT/<CASE_ROOT>/src")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !10, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !11, splitDebugInlining: false)
!10 = !DIFile(filename: "/SOURCE_ROOT/<CASE_ROOT>/src/main.st", directory: "/BUILD_ROOT")
"#);

    assert!(
        !snapshot.contains("../SOURCE_ROOT"),
        "expected mapped source root to stay absolute, got:\n{snapshot}"
    );
}

#[cfg(not(windows))]
#[test]
fn mapped_compile_dir_keeps_parent_relative_filename_without_host_leakage_in_codegen_debug_tests() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let case_dir =
        Builder::new().prefix("codegen-debug-paths-parent-relative-").tempdir_in(&cwd).expect("tempdir");
    let compile_dir = case_dir.path().join("test");
    fs::create_dir_all(&compile_dir).expect("failed to create compile dir");

    let source = case_dir.path().join("dbg.st");
    write_test_source(&source, source_code);

    let source = canonical_path(&source);
    let compile_dir = canonical_path(&compile_dir);

    let ir = codegen_multi_with_options(
        vec![SourceCode::new(source_code, source)],
        None,
        DebugLevel::Full(DEFAULT_DWARF_VERSION),
        &[(compile_dir.clone(), PathBuf::from("/root"))],
        Some(compile_dir.as_path()),
    )
    .join("\n");

    let case_name = case_dir_name(&case_dir);
    let snapshot = sanitize_debug_snapshot(&ir, &[(&case_name, "<CASE_ROOT>")]);

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "../dbg.st", directory: "/root")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !10, splitDebugInlining: false)
"#);

    assert!(
        !snapshot.contains("<CASE_ROOT>"),
        "expected no host path leakage in debug info, got:\n{snapshot}"
    );
}
