use crate::diagnostics::Diagnostic;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
mod array_validation_test;
mod assignment_validation_tests;
mod bitaccess_validation_test;
mod duplicates_validation_test;
mod generic_validation_tests;
mod literals_validation_tests;
mod pou_validation_tests;
mod recursive_validation_tests;
mod reference_resolve_tests;
mod statement_validation_tests;
mod variable_validation_tests;

pub fn make_readable(diagnostics: &Vec<Diagnostic>) -> String {
    let mut res = String::new();
    for ele in diagnostics {
        res.push_str(&format!("{:?}\n", ele));
    }
    res
}
