use std::{env, fs};

use crate::get_test_file;
use rusty::{build, FilePath, FormatOption, LinkOption, diagnostics::Diagnostic};

#[test]
fn link_as_shared_object() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let file2 = FilePath {
        path: get_test_file("linking/file2.st"),
    };

    let mut out = env::temp_dir();
    out.push("file1.so");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("file2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj

    build(
        vec![file2],
        &out2,
        FormatOption::Shared,
        Some("x86_64-unkown-linux-gnu".to_string()), 
        LinkOption::Compile,
        vec![],
        vec![],
        None,
        None,
    ).unwrap();
    
    //Compile file1 as shared object with file2 as param
    build(
        vec![file1, out2.as_str().into()],
        &out1,
        FormatOption::Shared,
        Some("x86_64-unkown-linux-gnu".to_string()), 
        LinkOption::Link,
        vec![],
        vec![],
        None,
        None,
    ).unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_as_pic_object() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let file2 = FilePath {
        path: get_test_file("linking/file2.st"),
    };

    let mut out = env::temp_dir();
    out.push("file1.so");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("file2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj

    build(
        vec![file2],
        &out2,
        FormatOption::PIC,
        Some("x86_64-unkown-linux-gnu".to_string()), 
        LinkOption::Compile,
        vec![],
        vec![],
        None,
        None,
    ).unwrap();
    
    //Compile file1 as shared object with file2 as param
    build(
        vec![file1, out2.as_str().into()],
        &out1,
        FormatOption::PIC,
        Some("x86_64-unkown-linux-gnu".to_string()), 
        LinkOption::Link,
        vec![],
        vec![],
        None,
        None,
    ).unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_as_static_object() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let file2 = FilePath {
        path: get_test_file("linking/file2.st"),
    };

    let mut out = env::temp_dir();
    out.push("file1.o");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("file2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj

    build(
        vec![file2],
        &out2,
        FormatOption::Static,
        Some("x86_64-unkown-linux-gnu".to_string()), 
        LinkOption::Compile,
        vec![],
        vec![],
        None,
        None,
    ).unwrap();
    
    //Compile file1 as shared object with file2 as param
    build(
        vec![file1, out2.as_str().into()],
        &out1,
        FormatOption::Static,
        Some("x86_64-unkown-linux-gnu".to_string()), 
        LinkOption::Link,
        vec![],
        vec![],
        None,
        None,
    ).unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_as_relocatable_object() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let file2 = FilePath {
        path: get_test_file("linking/file2.st"),
    };

    let mut out = env::temp_dir();
    out.push("file1.o");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("file2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj

    build(
        vec![file2],
        &out2,
        FormatOption::Static,
        Some("x86_64-unkown-linux-gnu".to_string()), 
        LinkOption::Compile,
        vec![],
        vec![],
        None,
        None,
    ).unwrap();
    
    //Compile file1 as shared object with file2 as param
    build(
        vec![file1, out2.as_str().into()],
        &out1,
        FormatOption::Relocatable,
        Some("x86_64-unkown-linux-gnu".to_string()), 
        LinkOption::Link,
        vec![],
        vec![],
        None,
        None,
    ).unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_missing_file() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let mut out = env::temp_dir();
    out.push("file1.o");
    let out = out.into_os_string().into_string().unwrap();
    //Compile file1 as shared object with file2 as param
    let res = build(
        vec![file1],
        &out,
        FormatOption::Static,
        Some("x86_64-unkown-linux-gnu".to_string()), 
        LinkOption::Link,
        vec![],
        vec![],
        None,
        None,
    );

    match res {
        Err(err) => {
            assert_eq!(Diagnostic::link_error("lld: error: undefined symbol: func2\n>>> referenced by main\n>>>               /tmp/file1.o:(func1)\n>>> did you mean: func1\n>>> defined in: /tmp/file1.o\n"), err);
        }
        _ => panic!("Expected link failure"),
    }

    //Delete it
    fs::remove_file(&out).unwrap();
}
