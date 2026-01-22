use inkwell::{
    basic_block::BasicBlock,
    types::BasicType,
    values::{BasicValue, IntValue},
};
use lazy_static::lazy_static;
use plc_ast::{
    ast::{
        self, flatten_expression_list, pre_process, AstFactory, AstNode, AstStatement, CompilationUnit,
        GenericBinding, LinkageType, Operator, TypeNature,
    },
    literals::AstLiteral,
    provider::IdProvider,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};
use rustc_hash::FxHashMap;

use crate::{
    codegen::{
        generators::expression_generator::{self, ExpressionCodeGenerator, ExpressionValue},
        CodegenError,
    },
    index::Index,
    lexer, parser,
    resolver::{
        self,
        generics::{generic_name_resolver, no_generic_name_resolver, GenericType},
        AnnotationMap, StatementAnnotation, TypeAnnotator, VisitorContext,
    },
    typesystem::{self, get_bigger_type, get_literal_actual_signed_type_name, DataTypeInformationProvider},
    validation::{
        statement::{validate_type_compatibility, validate_type_compatibility_with_data_types},
        Validator, Validators,
    },
};

// Defines a set of functions that are always included in a compiled application
lazy_static! {
    static ref BUILTIN: FxHashMap<&'static str, BuiltIn> = FxHashMap::from_iter([
        (
            "ADR",
            BuiltIn {
                decl: "
                {constant}
                FUNCTION ADR<U: ANY> : LWORD
                VAR_INPUT
                    in : U;
                END_VAR
                END_FUNCTION
            ",
                annotation: None,
                validation: Some(|validator, operator, parameters, _, _| {
                    validate_argument_count(validator, operator, &parameters, 1);
                }),
                generic_name_resolver: no_generic_name_resolver,
                code: |generator, params, location| {
                    if let [reference] = params {
                        let reference = extract_actual_parameter(reference);
                        // Return the pointer value of a function when dealing with them, e.g. `ADR(MyFb.myMethod)`
                        match generator.annotations.get(reference) {
                            Some(StatementAnnotation::Function { qualified_name, .. }) => {
                                if let Some(fn_value) = generator.llvm_index.find_associated_implementation(qualified_name) {
                                    return Ok(ExpressionValue::RValue(fn_value.as_global_value().as_pointer_value().as_basic_value_enum()));
                                }
                            }

                            Some(StatementAnnotation::Type { type_name }) => {
                                if generator.index.find_type(type_name).is_some_and(|opt| opt.information.is_function_block()) {
                                    if let Some(fn_value) = generator.llvm_index.find_associated_implementation(type_name) {
                                        return Ok(ExpressionValue::RValue(fn_value.as_global_value().as_pointer_value().as_basic_value_enum()));
                                    }
                                }
                            }

                            _ => (),
                        };

                        generator
                            .generate_lvalue(reference)
                            .map(|it| ExpressionValue::RValue(it.as_basic_value_enum()))
                    } else {
                        Err(Diagnostic::codegen_error(
                            "Expected exactly one parameter for REF", location).into()
                        )
                    }
                }
            },
        ),
        (
            "REF",
            BuiltIn {
                decl: "
                {constant}
                FUNCTION REF<U: ANY> : REF_TO U
                VAR_INPUT
                    in : U;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, _, operator, parameters, _| {
                    // invalid amount of parameters is checked during validation
                    let Some(params) = parameters else { return; };
                    // Get the input and annotate it with a pointer type
                    let input = flatten_expression_list(params);
                    let actual_input = extract_actual_parameter(input.first().expect("must exist; covered by validation"));
                    let input_type = annotator.annotation_map
                                            .get_type_or_void(actual_input, annotator.index)
                                            .get_type_information()
                                            .get_name()
                                            .to_owned();

                    let ptr_type = resolver::add_pointer_type(
                        &mut annotator.annotation_map.new_index,
                        input_type.clone(),
                        true,
                    );

                    if input.first().is_some_and(|opt| opt.is_assignment()){
                        annotator.annotation_map.annotate_type_hint(actual_input, StatementAnnotation::value(input_type));
                    }
                    annotator.annotate(
                        operator, resolver::StatementAnnotation::Function {
                            return_type: ptr_type, qualified_name: "REF".to_string(), generic_name: None, call_name: None
                        }
                    );
                }),
                validation: Some(|validator, operator, parameters, _, _| {
                    validate_argument_count(validator, operator, &parameters, 1);
                }),
                generic_name_resolver: no_generic_name_resolver,
                code: |generator, params, location| {
                    if let [reference] = params {
                        let reference = extract_actual_parameter(reference);
                        // Return the pointer value of a function when dealing with them, e.g. `ADR(MyFb.myMethod)`
                        if let Some(StatementAnnotation::Function { qualified_name, .. }) = generator.annotations.get(reference) {
                            if let Some(fn_value) = generator.llvm_index.find_associated_implementation(qualified_name) {
                                return Ok(ExpressionValue::RValue(fn_value.as_global_value().as_pointer_value().as_basic_value_enum()));
                            }
                        }

                        generator
                            .generate_lvalue(reference)
                            .map(|it| ExpressionValue::RValue(it.as_basic_value_enum()))
                    } else {
                        Err(Diagnostic::codegen_error("Expected exactly one parameter for REF",location).into())
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

                    let function_context = generator.get_function_context(params.first().expect("Param 0 exists"))?;
                    let insert_block = builder.get_insert_block().expect("Builder should have a block at this point");

                    //Generate an access from the first param
                    if let (&[k], params) = params.split_at(1) {
                        let type_hint = params.first()
                        .ok_or_else(|| Diagnostic::codegen_error("Invalid signature for MUX", location).into())
                        .and_then(|it| generator.get_type_hint_info_for(it))?;
                        //Create a temp var
                        let result_type = generator.llvm_index.get_associated_type(type_hint.get_name())?;
                        let result_var = generator.llvm.create_local_variable("", &result_type)?;
                        let k = generator.generate_expression(k)?;

                        let mut blocks = vec![];
                        for it in params.iter() {
                            let block = context.append_basic_block(function_context.function, "");
                            blocks.push((*it,block))
                        }

                        let continue_block = context.append_basic_block(function_context.function, "continue_block");
                        let cases = blocks.into_iter().enumerate().map::<Result<(IntValue, BasicBlock), CodegenError>, _>(|(index, (it, block))| {
                            let value = context.i32_type().const_int(index as u64, false);
                            builder.position_at_end(block);
                            generator.generate_store(result_var, type_hint.get_type_information(), it)?;
                            builder.build_unconditional_branch(continue_block)?;
                            Ok((value,block))
                        }).collect::<Result<Vec<_>,_>>()?;

                        builder.position_at_end(insert_block);
                        builder.build_switch(k.into_int_value(), continue_block, &cases)?;
                        builder.position_at_end(continue_block);
                        let pointee = result_type;
                        Ok(ExpressionValue::LValue(result_var, pointee))
                    } else {
                        Err(Diagnostic::codegen_error("Invalid signature for MUX", location).into())
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
                        // Handle named arguments by extracting actual parameters
                        let [actual_g, actual_in0, actual_in1] = [g, in0, in1].map(extract_actual_parameter);

                        // evaluate the parameters
                        let cond = expression_generator::to_i1(generator.generate_expression(actual_g)?.into_int_value(), &generator.llvm.builder)?;
                        // for aggregate types we need a ptr to perform memcpy
                        // use generate_expression_value(), this will return a gep
                        // generate_expression() would load the ptr
                        let in0 = if generator.annotations.get_type(actual_in0,generator.index).map(|it| it.get_type_information().is_aggregate()).unwrap_or_default() {
                            generator.generate_expression_value(actual_in0)?.get_basic_value_enum()
                        } else {
                            generator.generate_expression(actual_in0)?
                        };
                        let in1 = if generator.annotations.get_type(actual_in1,generator.index).map(|it| it.get_type_information().is_aggregate()).unwrap_or_default() {
                            generator.generate_expression_value(actual_in1)?.get_basic_value_enum()
                        } else {
                            generator.generate_expression(actual_in1)?
                        };
                        // generate an llvm select instruction
                        let sel = generator.llvm.builder.build_select(cond, in1, in0, "")?;

                        if sel.is_pointer_value() {
                            // The `select` instruction requires the to be selected values to be of the same
                            // type, hence for the pointee we can choose either one
                            let pointee = {
                                let datatype = generator.annotations.get_type(actual_in0, generator.index).unwrap();
                                generator.llvm_index.get_associated_type(datatype.get_name()).unwrap()
                            };

                            Ok(ExpressionValue::LValue(sel.into_pointer_value(), pointee))
                        } else {
                            Ok(ExpressionValue::RValue(sel))
                        }
                    } else {
                        Err(Diagnostic::codegen_error("Invalid signature for SEL", location).into())
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
                        let actual_param = extract_actual_parameter(params[0]);
                        generator.generate_expression(actual_param).map(ExpressionValue::RValue)
                    } else {
                        Err(Diagnostic::codegen_error("MOVE expects exactly one parameter", location).into())
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
                        let actual_param = extract_actual_parameter(reference);
                        // get name of datatype behind reference
                        let type_name = generator.annotations
                            .get_type(actual_param, generator.index)
                            .map(|it| generator.index.get_effective_type_or_void_by_name(it.get_name()))
                            .unwrap()
                            .get_name();

                        // return size of llvm type
                        let size = generator.llvm_index
                            .get_associated_type(type_name)?.size_of()
                            .ok_or_else(|| Diagnostic::codegen_error("Parameter type is not sized.", location))?
                            .as_basic_value_enum();
                            Ok(ExpressionValue::RValue(size))
                    } else {
                        Err(Diagnostic::codegen_error(
                            "Expected exactly one parameter for SIZEOF",
                            location
                        ).into())
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
                annotation: Some(|annotator, _, _, parameters, _| {
                    annotate_variable_length_array_bound_function(annotator, parameters);
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
                annotation: Some(|annotator, _, _, parameters, _| {
                    annotate_variable_length_array_bound_function(annotator, parameters);
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
        // Arithmetic functions
        (
            "ADD",
            BuiltIn {
                decl: "FUNCTION ADD<T: ANY_NUM> : T
                    VAR_INPUT
                        args: {sized} T...;
                    END_VAR
                    END_FUNCTION
                ",
                annotation: Some(|annotator, statement, operator, parameters, ctx| {
                    let Some(params) = parameters else {
                        return;
                    };

                    annotate_arithmetic_function(annotator, statement, operator, params, ctx, Operator::Plus)
                }),
                validation:Some(|validator, operator, parameters, annotations, index| {
                    validate_types(validator, &parameters, annotations, index);
                    validate_builtin_symbol_parameter_count(validator, operator, parameters, Operator::Plus);
                }),
                generic_name_resolver,
                code: |_, _, _| {
                    unreachable!("ADD is not generated as a function call");
                }
            }
        ),
        (
            "MUL",
            BuiltIn {
                decl: "FUNCTION MUL<T: ANY_NUM> : T
                VAR_INPUT
                    args: {sized} T...;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, statement, operator, parameters, ctx| {
                    let Some(params) = parameters else {
                        return;
                    };

                    annotate_arithmetic_function(annotator, statement, operator, params, ctx, Operator::Multiplication)
                }),
                validation: Some(|validator, operator, parameters, annotations, index| {
                    validate_types(validator, &parameters, annotations, index);
                    validate_builtin_symbol_parameter_count(validator, operator, parameters, Operator::Multiplication)
                }),
                generic_name_resolver,
                code: |_, _, _| {
                    unreachable!("MUL is not generated as a function call");
                }
            }
        ),
        (
            "SUB",
            BuiltIn {
                decl: "FUNCTION SUB<T1: ANY, T2: ANY> : T1
                VAR_INPUT
                    IN1 : T1;
                    IN2 : T2;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, statement, operator, parameters, ctx| {
                    let Some(params) = parameters else {
                        return;
                    };
                    annotate_arithmetic_function(annotator, statement, operator, params, ctx, Operator::Minus)
                }),
                validation:Some(|validator, operator, parameters, annotations, index| {
                    validate_types(validator, &parameters, annotations, index);
                    validate_builtin_symbol_parameter_count(validator, operator, parameters, Operator::Minus)
                }),
                generic_name_resolver,
                code: |_, _, _| {
                    unreachable!("SUB is not generated as a function call");
                }
            }
        ),
        (
            "DIV",
            BuiltIn {
                decl: "FUNCTION DIV<T1: ANY, T2: ANY> : T1
                VAR_INPUT
                    IN1 : T1;
                    IN2 : T2;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, statement, operator, parameters, ctx| {
                    let Some(params) = parameters else {
                        return;
                    };
                    annotate_arithmetic_function(annotator, statement, operator, params, ctx, Operator::Division)
                }),
                validation:Some(|validator, operator, parameters, annotations, index| {
                    validate_types(validator, &parameters, annotations, index);
                    validate_builtin_symbol_parameter_count(validator, operator, parameters, Operator::Division)
                }),
                generic_name_resolver,
                code: |_, _, _| {
                    unreachable!("DIV is not generated as a function call");
                }
            }
        ),
        // TODO: MOD and AND/OR/XOR/NOT ANY_BIT ( NOT also supports boolean ) - FIXME: these are all keywords and therefore conflicting
        (
            "GT",
            BuiltIn {
                decl: "FUNCTION GT<T: ANY_ELEMENTARY> : BOOL
                VAR_INPUT
                    IN : {sized} T...;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, statement, operator, parameters, ctx| {
                    let Some(params) = parameters else {
                        return;
                    };
                    annotate_comparison_function(annotator, statement, operator, params, ctx, Operator::Greater);
                }),
                validation:Some(|validator, operator, parameters, _, _| {
                    validate_builtin_symbol_parameter_count(validator, operator, parameters, Operator::Greater)
                }),
                generic_name_resolver: no_generic_name_resolver,
                code : |_, _, _| {
                    unreachable!("GT is not generated as a function call");
                }
            }
        ),
        (
            "GE",
            BuiltIn {
                decl: "FUNCTION GE<T: ANY_ELEMENTARY> : BOOL
                VAR_INPUT
                    IN : {sized} T...;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, statement, operator, parameters, ctx| {
                    let Some(params) = parameters else {
                        return;
                    };
                    annotate_comparison_function(annotator, statement, operator, params, ctx, Operator::GreaterOrEqual);
                }),
                validation:Some(|validator, operator, parameters, _, _| {
                    validate_builtin_symbol_parameter_count(validator, operator, parameters, Operator::GreaterOrEqual)
                }),
                generic_name_resolver: no_generic_name_resolver,
                code : |_, _, _| {
                    unreachable!("GE is not generated as a function call");
                }
            }
        ),
        (
            "EQ",
            BuiltIn {
                decl: "FUNCTION EQ<T: ANY_ELEMENTARY> : BOOL
                VAR_INPUT
                    IN : {sized} T...;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, statement, operator, parameters, ctx| {
                    let Some(params) = parameters else {
                        return;
                    };
                    annotate_comparison_function(annotator, statement, operator, params, ctx, Operator::Equal);
                }),
                validation:Some(|validator, operator, parameters, _, _| {
                    validate_builtin_symbol_parameter_count(validator, operator, parameters, Operator::Equal)
                }),
                generic_name_resolver: no_generic_name_resolver,
                code : |_, _, _| {
                    unreachable!("EQ is not generated as a function call");
                }
            }
        ),
        (
            "LE",
            BuiltIn {
                decl: "FUNCTION LE<T: ANY_ELEMENTARY> : BOOL
                VAR_INPUT
                    IN : {sized} T...;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, statement, operator, parameters, ctx| {
                    let Some(params) = parameters else {
                        return;
                    };
                    annotate_comparison_function(annotator, statement, operator, params, ctx, Operator::LessOrEqual);
                }),
                validation:Some(|validator, operator, parameters, _, _| {
                    validate_builtin_symbol_parameter_count(validator, operator, parameters, Operator::LessOrEqual)
                }),
                generic_name_resolver: no_generic_name_resolver,
                code : |_, _, _| {
                    unreachable!("LE is not generated as a function call");
                }
            }
        ),
        (
            "LT",
            BuiltIn {
                decl: "FUNCTION LT<T: ANY_ELEMENTARY> : BOOL
                VAR_INPUT
                    IN : {sized} T...;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, statement, operator, parameters, ctx| {
                    let Some(params) = parameters else {
                        return;
                    };
                    annotate_comparison_function(annotator, statement, operator, params, ctx, Operator::Less);
                }),
                validation:Some(|validator, operator, parameters, _, _| {
                    validate_builtin_symbol_parameter_count(validator, operator, parameters, Operator::Less)
                }),
                generic_name_resolver: no_generic_name_resolver,
                code : |_, _, _| {
                    unreachable!("LT is not generated as a function call");
                }
            }
        ),
        (
            "NE",
            BuiltIn {
                decl: "FUNCTION NE<T: ANY_ELEMENTARY> : BOOL
                VAR_INPUT
                    IN1 : T;
                    IN2 : T;
                END_VAR
                END_FUNCTION
                ",
                annotation: Some(|annotator, statement, operator, parameters, ctx| {
                    let Some(params) = parameters else {
                        return;
                    };
                    annotate_comparison_function(annotator, statement, operator, params, ctx, Operator::NotEqual);
                }),
                validation: Some(|validator, operator, parameters, _, _| {
                    validate_builtin_symbol_parameter_count(validator, operator, parameters, Operator::NotEqual)
                }),
                generic_name_resolver: no_generic_name_resolver,
                code : |_, _, _| {
                    unreachable!("NE is not generated as a function call");
                }
            }
        ),
        (
            "SHL",
            BuiltIn {
                decl: "
                FUNCTION SHL<T: ANY> : T
                VAR_INPUT
                    IN : T;
                    n : UDINT;
                END_VAR
                END_FUNCTION
            ",
                annotation: None,
                validation: Some(|validator, operator, parameters, annotations, index| {
                    validate_argument_count(validator, operator, &parameters, 2);
                    validate_types_are_compatible_with_int(validator, &parameters, annotations, index);
                }),
                generic_name_resolver: no_generic_name_resolver,
                code: |generator, params, _| {
                    let left = generator.generate_expression(params[0])?.into_int_value();
                    let right = generator.generate_expression_with_cast_to_type_of_secondary_expression(params[1], params[0])?.into_int_value();

                    let shl = generator.llvm.builder.build_left_shift(left, right, "")?;

                    Ok(ExpressionValue::RValue(shl.as_basic_value_enum()))
                }
            },
        ),
        (
            "SHR",
            BuiltIn {
                decl: "
                FUNCTION SHR<T: ANY> : T
                VAR_INPUT
                    IN : T;
                    n : UDINT;
                END_VAR
                END_FUNCTION
            ",
                annotation: None,
                validation: Some(|validator, operator, parameters, annotations, index| {
                    validate_argument_count(validator, operator, &parameters, 2);
                    validate_types_are_compatible_with_int(validator, &parameters, annotations, index);
                }),
                generic_name_resolver: no_generic_name_resolver,
                code: |generator, params, _| {
                    let left = generator.generate_expression(params[0])?.into_int_value();
                    let right = generator.generate_expression_with_cast_to_type_of_secondary_expression(params[1], params[0])?.into_int_value();

                    let shr = generator.llvm.builder.build_right_shift(left, right, left.get_type().is_sized(), "")?;

                    Ok(ExpressionValue::RValue(shr.as_basic_value_enum()))
                }
            },
        ),
    ]);
}

fn validate_types(
    validator: &mut Validator,
    parameters: &Option<&AstNode>,
    annotations: &dyn AnnotationMap,
    index: &Index,
) {
    let Some(params) = parameters else { return };

    let types: Vec<_> =
        flatten_expression_list(params).into_iter().map(|it| extract_actual_parameter(it).clone()).collect();
    let mut types = types.iter().peekable();

    while let Some(left) = types.next() {
        if let Some(right) = types.peek() {
            validate_type_compatibility(validator, annotations, index, left, right);
        }
    }
}

fn validate_types_are_compatible_with_int(
    validator: &mut Validator,
    parameters: &Option<&AstNode>,
    annotations: &dyn AnnotationMap,
    index: &Index,
) {
    let Some(params) = parameters else { return };

    let builtin_int = index.get_type(typesystem::INT_TYPE).unwrap();
    for left in flatten_expression_list(params).iter().map(|it| extract_actual_parameter(it)) {
        let ty_left = annotations.get_type_or_void(left, index);
        validate_type_compatibility_with_data_types(validator, ty_left, builtin_int, &left.location);
    }
}

fn validate_builtin_symbol_parameter_count(
    validator: &mut Validator,
    operator: &AstNode,
    parameters: Option<&AstNode>,
    operation: Operator,
) {
    let Some(params) = parameters else {
        validator.push_diagnostic(Diagnostic::invalid_argument_count(2, 0, operator));
        return;
    };

    let count = flatten_expression_list(params).len();
    match operation {
        // non-extensible operators
        Operator::Minus | Operator::Division | Operator::NotEqual => {
            if count != 2 {
                validator.push_diagnostic(Diagnostic::invalid_argument_count(2, count, operator));
            }
        }
        _ => {
            if count < 2 {
                validator.push_diagnostic(Diagnostic::invalid_argument_count(2, count, operator));
            }
        }
    }
}

// creates nested BinaryExpressions for each parameter, such that
// GT(a, b, c, d) ends up as (a > b) & (b > c) & (c > d)
fn annotate_comparison_function(
    annotator: &mut TypeAnnotator,
    statement: &AstNode,
    operator: &AstNode,
    parameters: &AstNode,
    ctx: VisitorContext,
    operation: Operator,
) {
    let mut ctx = ctx;
    let params_flattened = flatten_expression_list(parameters);
    if params_flattened.iter().any(|it| {
        !annotator
            .annotation_map
            .get_type_or_void(it, annotator.index)
            .has_nature(TypeNature::Elementary, annotator.index)
    }) {
        // we are trying to call this function with a non-elementary type, so we redirect back to the resolver
        annotator.annotate_arguments(operator, parameters, &ctx);
        return;
    }

    let comparisons = params_flattened
        .windows(2)
        .map(|window| {
            AstFactory::create_binary_expression(
                window[0].clone(),
                operation,
                window[1].clone(),
                ctx.id_provider.next_id(),
            )
        })
        .collect::<Vec<_>>();
    let Some(new_statement) = comparisons.first() else {
        // no windows => less than 2 parameters, caught during validation
        return;
    };
    let mut new_statement = new_statement.clone();
    comparisons.into_iter().skip(1).for_each(|right| {
        new_statement = AstFactory::create_binary_expression(
            new_statement.clone(),
            Operator::And,
            right,
            ctx.id_provider.next_id(),
        )
    });

    annotator.visit_statement(&ctx, &new_statement);
    annotator.update_expected_types(annotator.index.get_type_or_panic(typesystem::BOOL_TYPE), &new_statement);
    annotator.annotate(statement, StatementAnnotation::ReplacementAst { statement: new_statement });
    annotator.update_expected_types(annotator.index.get_type_or_panic(typesystem::BOOL_TYPE), statement);
}

fn annotate_arithmetic_function(
    annotator: &mut TypeAnnotator,
    statement: &AstNode,
    operator: &AstNode,
    parameters: &AstNode,
    ctx: VisitorContext,
    operation: Operator,
) {
    let params = flatten_expression_list(parameters);
    let params_extracted: Vec<_> =
        params.iter().map(|param| extract_actual_parameter(param).clone()).collect();

    // Add type hints (only named arguments)
    params
        .iter()
        .zip(&params_extracted)
        .filter(|(it, _)| matches!(it.get_stmt(), AstStatement::Assignment(_)))
        .for_each(|(_, extracted)| {
            let param_type = annotator
                .annotation_map
                .get_type_or_void(extracted, annotator.index)
                .get_type_information()
                .get_name()
                .to_owned();
            annotator.annotation_map.annotate_type_hint(extracted, StatementAnnotation::value(param_type));
        });

    if params_extracted.iter().any(|param| {
        !annotator
            .annotation_map
            .get_type_or_void(param, annotator.index)
            .has_nature(TypeNature::Num, annotator.index)
    }) {
        // we are trying to call this function with a non-numerical type, so we redirect back to the resolver
        annotator.annotate_arguments(operator, parameters, &ctx);
        return;
    }

    let mut ctx = ctx;
    // find biggest type to later annotate it as type hint. this is done in a closure to avoid a borrow-checker tantrum later on due to
    // mutable and immutable borrow of TypeAnnotator
    let find_biggest_param_type_name = |annotator: &TypeAnnotator| {
        let mut bigger = annotator
            .annotation_map
            .get_type_or_void(params_extracted.first().expect("must have this parameter"), annotator.index);

        for param in params_extracted.iter().skip(1) {
            let right_type = annotator.annotation_map.get_type_or_void(param, annotator.index);
            bigger = get_bigger_type(bigger, right_type, annotator.index);
        }

        bigger.get_name().to_owned()
    };

    let bigger_type = find_biggest_param_type_name(annotator);

    // create nested AstStatement::BinaryExpression for each parameter, such that
    // ADD(a, b, c, d) ends up as (((a + b) + c) + d)
    let left = (*params_extracted.first().expect("Must exist")).clone();
    let new_statement = params_extracted.into_iter().skip(1).fold(left, |left, right| {
        AstFactory::create_binary_expression(left, operation, right.clone(), ctx.id_provider.next_id())
    });

    annotator.visit_statement(&ctx, &new_statement);
    annotator.update_expected_types(annotator.index.get_type_or_panic(&bigger_type), &new_statement);
    annotator.annotate(statement, StatementAnnotation::ReplacementAst { statement: new_statement });
    annotator.update_expected_types(annotator.index.get_type_or_panic(&bigger_type), statement);
}

fn annotate_variable_length_array_bound_function(
    annotator: &mut TypeAnnotator,
    parameters: Option<&AstNode>,
) {
    let Some(parameters) = parameters else {
        return;
    };
    let params = ast::flatten_expression_list(parameters);
    let vla = params.first().expect("must exist; covered by validation");
    let vla_param = extract_actual_parameter(vla);
    // if the VLA parameter is a VLA struct, annotate it as such
    let vla_type = annotator.annotation_map.get_type_or_void(vla_param, annotator.index);
    let vla_type_name = if vla_type.get_nature() == TypeNature::__VLA {
        vla_type.get_name()
    } else {
        // otherwise annotate it with an internal, reserved VLA type
        typesystem::__VLA_TYPE
    };
    annotator.annotation_map.annotate_type_hint(vla_param, StatementAnnotation::value(vla_type_name));
    if let Some(dim) = params.get(1) {
        let dim_param = extract_actual_parameter(dim);
        let dim_type = annotator.annotation_map.get_type_or_void(dim_param, annotator.index);
        if !dim_type.is_void() {
            // Use the actual type of the dimension parameter
            annotator
                .annotation_map
                .annotate_type_hint(dim_param, StatementAnnotation::value(dim_type.get_name()));
        } else {
            // Fallback to a default integer type if no type is available
            annotator.annotation_map.annotate_type_hint(dim_param, StatementAnnotation::value("DINT"));
        }
    }
}

fn validate_variable_length_array_bound_function(
    validator: &mut Validator,
    operator: &AstNode,
    parameters: Option<&AstNode>,
    annotations: &dyn AnnotationMap,
    index: &Index,
) {
    let Some(parameters) = parameters else {
        validator.push_diagnostic(Diagnostic::invalid_argument_count(2, 0, operator));

        // no params, nothing to validate
        return;
    };

    let params = ast::flatten_expression_list(parameters);

    if let &[vla, dim] = params.as_slice() {
        let [actual_vla, actual_idx] = [vla, dim].map(extract_actual_parameter);

        let idx_type = annotations.get_type_or_void(actual_idx, index);

        if !idx_type.has_nature(TypeNature::Int, index) {
            validator.push_diagnostic(
                Diagnostic::new(format!(
                    "Invalid type nature for generic argument. {} is no {}",
                    idx_type.get_name(),
                    TypeNature::Int
                ))
                .with_error_code("E062")
                .with_location(actual_idx),
            )
        }

        // TODO: consider adding validation for consts and enums once https://github.com/PLC-lang/rusty/issues/847 has been implemented
        if let AstStatement::Literal(AstLiteral::Integer(dimension_idx)) = actual_idx.get_stmt() {
            let dimension_idx = *dimension_idx as usize;

            let Some(n_dimensions) =
                annotations.get_type_or_void(actual_vla, index).get_type_information().get_dimension_count()
            else {
                // not a vla, validated via type nature
                return;
            };

            if dimension_idx < 1 || dimension_idx > n_dimensions {
                validator.push_diagnostic(
                    Diagnostic::new("Index out of bound").with_error_code("E046").with_location(operator),
                )
            }
        };
    } else {
        validator.push_diagnostic(Diagnostic::invalid_argument_count(2, params.len(), operator));
    }
}

fn validate_argument_count(
    validator: &mut Validator,
    operator: &AstNode,
    parameters: &Option<&AstNode>,
    expected: usize,
) {
    let Some(params) = parameters else {
        validator.push_diagnostic(Diagnostic::invalid_argument_count(expected, 0, operator));
        return;
    };

    let params = flatten_expression_list(params);

    if params.len() != expected {
        validator.push_diagnostic(Diagnostic::invalid_argument_count(expected, params.len(), operator));
    }
}

/// Helper function to extract the actual parameter from Assignment nodes when dealing with named arguments
/// For named arguments like `func(param := value)`, the AST contains an Assignment node where we need
/// to extract the right-hand side (the actual value). For positional arguments, use the parameter directly.
fn extract_actual_parameter(param: &AstNode) -> &AstNode {
    if let AstStatement::Assignment(assignment) = param.get_stmt() {
        // Named argument: extract the actual value from the right side of the assignment
        assignment.right.as_ref()
    } else {
        // Positional argument: use the parameter directly
        param
    }
}

/// Generates the code for the LOWER- AND UPPER_BOUND built-in functions, returning an error if the function
/// arguments are incorrect.
fn generate_variable_length_array_bound_function<'ink>(
    generator: &ExpressionCodeGenerator<'ink, '_>,
    params: &[&AstNode],
    is_lower: bool,
    location: SourceLocation,
) -> Result<ExpressionValue<'ink>, CodegenError> {
    let llvm = generator.llvm;
    let builder = &generator.llvm.builder;

    if let &[vla, dim] = params {
        let [actual_vla, actual_dim] = [vla, dim].map(extract_actual_parameter);

        let data_type_information =
            generator.annotations.get_type_or_void(actual_vla, generator.index).get_type_information();

        // TODO: most of the codegen errors should already be caught during validation.
        // once we abort codegen on critical errors, revisit and change to unreachable where possible
        if !data_type_information.is_vla() {
            return Err(CodegenError::GenericError(
                format!("Expected VLA type, received {}", data_type_information.get_name()),
                location,
            ));
        };

        let pointee = generator.llvm_index.get_associated_type(data_type_information.get_name())?;
        let vla = generator.generate_lvalue(actual_vla).unwrap();
        let dim = builder.build_struct_gep(pointee, vla, 1, "dim").unwrap();

        let accessor = match actual_dim.get_stmt() {
            // e.g. LOWER_BOUND(arr, 1)
            AstStatement::Literal(kind) => {
                let AstLiteral::Integer(value) = kind else {
                    let Some(type_name) = get_literal_actual_signed_type_name(kind, false) else {
                        unreachable!("type cannot be VOID")
                    };
                    return Err(CodegenError::GenericError(
                        format!("Invalid literal type. Expected INT type, received {type_name} type"),
                        location,
                    ));
                };
                // array offset start- and end-values are adjacent values in a flattened array -> 2 values per dimension, so in order
                // to read the correct values, the given index needs to be doubled. Additionally, the value is adjusted for 0-indexing.
                let offset = if is_lower { (value - 1) as u64 * 2 } else { (value - 1) as u64 * 2 + 1 };
                llvm.i32_type().const_int(offset, false)
            }
            // e.g. LOWER_BOUND(arr, idx + 3)
            _ => {
                let expression_value = generator.generate_expression(actual_dim)?;
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
                    )?,
                    "",
                )?;
                if !is_lower {
                    builder.build_int_add(offset, llvm.i32_type().const_int(1, false), "")?
                } else {
                    offset
                }
            }
        };
        let pointee = pointee.into_struct_type().get_field_type_at_index(1).unwrap();
        let gep_bound = unsafe {
            llvm.builder.build_in_bounds_gep(pointee, dim, &[llvm.i32_type().const_zero(), accessor], "")
        }?;
        let bound = llvm.builder.build_load(llvm.i32_type(), gep_bound, "")?;

        Ok(ExpressionValue::RValue(bound))
    } else {
        Err(CodegenError::GenericError("Invalid signature for LOWER_BOUND/UPPER_BOUND".to_string(), location))
    }
}

type AnnotationFunction = fn(&mut TypeAnnotator, &AstNode, &AstNode, Option<&AstNode>, VisitorContext);
type GenericNameResolver = fn(&str, &[GenericBinding], &FxHashMap<String, GenericType>) -> String;
type CodegenFunction = for<'ink, 'b> fn(
    &'b ExpressionCodeGenerator<'ink, 'b>,
    &[&AstNode],
    SourceLocation,
) -> Result<ExpressionValue<'ink>, CodegenError>;
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
    ) -> Result<ExpressionValue<'ink>, CodegenError> {
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
    let src = BUILTIN.values().map(|it| it.decl).collect::<Vec<&str>>().join(" ");
    let mut unit = parser::parse(
        lexer::lex_with_ids(&src, id_provider.clone(), SourceLocationFactory::internal(&src)),
        LinkageType::BuiltIn,
        "<builtin>",
    )
    .0;

    pre_process(&mut unit, id_provider);
    unit
}

/// Returns the requested function from the builtin index or None
pub fn get_builtin(name: &str) -> Option<&'static BuiltIn> {
    BUILTIN.get(name.to_uppercase().as_str())
}
