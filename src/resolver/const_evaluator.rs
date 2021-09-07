use std::collections::VecDeque;

use crate::{
    ast::{AstStatement, Operator},
    index::{ConstantsIndex, Index, LiteralValue, VariableIndexEntry},
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
            (LiteralValue::Int(l), LiteralValue::Int(r)) => {
                Ok(LiteralValue::Int(l $op r))
            }
            (LiteralValue::Int(l), LiteralValue::Real(r)) => {
                Ok(LiteralValue::Real((*l as f64) $op r))
            }
            (LiteralValue::Real(l), LiteralValue::Int(r)) => {
                Ok(LiteralValue::Real(l $op (*r as f64)))
            }
            (LiteralValue::Real(l), LiteralValue::Real(r)) => {
                Ok(LiteralValue::Real(l $op r))
            }
            _ => cannot_eval!($left, $op_text, $right),
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
            _ => cannot_eval!($left, $op_text, $right),
        }
    };
}

macro_rules! compare_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr) => {
        match ($left, $right) {
            (LiteralValue::Int(l), LiteralValue::Int(r)) => Ok(LiteralValue::Bool(l $op r)),
            (LiteralValue::Real(_), LiteralValue::Real(_)) => {
                Err("Cannot compare Reals without epsilon".into())
            }
            (LiteralValue::Bool(l), LiteralValue::Bool(r)) => Ok(LiteralValue::Bool(l $op r)),
            _ => cannot_eval!($left, $op_text, $right),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct UnresolvableConstant {
    pub qualified_name: String,
    pub reason: String,
    //location
    //source-file
}

impl UnresolvableConstant {
    pub fn new(qualified_name: &str, reason: &str) -> Self {
        UnresolvableConstant {
            qualified_name: qualified_name.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn incomplete_initialzation(qualified_name: &str) -> Self {
        UnresolvableConstant::new(
            qualified_name,
            "Incomplete initialization - cannot evaluate const expressions",
        )
    }

    pub fn no_initial_value(qualified_name: &str) -> Self {
        UnresolvableConstant::new(qualified_name, "No initial value")
    }
}

/// returns the resolved constants index and a Vec of qualified names of constants that could not be resolved.
pub fn evaluate_constants(mut index: Index) -> (Index, Vec<UnresolvableConstant>) {
    let all_const_variables = index.get_all_variable_entries();

    let mut resolved_constants = ConstantsIndex::new();
    let mut unresolvable: Vec<UnresolvableConstant> = Vec::new();
    let mut remaining_constants: VecDeque<&VariableIndexEntry> =
        all_const_variables.filter(|it| it.is_constant()).collect();
    let mut tries_without_success = 0;
    //if we need more tries than entries we cannot solve the issue
    //TODO is can be more efficient
    // - we can know when retries are smart
    // - with recursion, we can remove all of a recursion ring
    while tries_without_success < remaining_constants.len() {
        if let Some(candidate) = remaining_constants.pop_front() {
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
                    (Ok(None), _) => {
                        tries_without_success += 1;
                        remaining_constants.push_back(candidate) //try again later
                    }
                    (Err(err_msg), _) => {
                        //error during resolving
                        unresolvable.push(UnresolvableConstant::new(
                            candidate.get_qualified_name(),
                            err_msg.as_str(),
                        ));
                    }
                }
            } else {
                //no initial value in a const ... well
                unresolvable.push(UnresolvableConstant::no_initial_value(
                    candidate.get_qualified_name(),
                ));
            }
        }
    }

    //import all constants that were note resolved in the loop above
    unresolvable.extend(
        remaining_constants
            .iter()
            .map(|it| UnresolvableConstant::incomplete_initialzation(it.get_qualified_name())),
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
        match &literal {
            LiteralValue::Int(v) => {
                if data_type.get_type_information().is_float() {
                    return LiteralValue::Real(*v as f64);
                }
            }
            LiteralValue::String(v) => {
                if matches!(
                    data_type.get_type_information(),
                    DataTypeInformation::String {
                        encoding: StringEncoding::Utf16,
                        ..
                    }
                ) {
                    return LiteralValue::WString(v.clone());
                }
            }
            LiteralValue::WString(v) => {
                if matches!(
                    data_type.get_type_information(),
                    DataTypeInformation::String {
                        encoding: StringEncoding::Utf8,
                        ..
                    }
                ) {
                    return LiteralValue::String(v.clone());
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
        AstStatement::LiteralString {
            value,
            is_wide: false,
            ..
        } => Some(LiteralValue::String(value.clone())),
        AstStatement::LiteralString {
            value,
            is_wide: true,
            ..
        } => Some(LiteralValue::WString(value.clone())),
        AstStatement::LiteralBool { value, .. } => Some(LiteralValue::Bool(*value)),
        AstStatement::Reference { name, .. } => cindex.get(name).cloned(),
        AstStatement::BinaryExpression {
            left,
            right,
            operator,
            ..
        } => {
            let eval_left = evaluate(left, cindex, index)?;
            let eval_right = evaluate(right, cindex, index)?;
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
        AstStatement::UnaryExpression {
            operator: Operator::Not,
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

fn is_zero(v: &LiteralValue) -> bool {
    matches!(v, LiteralValue::Int(0))
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
