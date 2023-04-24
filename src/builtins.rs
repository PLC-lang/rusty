use std::collections::HashMap;

use inkwell::{
    basic_block::BasicBlock,
    types::BasicType,
    values::{BasicValue, IntValue},
};
use lazy_static::lazy_static;

use crate::{
    ast::{
        flatten_expression_list, AstStatement, CompilationUnit, GenericBinding, LinkageType, SourceRange,
        SourceRangeFactory,
    },
    codegen::generators::expression_generator::{self, ExpressionCodeGenerator, ExpressionValue},
    diagnostics::Diagnostic,
    lexer::{self, IdProvider},
    parser,
    resolver::{
        self,
        generics::{no_generic_name_resolver, GenericType},
        AnnotationMap, StatementAnnotation, TypeAnnotator, VisitorContext,
    },
    typesystem::DataTypeInformation,
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
                            .map(|it| ExpressionValue::RValue(generator.ptr_as_value(it)))
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
                            .map(|it| ExpressionValue::RValue(it.as_basic_value_enum()))
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
                        Ok(ExpressionValue::LValue(result_var))
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
                        // evaluate the parameters
                        let cond = expression_generator::to_i1(generator.generate_expression(g)?.into_int_value(), &generator.llvm.builder);
                        // for aggregate types we need a ptr to perform memcpy
                        // use generate_expression_value(), this will return a gep
                        // generate_expression() would load the ptr
                        let in0 = if generator.annotations.get_type(in0,generator.index).map(|it| it.get_type_information().is_aggregate()).unwrap_or_default() {
                            generator.generate_expression_value(in0)?.get_basic_value_enum()
                        } else {
                            generator.generate_expression(in0)?
                        };
                        let in1 = if generator.annotations.get_type(in1,generator.index).map(|it| it.get_type_information().is_aggregate()).unwrap_or_default() {
                            generator.generate_expression_value(in1)?.get_basic_value_enum()
                        } else {
                            generator.generate_expression(in1)?
                        };
                        // generate an llvm select instruction
                        let sel = generator.llvm.builder.build_select(cond, in1, in0, "");

                        if sel.is_pointer_value(){
                            Ok(ExpressionValue::LValue(sel.into_pointer_value()))
                        } else {
                            Ok(ExpressionValue::RValue(sel))
                        }
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
                        generator.generate_expression(params[0]).map(ExpressionValue::RValue)
                    } else {
                        Err(Diagnostic::codegen_error("MOVE expects exactly one parameter", location))
                    }
                }
            }
        ),
        (
            "SIZEOF",
            BuiltIn {
                decl : "FUNCTION SIZEOF<U: ANY> : ULINT
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
                            Ok(ExpressionValue::RValue(size))
                    } else {
                        Err(Diagnostic::codegen_error(
                            "Expected exactly one parameter for SIZEOF",
                            location,
                        ))
                    }
                }
            }
        ),
        (
            "LOWER_BOUND",
            BuiltIn {
                decl: "FUNCTION LOWER_BOUND<U: ANY> : DINT
                VAR_INPUT
                    arr : U;
                    dim : DINT;
                END_VAR
                END_FUNCTION",
                annotation: Some(|annotator, _, parameters, _| { generate_annotation(annotator, parameters) }),
                generic_name_resolver: no_generic_name_resolver,
                code : |generator, params, location| {
                    generate_vla_helper_function_code(generator, params, location, false)
                }
            }
        ),
        (
            "UPPER_BOUND",
            BuiltIn {
                decl: "FUNCTION UPPER_BOUND<U: ANY> : DINT
                VAR_INPUT
                    arr : U;
                    dim : DINT;
                END_VAR
                END_FUNCTION",
                annotation: Some(|annotator, _, parameters, _| { generate_annotation(annotator, parameters) }),
                generic_name_resolver: no_generic_name_resolver,
                code : |generator, params, location| {
                    generate_vla_helper_function_code(generator, params, location, true)
                }
            }
        ),
    ]);
}

fn generate_annotation(
    annotator: &mut TypeAnnotator,
    parameters: Option<&AstStatement>,
) -> Result<(), Diagnostic> {
    let Some(AstStatement::ExpressionList { expressions, .. }) = parameters else { todo!("error") };

    // match expression[0] => vla_array

    match &expressions[1] {
        AstStatement::LiteralInteger { .. } => (),
        AstStatement::Reference { .. } => {
            let kind = annotator.annotation_map.get_type_or_void(&expressions[1], annotator.index);
            if !matches!(&kind.information, DataTypeInformation::Integer { .. }) {
                todo!("error");
            }
        }

        _ => todo!("error"),
    };

    Ok(())
}

//TODO: better name
fn generate_vla_helper_function_code<'ink, 'b>(
    generator: &ExpressionCodeGenerator<'ink, 'b>,
    params: &[&AstStatement],
    location: SourceRange,
    upper: bool,
) -> Result<ExpressionValue<'ink>, Diagnostic> {
    let llvm = generator.llvm;
    let builder = &generator.llvm.builder;

    let vla = generator.generate_element_pointer(params[0]).unwrap();
    let dim = builder.build_struct_gep(vla, 1, "dim").unwrap();

    let accessor = match params[1] {
        AstStatement::LiteralInteger { value, .. } => {
            if upper {
                llvm.i32_type().const_int((value - 1) as u64 * 2 + 1, false)
            } else {
                llvm.i32_type().const_int((value - 1) as u64 * 2, false)
            }
        }

        AstStatement::Reference { .. } => match generator.annotations.get(params[1]) {
            Some(StatementAnnotation::Variable { qualified_name, .. }) => {
                let ptr = generator.llvm_index.find_loaded_associated_variable_value(qualified_name).unwrap();
                let value = builder.build_load(ptr, "").into_int_value();

                // temp(generator, llvm, value)
                let sub = builder.build_int_sub(value, llvm.i32_type().const_int(1, false), "");
                let mut mul = builder.build_int_mul(llvm.i32_type().const_int(2, false), sub, "");

                if upper {
                    mul = builder.build_int_add(mul, llvm.i32_type().const_int(1, false), "")
                }

                mul
            }

            _ => todo!(),
        },

        _ => return Err(Diagnostic::codegen_error("Received an invalid argument", location)),
    };

    let gep_bound = unsafe { llvm.builder.build_gep(dim, &[llvm.i32_type().const_zero(), accessor], "") };
    let bound = llvm.builder.build_load(gep_bound, "").into_int_value().as_basic_value_enum();
    Ok(ExpressionValue::RValue(bound))
}

type AnnotationFunction =
    fn(&mut TypeAnnotator, &AstStatement, Option<&AstStatement>, VisitorContext) -> Result<(), Diagnostic>;
type GenericNameResolver = fn(&str, &[GenericBinding], &HashMap<String, GenericType>) -> String;
type CodegenFunction = for<'ink, 'b> fn(
    &'b ExpressionCodeGenerator<'ink, 'b>,
    &[&AstStatement],
    SourceRange,
) -> Result<ExpressionValue<'ink>, Diagnostic>;
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
    ) -> Result<ExpressionValue<'ink>, Diagnostic> {
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
