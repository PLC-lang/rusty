// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::{env, fs, path::PathBuf};

use encoding_rs::Encoding;
use rusty::{
    compile_to_bitcode, compile_to_ir, compile_to_shared_object, compile_to_shared_pic_object,
    compile_to_static_obj, FilePath,
};

fn compile_all(name: &str, encoding: Option<&'static Encoding>) {
    let path = get_file(name);
    let mut out = env::temp_dir();
    let out_name = format!("{}.out", &name);
    out.push(out_name);
    let out = out.into_os_string().into_string().unwrap();
    compile_to_ir(vec![FilePath { path: path.clone() }], encoding, &out).unwrap();
    fs::remove_file(&out).unwrap();
    compile_to_bitcode(vec![FilePath { path: path.clone() }], encoding, &out).unwrap();
    fs::remove_file(&out).unwrap();
    compile_to_shared_object(vec![FilePath { path: path.clone() }], encoding, &out, None).unwrap();
    fs::remove_file(&out).unwrap();
    compile_to_shared_pic_object(vec![FilePath { path: path.clone() }], encoding, &out, None)
        .unwrap();
    fs::remove_file(&out).unwrap();
    compile_to_static_obj(vec![FilePath { path: path.clone() }], encoding, &out, None).unwrap();
    fs::remove_file(&out).unwrap();
}

fn get_file(name: &str) -> String {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("tests");
    data_path.push("integration");
    data_path.push("data");
    data_path.push(name);

    assert!(data_path.exists());

    data_path.display().to_string()
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
