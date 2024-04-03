use inkwell::targets::{InitializationConfig, Target};
use rusty::codegen::CodegenContext;

use crate::{compile, compile_and_run, get_test_file};

#[test]
fn sources_accross_multiple_files_compiled() {
    let file1 = get_test_file("multi/func.st");
    let file2 = get_test_file("multi/prog.st");

    let res: i32 = compile_and_run(vec![file1, file2], &mut ());
    assert_eq!(42, res);
}

fn concat_date(y: i16, m: i16, d: i16) -> i64 {
    (y + m + d) as i64
}

#[test]
fn multiple_files_create_same_generic_implementation() {
    // GIVEN a generic function
    let gen_func = get_test_file("multi/concat_date.st");

    // AND two file requesting different implementations via generic call
    let file1 = get_test_file("multi/concat_date_prg1.st");
    let file2 = get_test_file("multi/concat_date_prg2.st");

    //WHEN i compile the project
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context = CodegenContext::create();

    let module = compile(&context, vec![gen_func, file1, file2]);

    // THEN both calls from foo1 and foo2 should target the same implementation
    module.add_global_function_mapping("CONCAT_DATE__INT", concat_date as usize);

    let res: i64 = module.run_no_param("foo1");
    assert_eq!(res, 1 + 2 + 3);

    let res: i64 = module.run_no_param("foo2");
    assert_eq!(res, 4 + 5 + 6);
}

#[test]
fn same_variant_name_enums_in_separate_files_dont_cause_symbol_conflict() {
    // GIVEN two enums with variants of the same name in different files
    let file1 = get_test_file("multi/enum1.st");
    let file2 = get_test_file("multi/enum2.st");
    // WHEN compiling
    let context = CodegenContext::create();
    // THEN we do not expect any duplicate symbol/linking diagnostics (i.e. no panic on `unwrap()` in `compile`)
    let _ = compile(&context, vec![file1, file2]);
}
