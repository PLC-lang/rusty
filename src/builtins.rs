use std::collections::HashMap;

use inkwell::values::{BasicValue, BasicValueEnum};
use lazy_static::lazy_static;

use crate::{
    ast::{AstStatement, CompilationUnit, LinkageType, SourceRange},
    codegen::generators::expression_generator::ExpressionCodeGenerator,
    diagnostics::Diagnostic,
    lexer::{self, IdProvider},
    parser,
};

// Defines a set of functions that are always included in a compiled application
lazy_static! {
    static ref BUILTIN: HashMap<&'static str, BuiltIn> = HashMap::from([
        (
            "ADR",
            BuiltIn {
                decl: "FUNCTION ADR<T: ANY> : LWORD
                VAR_INPUT
                    in : T;
                END_VAR
                END_FUNCTION
            ",
                code: |generator, params, location| {
                    if let [reference] = params {
                        generator
                            .generate_element_pointer(reference)
                            .map(|it| generator.ptr_as_value(it))
                    } else {
                        Err(Diagnostic::codegen_error(
                            "Expected exadtly one parameter for REF",
                            location,
                        ))
                    }
                }
            },
        ),
        (
            "REF",
            BuiltIn {
                decl: "FUNCTION REF<T: ANY> : REF_TO T
                VAR_INPUT
                    in : T;
                END_VAR
                END_FUNCTION
                ",
                code: |generator, params, location| {
                    if let [reference] = params {
                        generator
                            .generate_element_pointer(reference)
                            .map(|it| it.as_basic_value_enum())
                    } else {
                        Err(Diagnostic::codegen_error(
                            "Expected exadtly one parameter for REF",
                            location,
                        ))
                    }
                }
            },
        )
    ]);
}

pub struct BuiltIn {
    decl: &'static str,
    code: for<'ink, 'b> fn(
        &'b ExpressionCodeGenerator<'ink, 'b>,
        &[&AstStatement],
        SourceRange,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic>,
}

impl BuiltIn {
    pub fn codegen<'ink, 'b>(
        &self,
        generator: &'b ExpressionCodeGenerator<'ink, 'b>,
        params: &[&AstStatement],
        location: SourceRange,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        (self.code)(generator, params, location)
    }
}

pub fn parse_built_ins(id_provider: IdProvider) -> (CompilationUnit, Vec<Diagnostic>) {
    let src = BUILTIN
        .iter()
        .map(|(_, it)| it.decl)
        .collect::<Vec<&str>>()
        .join(" ");
    parser::parse(lexer::lex_with_ids(&src, id_provider), LinkageType::BuiltIn)
}

/// Returns the requested functio from the builtin index or None
pub fn get_builtin(name: &str) -> Option<&'static BuiltIn> {
    BUILTIN.get(name.to_uppercase().as_str())
}
