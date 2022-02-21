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

pub fn generate<'ink, 'b>(
    builtin: &str,
    generator: &'b ExpressionCodeGenerator<'ink, 'b>,
    params: Vec<&AstStatement>,
    source_location: SourceRange,
) -> Result<BasicValueEnum<'ink>, Diagnostic> {
    BUILTIN
        .get(builtin)
        .ok_or_else(|| {
            Diagnostic::codegen_error(
                &format!("Cannot find builtin function {}", builtin),
                source_location.clone(),
            )
        })
        .and_then(|it| it.codegen(generator, params.as_slice(), source_location))
}
