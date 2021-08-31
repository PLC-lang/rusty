// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use rusty::*;

type MainFunction<T> = unsafe extern "C" fn(*mut T) -> i32;

mod correctness {
    mod arrays;
    mod classes;
    mod control_flow;
    mod custom_datatypes;
    mod datatypes;
    mod external_functions;
    mod functions;
    mod global_variables;
    mod initial_values;
    mod pointers;
    mod sub_range_types;
    mod sums;
}

mod integration {
    mod external_files;
}

#[macro_export]
macro_rules! assert_almost_eq {
    ($left:expr, $right:expr, $prec:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                let diff = (left_val - right_val).abs();

                if diff > $prec {
                    panic!(
                        "assertion failed: `(left == right)`\n      left: `{:?}`,\n     right: `{:?}`",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
}

///
/// Compiles and runs the given source
/// Returns the std result as String
/// Source must define a main function that takes no arguments and returns an int and string
/// The int is the return value which can be verified
/// The string will eventually be the Stdout of the function.
///
pub fn compile(context: &Context, source: String) -> ExecutionEngine {
    let source = SourceCode {
        path: "external_test.st".to_string(),
        source,
    };
    let code_gen = compile_module(context, vec![source], None).unwrap();
    code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap()
}
pub fn compile_and_run<T>(source: String, params: &mut T) -> (i32, &'static str) {
    let context: Context = Context::create();
    let exec_engine = compile(&context, source);
    run::<T>(&exec_engine, "main", params)
}

pub fn run<T>(exec_engine: &ExecutionEngine, name: &str, params: &mut T) -> (i32, &'static str) {
    unsafe {
        let main: JitFunction<MainFunction<T>> = exec_engine.get_function(name).unwrap();
        let main_t_ptr = &mut *params as *mut _;
        let int_res = main.call(main_t_ptr);
        (int_res, "")
    }
}
