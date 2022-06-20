use std::path::PathBuf;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::context::Context;
use rusty::diagnostics::Diagnostician;
use rusty::*;

//Import the helper run methods into the tests
pub use rusty::runner::{compile, compile_and_run, run, MainType};

mod correctness {
    mod arrays;
    mod bitaccess;
    mod classes;
    mod constants;
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
    mod strings;
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
    mod linking;
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
