// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::{env, fs};

use encoding_rs::Encoding;
use rusty::{
    compile_to_bitcode, compile_to_ir, compile_to_shared_object, compile_to_shared_pic_object,
    compile_to_static_obj, diagnostics::Diagnostician, FilePath,
};

use crate::get_test_file;

fn compile_all(name: &str, encoding: Option<&'static Encoding>) {
    let path = get_test_file(name);
    let mut out = env::temp_dir();
    let out_name = format!("{}.out", &name);
    out.push(out_name);
    let out = out.into_os_string().into_string().unwrap();
    compile_to_ir(
        vec![FilePath { path: path.clone() }],
        encoding,
        &out,
        Diagnostician::default(),
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    compile_to_bitcode(
        vec![FilePath { path: path.clone() }],
        encoding,
        &out,
        Diagnostician::default(),
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    compile_to_shared_object(
        vec![FilePath { path: path.clone() }],
        encoding,
        &out,
        None,
        Diagnostician::default(),
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    compile_to_shared_pic_object(
        vec![FilePath { path: path.clone() }],
        encoding,
        &out,
        None,
        Diagnostician::default(),
    )
    .unwrap();
    fs::remove_file(&out).unwrap();
    compile_to_static_obj(
        vec![FilePath { path }],
        encoding,
        &out,
        None,
        Diagnostician::default(),
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
