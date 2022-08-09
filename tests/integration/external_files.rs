// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::{env, fs};

use encoding_rs::Encoding;
use rusty::{build_and_link, CompileOptions, ErrorFormat, FilePath};

use crate::get_test_file;

fn compile_all(name: &str, encoding: Option<&'static Encoding>) {
    let path = get_test_file(name);
    let mut out = env::temp_dir();
    let out_name = format!("{}.out", &name);
    out.push(out_name);
    let out = out.into_os_string().into_string().unwrap();
    build_and_link(
        vec![FilePath { path: path.clone() }],
        vec![],
        encoding,
        &CompileOptions {
            format: Some(rusty::FormatOption::IR),
            output: out.clone(),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::default(),
        },
        vec![],
        None,
        None,
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    build_and_link(
        vec![FilePath { path: path.clone() }],
        vec![],
        encoding,
        &CompileOptions {
            format: Some(rusty::FormatOption::Bitcode),
            output: out.clone(),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::default(),
        },
        vec![],
        None,
        None,
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    build_and_link(
        vec![FilePath { path: path.clone() }],
        vec![],
        encoding,
        &CompileOptions {
            format: Some(rusty::FormatOption::Shared),
            output: out.clone(),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::default(),
        },
        vec![],
        None,
        None,
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    build_and_link(
        vec![FilePath { path: path.clone() }],
        vec![],
        encoding,
        &CompileOptions {
            format: Some(rusty::FormatOption::PIC),
            output: out.clone(),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::default(),
        },
        vec![],
        None,
        None,
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    build_and_link(
        vec![FilePath { path }],
        vec![],
        encoding,
        &CompileOptions {
            format: Some(rusty::FormatOption::Static),
            output: out.clone(),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::default(),
        },
        vec![],
        None,
        None,
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
