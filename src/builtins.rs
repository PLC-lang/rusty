use std::collections::HashMap;

use inkwell::values::{BasicValue, BasicValueEnum};
use lazy_static::lazy_static;

use crate::{
    ast::{AstStatement, CompilationUnit, LinkageType, SourceRange},
    codegen::generators::expression_generator::{self, ExpressionCodeGenerator},
    diagnostics::Diagnostic,
    lexer::{self, IdProvider},
    parser,
    resolver::{get_type_for_annotation, AnnotationMap, TypeAnnotator},
    typesystem::{get_bigger_type, DataTypeInformation, DINT_TYPE},
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
                annotation : None,
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
                    G   : BOOL;
                    IN0 : U;
                    IN1 : U;
                END_VAR
                END_FUNCTION
                ",
                annotation: None,
                code: |generator, params, location| {
                    if let &[g,in0,in1] = params {
                        //Evaluate the parameters
                        let cond = expression_generator::to_i1(generator.generate_expression(g)?.into_int_value(), &generator.llvm.builder);
                        let in0 = generator.generate_expression(in0)?;
                        let in1 = generator.generate_expression(in1)?;
                        //Generate an llvm select instruction
                        Ok(generator.llvm.builder.build_select(cond, in1, in0, ""))
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
                annotation: None,
                code : |generator, params, location| {
                    if params.len() == 1 {
                        generator.generate_expression(params[0])
                    } else {
                        Err(Diagnostic::codegen_error("MOVE expects exactly one parameter", location))
                    }
                }
            }
        ),
        (
            "EXPT",
            BuiltIn {
                decl : "FUNCTION EXPT<U : ANY_NUM, V: ANY_NUM, W: ANY_NUM> : W
                VAR_INPUT
                    ELEMENT: U;
                    EXPONENT: V;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, operator, params| {
                    if let [element, exponant] = params {
                        //Resolve the parameter types
                        let element_type = annotator.annotation_map.get(element).and_then(|it| get_type_for_annotation(annotator.index, it));
                        let exponant_type = annotator.annotation_map.get(exponant).and_then(|it| get_type_for_annotation(annotator.index, it));
                        //Annotate the correct expected types
                        //Choose the best function fit based on the parameter types
                        //Adjust the return type
                        let (element_type, exponant_type) = if let (Some(element_type), Some(exponant_type)) = (element_type, exponant_type) {
                            match (element_type.get_type_information(), exponant_type.get_type_information()) {

                            //If both params are int types, convert to a common type and call an int power function
                            (DataTypeInformation::Integer { .. }, DataTypeInformation::Integer {..}) => {
                                //Convert both to minimum dint
                                let dint_type = annotator.index.get_type_or_panic(DINT_TYPE);
                                let target_type = get_bigger_type(
                                    get_bigger_type(element_type, exponant_type, annotator.index), dint_type, annotator.index);
                                //Set the function name as EXPT__<TYPE>__<TYPE>
                                //Set the return type to <TYPE>
                                (target_type, target_type)
                            },
                            //If left is real, then if right is int call powi
                            (DataTypeInformation::Float { .. }, DataTypeInformation::Integer {..}) => {
                                //Convert the exponent to minimum DINT
                                //Set the function name as EXPT__<ELE_TYPE>__<EXP_TYPE>
                                //Set the return type to <ELE_TYPE>
                            },
                            //If right is real convert to common real type and call powf
                            _ => {
                                //Convert left and right to minimum REAL
                                //Set the function name as EXPT__<TYPE>__<TYPE>
                                //Set the return type to <TYPE>
                            }

                        }
                        }
                    }
                    Ok(())
                }),
                code : |generator, params, location| {
                    if let [element, exponant] = params {
                        let element_type = generator.annotations.get_type(element, generator.index).map(|it| it.get_type_information());
                        let exponant_type = generator.annotations.get_type(exponant, generator.index).map(|it| it.get_type_information());
                        let element = generator.generate_expression(element);
                        let exponant = generator.generate_expression(exponant);
                        match (element_type,exponant_type) {
                            //If both params are int types, convert to a common type and call an int power function
                            (Some(DataTypeInformation::Integer { .. }), Some(DataTypeInformation::Integer {..})) => {

                            },
                            //If left is real, then if right is int call powi
                            (Some(DataTypeInformation::Float { .. }), Some(DataTypeInformation::Integer {..})) => {

                            },
                            //If right is real convert to common real type and call powf
                            _ => {
                                // let element = crate::codegen::llvm_typesystem::promote_value_if_needed(


                                // )?;
                            }

                        }

                        todo!("Comming soon");
                    } else {
                        Err(Diagnostic::codegen_error("Malformed exponent instruction", location))
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
