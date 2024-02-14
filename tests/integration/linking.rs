use std::{env, fs};

use crate::get_test_file;
use driver::compile;

static TARGET: Option<&str> = Some("x86_64-linux-gnu");

#[test]
fn link_as_shared_object() {
    let file1 = get_test_file("linking/file1.st");
    let file2 = get_test_file("linking/file2.st");

    let mut out = env::temp_dir();
    out.push("shared1.o");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("shared2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj
    compile(&["plc", file2.as_str(), "-o", out2.as_str(), "-c", "--target", TARGET.unwrap()]).unwrap();
    //Compile file1 as shared object with file2 as param
    compile(&[
        "plc",
        file1.as_str(),
        out2.as_str(),
        "-o",
        out1.as_str(),
        "--shared",
        "--target",
        TARGET.unwrap(),
    ])
    .unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_as_pic_object() {
    let file1 = get_test_file("linking/file1.st");
    let file2 = get_test_file("linking/file2.st");

    let mut out = env::temp_dir();
    out.push("pic1.o");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("pic2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj
    compile(&["plc", file2.as_str(), "-o", out2.as_str(), "-c", "--target", TARGET.unwrap()]).unwrap();
    //Compile file1 as shared object with file2 as param
    compile(&[
        "plc",
        file1.as_str(),
        out2.as_str(),
        "-o",
        out1.as_str(),
        "--pic",
        "--target",
        TARGET.unwrap(),
    ])
    .unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_as_static_object() {
    let file1 = get_test_file("linking/file1.st");
    let file2 = get_test_file("linking/file2.st");

    let mut out = env::temp_dir();
    out.push("static1.o");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("static2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj
    compile(&["plc", file2.as_str(), "-o", out2.as_str(), "-c", "--target", TARGET.unwrap()]).unwrap();
    //Compile file1 as shared object with file2 as param
    compile(&["plc", file1.as_str(), out2.as_str(), "-o", out1.as_str(), "--target", TARGET.unwrap()])
        .unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_as_relocatable_object() {
    let file1 = get_test_file("linking/file1.st");
    let file2 = get_test_file("linking/file2.st");

    let mut out = env::temp_dir();
    out.push("reloc1.o");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("reloc2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj
    compile(&["plc", file2.as_str(), "-o", out2.as_str(), "-c", "--target", TARGET.unwrap()]).unwrap();
    //Compile file1 as shared object with file2 as param
    compile(&[
        "plc",
        file1.as_str(),
        out2.as_str(),
        "-o",
        out1.as_str(),
        "--relocatable",
        "--target",
        TARGET.unwrap(),
    ])
    .unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_missing_file() {
    let file1 = get_test_file("linking/file1.st");
    let mut out = env::temp_dir();
    out.push("missing.o");

    let res = compile(&["plc", file1.as_str(), "-o", out.to_str().unwrap(), "--target", TARGET.unwrap()]);

    match res {
        Err(err) => {
            assert!(err.to_string().contains("Compilation aborted due to previous errors"));
        }
        _ => panic!("Expected link failure"),
    }
}

// TODO: Ghaith please fix this :)
#[test]
#[cfg_attr(target_os = "windows", ignore = "linker is not available for windows")]
#[cfg_attr(target_os = "macos", ignore = "ignoring for now...")]
//This is a regression, see #548
fn link_to_a_relative_location_with_no_parent() {
    let file1 = get_test_file("linking/relative.st");
    compile(&["plc", file1.as_str(), "-o", "output.o", "--target", TARGET.unwrap()]).unwrap();

    //Make sure the file exists in the test location
    let res = std::path::Path::new("output.o");
    assert!(res.exists());

    //Delete it
    fs::remove_file(res).unwrap();
}

#[test]
fn link_with_initial_values() {
    let file1 = get_test_file("linking/init.st");
    let file2 = get_test_file("linking/init2.st");
    let file3 = get_test_file("linking/init3.st");

    let mut out = env::temp_dir();
    out.push("extern.o");
    let out1 = out.into_os_string().into_string().unwrap();

    //Compile file1 as shared object with file2 as param
    compile(&[
        "plc",
        file1.as_str(),
        file2.as_str(),
        file3.as_str(),
        "-o",
        out1.as_str(),
        "--shared",
        "--target",
        TARGET.unwrap(),
    ])
    .unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
}

#[test]
fn link_constants() {
    let file1 = get_test_file("linking/consts.st");

    let mut out = env::temp_dir();
    out.push("consts.o");
    let out1 = out.into_os_string().into_string().unwrap();

    //Compile file1 as shared object with file2 as param
    compile(&["plc", file1.as_str(), "-o", out1.as_str(), "--shared", "--target", TARGET.unwrap()]).unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
}

#[test]
fn link_files_with_same_name() {
    let file1 = get_test_file("linking/folder1/vars.st");
    let file2 = get_test_file("linking/folder2/vars.st");

    let mut out = env::temp_dir();
    out.push("consts.o");
    let out1 = out.into_os_string().into_string().unwrap();

    //Compile file1 as shared object with file2 as param
    compile(&[
        "plc",
        file1.as_str(),
        file2.as_str(),
        "-o",
        out1.as_str(),
        "--shared",
        "--target",
        TARGET.unwrap(),
    ])
    .unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
}

#[test]
fn link_files_with_same_name_but_different_extension() {
    let file1 = get_test_file("linking/consts.st");
    let file2 = get_test_file("linking/consts.dt");

    // We want to make sure that generating object files for two or more files with the same name but different
    // extensions works. Previously this would fail because both `const.st` and `const.dt` would persist to a
    // `const.o` file, which causes linking issues and more specifically "duplicate symbol" errors. Hence we only
    // check whether the compilation resulted in some Ok value here.
    assert!(compile(&["plc", file1.as_str(), file2.as_str(), "--target", TARGET.unwrap()]).is_ok());
}
