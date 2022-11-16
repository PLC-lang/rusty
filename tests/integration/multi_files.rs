use inkwell::{
    context::Context,
    targets::{InitializationConfig, Target},
};
use rusty::{
    compile_module, diagnostics::Diagnostician, runner::run_no_param, DebugLevel, FilePath,
    OptimizationLevel,
};

use crate::{compile_and_run, get_test_file};

#[test]
fn sources_accross_multiple_files_compiled() {
    let file1 = FilePath {
        path: get_test_file("multi/func.st"),
    };
    let file2 = FilePath {
        path: get_test_file("multi/prog.st"),
    };

    let res: i32 = compile_and_run(vec![file1, file2], &mut ());
    assert_eq!(42, res);
}

fn concat_date(y: i16, m: i16, d: i16) -> i64 {
    (y + m + d) as i64
}

#[test]
fn multiple_files_create_same_generic_implementation() {
    // GIVEN a generic function
    let gen_func = FilePath {
        path: get_test_file("multi/concat_date.st"),
    };

    // AND two file requesting different implementations via generic call
    let file1 = FilePath {
        path: get_test_file("multi/concat_date_prg1.st"),
    };
    let file2 = FilePath {
        path: get_test_file("multi/concat_date_prg2.st"),
    };

    //WHEN i compile the project
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let context: Context = Context::create();

    let (_, code_gen) = compile_module(
        &context,
        vec![gen_func, file1, file2],
        vec![],
        None,
        Diagnostician::default(),
        OptimizationLevel::None,
        DebugLevel::None,
    )
    .unwrap();

    let exec_engine = code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    // THEN both calls from foo1 and foo2 should target the same implementation
    let fn_value = code_gen.module.get_function("CONCAT_DATE__INT").unwrap();
    exec_engine.add_global_mapping(&fn_value, concat_date as usize);

    let res: i64 = run_no_param(&exec_engine, "foo1");
    assert_eq!(res, 1 + 2 + 3);

    let res: i64 = run_no_param(&exec_engine, "foo2");
    assert_eq!(res, 4 + 5 + 6);
}
