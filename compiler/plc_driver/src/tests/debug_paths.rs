use std::{
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

fn write_test_source(path: &PathBuf) {
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
    value.replace("\\\\?\\", "").replace('\\', "/")
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

fn tempdir_base(cwd: &Path) -> &Path {
    cwd.parent().unwrap_or(cwd)
}

fn virtual_root(cwd: &Path) -> PathBuf {
    tempdir_base(cwd).join("__virtual__/TestProject")
}

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

#[test]
fn debug_compile_unit_uses_path_relative_to_current_working_directory_by_default() {
    let cwd = canonical_cwd();
    let tempdir = Builder::new().prefix("rusty-debug-paths-").tempdir_in(&cwd).expect("tempdir");
    let source = tempdir.path().join("src/main.st");
    write_test_source(&source);
    let source = canonical_path(&source);
    let relative = source.strip_prefix(&cwd).expect("source under cwd").to_string_lossy().to_string();

    let results = compile_to_string_with_options(
        vec![source],
        vec![],
        CompileOptions {
            debug_level: DebugLevel::Full(plc::DEFAULT_DWARF_VERSION),
            optimization: plc::OptimizationLevel::None,
            ..Default::default()
        },
    )
    .expect("compile succeeded");

    let ir = results.join("\n");
    let tempdir_name = tempdir.path().file_name().unwrap().to_string_lossy().to_string();
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&cwd.to_string_lossy()), "/cwd"),
            (&tempdir_name, "rusty-debug-paths-[id]"),
            (&normalize_snapshot_paths(&relative), "rusty-debug-paths-[id]/src/main.st"),
        ],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "main.st", directory: "/cwd/rusty-debug-paths-[id]/src")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !10, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !11, splitDebugInlining: false)
!10 = !DIFile(filename: "rusty-debug-paths-[id]/src/main.st", directory: "/cwd")
"#);
}

#[test]
fn debug_compile_unit_uses_relative_path_when_source_is_outside_compile_dir() {
    let cwd = canonical_cwd();
    let compile_root = Builder::new().prefix("rusty-compile-root-").tempdir_in(&cwd).expect("tempdir");
    let source_root =
        Builder::new().prefix("rusty-source-root-").tempdir_in(tempdir_base(&cwd)).expect("tempdir");
    let source_root_path = canonical_path(source_root.path());
    let source = source_root_path.join("nested/main.st");
    write_test_source(&source);
    let source = canonical_path(&source);

    let compile_root = canonical_path(compile_root.path());

    let results = compile_to_string_with_options(
        vec![source.clone()],
        vec![],
        CompileOptions {
            root: Some(compile_root.clone()),
            debug_level: DebugLevel::Full(plc::DEFAULT_DWARF_VERSION),
            optimization: plc::OptimizationLevel::None,
            ..Default::default()
        },
    )
    .expect("compile succeeded");

    let ir = results.join("\n");
    let compile_root_name = compile_root.file_name().unwrap().to_string_lossy().to_string();
    let source_root_name = source_root_path.file_name().unwrap().to_string_lossy().to_string();
    let relative_outside = relative_path_from(&compile_root, &source);
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&compile_root.to_string_lossy()), "/cwd/rusty-compile-root-[id]"),
            (&compile_root_name, "rusty-compile-root-[id]"),
            (
                &normalize_snapshot_paths(&relative_outside.to_string_lossy()),
                "[relative-outside-root]/nested/main.st",
            ),
            (&normalize_snapshot_paths(&source_root_path.to_string_lossy()), "/outside-root"),
            (&source_root_name, "source-root-[id]"),
        ],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "main.st", directory: "/outside-root/nested")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !10, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !11, splitDebugInlining: false)
!10 = !DIFile(filename: "[relative-outside-root]/nested/main.st", directory: "/cwd/rusty-compile-root-[id]")
"#);
}

#[test]
fn debug_project_config_inside_root_uses_relative_compile_unit_name() {
    let cwd = canonical_cwd();
    let project_dir = Builder::new().prefix("rusty-project-root-").tempdir_in(&cwd).expect("tempdir");
    let source = project_dir.path().join("src/main.st");
    write_test_source(&source);
    let config = project_dir.path().join("plc.json");
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
    let project_dir_name = project_dir.path().file_name().unwrap().to_string_lossy().to_string();
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&cwd.to_string_lossy()), "/cwd"),
            (&project_dir_name, "rusty-project-root-[id]"),
        ],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "main.st", directory: "/cwd/rusty-project-root-[id]/src")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !10, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !11, splitDebugInlining: false)
!10 = !DIFile(filename: "src/main.st", directory: "/cwd/rusty-project-root-[id]")
"#);
}

#[test]
fn debug_project_config_outside_root_uses_relative_compile_unit_name() {
    let cwd = canonical_cwd();
    let config_root = Builder::new().prefix("rusty-config-root-").tempdir_in(&cwd).expect("tempdir");
    let source_root =
        Builder::new().prefix("rusty-source-root-").tempdir_in(tempdir_base(&cwd)).expect("tempdir");
    let source_root_path = canonical_path(source_root.path());
    let source = source_root_path.join("nested/main.st");
    write_test_source(&source);
    let source = canonical_path(&source);
    let relative_outside = relative_path_from(config_root.path(), &source);
    let config = config_root.path().join("plc.json");
    fs::write(
        &config,
        format!(
            "{{\n  \"name\": \"TestProject\",\n  \"files\": [\"{}\"],\n  \"compile_type\": \"Shared\"\n}}\n",
            normalize_snapshot_paths(&relative_outside.to_string_lossy())
        ),
    )
    .expect("failed to write config");

    let ir = compile_build_config_to_string(&config, &["-g"]).expect("compile succeeded");
    let config_root_name = config_root.path().file_name().unwrap().to_string_lossy().to_string();
    let source_root_name = source_root_path.file_name().unwrap().to_string_lossy().to_string();
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&cwd.to_string_lossy()), "/cwd"),
            (&config_root_name, "rusty-config-root-[id]"),
            (
                &normalize_snapshot_paths(&relative_outside.to_string_lossy()),
                "[relative-outside-root]/nested/main.st",
            ),
            (&normalize_snapshot_paths(&source_root_path.to_string_lossy()), "/outside-root"),
            (&source_root_name, "source-root-[id]"),
        ],
    );

    assert_snapshot!(snapshot, @r#"
!2 = !DIFile(filename: "main.st", directory: "/outside-root/nested")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !10, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !11, splitDebugInlining: false)
!10 = !DIFile(filename: "[relative-outside-root]/nested/main.st", directory: "/cwd/rusty-config-root-[id]")
"#);
}

#[test]
fn debug_constructor_generation_keeps_debug_paths_stable() {
    let cwd = canonical_cwd();
    let tempdir = Builder::new().prefix("rusty-ctors-").tempdir_in(&cwd).expect("tempdir");
    let source = tempdir.path().join("test.st");
    fs::write(
        &source,
        "FUNCTION_BLOCK foo\nVAR\n    x : INT;\nEND_VAR\nEND_FUNCTION_BLOCK\n\nPROGRAM prg\nVAR\n    f : foo;\nEND_VAR\nEND_PROGRAM\n",
    )
    .expect("failed to write source");
    let source = canonical_path(&source);
    let relative = source.strip_prefix(&cwd).expect("source under cwd").to_string_lossy().to_string();

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
    let tempdir_name = tempdir.path().file_name().unwrap().to_string_lossy().to_string();
    let snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&cwd.to_string_lossy()), "/cwd"),
            (&tempdir_name, "rusty-ctors-[id]"),
            (&normalize_snapshot_paths(&relative), "rusty-ctors-[id]/test.st"),
        ],
    );

    assert_snapshot!(snapshot, @r#"
    !2 = !DIFile(filename: "test.st", directory: "/cwd/rusty-ctors-[id]")
    !10 = !DIFile(filename: "rusty-ctors-[id]/test.st", directory: "/cwd")
    !17 = distinct !DICompileUnit(language: DW_LANG_C, file: !10, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !18, splitDebugInlining: false)
    "#);
    assert!(ir.contains("@__unit_test_st__ctor"), "expected constructor function in IR, got:\n{ir}");
    assert!(
        !ir.contains("declare !dbg"),
        "expected constructors-only declarations without !dbg attachments, got:\n{ir}"
    );
}

#[test]
fn debug_prefix_map_rewrites_debug_paths_without_leaking_local_paths() {
    let cwd = canonical_cwd();
    let tempdir = Builder::new().prefix("rusty-prefix-map-").tempdir_in(&cwd).expect("tempdir");
    let source = tempdir.path().join("src/main.st");
    write_test_source(&source);
    let source = canonical_path(&source);
    let relative = source.strip_prefix(&cwd).expect("source under cwd").to_string_lossy().to_string();

    let virtual_root = virtual_root(&cwd);
    let results = compile_to_string_with_options(
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
    .expect("compile succeeded");

    let ir = results.join("\n");
    let tempdir_name = tempdir.path().file_name().unwrap().to_string_lossy().to_string();
    let di_snapshot = sanitize_debug_snapshot(
        &ir,
        &[
            (&normalize_snapshot_paths(&cwd.to_string_lossy()), "/cwd"),
            (&normalize_snapshot_paths(&virtual_root.to_string_lossy()), "/src/TestProject"),
            (&tempdir_name, "rusty-prefix-map-[id]"),
            (&normalize_snapshot_paths(&relative), "rusty-prefix-map-[id]/src/main.st"),
        ],
    );

    assert_snapshot!(di_snapshot, @r#"
!2 = !DIFile(filename: "main.st", directory: "/src/TestProject/rusty-prefix-map-[id]/src")
!9 = distinct !DICompileUnit(language: DW_LANG_C, file: !10, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !11, splitDebugInlining: false)
!10 = !DIFile(filename: "rusty-prefix-map-[id]/src/main.st", directory: "/src/TestProject")
"#);
}
