// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

macro_rules! assert_codegen {
    ($result:expr) => {
        if cfg!(target_os = "macos") {
            let mut result = $result;
            let regex = regex::Regex::new(r#"section "(?P<leading_comma>,).*?""#).unwrap();
            // let content = regex.replace_all(&content, "");
            let ranges = regex
                .captures_iter(&result)
                .flat_map(|it| it.name("leading_comma"))
                .map(|it| it.range())
                .collect::<Vec<_>>();

            let new = String::new();
            for (idx, range) in ranges.iter().enumerate() {
                result.replace_range(range.start - idx..range.end - idx, "");
            }

            insta::assert_snapshot!(result);
        } else {
            insta::assert_snapshot!($result);
        }
    };
}
pub(crate) use assert_codegen;

mod code_gen_tests;
mod codegen_error_messages_tests;
mod compare_instructions_tests;
mod constants_tests;
mod debug_tests;
mod directaccess_test;
mod expression_tests;
mod function_tests;
mod generics_test;
mod initialization_test;
mod multifile_codegen_tests;
mod parameters_tests;
mod statement_codegen_test;
mod string_tests;
#[cfg(feature = "verify")]
mod switch_case_tests;
mod typesystem_test;
mod vla_tests;
