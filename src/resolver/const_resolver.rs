use std::collections::VecDeque;

use indexmap::IndexMap;

use crate::{
    ast::AstStatement,
    index::{Index, VariableIndexEntry},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LiteralValue {
    IntLiteral ( i128 ),
    RealLiteral ( f64 ),
    BoolLiteral (bool),
}

type ConstantsIndex<'a> = IndexMap<String, LiteralValue>;

/// returns the resolved constants index and a Vec of qualified names of constants that could not be resolved.
pub fn resolve_constants(index: &Index) -> (ConstantsIndex, Vec<String>) {
    let all_const_variables = index.get_all_variable_entries();

    let mut resolved_constants = ConstantsIndex::new();

    let mut constants: VecDeque<&VariableIndexEntry> = all_const_variables
        .filter(|it| it.is_constant())
        .collect();
    let mut tries_without_success = 0;
    //if we need more tries than entries we cannot solve the issue
    //TODO is can be more efficient
    while tries_without_success < constants.len() {
        if let Some(candidate) = constants.pop_front() {
            if let Some(initial) = index.maybe_get_constant_expression(&candidate.initial_value) {
                if let Ok(Some(literal)) = resolve(initial, &resolved_constants) {
                    resolved_constants.insert(candidate.get_qualified_name().to_string(), literal);
                    tries_without_success = 0;
                } else {
                    tries_without_success += 1;
                    constants.push_back(candidate) //try again later
                }
                //TODO handle Ok(None)
            }
        } else {
            //TODO
        }
    }

    (resolved_constants, 
        constants.iter().map(|it| it.get_qualified_name().to_string()).collect())
}

fn resolve(initial: &AstStatement, cindex: &ConstantsIndex) -> Result<Option<LiteralValue>, String> {
    let literal = match initial {
        AstStatement::LiteralInteger{value, ..} => Some(LiteralValue::IntLiteral(*value as i128)),
        AstStatement::LiteralReal{value, ..} => Some(LiteralValue::RealLiteral(value.parse::<f64>().map_err(|_err| format!("Cannot parse {} as Real", value))?)),
        AstStatement::LiteralBool{value, ..} => Some(LiteralValue::BoolLiteral(*value)),
        _ => resolve_complex(initial, cindex)?
    };
    Ok(literal)
}

fn resolve_complex<'a> (statement: &AstStatement, cindex: &ConstantsIndex) -> Result<Option<LiteralValue>, String> {
    match statement {
        AstStatement::Reference{name, ..} => {
            Ok(cindex.get(name).copied())
        },
        _ => Err(format!("cannot resolve constants: {:#?}", statement)),
    }
}
