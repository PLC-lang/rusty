use std::collections::VecDeque;

use crate::{
    index::{
        const_expressions::{ConstExpression, ConstId, UnresolvableKind},
        Index,
    },
    typesystem::{DataType, DataTypeInformation, StringEncoding, VOID_TYPE},
};
use plc_source::source_location::SourceLocation;

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
        UnresolvableConstant { id, reason: reason.to_string() }
    }

    pub fn incomplete_initialzation(id: &ConstId) -> Self {
        UnresolvableConstant::new(*id, "Incomplete initialization - cannot evaluate const expressions")
    }

    pub fn no_initial_value(id: &ConstId) -> Self {
        UnresolvableConstant::new(*id, "No initial value")
    }
}

/// returns the resolved constants index and a Vec of qualified names of constants that could not be resolved.
pub fn evaluate_constants(mut index: Index) -> (Index, Vec<UnresolvableConstant>) {
    let mut unresolvable: Vec<UnresolvableConstant> = Vec::new();
    let constants = index.get_const_expressions();

    //todo should these be references?
    let mut remaining_constants: VecDeque<ConstId> = constants.into_iter().map(|(id, _)| id).collect();

    //count how many consecutive resolve-attempts failed
    let mut failed_tries = 0;
    //if we need more tries than entries we cannot solve the issue
    //TODO is can be more efficient
    // - we can know when retries are smart
    // - with recursion, we can remove all of a recursion ring
    while failed_tries < remaining_constants.len() {
        if let Some(candidate) = remaining_constants.pop_front() {
            if let (Some(const_expr), target_type) = (
                index.get_const_expressions().find_const_expression(&candidate),
                index.get_const_expressions().find_expression_target_type(&candidate),
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

                match (initial_value_literal, &candidates_type) {
                    //we found an Int-Value and we found the const's datatype to be an unsigned Integer type (e.g. WORD)
                    (
                        Ok(Some(AstStatement::Literal { kind: AstLiteral::Integer(i), id, location })),
                        Some(DataTypeInformation::Integer { size, signed: false, .. }),
                    ) => {
                        // since we store literal-ints as i128 we need to truncate all of them down to their
                        // original size to avoid negative numbers
                        let mask = 2_i128.pow(*size) - 1; // bitmask for this type's size
                        let masked_value = i & mask; //delete all bits > size of data_type

                        index
                            .get_mut_const_expressions()
                            .mark_resolved(
                                &candidate,
                                AstStatement::Literal {
                                    id,
                                    location,
                                    kind: AstLiteral::new_integer(masked_value),
                                },
                            )
                            .expect("unknown id for const-expression"); //panic if we dont know the id
                        failed_tries = 0;
                    }

                    // we were able to evaluate a valid statement
                    (Ok(Some(literal)), _) => {
                        let literal = cast_if_necessary(
                            literal,
                            &index.get_const_expressions().find_expression_target_type(&candidate),
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
                    (Err(kind), _) => {
                        //error during resolving
                        unresolvable.push(UnresolvableConstant::new(candidate, kind.get_reason()));
                        index
                            .get_mut_const_expressions()
                            .mark_unresolvable(&candidate, kind)
                            .expect("unknown id for const-expression"); //panic if we dont know the id
                    }
                }
            } else {
                //no initial value in a const ... well
                unresolvable.push(UnresolvableConstant::no_initial_value(&candidate));
            }
        }
    }

    //import all constants that were note resolved in the loop above
    unresolvable.extend(remaining_constants.iter().map(UnresolvableConstant::incomplete_initialzation));

    (index, unresolvable)
}

fn do_resolve_candidate(index: &mut Index, candidate: ConstId, new_statement: AstStatement) {
    index
        .get_mut_const_expressions()
        .mark_resolved(&candidate, new_statement)
        .expect("unknown id for const-expression");
}

/// returns true, if the given expression needs to be evaluated.
/// literals must not be further evaluated and can be known at
/// compile time
fn needs_evaluation(expr: &AstStatement) -> bool {
    match expr {
        AstStatement::Literal { kind, .. } => match &kind {
            &AstLiteral::Array(Array { elements: Some(elements), .. }) => match elements.as_ref() {
                AstStatement::ExpressionList { expressions, .. } => expressions.iter().any(needs_evaluation),
                _ => needs_evaluation(elements.as_ref()),
            },

            // We want to check if literals will overflow, hence they'll need to be evaluated
            AstLiteral::Integer(_) | AstLiteral::Real(_) => true,

            _ => false,
        },
        AstStatement::Assignment { right, .. } => needs_evaluation(right.as_ref()),
        AstStatement::ExpressionList { expressions, .. } => expressions.iter().any(needs_evaluation),
        AstStatement::RangeStatement { start, end, .. } => needs_evaluation(start) || needs_evaluation(end),
        _ => true,
    }
}

/// generates an ast-statement that initializes the given type with the registered default values
fn get_default_initializer(
    id: AstId,
    target_type: &str,
    index: &Index,
    location: &SourceLocation,
) -> Result<Option<AstStatement>, UnresolvableKind> {
    if let Some(init) = index.get_initial_value_for_type(target_type) {
        evaluate(init, None, index) //TODO do we ave a scope here?
    } else {
        let dt = index.get_type_information_or_void(target_type);
        let init = match dt {
            DataTypeInformation::Pointer { .. } => {
                Some(AstStatement::Literal { kind: AstLiteral::Null, location: location.clone(), id })
            }
            DataTypeInformation::Integer { .. } => Some(AstStatement::Literal {
                kind: AstLiteral::new_integer(0),
                location: location.clone(),
                id,
            }),
            DataTypeInformation::Enum { name, elements, .. } => elements
                .get(0)
                .and_then(|default_enum| index.find_enum_element(name, default_enum))
                .and_then(|enum_element| enum_element.initial_value)
                .and_then(|initial_val| {
                    index.get_const_expressions().get_resolved_constant_statement(&initial_val)
                })
                .cloned(),
            DataTypeInformation::Float { .. } => Some(AstStatement::Literal {
                kind: AstLiteral::new_real("0.0".to_string()),
                location: location.clone(),
                id,
            }),
            DataTypeInformation::String { encoding, .. } => Some(AstStatement::Literal {
                kind: AstLiteral::new_string("".to_string(), encoding == &StringEncoding::Utf16),
                location: location.clone(),
                id,
            }),
            DataTypeInformation::SubRange { referenced_type, .. }
            | DataTypeInformation::Alias { referenced_type, .. } => {
                return get_default_initializer(id, referenced_type, index, location)
            }
            _ => None,
        };
        Ok(init)
    }
}

/// transforms the given literal to better fit the datatype of the candidate
/// effectively this casts an IntLiteral to a RealLiteral if necessary
fn cast_if_necessary(
    statement: AstStatement,
    target_type_name: &Option<&str>,
    index: &Index,
) -> AstStatement {
    let Some(dti) = target_type_name.and_then(|it| index.find_effective_type_info(it)) else {
         return statement;
    };

    if let AstStatement::Literal { kind: literal, location, id } = &statement {
        match literal {
            AstLiteral::Integer(value) if dti.is_float() => {
                return AstStatement::new_real(value.to_string(), *id, location.to_owned())
            }

            AstLiteral::String(StringValue { value, is_wide: true }) if dti.is_string_utf8() => {
                return AstStatement::new_string(value, false, *id, location.to_owned())
            }

            AstLiteral::String(StringValue { value, is_wide: false }) if dti.is_string_utf16() => {
                return AstStatement::new_string(value, true, *id, location.to_owned())
            }

            _ => (),
        }
    };

    statement
}

/// Checks if a literal integer or float overflows based on its value, and if so returns true.
fn does_overflow(literal: &AstStatement, dti: Option<&DataTypeInformation>) -> bool {
    let Some(dti) = dti else { return false };
    let AstStatement::Literal { kind, .. } = literal else { return false };

    if !matches!(kind, AstLiteral::Integer(_) | AstLiteral::Real(_)) {
        return false;
    };

    match &dti {
        DataTypeInformation::Integer { signed, size, semantic_size, .. } => {
            let size = semantic_size.as_ref().unwrap_or(size).to_owned();
            match (kind, signed, size) {
                // Signed
                (AstLiteral::Integer(value), true, 8) => i8::try_from(*value).is_err(),
                (AstLiteral::Integer(value), true, 16) => i16::try_from(*value).is_err(),
                (AstLiteral::Integer(value), true, 32) => i32::try_from(*value).is_err(),
                (AstLiteral::Integer(value), true, 64) => i64::try_from(*value).is_err(),

                // Unsigned
                (AstLiteral::Integer(value), false, 8) => u8::try_from(*value).is_err(),
                (AstLiteral::Integer(value), false, 16) => u16::try_from(*value).is_err(),
                (AstLiteral::Integer(value), false, 32) => u32::try_from(*value).is_err(),
                (AstLiteral::Integer(value), false, 64) => u64::try_from(*value).is_err(),

                // We are dealing with e.g. booleans if we have a semantic size != 8, 16, 32 or 64
                (AstLiteral::Integer(value), _, _) => {
                    let min = if *signed { -(2_i128.pow(size - 1)) } else { 0 };
                    let max = 2_i128.pow(size) - 1;

                    *value < min || *value > max
                }

                // XXX: Returning an error might be a better idea, as we'll be catching edge-cases we missed?
                _ => false,
            }
        }

        DataTypeInformation::Float { size, .. } => match (kind, size) {
            (AstLiteral::Real(value), 32) => value.parse::<f32>().map(f32::is_infinite).unwrap_or(false),
            (AstLiteral::Real(value), 64) => value.parse::<f64>().map(f64::is_infinite).unwrap_or(false),

            // XXX: Returning an error might be a better idea, as we'll be catching edge-cases we missed?
            _ => false,
        },

        // Only interested in integers and floats
        _ => false,
    }
}

pub fn evaluate(
    initial: &AstStatement,
    scope: Option<&str>,
    index: &Index,
) -> Result<Option<AstStatement>, UnresolvableKind> {
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
fn evaluate_with_target_hint(
    initial: &AstStatement,
    scope: Option<&str>,
    index: &Index,
    target_type: Option<&str>,
) -> Result<Option<AstStatement>, UnresolvableKind> {
    if !needs_evaluation(initial) {
        return Ok(Some(initial.clone())); // TODO hmm ...
    }

    let literal = match initial {
        AstStatement::Literal { kind, location, id } => match kind {
            AstLiteral::Array(Array { elements: Some(elements) }) => {
                let tt = target_type
                    .and_then(|it| index.find_effective_type_info(it))
                    .and_then(|it| it.get_inner_array_type_name())
                    .or(target_type);

                let inner_elements = AstStatement::get_as_list(elements)
                    .iter()
                    .map(|e| evaluate_with_target_hint(e, scope, index, tt))
                    .collect::<Result<Vec<Option<AstStatement>>, UnresolvableKind>>()?
                    .into_iter()
                    .collect::<Option<Vec<AstStatement>>>();

                //return a new array, or return none if one was not resolvable
                inner_elements.map(|ie| AstStatement::Literal {
                    id: *id,
                    kind: AstLiteral::new_array(Some(Box::new(AstStatement::ExpressionList {
                        expressions: ie,
                        id: *id,
                    }))),
                    location: location.clone(),
                })
            }

            AstLiteral::Integer(_) | AstLiteral::Real(_) => {
                let dti = target_type.and_then(|it| index.find_effective_type_info(it));
                if does_overflow(initial, dti) {
                    return Err(UnresolvableKind::Overflow(
                        format!("This will overflow for type {}", dti.unwrap().get_name()),
                        initial.get_location(),
                    ));
                }

                return Ok(Some(initial.clone()));
            }

            _ => return Ok(Some(initial.clone())),
        },

        AstStatement::DefaultValue { location, .. } => {
            return get_default_initializer(
                initial.get_id(),
                target_type.unwrap_or(VOID_TYPE),
                index,
                location,
            )
        }
        AstStatement::ReferenceExpr {
            access: ReferenceAccess::Cast(target), base: Some(type_name), ..
        } => {
            let dti = type_name
                .get_flat_reference_name()
                .and_then(|type_name| index.find_effective_type_info(type_name));
            match dti {
                Some(DataTypeInformation::Enum { name: enum_name, .. }) => {
                    if let AstStatement::Identifier { name: ref_name, .. } = target.as_ref() {
                        return index
                            .find_enum_element(enum_name, ref_name)
                            .ok_or_else(|| {
                                UnresolvableKind::Misc(format!(
                                    "Cannot resolve constant enum {enum_name}#{ref_name}."
                                ))
                            })
                            .and_then(|v| resolve_const_reference(v, ref_name, index));
                    } else {
                        return Err(UnresolvableKind::Misc("Cannot resolve unknown constant.".to_string()));
                    }
                }
                Some(dti) => {
                    evaluate_with_target_hint(target, scope, index, Some(dti.get_name()))?;
                    Some(get_cast_statement_literal(target, dti.get_name(), scope, index)?)
                }
                None => return Err(UnresolvableKind::Misc("Cannot resolve unknown Type-Cast.".to_string())),
            }
        }
        AstStatement::ReferenceExpr { access: ReferenceAccess::Member(reference), base, .. } => {
            if let Some(name) = reference.get_flat_reference_name() {
                index
                    .find_variable(
                        base.as_ref().and_then(|it| it.get_flat_reference_name()).or(scope),
                        std::slice::from_ref(&name),
                    )
                    .map(|variable| resolve_const_reference(variable, name, index))
                    .transpose()?
                    .flatten()
            } else {
                None
            }
        }
        AstStatement::BinaryExpression { left, right, operator, id, .. } => {
            let eval_left = evaluate(left, scope, index)?;
            let eval_right = evaluate(right, scope, index)?;
            if let Some((left, right)) = eval_left.zip(eval_right).as_ref() {
                let evalualted = match operator {
                    Operator::Plus => arithmetic_expression!(left, +, right, "+", *id)?,
                    Operator::Minus => arithmetic_expression!(left, -, right, "-", *id)?,
                    Operator::Multiplication => arithmetic_expression!(left, *, right, "*", *id)?,
                    Operator::Division if right.is_zero() => {
                        return Err(UnresolvableKind::Misc("Attempt to divide by zero".to_string()))
                    }
                    Operator::Division => arithmetic_expression!(left, /, right, "/", *id)?,
                    Operator::Modulo if right.is_zero() => {
                        return Err(UnresolvableKind::Misc(
                            "Attempt to calculate the remainder with a divisor of zero".to_string(),
                        ))
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
                        return Err(UnresolvableKind::Misc(format!(
                            "Cannot resolve operator {operator:?} in constant evaluation"
                        )))
                    }
                };

                // We have to re-evaluate to detect overflows
                evaluate_with_target_hint(&evalualted, scope, index, target_type)?
            } else {
                None //not all operators can be resolved
            }
        }

        // NOT x
        AstStatement::UnaryExpression { operator: Operator::Not, value, .. } => {
            let eval = evaluate(value, scope, index)?;
            match eval.clone() {
                Some(AstStatement::Literal { kind: AstLiteral::Bool(v), id, location }) => {
                    Some(AstStatement::Literal { kind: AstLiteral::Bool(!v), id, location })
                }
                Some(AstStatement::Literal { kind: AstLiteral::Integer(v), id, location }) => {
                    evaluate_with_target_hint(eval.as_ref().unwrap(), scope, index, target_type)?;
                    Some(AstStatement::Literal { kind: AstLiteral::Integer(!v), id, location })
                }
                None => {
                    None //not yet resolvable
                }
                _ => return Err(UnresolvableKind::Misc(format!("Cannot resolve constant Not {value:?}"))),
            }
        }
        // - x
        AstStatement::UnaryExpression { operator: Operator::Minus, value, .. } => {
            match evaluate(value, scope, index)? {
                Some(AstStatement::Literal { kind: AstLiteral::Integer(v), id, location }) => {
                    Some(AstStatement::Literal { kind: AstLiteral::Integer(-v), id, location })
                }
                Some(AstStatement::Literal { kind: AstLiteral::Real(v), id, location }) => {
                    let lit = AstStatement::Literal {
                        kind: AstLiteral::new_real(format!(
                            "{:}",
                            -(v.parse::<f64>())
                                .map_err(|err| UnresolvableKind::Misc(format!("{err:}: {v:}")))?
                        )),
                        id,
                        location,
                    };
                    evaluate_with_target_hint(&lit, scope, index, target_type)?
                }
                None => {
                    None //not yet resolvable
                }
                _ => return Err(UnresolvableKind::Misc(format!("Cannot resolve constant Minus {value:?}"))),
            }
        }
        AstStatement::ExpressionList { expressions, id } => {
            let inner_elements = expressions
                .iter()
                .map(|e| evaluate(e, scope, index))
                .collect::<Result<Vec<Option<AstStatement>>, UnresolvableKind>>()?
                .into_iter()
                .collect::<Option<Vec<AstStatement>>>();

            //return a new array, or return none if one was not resolvable
            inner_elements.map(|ie| AstStatement::ExpressionList { expressions: ie, id: *id })
        }
        AstStatement::MultipliedStatement { element, id, multiplier, location } => {
            let inner_elements = AstStatement::get_as_list(element.as_ref())
                .iter()
                .map(|e| evaluate(e, scope, index))
                .collect::<Result<Vec<Option<AstStatement>>, UnresolvableKind>>()?
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
                        element: Box::new(AstStatement::ExpressionList { expressions: ie, id: *id }),
                        multiplier: *multiplier,
                        location: location.clone(),
                    }
                }
            })
        }
        AstStatement::Assignment { left, right, id } => {
            //Right needs evaluation
            if let Some(right) = evaluate(right, scope, index)? {
                Some(AstStatement::Assignment { left: left.clone(), right: Box::new(right), id: *id })
            } else {
                Some(initial.clone())
            }
        }
        AstStatement::RangeStatement { start, end, id } => {
            let start = Box::new(evaluate(start, scope, index)?.unwrap_or_else(|| *start.to_owned()));
            let end = Box::new(evaluate(end, scope, index)?.unwrap_or_else(|| *end.to_owned()));
            Some(AstStatement::RangeStatement { start, end, id: *id })
        }
        _ => return Err(UnresolvableKind::Misc(format!("Cannot resolve constant: {initial:#?}"))),
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
) -> Result<Option<AstStatement>, UnresolvableKind> {
    if !variable.is_constant() {
        return Err(UnresolvableKind::Misc(format!("'{name}' is no const reference")));
    }

    if let Some(ConstExpression::Resolved(statement)) =
        variable.initial_value.as_ref().and_then(|it| index.get_const_expressions().find_const_expression(it))
    {
        Ok(Some(statement.clone()))
    } else {
        Ok(None) //not resolved yet
    }
}

/// Transforms the given `cast_statement` into a literal. For example `WORD#FFFF` will be a
/// [`AstLiteral::Integer`] with value `65_535` whereas `INT#FFFF` will not evaluate because it overflows
/// (see also [`does_overflow`] and [`evaluate_with_target_hint`]).
fn get_cast_statement_literal(
    cast_statement: &AstStatement,
    type_name: &str,
    scope: Option<&str>,
    index: &Index,
) -> Result<AstStatement, UnresolvableKind> {
    let dti = index.find_effective_type_info(type_name);
    match dti {
        Some(&DataTypeInformation::Integer { .. }) => {
            let evaluated_initial = evaluate_with_target_hint(cast_statement, scope, index, Some(type_name))?
                .as_ref()
                .map(|v| {
                    if let AstStatement::Literal { kind: AstLiteral::Integer(value), .. } = v {
                        Ok(*value)
                    } else {
                        Err(UnresolvableKind::Misc(format!("Expected integer value, found {v:?}")))
                    }
                })
                .transpose()?;

            if let Some(value) = evaluated_initial {
                return Ok(AstStatement::Literal {
                    kind: AstLiteral::new_integer(value),
                    id: cast_statement.get_id(),
                    location: cast_statement.get_location(),
                });
            }

            Err(UnresolvableKind::Misc(format!("Cannot resolve constant: {type_name}#{cast_statement:?}")))
        }

        Some(DataTypeInformation::Float { .. }) => {
            let evaluated = evaluate(cast_statement, scope, index)?;
            let value = match evaluated {
                Some(AstStatement::Literal { kind: AstLiteral::Integer(value), .. }) => Some(value as f64),
                Some(AstStatement::Literal { kind: AstLiteral::Real(value), .. }) => {
                    value.parse::<f64>().ok()
                }
                _ => {
                    return Err(UnresolvableKind::Misc(format!(
                        "Expected floating point type, got: {evaluated:?}"
                    )))
                }
            };

            let Some(value) = value else {
                return Err(UnresolvableKind::Misc(format!("cannot resolve constant: {type_name}#{cast_statement:?}")))
            };

            Ok(AstStatement::Literal {
                kind: AstLiteral::new_real(value.to_string()),
                id: cast_statement.get_id(),
                location: cast_statement.get_location(),
            })
        }

        _ => Err(UnresolvableKind::Misc(format!("Cannot resolve constant: {type_name}#{cast_statement:?}"))),
    }
}

macro_rules! cannot_eval_error {
    ($left:expr, $op_text:expr, $right:expr) => {
        Err(UnresolvableKind::Misc(format!("Cannot evaluate {:?} {:} {:?}", $left, $op_text, $right)))
    };
}
use cannot_eval_error;

macro_rules! arithmetic_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr, $resulting_id:expr) => {
        match ($left, $right) {
            (   AstStatement::Literal{kind: AstLiteral::Integer(lvalue), location: loc_left, ..},
                AstStatement::Literal{kind: AstLiteral::Integer(rvalue), location: loc_right, ..}) => {
                Ok(AstStatement::Literal{
                    id: $resulting_id, kind: AstLiteral::new_integer(lvalue $op rvalue), location: loc_left.span(loc_right)
                })
            },
            (   AstStatement::Literal{kind: AstLiteral::Integer(lvalue), location: loc_left, ..},
                AstStatement::Literal{kind: AstLiteral::Real(rvalue), location: loc_right, ..}) => {
                    let rvalue = rvalue.parse::<f64>()
                        .map_err(|err| UnresolvableKind::Misc(err.to_string()))?;
                Ok(AstStatement::Literal{
                    id: $resulting_id, kind: AstLiteral::new_real((*lvalue as f64 $op rvalue).to_string()), location: loc_left.span(loc_right)
                })
            },
            (   AstStatement::Literal{kind: AstLiteral::Real(lvalue), location: loc_left, ..},
                AstStatement::Literal{kind: AstLiteral::Integer(rvalue), location: loc_right, ..}) => {
                    let lvalue = lvalue.parse::<f64>()
                        .map_err(|err| UnresolvableKind::Misc(err.to_string()))?;
                Ok(AstStatement::Literal{
                    id: $resulting_id, kind: AstLiteral::new_real((lvalue $op *rvalue as f64).to_string()), location: loc_left.span(loc_right)
                })
            },
            (   AstStatement::Literal{kind: AstLiteral::Real(lvalue), location: loc_left, ..},
                AstStatement::Literal{kind: AstLiteral::Real(rvalue), location: loc_right, ..}) => {
                    let lvalue = lvalue.parse::<f64>()
                        .map_err(|err| UnresolvableKind::Misc(err.to_string()))?;
                    let rvalue = rvalue.parse::<f64>()
                        .map_err(|err| UnresolvableKind::Misc(err.to_string()))?;
                Ok(AstStatement::Literal{
                    id: $resulting_id, kind: AstLiteral::new_real((lvalue $op rvalue).to_string()), location: loc_left.span(loc_right)
                })
            },
            _ => cannot_eval_error!($left, $op_text, $right),
        }
    }
}
use arithmetic_expression;

macro_rules! bitwise_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr, $resulting_id:expr) => {
        match ($left, $right) {
            (   AstStatement::Literal{kind: AstLiteral::Integer(lvalue), location: loc_left, ..},
                AstStatement::Literal{kind: AstLiteral::Integer(rvalue), location: loc_right, ..}) => {
                Ok(AstStatement::Literal{
                    id: $resulting_id, kind: AstLiteral::new_integer(lvalue $op rvalue), location: loc_left.span(loc_right)
                })
            },

            (   AstStatement::Literal{kind: AstLiteral::Bool(lvalue), location: loc_left, ..},
                AstStatement::Literal{kind: AstLiteral::Bool(rvalue), location: loc_right, ..}) => {
                Ok(AstStatement::Literal{
                    id: $resulting_id, kind: AstLiteral::new_bool(lvalue $op rvalue), location: loc_left.span(loc_right)
                })
            },
            _ => cannot_eval_error!($left, $op_text, $right),
        }
    };
}
use bitwise_expression;

macro_rules! compare_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr, $resulting_id:expr) => {
        match ($left, $right) {
            (   AstStatement::Literal{kind: AstLiteral::Integer(lvalue), location: loc_left, ..},
                AstStatement::Literal{kind: AstLiteral::Integer(rvalue), location: loc_right, ..}) => {
                Ok(AstStatement::Literal{
                    id: $resulting_id, kind: AstLiteral::new_bool(lvalue $op rvalue), location: SourceLocation::without_file_ranged(loc_left.get_start() .. loc_right.get_start())
                })
            },
            (   AstStatement::Literal{kind: AstLiteral::Real{..},  ..},
                AstStatement::Literal{kind: AstLiteral::Real{..}, ..}) => {
               Err(UnresolvableKind::Misc("Cannot compare Reals without epsilon".into()))
            },
            (   AstStatement::Literal{kind: AstLiteral::Bool(lvalue), location: loc_left, ..},
                AstStatement::Literal{kind: AstLiteral::Bool(rvalue), location: loc_right, ..}) => {
                Ok(AstStatement::Literal{
                    id: $resulting_id, kind: AstLiteral::new_bool(lvalue $op rvalue), location: SourceLocation::without_file_ranged(loc_left.get_start() .. loc_right.get_start())
                })
            },
            _ => cannot_eval_error!($left, $op_text, $right),
        }
    }
}
use compare_expression;
use plc_ast::{
    ast::{AstId, AstStatement, Operator, ReferenceAccess},
    literals::{Array, AstLiteral, StringValue},
};
