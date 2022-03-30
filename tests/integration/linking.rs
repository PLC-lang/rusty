use std::{env, fs};

use crate::get_test_file;
use rusty::{
    build, diagnostics::Diagnostic, get_target_triple, link, CompileOptions, FilePath, FormatOption,
};

static TARGET: Option<&str> = Some("x86_64-unkown-linux-gnu");

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
    let triple = get_target_triple(TARGET);

    //Compile file 2 into obj
    build(
        vec![file2],
        vec![],
        &CompileOptions {
            output: out2.clone(),
            format: FormatOption::Shared,
            target: TARGET.map(String::from),
            optimization: rusty::OptimizationLevel::Default,
        },
        None,
        &triple,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    let res = build(
        vec![file1, out2.as_str().into()],
        vec![],
        &CompileOptions {
            output: out1.clone(),
            format: FormatOption::Shared,
            target: TARGET.map(String::from),
            optimization: rusty::OptimizationLevel::Default,
        },
        None,
        &triple,
    )
    .unwrap();

    link(
        &out1,
        FormatOption::Shared,
        &res.objects,
        vec![],
        vec![],
        &triple,
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
    let triple = get_target_triple(TARGET);

    build(
        vec![file2],
        vec![],
        &CompileOptions {
            output: out2.clone(),
            format: FormatOption::PIC,
            target: TARGET.map(String::from),
            optimization: rusty::OptimizationLevel::Default,
        },
        None,
        &triple,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    let res = build(
        vec![file1, out2.as_str().into()],
        vec![],
        &CompileOptions {
            output: out1.clone(),
            format: FormatOption::PIC,
            target: TARGET.map(String::from),
            optimization: rusty::OptimizationLevel::Default,
        },
        None,
        &triple,
    )
    .unwrap();

    link(
        &out1,
        FormatOption::PIC,
        &res.objects,
        vec![],
        vec![],
        &triple,
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
    let triple = get_target_triple(TARGET);

    build(
        vec![file2],
        vec![],
        &CompileOptions {
            output: out2.clone(),
            format: FormatOption::Static,
            target: TARGET.map(String::from),
            optimization: rusty::OptimizationLevel::Default,
        },
        None,
        &triple,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    let res = build(
        vec![file1, out2.as_str().into()],
        vec![],
        &CompileOptions {
            output: out1.clone(),
            format: FormatOption::Static,
            target: TARGET.map(String::from),
            optimization: rusty::OptimizationLevel::Default,
        },
        None,
        &triple,
    )
    .unwrap();

    link(
        &out1,
        FormatOption::Static,
        &res.objects,
        vec![],
        vec![],
        &triple,
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
    let triple = get_target_triple(TARGET);

    build(
        vec![file2],
        vec![],
        &CompileOptions {
            output: out2.clone(),
            format: FormatOption::Static,
            target: TARGET.map(String::from),
            optimization: rusty::OptimizationLevel::Default,
        },
        None,
        &triple,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    let res = build(
        vec![file1, out2.as_str().into()],
        vec![],
        &CompileOptions {
            output: out1.clone(),
            format: FormatOption::Relocatable,
            target: TARGET.map(String::from),
            optimization: rusty::OptimizationLevel::Default,
        },
        None,
        &triple,
    )
    .unwrap();

    link(
        &out1,
        FormatOption::Relocatable,
        &res.objects,
        vec![],
        vec![],
        &triple,
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
    let triple = get_target_triple(TARGET);
    //Compile file1 as shared object with file2 as param
    let res = build(
        vec![file1],
        vec![],
        &CompileOptions {
            output: out.clone(),
            format: FormatOption::Static,
            target: TARGET.map(String::from),
            optimization: rusty::OptimizationLevel::Default,
        },
        None,
        &triple,
    )
    .unwrap();

    let res = link(
        &out,
        FormatOption::Static,
        &res.objects,
        vec![],
        vec![],
        &triple,
        None,
    );

    match res {
        Err(err) => {
            assert_eq!(Diagnostic::link_error(&format!("lld: error: undefined symbol: func2\n>>> referenced by main\n>>>               {}:(func1)\n>>> did you mean: func1\n>>> defined in: {}\n",out, out)), err);
        }
        _ => panic!("Expected link failure"),
    }

    //Delete it
    fs::remove_file(&out).unwrap();
}
