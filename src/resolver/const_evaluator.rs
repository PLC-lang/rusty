use std::collections::VecDeque;

use plc_ast::{
    ast::{
        AstFactory, AstId, AstNode, AstStatement, BinaryExpression, MultipliedStatement, Operator,
        ReferenceAccess, ReferenceExpr, UnaryExpression,
    },
    literals::{Array, AstLiteral, StringValue},
};
use plc_source::source_location::SourceLocation;
use serde::{Deserialize, Serialize};

use crate::{
    index::{
        const_expressions::{ConstExpression, ConstId, InitData, UnresolvableKind},
        Index,
    },
    typesystem::{DataType, DataTypeInformation, StringEncoding, VOID_TYPE},
};

/// a wrapper for an unresolvable const-expression with the reason
/// why it could not be resolved
#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct UnresolvableConstant {
    pub id: ConstId,
    pub kind: Option<UnresolvableKind>,
    //location
    //source-file
}

impl UnresolvableConstant {
    pub fn new(id: ConstId, reason: &str) -> Self {
        UnresolvableConstant { id, kind: Some(UnresolvableKind::Misc(reason.into())) }
    }

    pub fn with_kind(self, kind: UnresolvableKind) -> Self {
        UnresolvableConstant { id: self.id, kind: Some(kind) }
    }

    pub fn incomplete_initialzation(id: &ConstId) -> Self {
        UnresolvableConstant::new(*id, "Incomplete initialization - cannot evaluate const expressions")
    }

    pub fn no_initial_value(id: &ConstId) -> Self {
        UnresolvableConstant::new(*id, "No initial value")
    }

    pub fn get_reason(&self) -> Option<&str> {
        self.kind.as_ref().map(|it| it.get_reason())
    }
}

/// Returns the resolved constants index and a Vec of qualified names of constants that could not be resolved.
/// After constants have been evaluated, enum defaults are finalized.
/// TODO: revisit enum-defaults-fixup as part of `evaluate_constants` after @ghaith's changes to the initializer/constructor handling.
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
                    const_expr.get_lhs(),
                );

                match (initial_value_literal, &candidates_type) {
                    //we found an Int-Value and we found the const's datatype to be an unsigned Integer type (e.g. WORD)
                    (
                        Ok(Some(AstNode {
                            stmt: AstStatement::Literal(AstLiteral::Integer(i)),
                            id,
                            location,
                            ..
                        })),
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
                                AstNode::new_literal(AstLiteral::new_integer(masked_value), id, location),
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
                        unresolvable.push(
                            UnresolvableConstant::new(candidate, kind.get_reason()).with_kind(kind.clone()),
                        );
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

    // Fix up enum defaults after constants are resolved
    index.finalize_enum_defaults();

    //import all constants that were note resolved in the loop above
    unresolvable.extend(remaining_constants.iter().map(UnresolvableConstant::incomplete_initialzation));

    (index, unresolvable)
}

fn do_resolve_candidate(index: &mut Index, candidate: ConstId, new_statement: AstNode) {
    index
        .get_mut_const_expressions()
        .mark_resolved(&candidate, new_statement)
        .expect("unknown id for const-expression");
}

/// returns true, if the given expression needs to be evaluated.
/// literals must not be further evaluated and can be known at
/// compile time
fn needs_evaluation(expr: &AstNode) -> bool {
    match expr.get_stmt() {
        AstStatement::Literal(kind) => match &kind {
            &AstLiteral::Array(Array { elements: Some(elements), .. }) => match &elements.get_stmt() {
                AstStatement::ExpressionList(expressions) => expressions.iter().any(needs_evaluation),
                _ => needs_evaluation(elements.as_ref()),
            },

            // We want to check if literals will overflow, hence they'll need to be evaluated
            AstLiteral::Integer(_) | AstLiteral::Real(_) => true,

            _ => false,
        },
        AstStatement::Assignment(data) => needs_evaluation(data.right.as_ref()),
        AstStatement::ExpressionList(expressions) => expressions.iter().any(needs_evaluation),
        AstStatement::RangeStatement(data) => needs_evaluation(&data.start) || needs_evaluation(&data.end),
        _ => true,
    }
}

/// generates an ast-statement that initializes the given type with the registered default values
fn get_default_initializer(
    id: AstId,
    target_type: &str,
    index: &Index,
    location: &SourceLocation,
) -> Result<Option<AstNode>, UnresolvableKind> {
    if let Some(init) = index.get_initial_value_for_type(target_type) {
        evaluate(init, None, index, None) //TODO do we ave a scope here?
    } else {
        let dt = index.get_type_information_or_void(target_type);
        let init = match dt {
            DataTypeInformation::Pointer { .. } => {
                Some(AstFactory::create_literal(AstLiteral::Null, location.clone(), id))
            }
            DataTypeInformation::Integer { .. } => {
                Some(AstFactory::create_literal(AstLiteral::new_integer(0), location.clone(), id))
            }
            DataTypeInformation::Enum { name, variants, .. } => variants
                .first()
                .and_then(|default_enum| index.find_enum_variant(name, default_enum.get_name()))
                .and_then(|enum_element| enum_element.initial_value)
                .and_then(|initial_val| {
                    index.get_const_expressions().get_resolved_constant_statement(&initial_val)
                })
                .cloned(),
            DataTypeInformation::Float { .. } => Some(AstFactory::create_literal(
                AstLiteral::new_real("0.0".to_string()),
                location.clone(),
                id,
            )),
            DataTypeInformation::String { encoding, .. } => Some(AstFactory::create_literal(
                AstLiteral::new_string("".to_string(), encoding == &StringEncoding::Utf16),
                location.clone(),
                id,
            )),
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
fn cast_if_necessary(statement: AstNode, target_type_name: &Option<&str>, index: &Index) -> AstNode {
    let Some(dti) = target_type_name.and_then(|it| index.find_effective_type_info(it)) else {
        return statement;
    };

    if let AstStatement::Literal(literal) = statement.get_stmt() {
        let (id, location) = (statement.get_id(), statement.get_location());
        match literal {
            AstLiteral::Integer(value) if dti.is_float() => {
                return AstNode::new_real(value.to_string(), id, location)
            }

            AstLiteral::String(StringValue { value, is_wide: true }) if dti.is_string_utf8() => {
                return AstNode::new_string(value, false, id, location)
            }

            AstLiteral::String(StringValue { value, is_wide: false }) if dti.is_string_utf16() => {
                return AstNode::new_string(value, true, id, location)
            }

            _ => (),
        }
    };

    statement
}

/// Checks if a literal integer or float overflows based on its value, and if so returns true.
fn does_overflow(literal: &AstNode, dti: Option<&DataTypeInformation>) -> bool {
    let Some(dti) = dti else { return false };
    let AstStatement::Literal(kind) = literal.get_stmt() else { return false };

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

        // Only interested in integers. Floats can be infinite or nan which is _technically_ not
        // an overflow.
        _ => false,
    }
}

pub fn evaluate(
    initial: &AstNode,
    scope: Option<&str>,
    index: &Index,
    lhs: Option<&str>,
) -> Result<Option<AstNode>, UnresolvableKind> {
    evaluate_with_target_hint(initial, scope, index, None, lhs)
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
    initial: &AstNode,
    scope: Option<&str>,
    index: &Index,
    target_type: Option<&str>,
    lhs: Option<&str>,
) -> Result<Option<AstNode>, UnresolvableKind> {
    if !needs_evaluation(initial) {
        return Ok(Some(initial.clone())); // TODO hmm ...
    }
    let (id, location) = (initial.get_id(), initial.get_location());
    let literal = match initial.get_stmt() {
        AstStatement::Literal(kind) => match kind {
            AstLiteral::Array(Array { elements: Some(elements) }) => {
                let tt = target_type
                    .and_then(|it| index.find_effective_type_info(it))
                    .and_then(|it| it.get_inner_array_type_name())
                    .or(target_type);

                let inner_elements = AstNode::get_as_list(elements)
                    .iter()
                    .map(|e| evaluate_with_target_hint(e, scope, index, tt, lhs))
                    .collect::<Result<Vec<Option<AstNode>>, UnresolvableKind>>()?
                    .into_iter()
                    .collect::<Option<Vec<AstNode>>>();

                inner_elements.map(|ie| {
                    AstFactory::create_literal(
                        AstLiteral::new_array(Some(Box::new(AstFactory::create_expression_list(
                            ie,
                            location.clone(),
                            id,
                        )))),
                        location.clone(),
                        id,
                    )
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

        AstStatement::DefaultValue(_) => {
            return get_default_initializer(
                initial.get_id(),
                target_type.unwrap_or(VOID_TYPE),
                index,
                &location,
            )
        }
        AstStatement::ReferenceExpr(ReferenceExpr {
            access: ReferenceAccess::Cast(target),
            base: Some(type_name),
        }) => {
            let dti = type_name
                .get_flat_reference_name()
                .and_then(|type_name| index.find_effective_type_info(type_name));
            match dti {
                Some(DataTypeInformation::Enum { name: enum_name, .. }) => {
                    if let AstStatement::Identifier(ref_name) = target.get_stmt() {
                        return index
                            .find_enum_variant(enum_name, ref_name)
                            .ok_or_else(|| {
                                UnresolvableKind::Misc(format!(
                                    "Cannot resolve constant enum {enum_name}#{ref_name}."
                                ))
                            })
                            .and_then(|v| {
                                resolve_const_reference(v, ref_name, index, target_type, scope, lhs)
                            });
                    } else {
                        return Err(UnresolvableKind::Misc("Cannot resolve unknown constant.".to_string()));
                    }
                }
                Some(dti) => {
                    evaluate_with_target_hint(target, scope, index, Some(dti.get_name()), lhs)?;
                    Some(get_cast_statement_literal(target, dti.get_name(), scope, index, lhs)?)
                }
                None => return Err(UnresolvableKind::Misc("Cannot resolve unknown Type-Cast.".to_string())),
            }
        }
        AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Member(reference), base }) => {
            if let Some(name) = reference.get_flat_reference_name() {
                index
                    .find_variable(
                        base.as_ref().and_then(|it| it.get_flat_reference_name()).or(scope),
                        std::slice::from_ref(&name),
                    )
                    .map(|variable| resolve_const_reference(variable, name, index, target_type, scope, lhs))
                    .transpose()?
                    .flatten()
            } else {
                None
            }
        }
        AstStatement::BinaryExpression(BinaryExpression { left, right, operator }) => {
            let eval_left = evaluate(left, scope, index, lhs)?;
            let eval_right = evaluate(right, scope, index, lhs)?;

            if let Some((left, right)) = eval_left.zip(eval_right).as_ref() {
                let evalualted = match operator {
                    Operator::Plus => arithmetic_expression!(left, +, right, "+", id)?,
                    Operator::Minus => arithmetic_expression!(left, -, right, "-", id)?,
                    Operator::Multiplication => arithmetic_expression!(left, *, right, "*", id)?,
                    Operator::Division if !(right.is_real() || left.is_real()) && right.is_zero() => {
                        return Err(UnresolvableKind::Misc("Attempt to divide by zero".to_string()))
                    }
                    Operator::Division => arithmetic_expression!(left, /, right, "/", id)?,
                    Operator::Modulo if right.is_zero() => {
                        return Err(UnresolvableKind::Misc(
                            "Attempt to calculate the remainder with a divisor of zero".to_string(),
                        ))
                    }
                    Operator::Modulo => arithmetic_expression!(left, %, right, "MOD", id)?,
                    Operator::Equal => compare_expression!(left, ==, right, "=", id)?,
                    Operator::NotEqual => compare_expression!(left, !=, right, "<>", id)?,
                    Operator::Greater => compare_expression!(left, >, right, ">", id)?,
                    Operator::GreaterOrEqual => compare_expression!(left, >=, right, ">=", id)?,
                    Operator::Less => compare_expression!(left, <, right, "<", id)?,
                    Operator::LessOrEqual => compare_expression!(left, <=, right, "<=", id)?,
                    Operator::And => bitwise_expression!(left, & , right, "AND", id)?,
                    Operator::Or => bitwise_expression!(left, | , right, "OR", id)?,
                    Operator::Xor => bitwise_expression!(left, ^, right, "XOR", id)?,
                    _ => {
                        return Err(UnresolvableKind::Misc(format!(
                            "Cannot resolve operator {operator:?} in constant evaluation"
                        )))
                    }
                };

                // We have to re-evaluate to detect overflows
                evaluate_with_target_hint(&evalualted, scope, index, target_type, lhs)?
            } else {
                None //not all operators can be resolved
            }
        }

        // NOT x
        AstStatement::UnaryExpression(UnaryExpression { operator: Operator::Not, value }) => {
            let eval = evaluate(value, scope, index, lhs)?;

            match eval.as_ref() {
                Some(AstNode { stmt: AstStatement::Literal(AstLiteral::Bool(v)), id, location, .. }) => {
                    Some(AstFactory::create_literal(AstLiteral::Bool(!v), location.clone(), *id))
                }
                Some(AstNode {
                    stmt: AstStatement::Literal(AstLiteral::Integer(v)), id, location, ..
                }) => {
                    evaluate_with_target_hint(eval.as_ref().unwrap(), scope, index, target_type, lhs)?;
                    Some(AstFactory::create_literal(AstLiteral::Integer(!v), location.clone(), *id))
                }
                None => {
                    None //not yet resolvable
                }
                _ => return Err(UnresolvableKind::Misc(format!("Cannot resolve constant Not {value:?}"))),
            }
        }
        // - x
        AstStatement::UnaryExpression(UnaryExpression { operator: Operator::Minus, value }) => {
            match evaluate(value, scope, index, lhs)? {
                Some(AstNode {
                    stmt: AstStatement::Literal(AstLiteral::Integer(v)), id, location, ..
                }) => Some(AstNode::new(AstStatement::Literal(AstLiteral::Integer(-v)), id, location)),
                Some(AstNode { stmt: AstStatement::Literal(AstLiteral::Real(v)), id, location, .. }) => {
                    let lit = AstNode::new(
                        AstStatement::Literal(AstLiteral::new_real(format!(
                            "{:}",
                            -(v.parse::<f64>())
                                .map_err(|err| UnresolvableKind::Misc(format!("{err:}: {v:}")))?
                        ))),
                        id,
                        location,
                    );
                    evaluate_with_target_hint(&lit, scope, index, target_type, lhs)?
                }
                None => {
                    None //not yet resolvable
                }
                _ => return Err(UnresolvableKind::Misc(format!("Cannot resolve constant Minus {value:?}"))),
            }
        }
        AstStatement::UnaryExpression(UnaryExpression { operator: Operator::Plus, value }) => {
            evaluate(value, scope, index, lhs)?
        }
        AstStatement::ExpressionList(expressions) => {
            let inner_elements = expressions
                .iter()
                .map(|e| evaluate(e, scope, index, lhs))
                .collect::<Result<Vec<Option<AstNode>>, UnresolvableKind>>()?
                .into_iter()
                .collect::<Option<Vec<AstNode>>>();

            //return a new array, or return none if one was not resolvable
            inner_elements.map(|ie| AstNode::new(AstStatement::ExpressionList(ie), id, location))
        }
        AstStatement::MultipliedStatement(MultipliedStatement { element, multiplier }) => {
            let inner_elements = AstNode::get_as_list(element.as_ref())
                .iter()
                .map(|e| evaluate(e, scope, index, lhs))
                .collect::<Result<Vec<Option<AstNode>>, UnresolvableKind>>()?
                .into_iter()
                .collect::<Option<Vec<AstNode>>>();

            //return a new array, or return none if one was not resolvable
            inner_elements.map(|ie| {
                if let [ie] = ie.as_slice() {
                    AstFactory::create_multiplied_statement(*multiplier, ie.clone(), location.clone(), id)
                } else {
                    AstFactory::create_multiplied_statement(
                        *multiplier,
                        AstFactory::create_expression_list(ie, location.clone(), id),
                        location.clone(),
                        id,
                    )
                }
            })
        }
        AstStatement::Assignment(data) => {
            //Right needs evaluation
            match evaluate(&data.right, scope, index, lhs) {
                Ok(Some(value)) => Ok(Some(AstFactory::create_assignment(*data.left.clone(), value, id))),
                Ok(None) => Ok(Some(initial.clone())),
                Err(UnresolvableKind::Address(mut init)) => {
                    init.initializer.replace(initial.clone());
                    Err(UnresolvableKind::Address(init))
                }
                Err(why) => Err(why),
            }?
        }
        AstStatement::RangeStatement(data) => {
            let start = evaluate(&data.start, scope, index, lhs)?.unwrap_or_else(|| *data.start.to_owned());
            let end = evaluate(&data.end, scope, index, lhs)?.unwrap_or_else(|| *data.end.to_owned());

            Some(AstFactory::create_range_statement(start, end, id))
        }
        AstStatement::ParenExpression(expr) => {
            match evaluate_with_target_hint(expr, scope, index, target_type, lhs) {
                Ok(init) => Ok(init),
                Err(UnresolvableKind::Address(mut init)) => {
                    init.lhs = lhs.map(str::to_string);
                    init.initializer.replace(initial.clone());
                    Err(UnresolvableKind::Address(init))
                }
                Err(why) => Err(why),
            }?
        }
        AstStatement::CallStatement(plc_ast::ast::CallStatement { operator, .. }) => {
            if let Some(pou) = operator.as_ref().get_flat_reference_name().and_then(|it| index.find_pou(it)) {
                if !(pou.is_constant() && index.get_builtin_function(pou.get_name()).is_some()) {
                    return Err(UnresolvableKind::Misc(format!(
                        "Call-statement '{}' in initializer is not constant.",
                        pou.get_name()
                    )));
                }
            } else {
                // POU not found
                return Err(UnresolvableKind::Misc(format!("Cannot resolve constant: {:#?}", initial)));
            };

            return Err(UnresolvableKind::Address(InitData::new(Some(initial), target_type, scope, lhs)));
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
    target_type: Option<&str>,
    scope: Option<&str>,
    lhs: Option<&str>,
) -> Result<Option<AstNode>, UnresolvableKind> {
    if !variable.is_constant() {
        if !target_type
            .is_some_and(|it| index.find_effective_type_by_name(it).is_some_and(|it| it.is_pointer()))
        {
            return Err(UnresolvableKind::Misc(format!("`{name}` is no const reference")));
        } else {
            return Err(UnresolvableKind::Address(InitData::new(None, target_type, scope, lhs)));
        }
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
    cast_statement: &AstNode,
    type_name: &str,
    scope: Option<&str>,
    index: &Index,
    lhs: Option<&str>,
) -> Result<AstNode, UnresolvableKind> {
    let dti = index.find_effective_type_info(type_name);
    match dti {
        Some(&DataTypeInformation::Integer { .. }) => {
            let evaluated_initial =
                evaluate_with_target_hint(cast_statement, scope, index, Some(type_name), lhs)?
                    .as_ref()
                    .map(|v| {
                        if let AstStatement::Literal(AstLiteral::Integer(value)) = v.get_stmt() {
                            Ok(*value)
                        } else {
                            Err(UnresolvableKind::Misc(format!("Expected integer value, found {v:?}")))
                        }
                    })
                    .transpose()?;

            if let Some(value) = evaluated_initial {
                return Ok(AstNode::new(
                    AstStatement::Literal(AstLiteral::new_integer(value)),
                    cast_statement.get_id(),
                    cast_statement.get_location(),
                ));
            }

            Err(UnresolvableKind::Misc(format!("Cannot resolve constant: {type_name}#{cast_statement:?}")))
        }

        Some(DataTypeInformation::Float { .. }) => {
            let evaluated = evaluate(cast_statement, scope, index, lhs)?;
            let value = match evaluated.as_ref().map(|it| it.get_stmt()) {
                Some(AstStatement::Literal(AstLiteral::Integer(value))) => Some(*value as f64),
                Some(AstStatement::Literal(AstLiteral::Real(value))) => value.parse::<f64>().ok(),
                _ => {
                    return Err(UnresolvableKind::Misc(format!(
                        "Expected floating point type, got: {evaluated:?}"
                    )))
                }
            };

            let Some(value) = value else {
                return Err(UnresolvableKind::Misc(format!(
                    "cannot resolve constant: {type_name}#{cast_statement:?}"
                )));
            };

            Ok(AstNode::new(
                AstStatement::Literal(AstLiteral::new_real(value.to_string())),
                cast_statement.get_id(),
                cast_statement.get_location(),
            ))
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
        {
        let loc_left = $left.get_location();
        let loc_right = $right.get_location();
        match ($left.get_stmt(), $right.get_stmt()) {
            (   AstStatement::Literal(AstLiteral::Integer(lvalue)),
                AstStatement::Literal(AstLiteral::Integer(rvalue))) => {
                    Ok(AstStatement::Literal(AstLiteral::new_integer(lvalue $op rvalue)))
            },

            (   AstStatement::Literal(AstLiteral::Integer(lvalue)),
                AstStatement::Literal(AstLiteral::Real(rvalue))) => {
                    let rvalue = rvalue.parse::<f64>()
                        .map_err(|err| UnresolvableKind::Misc(err.to_string()))?;
                    Ok(AstStatement::Literal(
                        AstLiteral::new_real((*lvalue as f64 $op rvalue).to_string())))
            },

            (   AstStatement::Literal(AstLiteral::Real(lvalue)),
                AstStatement::Literal(AstLiteral::Integer(rvalue))) => {
                    let lvalue = lvalue.parse::<f64>()
                        .map_err(|err| UnresolvableKind::Misc(err.to_string()))?;
                    Ok(AstStatement::Literal(AstLiteral::new_real((lvalue $op *rvalue as f64).to_string())))
            },

            (   AstStatement::Literal(AstLiteral::Real(lvalue)),
                AstStatement::Literal(AstLiteral::Real(rvalue))) => {
                    let lvalue = lvalue.parse::<f64>()
                        .map_err(|err| UnresolvableKind::Misc(err.to_string()))?;
                    let rvalue = rvalue.parse::<f64>()
                        .map_err(|err| UnresolvableKind::Misc(err.to_string()))?;
                    Ok(AstStatement::Literal(
                        AstLiteral::new_real((lvalue $op rvalue).to_string()),
                    ))
            },
            _ => cannot_eval_error!($left, $op_text, $right),
        }.map(|it| AstNode::new(it, $resulting_id, loc_left.span(&loc_right)))
        }
    }
}
use arithmetic_expression;

macro_rules! bitwise_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr, $resulting_id:expr) => {{
        let loc_left = $left.get_location();
        let loc_right = $right.get_location();
        match ($left.get_stmt(), $right.get_stmt()) {
            (   AstStatement::Literal(AstLiteral::Integer(lvalue)),
                AstStatement::Literal(AstLiteral::Integer(rvalue))) => {
                Ok(AstStatement::Literal(AstLiteral::new_integer(lvalue $op rvalue)))
            },

            (   AstStatement::Literal(AstLiteral::Bool(lvalue)),
                AstStatement::Literal(AstLiteral::Bool(rvalue))) => {
                Ok(AstStatement::Literal(AstLiteral::new_bool(lvalue $op rvalue)))
            },
            _ => cannot_eval_error!($left, $op_text, $right),
        }.map(|it| AstNode::new(it, $resulting_id, loc_left.span(&loc_right)))
    }
}}
use bitwise_expression;

macro_rules! compare_expression {
    ($left:expr, $op:tt, $right:expr, $op_text:expr, $resulting_id:expr) => {{
        let loc_left = $left.get_location();
        let loc_right = $right.get_location();
        match ($left.get_stmt(), $right.get_stmt()) {
            (   AstStatement::Literal(AstLiteral::Integer(lvalue)),
                AstStatement::Literal(AstLiteral::Integer(rvalue))) => {
                Ok(AstStatement::Literal(
                    AstLiteral::new_bool(lvalue $op rvalue)))
            },
            (   AstStatement::Literal(AstLiteral::Real(..)),
                AstStatement::Literal(AstLiteral::Real(..))) => {
                    Err(UnresolvableKind::Misc("Cannot compare Reals without epsilon".into()))
            },
            (   AstStatement::Literal(AstLiteral::Bool(lvalue)),
                AstStatement::Literal(AstLiteral::Bool(rvalue))) => {
                    Ok(AstStatement::Literal(AstLiteral::new_bool(lvalue $op rvalue)))
            },
            _ => cannot_eval_error!($left, $op_text, $right),
        }.map(|it| AstNode::new(it, $resulting_id, loc_left.span(&loc_right)))
    }}
}
use compare_expression;
