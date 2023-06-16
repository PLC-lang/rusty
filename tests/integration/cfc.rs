use driver::runner::compile_and_run;

use crate::get_test_file;

#[test]
fn cfc_variables_pass_through() {
    let cfc_file = get_test_file("cfc/pass_through.xml");
    let st_file = get_test_file("cfc/call_pass_through.st");

    struct MainType {
        x: i32,
        y: i32,
    }

    let mut maintype = MainType { x: 100, y: 200 };
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut maintype);
    assert_eq!(maintype.x + maintype.y, res);
}
