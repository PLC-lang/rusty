use rusty::FilePath;

use crate::{compile_and_run_multi, get_test_file};

#[test]
fn sources_accross_multiple_files_compiled() {
    let file1 = FilePath { path : get_test_file("multi/func.st") };
    let file2 = FilePath { path : get_test_file("multi/prog.st") };

    let res : i32 = compile_and_run_multi(vec![file1,file2], &mut ());
    assert_eq!(42,res);

}