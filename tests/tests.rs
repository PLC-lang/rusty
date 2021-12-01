use std::path::PathBuf;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use rusty::diagnostics::Diagnostician;
use rusty::*;

type MainFunction<T, U> = unsafe extern "C" fn(*mut T) -> U;

#[allow(dead_code)]
#[repr(C)]
pub struct MainType {
    a: [usize; 1000],
}

impl Default for MainType {
    fn default() -> Self {
        MainType { a: [0; 1000] }
    }
}

mod correctness {
    mod arrays;
    mod bitaccess;
    mod classes;
    mod control_flow;
    mod custom_datatypes;
    mod datatypes;
    mod expressions;
    mod external_functions;
    mod functions;
    mod generic_functions;
    mod global_variables;
    mod initial_values;
    mod methods;
    mod pointers;
    mod sub_range_types;
    mod math_operators {
        mod addition;
        mod division;
        mod mixed;
        mod multiplication;
        mod substraction;
    }
}

mod integration {
    mod external_files;
    mod multi_files;
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

/// Gets a file from the integration data folder for tests
fn get_test_file(name: &str) -> String {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("tests");
    data_path.push("integration");
    data_path.push("data");
    data_path.push(name);

    assert!(data_path.exists());

    data_path.display().to_string()
}

pub trait Compilable {
    type T: SourceContainer;
    fn containers(self) -> Vec<Self::T>;
}

impl Compilable for &str {
    type T = SourceCode;
    fn containers(self) -> Vec<Self::T> {
        let code = Self::T::from(self);
        vec![code]
    }
}

impl Compilable for String {
    type T = SourceCode;
    fn containers(self) -> Vec<Self::T> {
        let code = self.into();
        vec![code]
    }
}

impl<S: SourceContainer> Compilable for Vec<S> {
    type T = S;
    fn containers(self) -> Vec<Self::T> {
        self
    }
}

///
/// Compiles and runs the given source
/// Returns the std result as String
/// Source must define a main function that takes no arguments and returns an int and string
/// The int is the return value which can be verified
/// The string will eventually be the Stdout of the function.
///
// pub fn compile(context: &Context, source: String) -> ExecutionEngine {
//     let source: Vec<SourceCode> = vec![source.as_str().into()];
//     compile_multi(context, source)
// }

// pub fn compile_and_run<T, U>(source: String, params: &mut T) -> U {
//     compile_and_run_multi::<T, U, SourceCode>(vec![source.as_str().into()], params)
// }

pub fn run<T, U>(exec_engine: &ExecutionEngine, name: &str, params: &mut T) -> U {
    unsafe {
        let main: JitFunction<MainFunction<T, U>> = exec_engine.get_function(name).unwrap();
        let main_t_ptr = &mut *params as *mut _;

        main.call(main_t_ptr)
    }
}

///
/// Compiles and runs the given sources
/// Returns the std result as String
/// Sources must define a main function that takes no arguments and returns an int and string
/// The int is the return value which can be verified
/// The string will eventually be the Stdout of the function.
///
pub fn compile<T: Compilable>(context: &Context, source: T) -> ExecutionEngine {
    let source = source.containers();
    let code_gen =
        compile_module(context, source, None, Diagnostician::null_diagnostician()).unwrap();
    println!("{}", code_gen.module.print_to_string());
    code_gen
        .module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap()
}

pub fn compile_and_run<T, U, S: Compilable>(source: S, params: &mut T) -> U {
    let context: Context = Context::create();
    let exec_engine = compile(&context, source);
    run::<T, U>(&exec_engine, "main", params)
}
