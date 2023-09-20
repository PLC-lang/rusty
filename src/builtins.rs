use std::collections::HashMap;

use inkwell::{
    basic_block::BasicBlock,
    types::BasicType,
    values::{BasicValue, IntValue},
};
use lazy_static::lazy_static;
use plc_ast::{
    ast::{
        self, flatten_expression_list, pre_process, AstNode, AstStatement, CompilationUnit, GenericBinding,
        LinkageType, TypeNature,
    },
    literals::AstLiteral,
    provider::IdProvider,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};

use crate::{
    codegen::generators::expression_generator::{self, ExpressionCodeGenerator, ExpressionValue},
    index::Index,
    lexer, parser,
    resolver::{
        self,
        generics::{no_generic_name_resolver, GenericType},
        AnnotationMap, StatementAnnotation, TypeAnnotator, VisitorContext,
    },
    typesystem::{self, get_literal_actual_signed_type_name},
    validation::{Validator, Validators},
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
                validation: None,
                generic_name_resolver: no_generic_name_resolver,
                code: |generator, params, location| {
                    if let [reference] = params {
                        generator
                            .generate_lvalue(reference)
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
                    // invalid amount of parameters is checked during validation
                    let Some(params) = parameters else { return; };
                    // Get the input and annotate it with a pointer type
                    let input = flatten_expression_list(params);
                    let Some(input) = input.get(0)  else { return; };
                    let input_type = annotator.annotation_map
                        .get_type_or_void(input, annotator.index)
                        .get_type_information()
                        .get_name()
                        .to_owned();

                    let ptr_type = resolver::add_pointer_type(
                        &mut annotator.annotation_map.new_index,
                        input_type
                    );

                    annotator.annotate(
                        operator, resolver::StatementAnnotation::Function {
                            return_type: ptr_type, qualified_name: "REF".to_string(), call_name: None
                        }
                    );
                }),
                validation: Some(|validator, operator, parameters, _, _| {
                    let Some(params) = parameters else {
                        validator.push_diagnostic(Diagnostic::invalid_parameter_count(1, 0, operator.get_location()));
                        return;
                    };

                    let params = flatten_expression_list(params);

                    if params.len() > 1 {
                        validator.push_diagnostic(Diagnostic::invalid_parameter_count(1, params.len(), operator.get_location()));
                    }
                }),
                generic_name_resolver: no_generic_name_resolver,
                code: |generator, params, location| {
                    if let [reference] = params {
                        generator
                            .generate_lvalue(reference)
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
                validation: None,
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
                validation: None,
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
                validation: None,
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
                validation: None,
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
                decl: "FUNCTION LOWER_BOUND<U: __ANY_VLA, T: ANY_INT> : DINT
                VAR_IN_OUT
                    arr : U;
                END_VAR
                VAR_INPUT
                    dim : T;
                END_VAR
                END_FUNCTION",
                annotation: Some(|annotator, _, parameters, _| {
                    annotate_variable_length_array_bound_function(annotator, parameters)
                }),
                validation: Some(|validator, operator, parameters, annotations, index| {
                    validate_variable_length_array_bound_function(validator, operator, parameters, annotations, index)
                }),
                generic_name_resolver: no_generic_name_resolver,
                code : |generator, params, location| {
                    generate_variable_length_array_bound_function(generator, params, true, location)
                }
            }
        ),
        (
            "UPPER_BOUND",
            BuiltIn {
                decl: "FUNCTION UPPER_BOUND<U: __ANY_VLA, T: ANY_INT> : DINT
                VAR_IN_OUT
                    arr : U;
                END_VAR
                VAR_INPUT
                    dim : T;
                END_VAR
                END_FUNCTION",
                annotation: Some(|annotator, _, parameters, _| {
                    annotate_variable_length_array_bound_function(annotator, parameters)
                }),
                validation: Some(|validator, operator, parameters, annotations, index| {
                    validate_variable_length_array_bound_function(validator, operator, parameters, annotations, index)
                }),
                generic_name_resolver: no_generic_name_resolver,
                code : |generator, params, location| {
                    generate_variable_length_array_bound_function(generator, params, false, location)
                }
            }
        ),
    ]);
}

fn annotate_variable_length_array_bound_function(
    annotator: &mut TypeAnnotator,
    parameters: Option<&AstNode>,
) {
    let Some(parameters) = parameters else {
        // caught during validation
        return;
    };
    let params = ast::flatten_expression_list(parameters);
    let Some(vla) = params.get(0) else {
        // caught during validation
        return;
    };

    // if the VLA parameter is a VLA struct, annotate it as such
    let vla_type = annotator.annotation_map.get_type_or_void(vla, annotator.index);
    let vla_type_name = if vla_type.get_nature() == TypeNature::__VLA {
        vla_type.get_name()
    } else {
        // otherwise annotate it with an internal, reserved VLA type
        typesystem::__VLA_TYPE
    };

    annotator.annotation_map.annotate_type_hint(vla, StatementAnnotation::value(vla_type_name));
}

fn validate_variable_length_array_bound_function(
    validator: &mut Validator,
    operator: &AstNode,
    parameters: Option<&AstNode>,
    annotations: &dyn AnnotationMap,
    index: &Index,
) {
    let Some(parameters) = parameters else {
        validator.push_diagnostic(Diagnostic::invalid_parameter_count(2, 0, operator.get_location()));
        // no params, nothing to validate
        return;
    };

    let params = ast::flatten_expression_list(parameters);

    if params.len() > 2 {
        validator.push_diagnostic(Diagnostic::invalid_parameter_count(
            2,
            params.len(),
            operator.get_location(),
        ));
    }

    match (params.get(0), params.get(1)) {
        (Some(vla), Some(idx)) => {
            let idx_type = annotations.get_type_or_void(idx, index);

            if !idx_type.has_nature(TypeNature::Int, index) {
                validator.push_diagnostic(Diagnostic::invalid_type_nature(
                    idx_type.get_name(),
                    &format!("{:?}", TypeNature::Int),
                    idx.get_location(),
                ))
            }

            // TODO: consider adding validation for consts and enums once https://github.com/PLC-lang/rusty/issues/847 has been implemented
            if let AstStatement::Literal(AstLiteral::Integer(dimension_idx)) = idx.get_stmt() {
                let dimension_idx = *dimension_idx as usize;

                let Some(n_dimensions) =
                    annotations.get_type_or_void(vla, index).get_type_information().get_dimensions()
                else {
                    // not a vla, validated via type nature
                    return;
                };

                if dimension_idx < 1 || dimension_idx > n_dimensions {
                    validator.push_diagnostic(Diagnostic::index_out_of_bounds(operator.get_location()))
                }
            };
        }
        (Some(_), None) => {
            validator.push_diagnostic(Diagnostic::invalid_parameter_count(2, 1, operator.get_location()))
        }
        _ => unreachable!(),
    }
}

/// Generates the code for the LOWER- AND UPPER_BOUND built-in functions, returning an error if the function
/// arguments are incorrect.
fn generate_variable_length_array_bound_function<'ink>(
    generator: &ExpressionCodeGenerator<'ink, '_>,
    params: &[&AstNode],
    is_lower: bool,
    location: SourceLocation,
) -> Result<ExpressionValue<'ink>, Diagnostic> {
    let llvm = generator.llvm;
    let builder = &generator.llvm.builder;
    let data_type_information =
        generator.annotations.get_type_or_void(params[0], generator.index).get_type_information();

    // TODO: most of the codegen errors should already be caught during validation.
    // once we abort codegen on critical errors, revisit and change to unreachable where possible
    if !data_type_information.is_vla() {
        return Err(Diagnostic::codegen_error(
            &format!("Expected VLA type, received {}", data_type_information.get_name()),
            location,
        ));
    };

    let vla = generator.generate_lvalue(params[0]).unwrap();
    let dim = builder.build_struct_gep(vla, 1, "dim").unwrap();

    let accessor = match params[1].get_stmt() {
        // e.g. LOWER_BOUND(arr, 1)
        AstStatement::Literal(kind) => {
            let AstLiteral::Integer(value) = kind else {
                let Some(type_name) = get_literal_actual_signed_type_name(kind, false) else {
                    unreachable!("type cannot be VOID")
                };
                return Err(Diagnostic::codegen_error(
                    &format!("Invalid literal type. Expected INT type, received {type_name} type"),
                    location,
                ));
            };
            // array offset start- and end-values are adjacent values in a flattened array -> 2 values per dimension, so in order
            // to read the correct values, the given index needs to be doubled. Additionally, the value is adjusted for 0-indexing.
            let offset = if is_lower { (value - 1) as u64 * 2 } else { (value - 1) as u64 * 2 + 1 };
            llvm.i32_type().const_int(offset, false)
        }
        AstStatement::CastStatement(data) => {
            let ExpressionValue::RValue(value) = generator.generate_expression_value(&data.target)? else {
                unreachable!()
            };

            if !value.is_int_value() {
                return Err(Diagnostic::codegen_error(
                    &format!("Expected INT value, found {}", value.get_type()),
                    location,
                ));
            };

            value.into_int_value()
        }
        // e.g. LOWER_BOUND(arr, idx + 3)
        _ => {
            let expression_value = generator.generate_expression(params[1])?;
            if !expression_value.is_int_value() {
                todo!()
            };
            // this operation mirrors the offset calculation of literal ints, but at runtime
            let offset = builder.build_int_mul(
                llvm.i32_type().const_int(2, false),
                builder.build_int_sub(
                    expression_value.into_int_value(),
                    llvm.i32_type().const_int(1, false),
                    "",
                ),
                "",
            );
            if !is_lower {
                builder.build_int_add(offset, llvm.i32_type().const_int(1, false), "")
            } else {
                offset
            }
        }
    };

    let gep_bound =
        unsafe { llvm.builder.build_in_bounds_gep(dim, &[llvm.i32_type().const_zero(), accessor], "") };
    let bound = llvm.builder.build_load(gep_bound, "");

    Ok(ExpressionValue::RValue(bound))
}

type AnnotationFunction = fn(&mut TypeAnnotator, &AstNode, Option<&AstNode>, VisitorContext);
type GenericNameResolver = fn(&str, &[GenericBinding], &HashMap<String, GenericType>) -> String;
type CodegenFunction = for<'ink, 'b> fn(
    &'b ExpressionCodeGenerator<'ink, 'b>,
    &[&AstNode],
    SourceLocation,
) -> Result<ExpressionValue<'ink>, Diagnostic>;
type ValidationFunction = fn(&mut Validator, &AstNode, Option<&AstNode>, &dyn AnnotationMap, &Index);

pub struct BuiltIn {
    decl: &'static str,
    annotation: Option<AnnotationFunction>,
    validation: Option<ValidationFunction>,
    generic_name_resolver: GenericNameResolver,
    code: CodegenFunction,
}

impl BuiltIn {
    pub fn codegen<'ink, 'b>(
        &self,
        generator: &'b ExpressionCodeGenerator<'ink, 'b>,
        params: &[&AstNode],
        location: SourceLocation,
    ) -> Result<ExpressionValue<'ink>, Diagnostic> {
        (self.code)(generator, params, location)
    }
    pub(crate) fn get_annotation(&self) -> Option<AnnotationFunction> {
        self.annotation
    }

    pub(crate) fn get_generic_name_resolver(&self) -> GenericNameResolver {
        self.generic_name_resolver
    }

    pub(crate) fn get_validation(&self) -> Option<ValidationFunction> {
        self.validation
    }
}

pub fn parse_built_ins(id_provider: IdProvider) -> CompilationUnit {
    let src = BUILTIN.iter().map(|(_, it)| it.decl).collect::<Vec<&str>>().join(" ");
    let mut unit = parser::parse(
        lexer::lex_with_ids(&src, id_provider.clone(), SourceLocationFactory::internal(&src)),
        LinkageType::BuiltIn,
        "<builtin>",
    )
    .0;

    pre_process(&mut unit, id_provider);
    unit
}

/// Returns the requested functio from the builtin index or None
pub fn get_builtin(name: &str) -> Option<&'static BuiltIn> {
    BUILTIN.get(name.to_uppercase().as_str())
}
