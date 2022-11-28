use std::collections::VecDeque;

use crate::{
    ast::{AstId, AstStatement, Operator, SourceRange},
    index::{
        const_expressions::{ConstExpression, ConstId},
        Index,
    },
    typesystem::{
        DataType, DataTypeInformation, NativeByteType, NativeDintType, NativeDwordType,
        NativeIntType, NativeLintType, NativeLwordType, NativeSintType, NativeWordType,
        StringEncoding, DINT_SIZE, INT_SIZE, LINT_SIZE, SINT_SIZE, VOID_TYPE,
    },
};

macro_rules! cannot_eval_error {
    ($left:expr, $op_text:expr, $right:expr) => {
        Err(format!(
            "Cannot evaluate {:?} {:} {:?}",
            $left, $op_text, $right
        ))
    };
}

macro_rules! arithmetic_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr, $resulting_id:expr) => {
        match ($left, $right) {
            (   AstStatement::LiteralInteger{value: lvalue, location: loc_left, ..},
                AstStatement::LiteralInteger{value: rvalue, location: loc_right, ..}) => {
                Ok(AstStatement::LiteralInteger{
                    id: $resulting_id, value: lvalue $op rvalue, location: loc_left.span(loc_right)
                })
            },
            (   AstStatement::LiteralInteger{value: lvalue, location: loc_left, ..},
                AstStatement::LiteralReal{value: rvalue, location: loc_right, ..}) => {
                    let rvalue = rvalue.parse::<f64>()
                        .map_err(|err| err.to_string())?;
                Ok(AstStatement::LiteralReal{
                    id: $resulting_id, value: (*lvalue as f64 $op rvalue).to_string(), location: loc_left.span(loc_right)
                })
            },
            (   AstStatement::LiteralReal{value: lvalue, location: loc_left, ..},
                AstStatement::LiteralInteger{value: rvalue, location: loc_right, ..}) => {
                    let lvalue = lvalue.parse::<f64>()
                        .map_err(|err| err.to_string())?;
                Ok(AstStatement::LiteralReal{
                    id: $resulting_id, value: (lvalue $op *rvalue as f64).to_string(), location: loc_left.span(loc_right)
                })
            },
            (   AstStatement::LiteralReal{value: lvalue, location: loc_left, ..},
                AstStatement::LiteralReal{value: rvalue, location: loc_right, ..}) => {
                    let lvalue = lvalue.parse::<f64>()
                        .map_err(|err| err.to_string())?;
                    let rvalue = rvalue.parse::<f64>()
                        .map_err(|err| err.to_string())?;
                Ok(AstStatement::LiteralReal{
                    id: $resulting_id, value: (lvalue $op rvalue).to_string(), location: loc_left.span(loc_right)
                })
            },
            _ => cannot_eval_error!($left, $op_text, $right),
        }
    }
}

macro_rules! bitwise_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr, $resulting_id:expr) => {
        match ($left, $right) {
            (   AstStatement::LiteralInteger{value: lvalue, location: loc_left, ..},
                AstStatement::LiteralInteger{value: rvalue, location: loc_right, ..}) => {
                Ok(AstStatement::LiteralInteger{
                    id: $resulting_id, value: lvalue $op rvalue, location: loc_left.span(loc_right)
                })
            },
            (   AstStatement::LiteralBool{value: lvalue, location: loc_left, ..},
                AstStatement::LiteralBool{value: rvalue, location: loc_right, ..}) => {
                Ok(AstStatement::LiteralBool{
                    id: $resulting_id, value: lvalue $op rvalue, location: loc_left.span(loc_right)
                })
            },
            _ => cannot_eval_error!($left, $op_text, $right),
        }
    };
}

macro_rules! compare_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr, $resulting_id:expr) => {
        match ($left, $right) {
            (   AstStatement::LiteralInteger{value: lvalue, location: loc_left, ..},
                AstStatement::LiteralInteger{value: rvalue, location: loc_right, ..}) => {
                Ok(AstStatement::LiteralBool{
                    id: $resulting_id, value: lvalue $op rvalue, location: SourceRange::without_file(loc_left.get_start() .. loc_right.get_start())
                })
            },
            (   AstStatement::LiteralReal{..},
                AstStatement::LiteralReal{..}) => {
                Err("Cannot compare Reals without epsilon".into())
            },
            (   AstStatement::LiteralBool{value: lvalue, location: loc_left, ..},
                AstStatement::LiteralBool{value: rvalue, location: loc_right, ..}) => {
                Ok(AstStatement::LiteralBool{
                    id: $resulting_id, value: lvalue $op rvalue, location: SourceRange::without_file(loc_left.get_start() .. loc_right.get_start())
                })
            },
            _ => cannot_eval_error!($left, $op_text, $right),
        }
    }
}

/// a wrapper for an unresolvable const-expression with the reason
/// why it could not be resolved
#[derive(PartialEq, Eq, Debug)]
pub struct UnresolvableConstant {
    pub id: ConstId,
    pub reason: String,
    //location
    //source-file
}

impl UnresolvableConstant {
    pub fn new(id: ConstId, reason: &str) -> Self {
        UnresolvableConstant {
            id,
            reason: reason.to_string(),
        }
    }

    pub fn incomplete_initialzation(id: &ConstId) -> Self {
        UnresolvableConstant::new(
            *id,
            "Incomplete initialization - cannot evaluate const expressions",
        )
    }

    pub fn no_initial_value(id: &ConstId) -> Self {
        UnresolvableConstant::new(*id, "No initial value")
    }
}

/// returns true, if the given expression needs to be evaluated.
/// literals must not be further evaluated and can be known at
/// compile time
fn needs_evaluation(expr: &AstStatement) -> bool {
    match &expr {
        AstStatement::LiteralBool { .. }
        | AstStatement::LiteralInteger { .. }
        | AstStatement::LiteralReal { .. }
        | AstStatement::LiteralDate { .. }
        | AstStatement::LiteralDateAndTime { .. }
        | AstStatement::LiteralTimeOfDay { .. }
        | AstStatement::LiteralTime { .. }
        | AstStatement::LiteralString { .. } => false,
        AstStatement::Assignment { right, .. } => needs_evaluation(right.as_ref()),
        &AstStatement::LiteralArray {
            elements: Some(elements),
            ..
        } => match elements.as_ref() {
            AstStatement::ExpressionList { expressions, .. } => {
                expressions.iter().any(needs_evaluation)
            }
            _ => needs_evaluation(elements.as_ref()),
        },
        AstStatement::ExpressionList { expressions, .. } => {
            expressions.iter().any(needs_evaluation)
        }
        AstStatement::RangeStatement { start, end, .. } => {
            needs_evaluation(start) || needs_evaluation(end)
        }
        _ => true,
    }
}

/// returns the resolved constants index and a Vec of qualified names of constants that could not be resolved.
pub fn evaluate_constants(mut index: Index) -> (Index, Vec<UnresolvableConstant>) {
    let mut unresolvable: Vec<UnresolvableConstant> = Vec::new();
    let constants = index.get_const_expressions();

    //todo should these be references?
    let mut remaining_constants: VecDeque<ConstId> =
        constants.into_iter().map(|(id, _)| id).collect();

    //count how many consecutive resolve-attempts failed
    let mut failed_tries = 0;
    //if we need more tries than entries we cannot solve the issue
    //TODO is can be more efficient
    // - we can know when retries are smart
    // - with recursion, we can remove all of a recursion ring
    while failed_tries < remaining_constants.len() {
        if let Some(candidate) = remaining_constants.pop_front() {
            if let (Some(const_expr), target_type) = (
                index
                    .get_const_expressions()
                    .find_const_expression(&candidate),
                index
                    .get_const_expressions()
                    .find_expression_target_type(&candidate),
            ) {
                let candidates_type = target_type
                    .and_then(|type_name| index.find_effective_type_by_name(type_name))
                    .map(DataType::get_type_information);

                if candidates_type
                    .map(|it| (it.is_struct() || it.is_array()) && const_expr.is_default())
                    .unwrap_or(false)
                {
                    // we skip structs and arrays with default-initializers since they cannot be used inside expressions of other consts.
                    // we leave generating the default value to the llvm-index later.
                    // And we resolve it so we dont get a validation problem
                    let expr_clone = const_expr.get_statement().clone();
                    do_resolve_candidate(&mut index, candidate, expr_clone);
                    continue;
                }

                let initial_value_literal = evaluate_with_target_hint(
                    const_expr.get_statement(),
                    const_expr.get_qualifier(),
                    &index,
                    target_type,
                );

                match (initial_value_literal, candidates_type) {
                    //we found an Int-Value and we found the const's datatype to be an unsigned Integer type (e.g. WORD)
                    (
                        Ok(Some(AstStatement::LiteralInteger {
                            value,
                            id,
                            location,
                        })),
                        Some(DataTypeInformation::Integer {
                            size,
                            signed: false,
                            ..
                        }),
                    ) => {
                        // since we store literal-ints as i128 we need to truncate all of them down to their
                        // original size to avoid negative numbers
                        let mask = 2_i128.pow(*size) - 1; // bitmask for this type's size
                        let masked_value = value & mask; //delete all bits > size of data_type

                        index
                            .get_mut_const_expressions()
                            .mark_resolved(
                                &candidate,
                                AstStatement::LiteralInteger {
                                    id,
                                    location,
                                    value: masked_value,
                                },
                            )
                            .expect("unknown id for const-expression"); //panic if we dont know the id
                        failed_tries = 0;
                    }

                    // we were able to evaluate a valid statement
                    (Ok(Some(literal)), _) => {
                        let literal = cast_if_necessary(
                            literal,
                            &index
                                .get_const_expressions()
                                .find_expression_target_type(&candidate),
                            &index,
                        );
                        do_resolve_candidate(&mut index, candidate, literal);
                        failed_tries = 0;
                    }

                    // we could not evaluate a valid statement - maybe later?
                    (Ok(None), _) => {
                        failed_tries += 1;
                        remaining_constants.push_back(candidate) //try again later
                    }

                    // there was an error during evaluation
                    (Err(err_msg), _) => {
                        //error during resolving
                        index
                            .get_mut_const_expressions()
                            .mark_unresolvable(&candidate, err_msg.as_str())
                            .expect("unknown id for const-expression"); //panic if we dont know the id

                        unresolvable.push(UnresolvableConstant::new(candidate, err_msg.as_str()))
                    }
                }
            } else {
                //no initial value in a const ... well
                unresolvable.push(UnresolvableConstant::no_initial_value(&candidate));
            }
        }
    }

    //import all constants that were note resolved in the loop above
    unresolvable.extend(
        remaining_constants
            .iter()
            .map(UnresolvableConstant::incomplete_initialzation),
    );

    (index, unresolvable)
}

fn do_resolve_candidate(
    index: &mut Index,
    candidate: generational_arena::Index,
    new_statement: AstStatement,
) {
    index
        .get_mut_const_expressions()
        .mark_resolved(&candidate, new_statement)
        .expect("unknown id for const-expression");
}

/// generates an ast-statement that initializes the given type with the registered default values
fn get_default_initializer(
    id: AstId,
    target_type: &str,
    index: &Index,
    location: &SourceRange,
) -> Result<Option<AstStatement>, String> {
    if let Some(init) = index.get_initial_value_for_type(target_type) {
        evaluate(init, None, index) //TODO do we ave a scope here?
    } else {
        let dt = index.get_type_information_or_void(target_type);
        let init = match dt {
            DataTypeInformation::Pointer { .. } => Some(AstStatement::LiteralNull {
                location: location.clone(),
                id,
            }),
            DataTypeInformation::Integer { .. } => Some(AstStatement::LiteralInteger {
                value: 0,
                location: location.clone(),
                id,
            }),
            DataTypeInformation::Enum { name, elements, .. } => elements
                .get(0)
                .and_then(|default_enum| index.find_enum_element(name, default_enum))
                .and_then(|enum_element| enum_element.initial_value)
                .and_then(|initial_val| {
                    index
                        .get_const_expressions()
                        .get_resolved_constant_statement(&initial_val)
                })
                .cloned(),
            DataTypeInformation::Float { .. } => Some(AstStatement::LiteralReal {
                value: "0.0".to_string(),
                location: location.clone(),
                id,
            }),
            DataTypeInformation::String { encoding, .. } => Some(AstStatement::LiteralString {
                value: "".to_string(),
                is_wide: encoding == &StringEncoding::Utf16,
                location: location.clone(),
                id,
            }),
            DataTypeInformation::SubRange {
                referenced_type, ..
            }
            | DataTypeInformation::Alias {
                referenced_type, ..
            } => return get_default_initializer(id, referenced_type, index, location),
            _ => None,
        };
        Ok(init)
    }
}

/// transforms the given literal to better fit the datatype of the candidate
/// effectively this casts an IntLiteral to a RealLiteral if necessary
fn cast_if_necessary(
    literal: AstStatement,
    target_type_name: &Option<&str>,
    index: &Index,
) -> AstStatement {
    if let Some(data_type) = target_type_name.and_then(|it| index.find_effective_type_by_name(it)) {
        match &literal {
            AstStatement::LiteralInteger {
                value,
                id,
                location,
            } => {
                if data_type.get_type_information().is_float() {
                    return AstStatement::LiteralReal {
                        value: format!("{:}", value),
                        id: *id,
                        location: location.clone(),
                    };
                }
            }
            AstStatement::LiteralString {
                value,
                id,
                location,
                is_wide: false,
            } => {
                if matches!(
                    data_type.get_type_information(),
                    DataTypeInformation::String {
                        encoding: StringEncoding::Utf16,
                        ..
                    }
                ) {
                    return AstStatement::LiteralString {
                        value: value.clone(),
                        id: *id,
                        location: location.clone(),
                        is_wide: true,
                    };
                }
            }
            AstStatement::LiteralString {
                value,
                id,
                location,
                is_wide: true,
            } => {
                if matches!(
                    data_type.get_type_information(),
                    DataTypeInformation::String {
                        encoding: StringEncoding::Utf8,
                        ..
                    }
                ) {
                    return AstStatement::LiteralString {
                        value: value.clone(),
                        id: *id,
                        location: location.clone(),
                        is_wide: false,
                    };
                }
            }
            _ => {}
        }
    }
    literal
}

pub fn evaluate(
    initial: &AstStatement,
    scope: Option<&str>,
    index: &Index,
) -> Result<Option<AstStatement>, String> {
    evaluate_with_target_hint(initial, scope, index, None)
}

/// evaluates the given Syntax-Tree `initial` to a `LiteralValue` if possible
/// ## Arguments
/// - `initial` the constant expression to resolve
/// - `scope` an optional qualifier to be used when resolving references
/// - `index` the global symbol-table
/// ## Returns
/// - returns an Err if resolving caused an internal error (e.g. number parsing)
/// - returns None if the initializer cannot be resolved  (e.g. missing value)
pub fn evaluate_with_target_hint(
    initial: &AstStatement,
    scope: Option<&str>,
    index: &Index,
    target_type: Option<&str>,
) -> Result<Option<AstStatement>, String> {
    if !needs_evaluation(initial) {
        return Ok(Some(initial.clone())); // TODO hmm ...
    }

    let literal = match initial {
        AstStatement::DefaultValue { location, .. } => {
            return get_default_initializer(
                initial.get_id(),
                target_type.unwrap_or(VOID_TYPE),
                index,
                location,
            )
        }
        AstStatement::CastStatement {
            target, type_name, ..
        } => match index.find_effective_type_info(type_name) {
            Some(DataTypeInformation::Enum {
                name: enum_name, ..
            }) => {
                if let AstStatement::Reference { name: ref_name, .. } = target.as_ref() {
                    return index
                        .find_enum_element(enum_name, ref_name)
                        .ok_or_else(|| {
                            format!("Cannot resolve constant enum {}#{}.", enum_name, ref_name)
                        })
                        .and_then(|v| resolve_const_reference(v, ref_name, index));
                } else {
                    return Err("Cannot resolve unknown constant.".to_string());
                }
            }
            _ => Some(get_cast_statement_literal(target, type_name, scope, index)?),
        },
        AstStatement::Reference { name, .. } => index
            .find_variable(scope, std::slice::from_ref(&name.as_str()))
            .map(|variable| resolve_const_reference(variable, name, index))
            .transpose()?
            .flatten(),
        AstStatement::QualifiedReference { elements, .. } => {
            // we made sure that there are exactly two references
            //TODO https://github.com/ghaith/rusty/issues/291 - once we can initialize structs, we need to allow generic qualified references here
            if elements.len() == 2 {
                if let (
                    AstStatement::Reference { name: pou_name, .. },
                    AstStatement::Reference {
                        name: variable_name,
                        ..
                    },
                ) = (&elements[0], &elements[1])
                {
                    return index
                        .find_member(pou_name, variable_name)
                        .ok_or_else(|| "Cannot resolve unknown constant.".to_string())
                        .and_then(|variable| {
                            resolve_const_reference(variable, variable_name, index)
                        });
                }
            }
            return Err("Qualified references only allow references to qualified variables in the form of 'POU.variable'".to_string());
        }
        AstStatement::BinaryExpression {
            left,
            right,
            operator,
            id,
            ..
        } => {
            let eval_left = evaluate(left, scope, index)?;
            let eval_right = evaluate(right, scope, index)?;
            if let Some((left, right)) = eval_left.zip(eval_right).as_ref() {
                Some(match operator {
                    Operator::Plus => arithmetic_expression!(left, +, right, "+", *id)?,
                    Operator::Minus => arithmetic_expression!(left, -, right, "-", *id)?,
                    Operator::Multiplication => arithmetic_expression!(left, *, right, "*", *id)?,
                    Operator::Division if is_zero(right) => {
                        return Err("Attempt to divide by zero".to_string())
                    }
                    Operator::Division => arithmetic_expression!(left, /, right, "/", *id)?,
                    Operator::Modulo if is_zero(right) => {
                        return Err(
                            "Attempt to calculate the remainder with a divisor of zero".to_string()
                        )
                    }
                    Operator::Modulo => arithmetic_expression!(left, %, right, "MOD", *id)?,
                    Operator::Equal => compare_expression!(left, ==, right, "=", *id)?,
                    Operator::NotEqual => compare_expression!(left, !=, right, "<>", *id)?,
                    Operator::Greater => compare_expression!(left, >, right, ">", *id)?,
                    Operator::GreaterOrEqual => compare_expression!(left, >=, right, ">=", *id)?,
                    Operator::Less => compare_expression!(left, <, right, "<", *id)?,
                    Operator::LessOrEqual => compare_expression!(left, <=, right, "<=", *id)?,
                    Operator::And => bitwise_expression!(left, & , right, "AND", *id)?,
                    Operator::Or => bitwise_expression!(left, | , right, "OR", *id)?,
                    Operator::Xor => bitwise_expression!(left, ^, right, "XOR", *id)?,
                    _ => {
                        return Err(format!(
                            "Cannot resolve operator {:?} in constant evaluation",
                            operator
                        ))
                    }
                })
            } else {
                None //not all operators can be resolved
            }
        }

        // NOT x
        AstStatement::UnaryExpression {
            operator: Operator::Not,
            value,
            ..
        } => match evaluate(value, scope, index)? {
            Some(AstStatement::LiteralBool {
                value: v,
                id,
                location,
            }) => Some(AstStatement::LiteralBool {
                value: !v,
                id,
                location,
            }),
            Some(AstStatement::LiteralInteger {
                value: v,
                id,
                location,
            }) => Some(AstStatement::LiteralInteger {
                value: !v,
                id,
                location,
            }),
            None => {
                None //not yet resolvable
            }
            _ => return Err(format!("Cannot resolve constant Not {:?}", value)),
        },
        // - x
        AstStatement::UnaryExpression {
            operator: Operator::Minus,
            value,
            ..
        } => match evaluate(value, scope, index)? {
            Some(AstStatement::LiteralInteger {
                value: v,
                id,
                location,
            }) => Some(AstStatement::LiteralInteger {
                value: -v,
                id,
                location,
            }),
            Some(AstStatement::LiteralReal {
                value: v,
                id,
                location,
            }) => Some(AstStatement::LiteralReal {
                value: format!(
                    "{:}",
                    -(v.parse::<f64>()).map_err(|err| format!("{:}: {:}", err, v))?
                ),
                id,
                location,
            }),
            None => {
                None //not yet resolvable
            }
            _ => return Err(format!("Cannot resolve constant Minus {:?}", value)),
        },
        AstStatement::LiteralArray {
            id,
            elements: Some(elements),
            location,
            ..
        } => {
            let inner_elements = AstStatement::get_as_list(elements)
                .iter()
                .map(|e| evaluate(e, scope, index))
                .collect::<Result<Vec<Option<AstStatement>>, String>>()?
                .into_iter()
                .collect::<Option<Vec<AstStatement>>>();

            //return a new array, or return none if one was not resolvable
            inner_elements.map(|ie| AstStatement::LiteralArray {
                id: *id,
                elements: Some(Box::new(AstStatement::ExpressionList {
                    expressions: ie,
                    id: *id,
                })),
                location: location.clone(),
            })
        }
        AstStatement::ExpressionList { expressions, id } => {
            let inner_elements = expressions
                .iter()
                .map(|e| evaluate(e, scope, index))
                .collect::<Result<Vec<Option<AstStatement>>, String>>()?
                .into_iter()
                .collect::<Option<Vec<AstStatement>>>();

            //return a new array, or return none if one was not resolvable
            inner_elements.map(|ie| AstStatement::ExpressionList {
                expressions: ie,
                id: *id,
            })
        }
        AstStatement::MultipliedStatement {
            element,
            id,
            multiplier,
            location,
        } => {
            let inner_elements = AstStatement::get_as_list(element.as_ref())
                .iter()
                .map(|e| evaluate(e, scope, index))
                .collect::<Result<Vec<Option<AstStatement>>, String>>()?
                .into_iter()
                .collect::<Option<Vec<AstStatement>>>();

            //return a new array, or return none if one was not resolvable
            inner_elements.map(|ie| {
                if let [ie] = ie.as_slice() {
                    AstStatement::MultipliedStatement {
                        id: *id,
                        element: Box::new(ie.clone()), //TODO
                        multiplier: *multiplier,
                        location: location.clone(),
                    }
                } else {
                    AstStatement::MultipliedStatement {
                        id: *id,
                        element: Box::new(AstStatement::ExpressionList {
                            expressions: ie,
                            id: *id,
                        }),
                        multiplier: *multiplier,
                        location: location.clone(),
                    }
                }
            })
        }
        AstStatement::Assignment { left, right, id } => {
            //Right needs evaluation
            if let Some(right) = evaluate(right, scope, index)? {
                Some(AstStatement::Assignment {
                    left: left.clone(),
                    right: Box::new(right),
                    id: *id,
                })
            } else {
                Some(initial.clone())
            }
        }
        AstStatement::RangeStatement { start, end, id } => {
            let start =
                Box::new(evaluate(start, scope, index)?.unwrap_or_else(|| *start.to_owned()));
            let end = Box::new(evaluate(end, scope, index)?.unwrap_or_else(|| *end.to_owned()));
            Some(AstStatement::RangeStatement {
                start,
                end,
                id: *id,
            })
        }
        _ => return Err(format!("Cannot resolve constant: {:#?}", initial)),
    };
    Ok(literal)
}

/// attempts to resolve the inital value of this reference's target
/// may return Ok(None) if the variable's initial value can not be
/// resolved yet
fn resolve_const_reference(
    variable: &crate::index::VariableIndexEntry,
    name: &str,
    index: &Index,
) -> Result<Option<AstStatement>, String> {
    if variable.is_constant() {
        if let Some(ConstExpression::Resolved(statement)) = variable
            .initial_value
            .as_ref()
            .and_then(|it| index.get_const_expressions().find_const_expression(it))
        {
            Ok(Some(statement.clone()))
        } else {
            Ok(None) //not resolved yet
        }
    } else {
        //the referenced variabale is no const!
        Err(format!("'{:}' is no const reference", name))
    }
}

fn is_zero(v: &AstStatement) -> bool {
    matches!(v, AstStatement::LiteralInteger { value: 0, .. })
}

/// takes the given cast_statement transform it into a literal that better represents
/// the data_type given by the `type_name`
/// (e.g. WORD#FFFF ... =-1 vs. DINT#FFFF ... =0x0000_FFFF)
fn get_cast_statement_literal(
    cast_statement: &AstStatement,
    type_name: &str,
    scope: Option<&str>,
    index: &Index,
) -> Result<AstStatement, String> {
    match index
        .find_effective_type_by_name(type_name)
        .map(DataType::get_type_information)
    {
        Some(&crate::typesystem::DataTypeInformation::Integer {
            signed,
            size,
            semantic_size,
            ..
        }) => {
            let evaluated_initial = evaluate(cast_statement, scope, index)?
                .as_ref()
                .map(|v| {
                    if let AstStatement::LiteralInteger { value, .. } = v {
                        Ok(*value)
                    } else {
                        Err(format!("Expected integer value, found {:?}", v))
                    }
                })
                .transpose()?;
            if let Some(value) = evaluated_initial {
                const SIGNED: bool = true;
                const UNSIGNED: bool = false;
                let value: i128 = match (signed, semantic_size.unwrap_or(size)) {
                    //signed
                    (SIGNED, SINT_SIZE) => (value as NativeSintType) as i128,
                    (SIGNED, INT_SIZE) => (value as NativeIntType) as i128,
                    (SIGNED, DINT_SIZE) => (value as NativeDintType) as i128,
                    (SIGNED, LINT_SIZE) => (value as NativeLintType) as i128,
                    //unsigned
                    (UNSIGNED, SINT_SIZE) => (value as NativeByteType) as i128,
                    (UNSIGNED, INT_SIZE) => (value as NativeWordType) as i128,
                    (UNSIGNED, DINT_SIZE) => (value as NativeDwordType) as i128,
                    (UNSIGNED, LINT_SIZE) => (value as NativeLwordType) as i128,
                    _ => {
                        return Err(format!(
                            "Cannot resolve constant: {:}#{:?}",
                            type_name, cast_statement
                        ))
                    }
                };
                Ok(AstStatement::LiteralInteger {
                    value,
                    id: cast_statement.get_id(),
                    location: cast_statement.get_location(),
                })
            } else {
                Err(format!(
                    "Cannot resolve constant: {:}#{:?}",
                    type_name, cast_statement
                ))
            }
        }

        //Some(&crate::typesystem::DataTypeInformation::Float{..}) => {},
        _ => Err(format!(
            "Cannot resolve constant: {:}#{:?}",
            type_name, cast_statement
        )),
    }
}
