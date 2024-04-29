// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::path::PathBuf;

//Import the helper run methods into the tests
pub use driver::runner::{compile, compile_and_run, MainType};
pub use inkwell::context::Context;

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
        pub(super) mod addition;
        mod division;
        mod mixed;
        mod multiplication;
        mod substraction;
    }
    mod arithmetic_functions {
        mod addition;
        mod division;
        mod multiplication;
        mod substraction;
    }
    mod vla;
    mod comparison_functions {
        mod equal;
        mod greater_than;
        mod greater_than_or_equal;
        mod less_than;
        mod less_than_or_equal;
        mod not_equal;
    }
}
mod integration {
    mod build_description_tests;
    mod cfc;
    mod command_line_compile;
    mod external_files;
    mod linking;
    mod multi_files;
}

#[macro_use]
extern crate serial_test;

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

/// # Safety
///
/// Unsafe by design, it dereferences a pointer
pub unsafe fn new_cstr<'a>(chars: *const i8) -> &'a std::ffi::CStr {
    // Depending on the architecture `CStr::from_ptr` might either take
    // `i8` or `u8` as an argument. For example x86_64 would yield `i8`
    // whereas aarch64 would yield `u8`. Instead of relying on conditional
    // compilation we can ask the compiler to deduce the right type here,
    // i.e. by casting with `as *const _`.
    // For more information regarding `CStr::from_ptr` see:
    // * https://doc.rust-lang.org/nightly/src/core/ffi/mod.rs.html#54
    // * https://doc.rust-lang.org/nightly/src/core/ffi/mod.rs.html#104
    std::ffi::CStr::from_ptr(chars as *const _)
}
