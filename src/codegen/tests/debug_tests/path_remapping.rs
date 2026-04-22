use std::{
    env, fs,
    path::{Path, PathBuf},
};

use insta::assert_snapshot;
use plc_source::SourceCode;
use tempfile::Builder;

use crate::{test_utils::tests::codegen_multi_with_options, DebugLevel, DEFAULT_DWARF_VERSION};

fn write_test_source(path: &Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create source dir");
    }
    fs::write(path, "PROGRAM prg\nVAR\n    x : INT;\nEND_VAR\n    x := 1;\nEND_PROGRAM\n")
        .expect("failed to write source");
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

fn virtual_root(cwd: &Path) -> PathBuf {
    cwd.parent().unwrap_or(cwd).join("__virtual__/TestProject")
}

#[test]
fn normalize_snapshot_paths_handles_windows_runner_style_paths() {
    let raw = r#"
!2 = !DIFile(filename: "main.st", directory: "///D://a//rusty//__virtual__//TestProject//rusty-codegen-prefix-[id]//src")
!10 = !DIFile(filename: "rusty-codegen-prefix-[id]//src//main.st", directory: "///D://a//rusty//__virtual__//TestProject")
!2 = !DIFile(filename: "main.st", directory: "///D://a//rusty//rusty//rusty-codegen-debug-[id]//src")
!10 = !DIFile(filename: "rusty-codegen-debug-[id]//src//main.st", directory: "///D://a//rusty//rusty")
"#;

    let sanitized = sanitize_debug_snapshot(
        raw,
        &[("D:/a/rusty/__virtual__/TestProject", "/src/TestProject"), ("D:/a/rusty/rusty", "/cwd")],
    );

    assert_eq!(
        sanitized,
        concat!(
            "!2 = !DIFile(filename: \"main.st\", directory: \"/src/TestProject/rusty-codegen-prefix-[id]/src\")\n",
            "!10 = !DIFile(filename: \"rusty-codegen-prefix-[id]/src/main.st\", directory: \"/src/TestProject\")\n",
            "!2 = !DIFile(filename: \"main.st\", directory: \"/cwd/rusty-codegen-debug-[id]/src\")\n",
            "!10 = !DIFile(filename: \"rusty-codegen-debug-[id]/src/main.st\", directory: \"/cwd\")"
        )
    );
}

#[test]
fn compile_unit_name_is_relative_to_root_in_codegen_debug_tests() {
    let cwd = canonical_cwd();
    let tempdir = Builder::new().prefix("rusty-codegen-debug-").tempdir_in(&cwd).expect("tempdir");
    let source = tempdir.path().join("src/main.st");
    write_test_source(&source);
    let source = canonical_path(&source);
    let relative = source.strip_prefix(&cwd).expect("source under cwd").to_string_lossy().to_string();

    let ir = codegen_multi_with_options(
        vec![SourceCode::new(
            "PROGRAM prg\nVAR\n    x : INT;\nEND_VAR\n    x := 1;\nEND_PROGRAM\n",
            source.clone(),
        )],
        Some(cwd.as_path()),
        DebugLevel::Full(DEFAULT_DWARF_VERSION),
        &[],
        None,
    )
    .join("\n");

    let tempdir_name = tempdir.path().file_name().unwrap().to_string_lossy().to_string();
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&cwd.to_string_lossy()), "/cwd"),
            (&tempdir_name, "rusty-codegen-debug-[id]"),
            (&normalize_snapshot_paths(&relative), "rusty-codegen-debug-[id]/src/main.st"),
        ],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "main.st", directory: "/cwd/rusty-codegen-debug-[id]/src")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !10, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !11, splitDebugInlining: false)
!10 = !DIFile(filename: "rusty-codegen-debug-[id]/src/main.st", directory: "/cwd")
"#);
}

#[test]
fn prefix_map_and_debug_compilation_dir_are_applied_in_codegen_debug_tests() {
    let cwd = canonical_cwd();
    let tempdir = Builder::new().prefix("rusty-codegen-prefix-").tempdir_in(&cwd).expect("tempdir");
    let source = tempdir.path().join("src/main.st");
    write_test_source(&source);
    let source = canonical_path(&source);
    let relative = source.strip_prefix(&cwd).expect("source under cwd").to_string_lossy().to_string();

    let virtual_root = virtual_root(&cwd);
    let ir = codegen_multi_with_options(
        vec![SourceCode::new(
            "PROGRAM prg\nVAR\n    x : INT;\nEND_VAR\n    x := 1;\nEND_PROGRAM\n",
            source.clone(),
        )],
        None,
        DebugLevel::Full(DEFAULT_DWARF_VERSION),
        &[(cwd.clone(), virtual_root.clone())],
        Some(virtual_root.as_path()),
    )
    .join("\n");

    let tempdir_name = tempdir.path().file_name().unwrap().to_string_lossy().to_string();
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&cwd.to_string_lossy()), "/cwd"),
            (&normalize_snapshot_paths(&virtual_root.to_string_lossy()), "/src/TestProject"),
            (&tempdir_name, "rusty-codegen-prefix-[id]"),
            (&normalize_snapshot_paths(&relative), "rusty-codegen-prefix-[id]/src/main.st"),
        ],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "main.st", directory: "/src/TestProject/rusty-codegen-prefix-[id]/src")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !10, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !11, splitDebugInlining: false)
!10 = !DIFile(filename: "rusty-codegen-prefix-[id]/src/main.st", directory: "/src/TestProject")
"#);
}
