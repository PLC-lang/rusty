use std::{
    cmp::Reverse,
    env, fs,
    path::{Path, PathBuf},
};

use insta::assert_snapshot;
use plc::DebugLevel;
use tempfile::Builder;

use crate::{
    tests::{compile_args_to_string, compile_build_config_to_string, compile_to_string_with_options},
    CompileOptions,
};

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

#[cfg(not(windows))]
fn tempdir_base(cwd: &Path) -> &Path {
    cwd.parent().unwrap_or(cwd)
}

#[cfg(not(windows))]
fn virtual_root(cwd: &Path) -> PathBuf {
    tempdir_base(cwd).join("__virtual__/TestProject")
}

#[cfg(not(windows))]
fn relative_path_from(base: &Path, path: &Path) -> PathBuf {
    let mut path_components = path.components().peekable();
    let mut base_components = base.components().peekable();

    while path_components.peek().is_some()
        && base_components.peek().is_some()
        && path_components.peek() == base_components.peek()
    {
        path_components.next();
        base_components.next();
    }

    let mut relative = PathBuf::new();
    for component in base_components {
        if matches!(component, std::path::Component::Normal(_) | std::path::Component::ParentDir) {
            relative.push("..");
        }
    }
    for component in path_components {
        relative.push(component.as_os_str());
    }
    relative
}

#[cfg(windows)]
fn has_windows_drive_prefix(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.windows(3).any(|w| w[0].is_ascii_alphabetic() && w[1] == b':' && (w[2] == b'/' || w[2] == b'\\'))
}

#[cfg(windows)]
#[test]
fn windows_debug_paths_use_virtual_roots_without_drive_prefixes() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let case_dir = Builder::new().prefix("debug-paths-win-").tempdir_in(&cwd).expect("tempdir");
    let source = case_dir.path().join("src/main.st");
    write_test_source(&source, source_code);

    let ir = compile_to_string_with_options(
        vec![canonical_path(&source)],
        vec![],
        CompileOptions {
            debug_level: DebugLevel::Full(plc::DEFAULT_DWARF_VERSION),
            optimization: plc::OptimizationLevel::None,
            debug_prefix_maps: vec![(cwd.clone(), PathBuf::from("/SOURCE_ROOT"))],
            debug_compilation_dir: Some(PathBuf::from("/BUILD_ROOT")),
            ..Default::default()
        },
    )
    .expect("compile succeeded")
    .join("\n");

    let case_name = case_dir_name(&case_dir);
    let snapshot = sanitize_debug_snapshot(&ir, &[(&case_name, "<CASE_ROOT>")]);

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "main.st", directory: "/SOURCE_ROOT/<CASE_ROOT>/src")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !10, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !11, splitDebugInlining: false)
!10 = !DIFile(filename: "/SOURCE_ROOT/<CASE_ROOT>/src/main.st", directory: "/BUILD_ROOT")
"#);

    assert!(
        !has_windows_drive_prefix(&snapshot),
        "expected no drive-letter paths in debug info:\n{snapshot}"
    );
}

#[cfg(not(windows))]
#[test]
fn debug_compile_unit_uses_path_relative_to_current_working_directory_by_default() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let case_dir = Builder::new().prefix("debug-paths-default-").tempdir_in(&cwd).expect("tempdir");
    let source = case_dir.path().join("src/main.st");
    write_test_source(&source, source_code);
    let source = canonical_path(&source);

    let ir = compile_to_string_with_options(
        vec![source],
        vec![],
        CompileOptions {
            debug_level: DebugLevel::Full(plc::DEFAULT_DWARF_VERSION),
            optimization: plc::OptimizationLevel::None,
            ..Default::default()
        },
    )
    .expect("compile succeeded")
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
fn debug_compile_unit_uses_relative_path_when_source_is_outside_compile_dir() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let compile_root = Builder::new().prefix("debug-paths-build-root-").tempdir_in(&cwd).expect("tempdir");
    let source_root =
        Builder::new().prefix("debug-paths-source-root-").tempdir_in(tempdir_base(&cwd)).expect("tempdir");

    let source_root_path = canonical_path(source_root.path());
    let source = source_root_path.join("nested/main.st");
    write_test_source(&source, source_code);

    let source = canonical_path(&source);
    let compile_root = canonical_path(compile_root.path());
    let relative_outside = relative_path_from(&compile_root, &source);

    let ir = compile_to_string_with_options(
        vec![source],
        vec![],
        CompileOptions {
            root: Some(compile_root.clone()),
            debug_level: DebugLevel::Full(plc::DEFAULT_DWARF_VERSION),
            optimization: plc::OptimizationLevel::None,
            ..Default::default()
        },
    )
    .expect("compile succeeded")
    .join("\n");

    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&compile_root.to_string_lossy()), "<BUILD_ROOT_REAL>"),
            (&normalize_snapshot_paths(&source_root_path.to_string_lossy()), "<SOURCE_ROOT_REAL>"),
            (&normalize_snapshot_paths(&relative_outside.to_string_lossy()), "<RELATIVE_OUTSIDE_SOURCE>"),
        ],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "<RELATIVE_OUTSIDE_SOURCE>", directory: "<BUILD_ROOT_REAL>")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !10, splitDebugInlining: false)
"#);
}

#[cfg(not(windows))]
#[test]
fn debug_project_config_inside_root_uses_relative_compile_unit_name() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let project_root = Builder::new().prefix("debug-paths-project-root-").tempdir_in(&cwd).expect("tempdir");

    let source = project_root.path().join("src/main.st");
    write_test_source(&source, source_code);

    let config = project_root.path().join("plc.json");
    fs::write(
        &config,
        r#"{
  "name": "TestProject",
  "files": ["src/main.st"],
  "compile_type": "Shared"
}
"#,
    )
    .expect("failed to write config");

    let ir = compile_build_config_to_string(&config, &["-g"]).expect("compile succeeded");
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[(&normalize_snapshot_paths(&project_root.path().to_string_lossy()), "<PROJECT_ROOT>")],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "src/main.st", directory: "<PROJECT_ROOT>")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !10, splitDebugInlining: false)
"#);
}

#[cfg(not(windows))]
#[test]
fn debug_project_config_outside_root_uses_relative_compile_unit_name() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let config_root = Builder::new().prefix("debug-paths-config-root-").tempdir_in(&cwd).expect("tempdir");
    let source_root =
        Builder::new().prefix("debug-paths-source-root-").tempdir_in(tempdir_base(&cwd)).expect("tempdir");

    let source_root_path = canonical_path(source_root.path());
    let source = source_root_path.join("nested/main.st");
    write_test_source(&source, source_code);

    let source = canonical_path(&source);
    let relative_outside = relative_path_from(config_root.path(), &source);

    let config = config_root.path().join("plc.json");
    fs::write(
        &config,
        format!(
            r#"{{
  "name": "TestProject",
  "files": ["{}"],
  "compile_type": "Shared"
}}
"#,
            relative_outside.display()
        ),
    )
    .expect("failed to write config");

    let ir = compile_build_config_to_string(&config, &["-g"]).expect("compile succeeded");
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&config_root.path().to_string_lossy()), "<CONFIG_ROOT>"),
            (&normalize_snapshot_paths(&source_root_path.to_string_lossy()), "<SOURCE_ROOT_REAL>"),
            (&normalize_snapshot_paths(&relative_outside.to_string_lossy()), "<RELATIVE_OUTSIDE_SOURCE>"),
        ],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "<RELATIVE_OUTSIDE_SOURCE>", directory: "<CONFIG_ROOT>")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !10, splitDebugInlining: false)
"#);
}

#[cfg(not(windows))]
#[test]
fn debug_constructor_generation_keeps_debug_paths_stable() {
    let source_code = r#"FUNCTION_BLOCK foo
VAR
    x : INT;
END_VAR
END_FUNCTION_BLOCK

PROGRAM prg
VAR
    f : foo;
END_VAR
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let case_dir = Builder::new().prefix("debug-paths-ctors-").tempdir_in(&cwd).expect("tempdir");
    let source = case_dir.path().join("test.st");
    write_test_source(&source, source_code);
    let source = canonical_path(&source);

    let ir = compile_args_to_string(&[
        "plc".to_string(),
        source.to_string_lossy().to_string(),
        "--ir".to_string(),
        "--single-module".to_string(),
        "-O".to_string(),
        "none".to_string(),
        "--constructors-only".to_string(),
        "-g".to_string(),
    ])
    .expect("compile succeeded");

    let case_name = case_dir_name(&case_dir);
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[(&normalize_snapshot_paths(&cwd.to_string_lossy()), "<CWD>"), (&case_name, "<CASE_ROOT>")],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "<CASE_ROOT>/test.st", directory: "<CWD>")
!16 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !17, splitDebugInlining: false)
"#);

    assert!(ir.contains("@__unit_test_st__ctor"), "expected constructor function in IR, got:\n{ir}");
    assert!(
        !ir.contains("declare !dbg"),
        "expected constructors-only declarations without !dbg attachments, got:\n{ir}"
    );
}

#[cfg(not(windows))]
#[test]
fn debug_prefix_map_rewrites_debug_paths_without_leaking_local_paths() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let case_dir = Builder::new().prefix("debug-paths-prefix-map-").tempdir_in(&cwd).expect("tempdir");
    let source = case_dir.path().join("src/main.st");
    write_test_source(&source, source_code);
    let source = canonical_path(&source);

    let virtual_root = virtual_root(&cwd);
    let ir = compile_to_string_with_options(
        vec![source],
        vec![],
        CompileOptions {
            debug_level: DebugLevel::Full(plc::DEFAULT_DWARF_VERSION),
            optimization: plc::OptimizationLevel::None,
            debug_prefix_maps: vec![(cwd.clone(), virtual_root.clone())],
            debug_compilation_dir: Some(virtual_root.clone()),
            ..Default::default()
        },
    )
    .expect("compile succeeded")
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
fn debug_mapped_source_root_outside_mapped_build_root_stays_absolute() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let case_dir = Builder::new().prefix("debug-paths-virtual-roots-").tempdir_in(&cwd).expect("tempdir");
    let source = case_dir.path().join("src/main.st");
    write_test_source(&source, source_code);
    let source = canonical_path(&source);

    let ir = compile_to_string_with_options(
        vec![source],
        vec![],
        CompileOptions {
            debug_level: DebugLevel::Full(plc::DEFAULT_DWARF_VERSION),
            optimization: plc::OptimizationLevel::None,
            debug_prefix_maps: vec![(cwd.clone(), PathBuf::from("/SOURCE_ROOT"))],
            debug_compilation_dir: Some(PathBuf::from("/BUILD_ROOT")),
            ..Default::default()
        },
    )
    .expect("compile succeeded")
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
fn debug_compilation_dir_mapping_keeps_parent_relative_filename_without_host_leakage() {
    let source_code = r#"PROGRAM prg
VAR
    x : INT;
END_VAR
    x := 1;
END_PROGRAM
"#;

    let cwd = canonical_cwd();
    let case_dir = Builder::new().prefix("debug-paths-parent-relative-").tempdir_in(&cwd).expect("tempdir");
    let compile_dir = case_dir.path().join("test");
    fs::create_dir_all(&compile_dir).expect("failed to create compile dir");

    let source = case_dir.path().join("dbg.st");
    write_test_source(&source, source_code);

    let source = canonical_path(&source);
    let compile_dir = canonical_path(&compile_dir);

    let ir = compile_args_to_string(&[
        "plc".to_string(),
        "--ir".to_string(),
        "-g".to_string(),
        source.to_string_lossy().to_string(),
        "--file-prefix-map".to_string(),
        format!("{}=/root", compile_dir.to_string_lossy()),
        "--debug-compilation-dir".to_string(),
        compile_dir.to_string_lossy().to_string(),
    ])
    .expect("compile succeeded");

    let case_name = case_dir_name(&case_dir);
    let snapshot = sanitize_debug_snapshot(&ir, &[(&case_name, "<CASE_ROOT>")]);

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "../dbg.st", directory: "/root")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, globals: !10, splitDebugInlining: false)
"#);

    assert!(
        !snapshot.contains("<CASE_ROOT>"),
        "expected no host path leakage in debug info, got:\n{snapshot}"
    );
}
