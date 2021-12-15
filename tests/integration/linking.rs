use std::{env, fs};

use crate::get_test_file;
use rusty::{build, diagnostics::Diagnostic, CompileOptions, FilePath, FormatOption, LinkOptions};

#[test]
fn link_as_shared_object() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let file2 = FilePath {
        path: get_test_file("linking/file2.st"),
    };

    let mut out = env::temp_dir();
    out.push("shared1.so");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("shared2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj
    build(
        vec![file2],
        CompileOptions {
            output: out2.clone(),
            format: FormatOption::Shared,
            target: Some("x86_64-unkown-linux-gnu".to_string()),
        },
        None,
        None,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build(
        vec![file1, out2.as_str().into()],
        CompileOptions {
            output: out1.clone(),
            format: FormatOption::Shared,
            target: Some("x86_64-unkown-linux-gnu".to_string()),
        },
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            sysroot: None,
        }),
        None,
    )
    .unwrap();

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
    out.push("pic1.so");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("pic2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj

    build(
        vec![file2],
        CompileOptions {
            output: out2.clone(),
            format: FormatOption::PIC,
            target: Some("x86_64-unkown-linux-gnu".to_string()),
        },
        None,
        None,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build(
        vec![file1, out2.as_str().into()],
        CompileOptions {
            output: out1.clone(),
            format: FormatOption::PIC,
            target: Some("x86_64-unkown-linux-gnu".to_string()),
        },
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            sysroot: None,
        }),
        None,
    )
    .unwrap();

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
    out.push("static1.o");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("static2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj

    build(
        vec![file2],
        CompileOptions {
            output: out2.clone(),
            format: FormatOption::Static,
            target: Some("x86_64-unkown-linux-gnu".to_string()),
        },
        None,
        None,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build(
        vec![file1, out2.as_str().into()],
        CompileOptions {
            output: out1.clone(),
            format: FormatOption::Static,
            target: Some("x86_64-unkown-linux-gnu".to_string()),
        },
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            sysroot: None,
        }),
        None,
    )
    .unwrap();

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
    out.push("reloc1.o");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("reloc2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj

    build(
        vec![file2],
        CompileOptions {
            output: out2.clone(),
            format: FormatOption::Static,
            target: Some("x86_64-unkown-linux-gnu".to_string()),
        },
        None,
        None,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build(
        vec![file1, out2.as_str().into()],
        CompileOptions {
            output: out1.clone(),
            format: FormatOption::Relocatable,
            target: Some("x86_64-unkown-linux-gnu".to_string()),
        },
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            sysroot: None,
        }),
        None,
    )
    .unwrap();

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
    out.push("missing.o");
    let out = out.into_os_string().into_string().unwrap();
    //Compile file1 as shared object with file2 as param
    let res = build(
        vec![file1],
        CompileOptions {
            output: out.clone(),
            format: FormatOption::Static,
            target: Some("x86_64-unkown-linux-gnu".to_string()),
        },
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            sysroot: None,
        }),
        None,
    );

    match res {
        Err(err) => {
            assert_eq!(Diagnostic::link_error("lld: error: undefined symbol: func2\n>>> referenced by main\n>>>               /tmp/missing.o:(func1)\n>>> did you mean: func1\n>>> defined in: /tmp/missing.o\n"), err);
        }
        _ => panic!("Expected link failure"),
    }

    //Delete it
    fs::remove_file(&out).unwrap();
}
