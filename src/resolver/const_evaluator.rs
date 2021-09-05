use std::collections::VecDeque;

use indexmap::IndexMap;

use crate::{
    ast::AstStatement,
    index::{Index, VariableIndexEntry},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LiteralValue {
    Int(i128),
    Real(f64),
    Bool(bool),
}

macro_rules! arith {
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

//TODO this is a evaluator, not a resolver!

type ConstantsIndex<'a> = IndexMap<String, LiteralValue>;

/// returns the resolved constants index and a Vec of qualified names of constants that could not be resolved.
pub fn evaluate_constants(index: &Index) -> (ConstantsIndex, Vec<String>) {
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
                if let Ok(Some(literal)) = evaluate(initial, &resolved_constants) {
                    resolved_constants.insert(
                        candidate.get_qualified_name().to_string(),
                        cast_if_necessary(literal, candidate, index),
                    );
                    tries_without_success = 0;
                } else {
                    tries_without_success += 1;
                    constants.push_back(candidate) //try again later
                }
                //TODO handle Ok(None)
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

    (resolved_constants, unresolvable)
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
fn evaluate(
    initial: &AstStatement,
    cindex: &ConstantsIndex,
) -> Result<Option<LiteralValue>, String> {
    let literal = match initial {
        AstStatement::LiteralInteger { value, .. } => {
            Some(LiteralValue::Int(*value as i128))
        }
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
            if let (Some(left), Some(right)) = (evaluate(left, cindex)?, evaluate(right, cindex)?) {
                Some(match operator {
                    crate::ast::Operator::Plus => arith!(left, +, right, "+")?,
                    crate::ast::Operator::Minus => arith!(left, -, right, "-")?,
                    crate::ast::Operator::Multiplication => arith!(left, *, right, "*")?,
                    crate::ast::Operator::Division => arith!(left, /, right, "/")?,
                    crate::ast::Operator::Modulo => modulo(&left, &right)?,
                    crate::ast::Operator::Equal => eq(&left, &right)?,
                    crate::ast::Operator::NotEqual => neq(&left, &right)?,
                    crate::ast::Operator::And => {
                        LiteralValue::Bool(expect_bool(left)? & expect_bool(right)?)
                    }
                    crate::ast::Operator::Or => {
                        LiteralValue::Bool(expect_bool(left)? | expect_bool(right)?)
                    }
                    crate::ast::Operator::Xor => {
                        LiteralValue::Bool(expect_bool(left)? ^ expect_bool(right)?)
                    }
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
        } => evaluate(value, cindex)
            .and_then(|it| it.map(expect_bool).transpose())?
            .map(|it| LiteralValue::Bool(!it)),
        _ => return Err(format!("cannot resolve constants: {:#?}", initial)),
    };
    Ok(literal)
}

/// checks if the give LiteralValue is a bool and returns its value.
/// will return an Err if it is not a BoolLiteral
fn expect_bool(lit: LiteralValue) -> Result<bool, String> {
    if let LiteralValue::Bool(v) = lit {
        return Ok(v);
    }
    return Err(format!("Expected BoolLiteral but found {:?}", lit));
}

fn modulo(left: &LiteralValue, right: &LiteralValue) -> Result<LiteralValue, String> {
    match (left, right) {
        (LiteralValue::Int(l), LiteralValue::Int(r)) => {
            Ok(LiteralValue::Int(l % r))
        }
        (LiteralValue::Int(l), LiteralValue::Real(r)) => {
            Ok(LiteralValue::Real((*l as f64) % r))
        }
        (LiteralValue::Real(l), LiteralValue::Int(r)) => {
            Ok(LiteralValue::Real(l % (*r as f64)))
        }
        (LiteralValue::Real(l), LiteralValue::Real(r)) => {
            Ok(LiteralValue::Real(l % r))
        }
        _ => Err(format!("Cannot evaluate {:?} MOD {:?}", left, right)),
    }
}

fn eq(left: &LiteralValue, right: &LiteralValue) -> Result<LiteralValue, String> {
    match (left, right) {
        (LiteralValue::Int(l), LiteralValue::Int(r)) => {
            Ok(LiteralValue::Bool(l == r))
        }
        (LiteralValue::Real(l), LiteralValue::Real(r)) => {
            Err("Cannot compare Reals without epsilon".into())
        }
        (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
            Ok(LiteralValue::Bool(l == r))
        }
        _ => Err(format!("Cannot evaluate {:?} = {:?}", left, right)),
    }
}

fn neq(left: &LiteralValue, right: &LiteralValue) -> Result<LiteralValue, String> {
    match (left, right) {
        (LiteralValue::Int(l), LiteralValue::Int(r)) => {
            Ok(LiteralValue::Bool(l != r))
        }
        (LiteralValue::Real(_), LiteralValue::Real(_)) => {
            Err("Cannot compare Reals without epsilon".into())
        }
        (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
            Ok(LiteralValue::Bool(l != r))
        }
        _ => Err(format!("Cannot evaluate {:?} <> {:?}", left, right)),
    }
}
