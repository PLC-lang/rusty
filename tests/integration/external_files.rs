// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::{env, fs};

use driver::compile;
use encoding_rs::Encoding;

use crate::get_test_file;

fn compile_all(name: &str, encoding: Option<&'static Encoding>) {
    let path = get_test_file(name);
    let mut out = env::temp_dir();
    let out_name = format!("{}.out", &name);
    out.push(out_name);
    let out = out.into_os_string().into_string().unwrap();
    let mut main_args = vec!["plc", &path, "-o", &out, "-Odefault", "--target", "x86_64-linux-pc"];
    if let Some(encoding) = encoding {
        main_args.push("--encoding");
        main_args.push(encoding.name());
    }

    let mut args = main_args.clone();
    args.push("--ir");
    compile(&args).unwrap();
    fs::remove_file(&out).unwrap();
    let mut args = main_args.clone();
    args.push("--bc");
    compile(&args).unwrap();
    fs::remove_file(&out).unwrap();
    let mut args = main_args.clone();
    args.push("--shared");
    compile(&args).unwrap();
    fs::remove_file(&out).unwrap();
    let mut args = main_args.clone();
    args.push("--pic");
    compile(&args).unwrap();
    fs::remove_file(&out).unwrap();
    let mut args = main_args.clone();
    args.push("--static");
    compile(&args).unwrap();
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
