use std::{env, fs};

use crate::get_test_file;
use inkwell::context::Context;
use rusty::{
    build_and_link, compile_module,
    diagnostics::{Diagnostic, Diagnostician},
    link, persist, CompileOptions, ErrorFormat, FilePath, FormatOption, LinkOptions, Target,
};

static TARGET: Option<&str> = Some("x86_64-linux-gnu");

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
    build_and_link(
        vec![file2],
        vec![],
        None,
        &CompileOptions {
            build_location: None,
            output: out2.clone(),
            format: FormatOption::Shared,
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
            debug_level: rusty::DebugLevel::None,
        },
        vec![TARGET.unwrap().into()],
        None,
        Default::default(),
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build_and_link(
        vec![file1, out2.as_str().into()],
        vec![],
        None,
        &CompileOptions {
            build_location: None,
            output: out1.clone(),
            format: FormatOption::Shared,
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
            debug_level: rusty::DebugLevel::None,
        },
        vec![TARGET.unwrap().into()],
        None,
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            format: FormatOption::Shared,
            linker: None,
        }),
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
    build_and_link(
        vec![file2],
        vec![],
        None,
        &CompileOptions {
            build_location: None,
            output: out2.clone(),
            format: FormatOption::PIC,
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
            debug_level: rusty::DebugLevel::None,
        },
        vec![TARGET.unwrap().into()],
        None,
        Default::default(),
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build_and_link(
        vec![file1, out2.as_str().into()],
        vec![],
        None,
        &CompileOptions {
            build_location: None,
            output: out1.clone(),
            format: FormatOption::PIC,
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
            debug_level: rusty::DebugLevel::None,
        },
        vec![TARGET.unwrap().into()],
        None,
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            format: FormatOption::PIC,
            linker: None,
        }),
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
    build_and_link(
        vec![file2],
        vec![],
        None,
        &CompileOptions {
            build_location: None,
            output: out2.clone(),
            format: FormatOption::Object,
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
            debug_level: rusty::DebugLevel::None,
        },
        vec![TARGET.unwrap().into()],
        None,
        Default::default(),
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build_and_link(
        vec![file1, out2.as_str().into()],
        vec![],
        None,
        &CompileOptions {
            build_location: None,
            output: out1.clone(),
            format: FormatOption::Static,
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
            debug_level: rusty::DebugLevel::None,
        },
        vec![TARGET.unwrap().into()],
        None,
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            format: FormatOption::Static,
            linker: None,
        }),
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
    build_and_link(
        vec![file2],
        vec![],
        None,
        &CompileOptions {
            build_location: None,
            output: out2.clone(),
            format: FormatOption::Object,
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
            debug_level: rusty::DebugLevel::None,
        },
        vec![TARGET.unwrap().into()],
        None,
        Default::default(),
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build_and_link(
        vec![file1, out2.as_str().into()],
        vec![],
        None,
        &CompileOptions {
            build_location: None,
            output: out1.clone(),
            format: FormatOption::Relocatable,
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
            debug_level: rusty::DebugLevel::None,
        },
        vec![TARGET.unwrap().into()],
        None,
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            format: FormatOption::Relocatable,
            linker: None,
        }),
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
    let target: Target = TARGET.unwrap().into();
    //Compile file1 as shared object with file2 as param
    let context = Context::create();
    let diagnostician = Diagnostician::default();
    let (_, codegen) = compile_module(
        &context,
        vec![file1],
        vec![],
        None,
        diagnostician,
        rusty::OptimizationLevel::None,
        rusty::DebugLevel::None,
    )
    .unwrap();
    let object = persist(
        &codegen,
        &out,
        FormatOption::Static,
        &target.get_target_triple(),
        rusty::OptimizationLevel::Default,
    )
    .unwrap();
    let res = link(
        &out,
        FormatOption::Static,
        &[object],
        &[],
        &[],
        &target,
        None,
    );

    match res {
        Err(err) => {
            let out = out.to_string_lossy();
            assert_eq!(Diagnostic::link_error(&format!("lld: error: undefined symbol: func2\n>>> referenced by main\n>>>               {}:(func1)\n>>> did you mean: func1\n>>> defined in: {}\n",out, out)), err);
        }
        _ => panic!("Expected link failure"),
    }

    //Delete it
    fs::remove_file(&out).unwrap();
}

#[test]
#[cfg_attr(target_os = "windows", ignore = "linker is not available for windows")]
//This is a regression, see #548
fn link_to_a_relative_location_with_no_parent() {
    let file1 = FilePath {
        path: get_test_file("linking/relative.st"),
    };

    //Compile file1 as shared object with file2 as param
    build_and_link(
        vec![file1],
        vec![],
        None,
        &CompileOptions {
            build_location: None,
            output: "output.o".into(),
            format: FormatOption::Static,
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
            debug_level: rusty::DebugLevel::None,
        },
        vec![],
        None,
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            format: FormatOption::Static,
            linker: None,
        }),
    )
    .unwrap();

    //Make sure the file exists in the test location
    let res = std::path::Path::new("output.o");
    assert!(res.exists());

    //Delete it
    fs::remove_file(&res).unwrap();
}
