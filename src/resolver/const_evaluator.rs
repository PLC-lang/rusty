use std::collections::VecDeque;

use crate::{
    ast::{AstStatement, Operator, SourceRange},
    index::{
        const_expressions::{ConstExpression, ConstId},
        Index,
    },
    typesystem::{
        DataType, DataTypeInformation, NativeByteType, NativeDintType, NativeDwordType,
        NativeIntType, NativeLintType, NativeLwordType, NativeSintType, NativeWordType,
        StringEncoding, DINT_SIZE, INT_SIZE, LINT_SIZE, SINT_SIZE,
    },
};

macro_rules! cannot_eval {
    ($left:expr, $op_text:expr, $right:expr) => {
        Err(format!(
            "Cannot evaluate {:?} {:} {:?}",
            $left, $op_text, $right
        ))
    };
}

macro_rules! arithmetic_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr) => {
        match ($left, $right) {
            (   AstStatement::LiteralInteger{value: lvalue, id: left_id, location: loc_left},
                AstStatement::LiteralInteger{value: rvalue, location: loc_right, ..}) => {
                Ok(AstStatement::LiteralInteger{
                    id: *left_id, value: lvalue $op rvalue, location: SourceRange::new(loc_left.get_start() .. loc_right.get_start())
                })
            },
            (   AstStatement::LiteralInteger{value: lvalue, id: left_id, location: loc_left},
                AstStatement::LiteralReal{value: rvalue, location: loc_right, ..}) => {
                    let rvalue = rvalue.parse::<f64>()
                        .map_err(|err| err.to_string())?;
                Ok(AstStatement::LiteralReal{
                    id: *left_id, value: (*lvalue as f64 $op rvalue).to_string(), location: SourceRange::new(loc_left.get_start() .. loc_right.get_start())
                })
            },
            (   AstStatement::LiteralReal{value: lvalue, id: left_id, location: loc_left},
                AstStatement::LiteralInteger{value: rvalue, location: loc_right, ..}) => {
                    let lvalue = lvalue.parse::<f64>()
                        .map_err(|err| err.to_string())?;
                Ok(AstStatement::LiteralReal{
                    id: *left_id, value: (lvalue $op *rvalue as f64).to_string(), location: SourceRange::new(loc_left.get_start() .. loc_right.get_start())
                })
            },
            (   AstStatement::LiteralReal{value: lvalue, id: left_id, location: loc_left},
                AstStatement::LiteralReal{value: rvalue, location: loc_right, ..}) => {
                    let lvalue = lvalue.parse::<f64>()
                        .map_err(|err| err.to_string())?;
                    let rvalue = rvalue.parse::<f64>()
                        .map_err(|err| err.to_string())?;
                Ok(AstStatement::LiteralReal{
                    id: *left_id, value: (lvalue $op rvalue).to_string(), location: SourceRange::new(loc_left.get_start() .. loc_right.get_start())
                })
            },
            _ => cannot_eval!($left, $op_text, $right),
        }
    }
}

macro_rules! bitwise_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr) => {
        match ($left, $right) {
            (   AstStatement::LiteralInteger{value: lvalue, id: left_id, location: loc_left},
                AstStatement::LiteralInteger{value: rvalue, location: loc_right, ..}) => {
                Ok(AstStatement::LiteralInteger{
                    id: *left_id, value: lvalue $op rvalue, location: SourceRange::new(loc_left.get_start() .. loc_right.get_start())
                })
            },
            (   AstStatement::LiteralBool{value: lvalue, id: left_id, location: loc_left},
                AstStatement::LiteralBool{value: rvalue, location: loc_right, ..}) => {
                Ok(AstStatement::LiteralBool{
                    id: *left_id, value: lvalue $op rvalue, location: SourceRange::new(loc_left.get_start() .. loc_right.get_start())
                })
            },
            _ => cannot_eval!($left, $op_text, $right),
        }
    };
}

macro_rules! compare_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr) => {
        match ($left, $right) {
            (   AstStatement::LiteralInteger{value: lvalue, id: left_id, location: loc_left},
                AstStatement::LiteralInteger{value: rvalue, location: loc_right, ..}) => {
                Ok(AstStatement::LiteralBool{
                    id: *left_id, value: lvalue $op rvalue, location: SourceRange::new(loc_left.get_start() .. loc_right.get_start())
                })
            },
            (   AstStatement::LiteralReal{..},
                AstStatement::LiteralReal{..}) => {
                Err("Cannot compare Reals without epsilon".into())
            },
            (   AstStatement::LiteralBool{value: lvalue, id: left_id, location: loc_left},
                AstStatement::LiteralBool{value: rvalue, location: loc_right, ..}) => {
                Ok(AstStatement::LiteralBool{
                    id: *left_id, value: lvalue $op rvalue, location: SourceRange::new(loc_left.get_start() .. loc_right.get_start())
                })
            },
            _ => cannot_eval!($left, $op_text, $right),
        }
    }
}

#[derive(PartialEq, Debug)]
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
        &AstStatement::LiteralArray {
            elements: Some(elements),
            ..
        } => match elements.as_ref() {
            AstStatement::ExpressionList { expressions, .. } => {
                expressions.iter().any(|it| needs_evaluation(it))
            }
            _ => needs_evaluation(elements.as_ref()),
        },
        _ => true,
    }
}

/// returns the resolved constants index and a Vec of qualified names of constants that could not be resolved.
pub fn evaluate_constants(mut index: Index) -> (Index, Vec<UnresolvableConstant>) {
    let mut unresolvable: Vec<UnresolvableConstant> = Vec::new();
    // let mut remaining_constants: VecDeque<&VariableIndexEntry> =
    // all_const_variables.filter(|it| it.is_constant()).collect();

    let constants = index.get_const_expressions();
    //todo should these be references?
    let mut remaining_constants: VecDeque<ConstId> = constants
        .into_iter()
        //.filter(|(_, expr)| needs_evaluation(expr)) //TODO can we optimize this?
        .map(|(id, _)| id)
        .collect();

    let mut tries_without_success = 0;
    //if we need more tries than entries we cannot solve the issue
    //TODO is can be more efficient
    // - we can know when retries are smart
    // - with recursion, we can remove all of a recursion ring
    while tries_without_success < remaining_constants.len() {
        if let Some(candidate) = remaining_constants.pop_front() {
            if let (Some(const_expr), target_type) = (
                index.get_resolved_const_statement(&candidate),
                index
                    .get_const_expressions()
                    .find_expression_target_type(&candidate),
            ) {
                let initial = const_expr.get_statement();

                let candidates_type = target_type
                    .and_then(|type_name| index.find_effective_type_by_name(type_name))
                    .map(DataType::get_type_information);

                let initial_value_literal = evaluate(initial, &index);

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
                            .unwrap(); //panic if we dont know the id
                        tries_without_success = 0;
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
                        index
                            .get_mut_const_expressions()
                            .mark_resolved(&candidate, literal)
                            .unwrap(); //panic if we dont know the id
                        tries_without_success = 0;
                    }

                    // we could not evaluate a valid statement - maybe later?
                    (Ok(None), _) => {
                        tries_without_success += 1;
                        remaining_constants.push_back(candidate) //try again later
                    }

                    // there was an error during evaluation
                    (Err(err_msg), _) => {
                        //error during resolving
                        index
                            .get_mut_const_expressions()
                            .mark_unresolvable(&candidate, err_msg.as_str())
                            .unwrap(); //panic at unknown Id

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
            .map(|it| UnresolvableConstant::incomplete_initialzation(it)),
    );

    (index, unresolvable)
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

/// evaluates the given Syntax-Tree `initial` to a `LiteralValue` if possible
/// - returns an Err if resolving caused an internal error (e.g. number parsing)
/// - returns None if the initializer cannot be resolved  (e.g. missing value)
pub fn evaluate(initial: &AstStatement, index: &Index) -> Result<Option<AstStatement>, String> {
    if !needs_evaluation(initial) {
        return Ok(Some(initial.clone())); // TODO hmm ...
    }

    let literal = match initial {
        AstStatement::CastStatement {
            target, type_name, ..
        } => Some(get_cast_literal(target, type_name, index)?),
        AstStatement::Reference { name, .. } => {
            //TODO respect scoping
            let variable = index.find_global_variable(name);
            resolve_const_reference(variable, name, index)?
        }
        AstStatement::QualifiedReference { elements, .. } => {
            // we made sure that there are exactly two references
            if elements.len() == 2 {
                if let (
                    AstStatement::Reference { name: pou_name, .. },
                    AstStatement::Reference {
                        name: variable_name,
                        ..
                    },
                ) = (&elements[0], &elements[1])
                {
                    let variable = index.find_member(pou_name, variable_name);
                    return resolve_const_reference(variable, variable_name, index);
                }
            }
            return Err("Qualified references only allow references to qualified variables in the form of 'POU.variable'".to_string());
        }
        AstStatement::BinaryExpression {
            left,
            right,
            operator,
            ..
        } => {
            let eval_left = evaluate(left, index)?;
            let eval_right = evaluate(right, index)?;
            if let Some((left, right)) = eval_left.zip(eval_right).as_ref() {
                Some(match operator {
                    Operator::Plus => arithmetic_expression!(left, +, right, "+")?,
                    Operator::Minus => arithmetic_expression!(left, -, right, "-")?,
                    Operator::Multiplication => arithmetic_expression!(left, *, right, "*")?,
                    Operator::Division if is_zero(right) => {
                        return Err("Attempt to divide by zero".to_string())
                    }
                    Operator::Division => arithmetic_expression!(left, /, right, "/")?,
                    Operator::Modulo if is_zero(right) => {
                        return Err(
                            "Attempt to calculate the remainder with a divisor of zero".to_string()
                        )
                    }
                    Operator::Modulo => arithmetic_expression!(left, %, right, "MOD")?,
                    Operator::Equal => compare_expression!(left, ==, right, "=")?,
                    Operator::NotEqual => compare_expression!(left, !=, right, "<>")?,
                    Operator::Greater => compare_expression!(left, >, right, ">")?,
                    Operator::GreaterOrEqual => compare_expression!(left, >=, right, ">=")?,
                    Operator::Less => compare_expression!(left, <, right, "<")?,
                    Operator::LessOrEqual => compare_expression!(left, <=, right, "<=")?,
                    Operator::And => bitwise_expression!(left, & , right, "AND")?,
                    Operator::Or => bitwise_expression!(left, | , right, "OR")?,
                    Operator::Xor => bitwise_expression!(left, ^, right, "XOR")?,
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
        } => match evaluate(value, index)? {
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
        } => match evaluate(value, index)? {
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
                    -(v.parse::<f64>()).map_err(|err| format!("{:}: {:}", err.to_string(), v))?
                ),
                id,
                location,
            }),
            None => {
                None //not yet resolvable
            }
            _ => return Err(format!("Cannot resolve constant Minus {:?}", value)),
        },
        _ => return Err(format!("Cannot resolve constant: {:#?}", initial)),
    };
    Ok(literal)
}

/// attempts to resolve the inital value of the given variable
/// may return Ok(None) if the variable's initial value can not be
/// resolved yet
fn resolve_const_reference(
    variable: Option<&crate::index::VariableIndexEntry>,
    name: &str,
    index: &Index,
) -> Result<Option<AstStatement>, String> {
    if variable.filter(|it| !it.is_constant()).is_some() {
        //the referenced variable is no const!
        return Err(format!("'{:}' is no const reference", name));
    }
    Ok(
        if let Some(ConstExpression::Resolved(statement)) = variable
            .and_then(|it| it.initial_value.as_ref())
            .and_then(|it| index.get_resolved_const_statement(it))
        {
            Some(statement.clone())
        } else {
            None
        },
    )
}

fn is_zero(v: &AstStatement) -> bool {
    matches!(v, AstStatement::LiteralInteger { value: 0, .. })
}

fn get_cast_literal(
    initial: &AstStatement,
    type_name: &str,
    index: &Index,
) -> Result<AstStatement, String> {
    match index
        .find_effective_type_by_name(type_name)
        .map(DataType::get_type_information)
    {
        Some(&crate::typesystem::DataTypeInformation::Integer { size, signed, .. }) => {
            let evaluated_initial = evaluate(initial, index)?
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
                let value: i128 = match (signed, size) {
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
                            type_name, initial
                        ))
                    }
                };
                Ok(AstStatement::LiteralInteger {
                    value,
                    id: initial.get_id(),
                    location: initial.get_location(),
                })
            } else {
                Err(format!(
                    "Cannot resolve constant: {:}#{:?}",
                    type_name, initial
                ))
            }
        }
        //Some(&crate::typesystem::DataTypeInformation::Float{..}) => {},
        _ => Err(format!(
            "Cannot resolve constant: {:}#{:?}",
            type_name, initial
        )),
    }
}
