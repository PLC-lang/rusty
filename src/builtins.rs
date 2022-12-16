use std::collections::HashMap;

use inkwell::{
    basic_block::BasicBlock,
    types::BasicType,
    values::{BasicValue, BasicValueEnum, IntValue},
};
use lazy_static::lazy_static;

use crate::{
    ast::{
        flatten_expression_list, AstStatement, CompilationUnit, GenericBinding, LinkageType, SourceRange,
        SourceRangeFactory,
    },
    codegen::generators::expression_generator::{self, ExpressionCodeGenerator},
    diagnostics::Diagnostic,
    lexer::{self, IdProvider},
    parser,
    resolver::{
        self,
        generics::{generic_name_resolver, no_generic_name_resolver, GenericType},
        AnnotationMap, TypeAnnotator, VisitorContext,
    },
    typesystem::{get_bigger_type, DataTypeInformation, DINT_SIZE, DINT_TYPE, REAL_TYPE, LINT_TYPE},
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
                generic_name_resolver: no_generic_name_resolver,
                code: |generator, params, location| {
                    if let [reference] = params {
                        generator
                            .generate_element_pointer(reference)
                            .map(|it| generator.ptr_as_value(it))
                    } else {
                        Err(Diagnostic::codegen_error(
                            "Expected exactly one parameter for REF",
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
                annotation: Some(|annotator, operator, parameters, _| {
                    let params = parameters.ok_or_else(|| Diagnostic::codegen_error("REF requires parameters", operator.get_location()))?;
                        // Get the input and annotate it with a pointer type
                        if let [input] = flatten_expression_list(params)[..] {
                            let input_type = annotator.annotation_map
                                .get_type_or_void(input, annotator.index)
                                .get_type_information()
                                .get_name()
                                .to_owned();

                            let ptr_type = resolver::add_pointer_type(
                                &mut annotator.annotation_map.new_index,
                                input_type
                            );

                            annotator.annotation_map.annotate(
                                operator, resolver::StatementAnnotation::Function {
                                    return_type: ptr_type, qualified_name: "REF".to_string(), call_name: None
                                }
                            );

                            Ok(())
                        } else {
                            unreachable!()
                        }
                    }
            ),
                generic_name_resolver: no_generic_name_resolver,
                code: |generator, params, location| {
                    if let [reference] = params {
                        generator
                            .generate_element_pointer(reference)
                            .map(|it| it.as_basic_value_enum())
                    } else {
                        Err(Diagnostic::codegen_error(
                            "Expected exactly one parameter for REF",
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
                generic_name_resolver: no_generic_name_resolver,
                code: |generator, params, location| {
                    let llvm = generator.llvm;
                    let context = llvm.context;
                    let builder = &llvm.builder;

                    let function_context = generator.get_function_context(params.get(0).expect("Param 0 exists"))?;
                    let insert_block = builder.get_insert_block().expect("Builder should have a block at this point");

                    //Generate an access from the first param
                    if let (&[k], params) = params.split_at(1) {
                        //Create a temp var
                        let result_type = params.get(0)
                            .ok_or_else(|| Diagnostic::codegen_error("Invalid signature for MUX", location))
                            .and_then(|it| generator.get_type_hint_info_for(it))
                            .and_then(|it| generator.llvm_index.get_associated_type(it.get_name()))?;
                        let result_var = generator.llvm.create_local_variable("", &result_type);
                        let k = generator.generate_expression(k)?;

                        let mut blocks = vec![];
                        for it in params.iter() {
                            let block = context.append_basic_block(function_context.function, "");
                            blocks.push((*it,block))
                        }
                        let continue_block = context.append_basic_block(function_context.function, "continue_block");

                        let cases = blocks.into_iter().enumerate().map::<Result<(IntValue, BasicBlock), Diagnostic>, _>(|(index, (it, block))| {
                            let value = context.i32_type().const_int(index as u64, false);
                            builder.position_at_end(block);
                            let expr = generator.generate_expression(it)?;
                            builder.build_store(result_var, expr);
                            builder.build_unconditional_branch(continue_block);
                            Ok((value,block))
                        }).collect::<Result<Vec<_>,_>>()?;
                        builder.position_at_end(insert_block);
                        builder.build_switch(k.into_int_value(), continue_block, &cases);
                        builder.position_at_end(continue_block);
                        let result_var = builder.build_load(result_var, "");
                        Ok(result_var)
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
                generic_name_resolver: no_generic_name_resolver,
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
                generic_name_resolver: no_generic_name_resolver,
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
                decl : "FUNCTION EXPT<U : ANY_NUM, V: ANY_NUM> : U
                VAR_INPUT
                    ELEMENT: U;
                    EXPONENT: V;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, operator , parameters, ctx| {
                    let params = parameters.ok_or_else(|| Diagnostic::codegen_error("EXPT requires parameters", operator.get_location()))?;
                    if let [element, exponent] = flatten_expression_list(params)[..] {
                        //Resolve the parameter types
                        let element_type = annotator.annotation_map.get_type(element, annotator.index);
                        let exponent_type = annotator.annotation_map.get_type(exponent, annotator.index);
                        let dint_type = annotator.index.get_type_or_panic(DINT_TYPE);
                        let lint_type = annotator.index.get_type_or_panic(LINT_TYPE);
                        let real_type = annotator.index.get_type_or_panic(REAL_TYPE);
                        let is_exponent_positive_literal = if let AstStatement::LiteralInteger { value, .. } = exponent { value.is_positive() } else {false};
                        if let (Some(element_type), Some(exponent_type)) = (element_type, exponent_type) {
                            let (element_type, exponent_type)  = match (element_type.get_type_information(), exponent_type.get_type_information()) {
                                //If both params are int types, convert to a common type and call an int power function
                                (DataTypeInformation::Integer { .. }, DataTypeInformation::Integer {signed : false, size, ..})
                                | (DataTypeInformation::Integer { .. }, DataTypeInformation::Integer {signed : true, size, ..}) if is_exponent_positive_literal => {
                                    //Convert both to minimum dint
                                    let element_type = get_bigger_type(element_type, dint_type, annotator.index);
                                    let exponent_type = if *size <= DINT_SIZE {
                                        lint_type
                                    } else {
                                        exponent_type
                                    };
                                    (element_type.get_name(), exponent_type.get_name())
                                },
                                //If left is real, then if right is int call powi
                                (_, DataTypeInformation::Integer {size, ..}) => {
                                    //Convert the exponent to minimum DINT
                                    let target_type = get_bigger_type(element_type, real_type, annotator.index);

                                    // For integer powers, only a 32 bit integer is allowed
                                    let exponent_type = if *size <= DINT_SIZE {
                                        dint_type
                                    } else {
                                        //For bigger types, we move to the target type (Effectively always LREAL)
                                        get_bigger_type(target_type, exponent_type, annotator.index)
                                    };
                                    let target_type = get_bigger_type(target_type, exponent_type, annotator.index);
                                    (target_type.get_name(), exponent_type.get_name())
                                },
                                //If right is real convert to common real type and call powf
                                _ => {
                                    //Convert left and right to minimum REAL
                                    let target_type = get_bigger_type(
                                        get_bigger_type(element_type, exponent_type, annotator.index), real_type, annotator.index);
                                    (target_type.get_name(), target_type.get_name())
                                }
                            };
                            let mut generics_candidates = HashMap::new();
                            generics_candidates.insert("U".to_string(), vec![element_type.to_string()]);
                            generics_candidates.insert("V".to_string(), vec![exponent_type.to_string()]);
                            annotator.update_generic_call_statement(generics_candidates, "EXPT", operator, parameters, ctx)
                        } else {
                            unreachable!("Exponent types should be available at this point")
                        }

                    }
                    Ok(())
                }),
                generic_name_resolver,
                code : |_,_,_| {
                    unreachable!("Expt will always end up calling the real functions by the resolver magic")
                }
            }
        ),
        (
            "SIZEOF",
            BuiltIn {
                decl : "FUNCTION SIZEOF<U: ANY> : DINT
                VAR_INPUT
                    in : U;
                END_VAR
                END_FUNCTION",
                annotation: None,
                generic_name_resolver: no_generic_name_resolver,
                code : |generator, params, location| {
                    if let [reference] = params {
                        // get name of datatype behind reference
                        let type_name = generator.annotations
                            .get_type(reference, generator.index)
                            .map(|it| generator.index.get_effective_type_or_void_by_name(it.get_name()))
                            .unwrap()
                            .get_name();

                        // return size of llvm type
                        let size = generator.llvm_index
                            .get_associated_type(type_name)
                            .map_err(|_| Diagnostic::codegen_error(&format!("Could not find associated data type: {type_name}"), location.clone())
                            )?.size_of()
                            .ok_or_else(|| Diagnostic::codegen_error("Parameter type is not sized.", location.clone()))?
                            .as_basic_value_enum();
                            Ok(size)
                    } else {
                        Err(Diagnostic::codegen_error(
                            "Expected exactly one parameter for SIZEOF",
                            location,
                        ))
                    }
                }
            }
        ),
    ]);
}

type AnnotationFunction =
    fn(&mut TypeAnnotator, &AstStatement, Option<&AstStatement>, VisitorContext) -> Result<(), Diagnostic>;
type GenericNameResolver = fn(&str, &[GenericBinding], &HashMap<String, GenericType>) -> String;
type CodegenFunction = for<'ink, 'b> fn(
    &'b ExpressionCodeGenerator<'ink, 'b>,
    &[&AstStatement],
    SourceRange,
) -> Result<BasicValueEnum<'ink>, Diagnostic>;
pub struct BuiltIn {
    decl: &'static str,
    annotation: Option<AnnotationFunction>,
    generic_name_resolver: GenericNameResolver,
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

    pub(crate) fn get_generic_name_resolver(&self) -> GenericNameResolver {
        self.generic_name_resolver
    }
}

pub fn parse_built_ins(id_provider: IdProvider) -> CompilationUnit {
    let src = BUILTIN.iter().map(|(_, it)| it.decl).collect::<Vec<&str>>().join(" ");
    let mut unit = parser::parse(
        lexer::lex_with_ids(&src, id_provider.clone(), SourceRangeFactory::internal()),
        LinkageType::BuiltIn,
        "<builtin>",
    )
    .0;
    crate::ast::pre_process(&mut unit, id_provider);
    unit
}

/// Returns the requested functio from the builtin index or None
pub fn get_builtin(name: &str) -> Option<&'static BuiltIn> {
    BUILTIN.get(name.to_uppercase().as_str())
}
