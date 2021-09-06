use std::collections::VecDeque;

use crate::{
    ast::AstStatement,
    index::{ConstantsIndex, Index, LiteralValue, VariableIndexEntry},
    typesystem::{
        DataType, DINT_SIZE, INT_SIZE, LINT_SIZE, NATIVE_BYTE_TYPE, NATIVE_DINT_TYPE,
        NATIVE_DWORD_TYPE, NATIVE_INT_TYPE, NATIVE_LINT_TYPE, NATIVE_LWORD_TYPE, NATIVE_SINT_TYPE,
        NATIVE_WORD_TYPE, SINT_SIZE,
    },
};

macro_rules! arithmetic_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr) => {
        match ($left, $right) {
            (LiteralValue::Int(l), LiteralValue::Int(r)) => {
                Ok(LiteralValue::Int(l $op r))
            }
            (LiteralValue::Int(l), LiteralValue::Real(r)) => {
                Ok(LiteralValue::Real((l as f64) $op r))
            }
            (LiteralValue::Real(l), LiteralValue::Int(r)) => {
                Ok(LiteralValue::Real(l $op (r as f64)))
            }
            (LiteralValue::Real(l), LiteralValue::Real(r)) => {
                Ok(LiteralValue::Real(l $op r))
            }
            _ => Err(format!("Cannot evaluate {:?} {:} {:?}", $left, $op_text, $right)),
        }
    };
}

macro_rules! bitwise_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr) => {
        match ($left, $right) {
            (LiteralValue::Int(l), LiteralValue::Int(r)) => {
                Ok(LiteralValue::Int(l $op r))
            }
            (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
                Ok(LiteralValue::Bool(l $op r))
            }
            _ => Err(format!("Cannot evaluate {:?} {:} {:?}", $left, $op_text, $right)),
        }
    };
}

/// returns the resolved constants index and a Vec of qualified names of constants that could not be resolved.
pub fn evaluate_constants(mut index: Index) -> (Index, Vec<String>) {
    let all_const_variables = index.get_all_variable_entries();

    let mut resolved_constants = ConstantsIndex::new();
    let mut unresolvable: Vec<String> = Vec::new();
    let mut constants: VecDeque<&VariableIndexEntry> =
        all_const_variables.filter(|it| it.is_constant()).collect();
    let mut tries_without_success = 0;
    //if we need more tries than entries we cannot solve the issue
    //TODO is can be more efficient
    // - we can know when retries are smart
    // - with recursion, we can remove all of a recursion ring
    while tries_without_success < constants.len() {
        if let Some(candidate) = constants.pop_front() {
            if let Some(initial) = index.maybe_get_constant_expression(&candidate.initial_value) {
                let candidates_type = index
                    .find_effective_type_by_name(candidate.get_type_name())
                    .map(DataType::get_type_information);
                let initial_value_literal = evaluate(initial, &resolved_constants, &index);

                match (initial_value_literal, candidates_type) {
                    (
                        Ok(Some(LiteralValue::Int(v))),
                        Some(&crate::typesystem::DataTypeInformation::Integer {
                            signed: false,
                            size,
                            ..
                        }),
                    ) => {
                        //we found an Int-Value and we found the const's datatype to be an unsigned Integer type (e.g. WORD)

                        // since we store literal-ints as i128 we need to truncate all of them down to their
                        // original size to avoid negative numbers
                        let mask = 2_i128.pow(size) - 1; // bitmask for this type's size
                        let masked_value = v & mask; //delete all bits > size of data_type
                        resolved_constants.insert(
                            candidate.get_qualified_name().to_string(),
                            cast_if_necessary(LiteralValue::Int(masked_value), candidate, &index),
                        );
                        tries_without_success = 0;
                    }
                    (Ok(Some(literal)), _) => {
                        resolved_constants.insert(
                            candidate.get_qualified_name().to_string(),
                            cast_if_necessary(literal, candidate, &index),
                        );
                        tries_without_success = 0;
                    }
                    //TODO handle Ok(None)
                    _ => {
                        tries_without_success += 1;
                        constants.push_back(candidate) //try again later
                    }
                }
            } else {
                //no initial value in a const ... well
                unresolvable.push(candidate.get_qualified_name().to_string());
            }
        }
    }

    //import all constants that were note resolved in the loop above
    unresolvable.extend(
        constants
            .iter()
            .map(|it| it.get_qualified_name().to_string()),
    );

    index.import_resolved_constants(resolved_constants);
    (index, unresolvable)
}

/// transforms the given literal to better fit the datatype of the candidate
/// effectively this casts an IntLiteral to a RealLiteral if necessary
fn cast_if_necessary(
    literal: LiteralValue,
    candidate: &VariableIndexEntry,
    index: &Index,
) -> LiteralValue {
    if let Some(data_type) = index.find_effective_type_by_name(candidate.get_type_name()) {
        if let LiteralValue::Int(v) = literal {
            if data_type.get_type_information().is_float() {
                return LiteralValue::Real(v as f64);
            }
        }
    }
    literal
}

/// evaluates the given Syntax-Tree `initial` to a `LiteralValue` if possible
/// - returns an Err if resolving caused an internal error (e.g. number parsing)
/// - returns None if the initializer cannot be resolved  (e.g. missing value)
pub fn evaluate(
    initial: &AstStatement,
    cindex: &ConstantsIndex,
    index: &Index,
) -> Result<Option<LiteralValue>, String> {
    let literal = match initial {
        AstStatement::LiteralInteger { value, .. } => Some(LiteralValue::Int(*value as i128)),
        AstStatement::CastStatement {
            target, type_name, ..
        } => Some(get_cast_literal(target, type_name, index)?),
        AstStatement::LiteralReal { value, .. } => Some(LiteralValue::Real(
            value
                .parse::<f64>()
                .map_err(|_err| format!("Cannot parse {} as Real", value))?,
        )),
        AstStatement::LiteralBool { value, .. } => Some(LiteralValue::Bool(*value)),
        AstStatement::Reference { name, .. } => cindex.get(name).copied(),
        AstStatement::BinaryExpression {
            left,
            right,
            operator,
            ..
        } => {
            if let (Some(left), Some(right)) = (
                evaluate(left, cindex, index)?,
                evaluate(right, cindex, index)?,
            ) {
                Some(match operator {
                    crate::ast::Operator::Plus => arithmetic_expression!(left, +, right, "+")?,
                    crate::ast::Operator::Minus => arithmetic_expression!(left, -, right, "-")?,
                    crate::ast::Operator::Multiplication => {
                        arithmetic_expression!(left, *, right, "*")?
                    }
                    crate::ast::Operator::Division => arithmetic_expression!(left, /, right, "/")?,
                    crate::ast::Operator::Modulo => modulo(&left, &right)?,
                    crate::ast::Operator::Equal => eq(&left, &right)?,
                    crate::ast::Operator::NotEqual => neq(&left, &right)?,
                    crate::ast::Operator::And => bitwise_expression!(left, & , right, "AND")?,
                    crate::ast::Operator::Or => bitwise_expression!(left, | , right, "OR")?,
                    crate::ast::Operator::Xor => bitwise_expression!(left, ^, right, "XOR")?,
                    _ => return Err(format!("cannot resolve operation: {:#?}", operator)),
                })
            } else {
                None //not all operators can be resolved
            }
        }
        AstStatement::UnaryExpression {
            operator: crate::ast::Operator::Not,
            value,
            ..
        } => match evaluate(value, cindex, index)? {
            Some(LiteralValue::Bool(v)) => Some(LiteralValue::Bool(!v)),
            Some(LiteralValue::Int(v)) => Some(LiteralValue::Int(!v)),
            _ => return Err(format!("Cannot resolve constant NOT {:?}", value)),
        },
        _ => return Err(format!("Cannot resolve constant: {:#?}", initial)),
    };
    Ok(literal)
}

fn get_cast_literal(
    initial: &AstStatement,
    type_name: &str,
    index: &Index,
) -> Result<LiteralValue, String> {
    match index
        .find_effective_type_by_name(type_name)
        .map(DataType::get_type_information)
    {
        Some(&crate::typesystem::DataTypeInformation::Integer { size, signed, .. }) => {
            let evaluated_initial = evaluate(initial, &ConstantsIndex::default(), index)?
                .map(|v| v.get_int_value())
                .transpose()?;
            if let Some(value) = evaluated_initial {
                let value: i128 = match signed {
                    //signed
                    true if size == SINT_SIZE => (value as NATIVE_SINT_TYPE) as i128,
                    true if size == INT_SIZE => (value as NATIVE_INT_TYPE) as i128,
                    true if size == DINT_SIZE => (value as NATIVE_DINT_TYPE) as i128,
                    true if size == LINT_SIZE => (value as NATIVE_LINT_TYPE) as i128,
                    //unsigned
                    false if size == SINT_SIZE => (value as NATIVE_BYTE_TYPE) as i128,
                    false if size == INT_SIZE => (value as NATIVE_WORD_TYPE) as i128,
                    false if size == DINT_SIZE => (value as NATIVE_DWORD_TYPE) as i128,
                    false if size == LINT_SIZE => (value as NATIVE_LWORD_TYPE) as i128,
                    _ => {
                        return Err(format!(
                            "Cannot resolve constant: {:}#{:?}",
                            type_name, initial
                        ))
                    }
                };
                Ok(LiteralValue::Int(value /*& mask*/)) //delete all bits > size of data_type
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

fn modulo(left: &LiteralValue, right: &LiteralValue) -> Result<LiteralValue, String> {
    match (left, right) {
        (LiteralValue::Int(l), LiteralValue::Int(r)) => Ok(LiteralValue::Int(l % r)),
        (LiteralValue::Int(l), LiteralValue::Real(r)) => Ok(LiteralValue::Real((*l as f64) % r)),
        (LiteralValue::Real(l), LiteralValue::Int(r)) => Ok(LiteralValue::Real(l % (*r as f64))),
        (LiteralValue::Real(l), LiteralValue::Real(r)) => Ok(LiteralValue::Real(l % r)),
        _ => Err(format!("Cannot evaluate {:?} MOD {:?}", left, right)),
    }
}

fn eq(left: &LiteralValue, right: &LiteralValue) -> Result<LiteralValue, String> {
    match (left, right) {
        (LiteralValue::Int(l), LiteralValue::Int(r)) => Ok(LiteralValue::Bool(l == r)),
        (LiteralValue::Real(_), LiteralValue::Real(_)) => {
            Err("Cannot compare Reals without epsilon".into())
        }
        (LiteralValue::Bool(l), LiteralValue::Bool(r)) => Ok(LiteralValue::Bool(l == r)),
        _ => Err(format!("Cannot evaluate {:?} = {:?}", left, right)),
    }
}

fn neq(left: &LiteralValue, right: &LiteralValue) -> Result<LiteralValue, String> {
    match (left, right) {
        (LiteralValue::Int(l), LiteralValue::Int(r)) => Ok(LiteralValue::Bool(l != r)),
        (LiteralValue::Real(_), LiteralValue::Real(_)) => {
            Err("Cannot compare Reals without epsilon".into())
        }
        (LiteralValue::Bool(l), LiteralValue::Bool(r)) => Ok(LiteralValue::Bool(l != r)),
        _ => Err(format!("Cannot evaluate {:?} <> {:?}", left, right)),
    }
}
