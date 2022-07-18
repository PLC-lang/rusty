use std::collections::HashMap;

use inkwell::values::{BasicValue, BasicValueEnum};
use lazy_static::lazy_static;

use crate::{
    ast::{AstStatement, CompilationUnit, LinkageType, SourceRange},
    codegen::generators::expression_generator::ExpressionCodeGenerator,
    diagnostics::Diagnostic,
    lexer::{self, IdProvider},
    parser,
    resolver::{get_type_for_annotation, AnnotationMap, StatementAnnotation, TypeAnnotator},
    typesystem,
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
                annotation: None,
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
                annotation: None,
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
                annotation: Some(|annotator, operator, parameters| {
                    //Derive a common type for all parameters and hint it
                    let target_type = parameters.iter().skip(1) //skip the first param
                        .filter_map(|it| annotator.annotation_map.get(it))
                        .filter_map(|it| get_type_for_annotation(annotator.index, it))
                        .reduce(|accumulator, it| {
                            typesystem::get_bigger_type(accumulator, it, annotator.index)
                        }).expect("at least one type will be returned");
                    for param in parameters.iter().skip(1) {
                        annotator.annotation_map.annotate_type_hint(
                            param,
                            StatementAnnotation::value(target_type.get_name()),
                        );
                    }
                    //Update the function's return type
                    let qualified_name = if let Some(StatementAnnotation::Function{qualified_name, ..}) = annotator.annotation_map.get(operator) {
                        Some(qualified_name.to_string())
                    } else {
                        None
                    };
                    //Note : This is done in 2 steps to avoid borrowing the annotation map as immutable and then mutable right after.
                    // At this stage the annotation map is not borrowed as immutable because the qualified name was cloned to a string
                    if let Some(qualified_name) = qualified_name {
                        annotator.annotation_map.annotate(operator, StatementAnnotation::Function { return_type: target_type.get_name().to_string(), qualified_name})
                    }

                    Ok(())
                }),
                code: |generator, params, location| {
                    //Generate an access from the first param
                    if let (&[k], params) = params.split_at(1) {
                        let k = generator.generate_expression(k)?;
                        let pou = generator.index.find_pou("MUX").expect("MUX exists as builtin");
                        //Generate a pointer for the rest of the params
                        let params = generator.generate_variadic_arguments_list(pou, params)?;
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
                annotation: Some(|annotator, operator, parameters| {
                    //Dissect the parameters
                    if let &[_g, in0, in1] = parameters {
                        //g can be ignored
                        //annotate in1 and in2 with the same type
                        let in0_type = annotator.annotation_map.get(in0).and_then(|it| get_type_for_annotation(annotator.index, it));
                        let in1_type = annotator.annotation_map.get(in1).and_then(|it| get_type_for_annotation(annotator.index, it));
                        if let (Some(in0_type),Some(in1_type)) = (in0_type, in1_type) {
                            let target_type = typesystem::get_bigger_type(in0_type, in1_type, annotator.index);
                            annotator.annotation_map.annotate_type_hint(in0, StatementAnnotation::Value { resulting_type: target_type.get_name().to_string() });
                            annotator.annotation_map.annotate_type_hint(in1, StatementAnnotation::Value { resulting_type: target_type.get_name().to_string() });
                            //Update the function's return type
                            let qualified_name = if let Some(StatementAnnotation::Function{qualified_name, ..}) = annotator.annotation_map.get(operator) {
                                Some(qualified_name.to_string())
                            } else {
                                None
                            };
                            //Note : This is done in 2 steps to avoid borrowing the annotation map as immutable and then mutable right after.
                            // At this stage the annotation map is not borrowed as immutable because the qualified name was cloned to a string
                            if let Some(qualified_name) = qualified_name {
                                annotator.annotation_map.annotate(operator, StatementAnnotation::Function { return_type: target_type.get_name().to_string(), qualified_name})
                            }
                        }

                    }
                    Ok(())
                }),
                code: |generator, params, location| {
                    if let &[g,in0,in1] = params {
                        //Evaluate the parameters
                        let cond = generator.generate_expression(g)?;
                        let in0 = generator.generate_expression(in0)?;
                        let in1 = generator.generate_expression(in1)?;
                        //Generate an llvm select instruction
                        Ok(generator.llvm.builder.build_select(cond.into_int_value(), in1, in0, ""))
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
                annotation: Some(|annotator, operator, parameters| {
                    if let &[param] = parameters {
                        //Get param type, annotate the return with it
                        if let Some(param_type) = annotator.annotation_map.get(param).and_then(|it| get_type_for_annotation(annotator.index, it)) {
                            //Update the function's return type
                            let qualified_name = if let Some(StatementAnnotation::Function{qualified_name, ..}) = annotator.annotation_map.get(operator) {
                                Some(qualified_name.to_string())
                            } else {
                                None
                            };
                            //Note : This is done in 2 steps to avoid borrowing the annotation map as immutable and then mutable right after.
                            // At this stage the annotation map is not borrowed as immutable because the qualified name was cloned to a string
                            if let Some(qualified_name) = qualified_name {
                                annotator.annotation_map.annotate(operator, StatementAnnotation::Function { return_type: param_type.get_name().to_string(), qualified_name})
                            }
                        }
                    }
                    Ok(())
                }),
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

type AnnotationFunction =
    fn(&mut TypeAnnotator, &AstStatement, &[&AstStatement]) -> Result<(), Diagnostic>;
type CodegenFunction = for<'ink, 'b> fn(
    &'b ExpressionCodeGenerator<'ink, 'b>,
    &[&AstStatement],
    SourceRange,
) -> Result<BasicValueEnum<'ink>, Diagnostic>;
pub struct BuiltIn {
    decl: &'static str,
    annotation: Option<AnnotationFunction>,
    code: CodegenFunction,
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
    pub(crate) fn get_annotation(&self) -> Option<AnnotationFunction> {
        self.annotation
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
