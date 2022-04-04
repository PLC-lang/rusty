// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::{env, fs};

use encoding_rs::Encoding;
use rusty::{build, get_target_triple, CompileOptions, FilePath};

use crate::get_test_file;

fn compile_all(name: &str, encoding: Option<&'static Encoding>) {
    let path = get_test_file(name);
    let mut out = env::temp_dir();
    let out_name = format!("{}.out", &name);
    out.push(out_name);
    let out = out.into_os_string().into_string().unwrap();
    let target = get_target_triple(None);
    build(
        vec![FilePath { path: path.clone() }],
        vec![],
        &CompileOptions {
            format: rusty::FormatOption::IR,
            output: out.clone(),
            target: None,
            optimization: rusty::OptimizationLevel::Default,
        },
        encoding,
        "rich",
        &target,
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    build(
        vec![FilePath { path: path.clone() }],
        vec![],
        &CompileOptions {
            format: rusty::FormatOption::Bitcode,
            output: out.clone(),
            target: None,
            optimization: rusty::OptimizationLevel::Default,
        },
        encoding,
        "rich",
        &target,
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    build(
        vec![FilePath { path: path.clone() }],
        vec![],
        &CompileOptions {
            format: rusty::FormatOption::Shared,
            output: out.clone(),
            target: None,
            optimization: rusty::OptimizationLevel::Default,
        },
        encoding,
        "rich",
        &target,
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    build(
        vec![FilePath { path: path.clone() }],
        vec![],
        &CompileOptions {
            format: rusty::FormatOption::PIC,
            output: out.clone(),
            target: None,
            optimization: rusty::OptimizationLevel::Default,
        },
        encoding,
        "rich",
        &target,
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    build(
        vec![FilePath { path }],
        vec![],
        &CompileOptions {
            format: rusty::FormatOption::Static,
            output: out.clone(),
            target: None,
            optimization: rusty::OptimizationLevel::Default,
        },
        encoding,
        "rich",
        &target,
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
}

#[test]
fn compile_external_file() {
    compile_all("test_file.st", None);
}

#[test]
fn compile_external_file_with_encoding() {
    compile_all("encoding_utf_16.st", None);
    compile_all("encoding_win.st", Some(encoding_rs::WINDOWS_1252));
}
