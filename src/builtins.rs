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
                decl: "FUNCTION ADR<U: ANY> : LWORD
                VAR_INPUT
                    in : U;
                END_VAR
                END_FUNCTION
            ",
                transformation: |it| it,
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
                decl: "FUNCTION REF<U: ANY> : REF_TO U
                VAR_INPUT
                    in : U;
                END_VAR
                END_FUNCTION
                ",
                transformation: |it| it,
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
        ),
        (
            "MUX",
            BuiltIn {
                decl: "FUNCTION MUX<U: ANY> : U
                VAR_INPUT
                    K : DINT;
                    args : {sized} U...;
                END_VAR
                END_FUNCTION
                ",
                transformation: |it| it,
                code: |generator, params, location| {
                    //Generate an access from the first param
                    if let &[k, ..] = params {
                        let k = generator.generate_expression(k)?;
                        let pou = generator.index.find_pou("MUX").expect("MUX exists as builtin");
                        //Generate a pointer for the rest of the params
                        let params = generator.generate_variadic_arguments_list(pou, &params[1..])?;
                        //First access is into the array
                        let ptr = generator.llvm.load_array_element(params[1].into_pointer_value(),&[generator.llvm.context.i32_type().const_zero(), k.into_int_value()],"")?;
                        Ok(generator.llvm.builder.build_load(ptr, ""))
                    } else {
                        Err(Diagnostic::codegen_error("Invalid signature for MUX", location))
                    }

                }
            },
        ),
        (
            "SEL",
            BuiltIn {
                decl: "FUNCTION SEL<U: ANY> : U
                VAR_INPUT
                    G : BOOL;
                    IN0 : U;
                    IN1 : U;
                END_VAR
                END_FUNCTION
                ",
                transformation: |it| it,
                code: |generator, params, location| {
                    if let &[g,..] = params {
                        //evaluate G
                        let cond = generator.generate_expression(g)?;
                        //Use the mux definition for the varargs
                        let pou = generator.index.find_pou("MUX").expect("MUX exists as builtin");
                        //Generate a pointer for the rest of the params
                        let params = generator.generate_variadic_arguments_list(pou, &params[1..])?;
                        let ptr = generator.llvm.load_array_element(params[1].into_pointer_value(),&[generator.llvm.context.i32_type().const_zero(), cond.into_int_value()],"")?; //First access is into the array
                        Ok(generator.llvm.builder.build_load(ptr, ""))
                    } else {
                        Err(Diagnostic::codegen_error("Invalid signature for SEL", location))
                    }

                }
            }
        ),
        (
            "MOVE",
            BuiltIn {
                decl : "FUNCTION MOVE<U: ANY> : U
                VAR_INPUT
                    in : U;
                END_VAR
                END_FUNCTION",
                transformation: |it| it,
                code : |generator, params, location| {
                    if params.len() == 1 {
                        generator.generate_expression(params[0])
                    } else {
                        Err(Diagnostic::codegen_error("MOVE expects exactly one parameter", location))
                    }
                }
            }
        )
    ]);
}

pub struct BuiltIn {
    decl: &'static str,
    transformation: fn(AstStatement) -> AstStatement,
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

    pub fn transform(&self, statement: AstStatement) -> AstStatement {
        (self.transformation)(statement)
    }
}

pub fn parse_built_ins(id_provider: IdProvider) -> CompilationUnit {
    let src = BUILTIN
        .iter()
        .map(|(_, it)| it.decl)
        .collect::<Vec<&str>>()
        .join(" ");
    let mut unit = parser::parse(
        lexer::lex_with_ids(&src, id_provider.clone()),
        LinkageType::BuiltIn,
    )
    .0;
    crate::ast::pre_process(&mut unit, id_provider);
    unit
}

/// Returns the requested functio from the builtin index or None
pub fn get_builtin(name: &str) -> Option<&'static BuiltIn> {
    BUILTIN.get(name.to_uppercase().as_str())
}
