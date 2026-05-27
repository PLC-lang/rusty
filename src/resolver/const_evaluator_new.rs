use crate::index::VariableIndexEntry;
use crate::resolver::StatementAnnotation;
use crate::{
    index::{
        const_expressions::{ConstId, UnresolvableKind},
        Index,
    },
    resolver::{AnnotationMap, AnnotationMapImpl},
    typesystem::DataTypeInformation,
};
use plc_ast::ast::CallStatement;
use plc_ast::literals::Array;
use plc_ast::{
    ast::{
        Assignment, AstFactory, AstNode, AstStatement, BinaryExpression, Operator, ReferenceAccess,
        ReferenceExpr, UnaryExpression,
    },
    literals::{AstLiteral, StringValue},
};
use plc_source::source_location::SourceLocation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//States for detecting cycles and other
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstEvalState {
    Unresolved,
    Resolving,
    Resolved,
    NotResolvable,
}

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

    pub fn incomplete_initialization(id: &ConstId) -> Self {
        UnresolvableConstant::new(*id, "Incomplete initialization - cannot evaluate const expressions")
    }

    pub fn no_initial_value(id: &ConstId) -> Self {
        UnresolvableConstant::new(*id, "No initial value")
    }

    pub fn get_reason(&self) -> Option<&str> {
        self.kind.as_ref().map(|it| it.get_reason())
    }
}

// Internal value representation for constant evaluation.
// We evaluate on typed runtime values (Int/Real/Bool) and only convert
// to/from AST literals at the evaluator boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListKind {
    /// AST expression for struct inits `(a := 10, b := 10)`.
    ExpressionList,
    /// AST array literal `[a, b, c]`.
    ArrayLiteral,
}

#[derive(Debug, Clone)]
struct AssignmentValue {
    left: AstNode,
    right: Box<ConstValue>,
    assign_id: usize,
    right_id: usize,
    right_location: SourceLocation,
}

impl AssignmentValue {
    fn new(data: &Assignment, right_value: ConstValue, assign_id: usize) -> Self {
        Self {
            left: data.left.as_ref().clone(),
            right: Box::new(right_value),
            assign_id,
            right_id: data.right.get_id(),
            right_location: data.right.get_location(),
        }
    }
}

#[derive(Debug, Clone)]
enum ConstValue {
    Int(i128),
    Real(f64),
    Bool(bool),
    Str(String, bool),
    Null,
    Assignment(AssignmentValue),
    /// Repetition (e.g., `5(10)` for five tens).
    Multiplied(Box<ConstValue>, usize, usize),
    /// Grouped values. `kind` determines AST reconstruction (Struct vs. Array).
    List {
        elements: Vec<(ConstValue, usize)>,
        kind: ListKind,
        /// Original Node ID (ArrayLiterals only).
        list_id: Option<usize>,
    },
}

impl ConstValue {
    /// Convert an AST-Node to the internal used ConstValue for evaluating
    fn from_ast(node: &AstNode) -> Result<Self, UnresolvableKind> {
        match node.get_stmt() {
            AstStatement::Literal(AstLiteral::Integer(v)) => Ok(Self::Int(*v)),
            AstStatement::Literal(AstLiteral::Real(v)) => {
                let parsed = v
                    .parse::<f64>()
                    .map_err(|e| UnresolvableKind::Misc(format!("Cannot parse real literal `{v}`: {e}")))?;
                Ok(Self::Real(parsed))
            }
            AstStatement::Literal(AstLiteral::Bool(v)) => Ok(Self::Bool(*v)),
            AstStatement::Literal(AstLiteral::String(StringValue { value, is_wide })) => {
                Ok(Self::Str(value.clone(), *is_wide))
            }
            AstStatement::Literal(AstLiteral::Null) => Ok(Self::Null),
            // CASE: Extracting values from an array literal [x, y, z].
            AstStatement::Literal(AstLiteral::Array(Array { elements })) => {
                let values = Self::collect_from_nodes(elements.as_deref())?;
                // Keep `list_id` empty so `into_ast_node` reuses the array literal's id for the inner ExpressionList.
                // This matches the previous behavior and keeps downstream annotation/type-hint lookups stable.
                Ok(Self::List { elements: values, kind: ListKind::ArrayLiteral, list_id: None })
            }
            // CASE: Unpacking struct initializers like (x := 10, y := 20).
            AstStatement::ExpressionList(_expressions) => {
                let values = Self::collect_from_nodes(Some(node))?;
                Ok(Self::List { elements: values, kind: ListKind::ExpressionList, list_id: None })
            }
            AstStatement::Assignment(data) => {
                let right = Self::from_ast(data.right.as_ref())?;
                Ok(Self::Assignment(AssignmentValue::new(data, right, node.get_id())))
            }
            _ => Err(UnresolvableKind::Misc(format!(
                "Cannot convert AST node to const value: {:?}",
                node.get_stmt()
            ))),
        }
    }

    /// Collects ConstValues from a list of AST nodes, preserving IDs.
    fn collect_from_nodes(nodes: Option<&AstNode>) -> Result<Vec<(Self, usize)>, UnresolvableKind> {
        let Some(node) = nodes else {
            return Ok(Vec::new());
        };
        match node.get_stmt() {
            AstStatement::ExpressionList(expressions) => {
                expressions.iter().map(|n| Self::from_ast(n).map(|cv| (cv, n.get_id()))).collect()
            }
            _ => Ok(vec![(Self::from_ast(node)?, node.get_id())]),
        }
    }

    /// Converts the resolved Constants back in an AST-Node
    fn into_ast_node(self, id: usize, location: SourceLocation, dti: DataTypeInformation) -> AstNode {
        match self {
            Self::Int(v) => AstNode::new_literal(AstLiteral::new_integer(v), id, location),
            Self::Real(v) => AstNode::new_real(v.to_string(), id, location),
            Self::Null => AstNode::new_literal(AstLiteral::new_null(), id, location),
            Self::Bool(v) => AstNode::new_literal(AstLiteral::new_bool(v), id, location),
            Self::Str(v, is_wide) => AstNode::new_string(v, is_wide, id, location),
            Self::List { elements, kind, list_id } => {
                let inner_nodes: Vec<AstNode> = elements
                    .into_iter()
                    .map(|(val, child_id)| val.into_ast_node(child_id, location.clone(), dti.clone()))
                    .collect();

                match kind {
                    ListKind::ExpressionList => {
                        let expr_id = list_id.unwrap_or(id);
                        AstNode::new(AstStatement::ExpressionList(inner_nodes), expr_id, location)
                    }
                    ListKind::ArrayLiteral => {
                        // Always wrap array elements in an ExpressionList (even for a single element).
                        let list_id = list_id.unwrap_or(id);
                        let list_node = AstNode::new(
                            AstStatement::ExpressionList(inner_nodes),
                            list_id,
                            location.clone(),
                        );
                        AstNode::new_literal(AstLiteral::new_array(Some(Box::new(list_node))), id, location)
                    }
                }
            }
            Self::Multiplied(element, multiplier, element_id) => {
                let element_node = element.into_ast_node(element_id, location.clone(), dti);
                AstFactory::create_multiplied_statement(multiplier as u32, element_node, location, id)
            }
            Self::Assignment(assign) => {
                let right_node = assign.right.into_ast_node(assign.right_id, assign.right_location, dti);
                AstFactory::create_assignment(assign.left, right_node, assign.assign_id)
            }
        }
    }

    pub fn get_datatype(&self) -> &'static str {
        match self {
            Self::Int(_) => "INT",
            Self::Real(_) => "REAL",
            Self::Bool(_) => "BOOL",
            Self::Str(_, _) => "STRING",
            Self::Null => "NULL",
            Self::Assignment(_) => "STRUCT",
            Self::List { kind: ListKind::ExpressionList, .. } => "STRUCT",
            Self::List { kind: ListKind::ArrayLiteral, .. } => "ARRAY",
            Self::Multiplied(_, _, _) => "ARRAY",
        }
    }
}

/// Orchestrates the iterative evaluation of all registered constant expressions.
/// This function attempts to resolve constant values by traversing the AST,
/// detecting potential cycles via the `ConstEvalState`, and tracking any
/// expressions that fail to resolve due to dependencies or invalid syntax.
pub fn evaluate_constants_new(
    mut index: Index,
    annotations: &mut AnnotationMapImpl,
) -> (Index, Vec<UnresolvableConstant>) {
    let constants: Vec<ConstId> = index.get_const_expressions().into_iter().map(|(id, _)| id).collect();
    let mut unresolved_kinds = Vec::new();

    let (resolved_constants, unresolvable_constants) = {
        let mut evaluator = ConstEvaluator::new(&index, annotations);
        let mut unresolvable_constants = Vec::new();

        for candidate in constants {
            if evaluator.state_of(candidate) == ConstEvalState::Resolved {
                continue;
            }

            if let Err(kind) = evaluator.evaluate(candidate) {
                unresolvable_constants
                    .push(UnresolvableConstant::new(candidate, kind.get_reason()).with_kind(kind.clone()));
                unresolved_kinds.push((candidate, kind));
            }
        }

        (evaluator.into_resolved_constants(), unresolvable_constants)
    };

    // write the resolved & unresolved constants back to the index!
    persist_constants(&mut index, resolved_constants, unresolved_kinds);

    (index, unresolvable_constants)
}

/// This is the entry point for evaluation switch case statements!
pub fn evaluate_expression(
    initial: &AstNode,
    scope: Option<&str>,
    index: &Index,
) -> Result<Option<AstNode>, UnresolvableKind> {
    let mut annotations = AnnotationMapImpl::default();
    let mut evaluator = ConstEvaluator::new(index, &mut annotations);

    let value = evaluator.traverse_ast(initial, scope)?;
    let target_type = value.get_datatype();
    let dti = index
        .find_effective_type_info(target_type)
        .cloned()
        .ok_or_else(|| UnresolvableKind::Misc(format!("Type info for {target_type} not found")))?;

    Ok(Some(value.into_ast_node(initial.get_id(), initial.get_location(), dti)))
}

fn persist_constants(index: &mut Index,resolved_constants: HashMap<ConstId, AstNode>,unresolved_kinds: Vec<(ConstId, UnresolvableKind)>,
) {
    for (const_id, resolved_constant) in resolved_constants {
        let _ = index.get_mut_const_expressions().mark_resolved(&const_id, resolved_constant);
    }
    for (const_id, kind) in unresolved_kinds {
        index.get_mut_const_expressions().mark_unresolvable(&const_id, kind).expect("TODO: panic message");
    }
}

struct ConstEvaluator<'a> {
    index: &'a Index,
    annotations: &'a mut AnnotationMapImpl,
    states: HashMap<ConstId, ConstEvalState>,
    current_const_target_type: Option<String>,
    const_cache: HashMap<ConstId, ConstValue>,
    resolved_constants: HashMap<ConstId, AstNode>,
    call_statement_annotations: HashMap<usize, String>,
}

impl<'a> ConstEvaluator<'a> {
    fn new(index: &'a Index, annotations: &'a mut AnnotationMapImpl) -> Self {
        Self {
            index,
            annotations,
            states: HashMap::new(),
            current_const_target_type: None,
            const_cache: HashMap::new(),
            resolved_constants: HashMap::new(),
            call_statement_annotations: HashMap::new(),
        }
    }

    fn state_of(&self, id: ConstId) -> ConstEvalState {
        *self.states.get(&id).unwrap_or(&ConstEvalState::Unresolved)
    }

    fn set_state(&mut self, id: ConstId, state: ConstEvalState) {
        self.states.insert(id, state);
    }

    fn into_resolved_constants(self) -> HashMap<ConstId, AstNode> {
        self.resolved_constants
    }

    /// Applies all collected CallStatement annotations to the annotation map.
    /// This ensures that resolved call expressions at any nesting level are properly annotated.
    fn apply_call_statement_annotations(&mut self) {
        for (node_id, resulting_type) in self.call_statement_annotations.drain() {
            self.annotations.annotate_with_id(node_id, StatementAnnotation::Value { resulting_type });
        }
    }

    fn evaluate(&mut self, const_id: ConstId) -> Result<AstNode, UnresolvableKind> {
        match self.state_of(const_id) {
            ConstEvalState::Resolved => {
                return self.resolved_constants.get(&const_id).cloned().ok_or_else(|| {
                    UnresolvableKind::Misc(format!("Const {const_id:?} is marked resolved but has no value"))
                });
            }
            ConstEvalState::Resolving => {
                return Err(UnresolvableKind::Misc(format!(
                    "Cycle detected while evaluating const {const_id:?}"
                )));
            }
            ConstEvalState::NotResolvable => {
                return Err(UnresolvableKind::Misc(format!("Const {const_id:?} is not resolvable")));
            }
            ConstEvalState::Unresolved => {}
        }

        // Retrieve the constant's base data (AST, type name, scope) from the index
        let Some((ast, target_type, scope, _lhs)) = self.index.get_const_expressions().clone(&const_id)
        else {
            self.set_state(const_id, ConstEvalState::NotResolvable);
            return Err(UnresolvableKind::Misc(format!("Unknown const id: {const_id:?}")));
        };

        // resolve the DataTypeInformation for the target type
        let Some(dti) = self.index.find_effective_type_info(&target_type).cloned() else {
            self.set_state(const_id, ConstEvalState::NotResolvable);
            return Err(UnresolvableKind::Misc(format!("Type info for {target_type} not found")));
        };

        // we skip structs and arrays with default-initializers since they cannot be used inside expressions of other consts.
        // we leave generating the default value to the llvm-index later.
        // And we resolve it so we don't get a validation problem
        if ast.is_default_value() {
            if dti.is_struct() || dti.is_array() {
                let literal = ast.clone();
                self.resolved_constants.insert(const_id, literal.clone());
                self.set_state(const_id, ConstEvalState::Resolved);
                return Ok(literal);
            }
        }

        // mark as currently resolving to detect cycles
        self.set_state(const_id, ConstEvalState::Resolving);
        let previous_target_type = self.current_const_target_type.replace(target_type.clone());

        // we traverse the ast and evaluate it bottom-up. If this returns an error,
        // we mark the constant as not resolvable and return the error reason
        let value = match self.traverse_ast(&ast, scope.as_deref()) {
            Ok(value) => value,
            Err(err) => {
                self.current_const_target_type = previous_target_type;
                self.set_state(const_id, ConstEvalState::NotResolvable);
                return Err(err);
            }
        };

        // after we got the value, we check if we need to cast it to the target type and do so if needed
        let value = match self.cast_to_target_type(value, &target_type, ast.get_id()) {
            Ok(value) => value,
            Err(err) => {
                self.current_const_target_type = previous_target_type;
                self.set_state(const_id, ConstEvalState::NotResolvable);
                return Err(err);
            }
        };

        // put the const in the cache for further use
        self.const_cache.insert(const_id, value.clone());
        // keep resolved constants local; the caller can persist them to the index afterward
        let resolved_constant = value.into_ast_node(ast.get_id(), ast.get_location(), dti.clone());
        self.resolved_constants.insert(const_id, resolved_constant.clone());

        // Apply all collected call statement annotations
        self.apply_call_statement_annotations();

        // mark as resolved
        self.current_const_target_type = previous_target_type;
        self.set_state(const_id, ConstEvalState::Resolved);

        Ok(resolved_constant)
    }

    //TODO INDEX zugriff bei Array auf multiplied
    //TODO cast bei index zugriff? geht das
    /// This is the recursive heart of the constant evaluator. It performs a bottom-up
    /// traversal of the expression tree, transforming AST nodes into `ConstValue`
    /// types. Unsupported constructs are explicitly rejected to ensure predictable
    /// behavior during constant folding.
    fn traverse_ast(&mut self, node: &AstNode, scope: Option<&str>) -> Result<ConstValue, UnresolvableKind> {
        let evaluated = match node.get_stmt() {
            AstStatement::Literal(AstLiteral::Array(Array { elements: Some(elements) })) => {
                self.eval_array_literal(elements, scope)
            }
            AstStatement::Literal(_) => ConstValue::from_ast(node),
            AstStatement::ParenExpression(inner) => self.traverse_ast(inner, scope),
            AstStatement::UnaryExpression(UnaryExpression { operator, value }) => {
                let child = self.traverse_ast(value, scope)?;
                self.eval_unary_expression(*operator, child, node)
            }
            AstStatement::BinaryExpression(BinaryExpression { left, right, operator }) => {
                let left = self.traverse_ast(left, scope)?;
                let right = self.traverse_ast(right, scope)?;
                self.eval_binary_expression(*operator, left, right, node)
            }
            AstStatement::ReferenceExpr(ReferenceExpr { access, base }) => {
                match access {
                    ReferenceAccess::Member(reference) => {
                        if let Some(field_name) = reference.get_flat_reference_name() {
                            let qualifier = base.as_ref().and_then(|it| it.get_flat_reference_name());

                            // Struct field access can be nested, e.g. `outer.point.x`, so the base is
                            // not always a flat variable name. Use the annotated/effective base type.
                            if let Some(base_node) = base.as_deref() {
                                if let Some(base_dti) =
                                    self.effective_type_info_for_field_base(base_node, scope)
                                {
                                    if base_dti.is_struct() {
                                        return self.eval_struct_field_access(
                                            base_node, field_name, scope, &base_dti,
                                        );
                                    }
                                }
                            }
                            // Not a struct field access, treat as variable reference with qualifier
                            self.eval_variable_reference(field_name, qualifier, scope)
                        } else {
                            Err(UnresolvableKind::Misc("Unsupported member reference".to_string()))
                        }
                    }
                    ReferenceAccess::Cast(target_type) => {
                        self.eval_cast_reference(target_type, base.as_deref(), scope, node)
                    }
                    ReferenceAccess::Index(reference) => self.eval_index_reference(base, reference, scope),
                    _ => Err(UnresolvableKind::Misc("Unsupported ReferenceAccess".to_string())),
                }
            }
            AstStatement::ExpressionList(expressions) => {
                self.eval_expression_list(expressions, scope, node.get_id())
            }
            AstStatement::CallStatement(call) => {
                let result = self.eval_call_statement(call, scope)?;
                // Track this resolved call statement for annotation
                self.call_statement_annotations.insert(node.get_id(), result.get_datatype().to_string());
                Ok(result)
            }
            AstStatement::MultipliedStatement(data) => {
                // just resolve the element and write it back as a multiplier AST-Node
                let element_value = self.traverse_ast(&data.element, scope)?;
                Ok(ConstValue::Multiplied(
                    Box::new(element_value),
                    data.multiplier as usize,
                    data.element.get_id(),
                ))
            }
            AstStatement::RangeStatement(_) => {
                Err(UnresolvableKind::Misc("Range statements are not supported in this Context!".to_string()))
            }
            AstStatement::Assignment(data) => self.eval_assignment(data, scope, node),
            AstStatement::DefaultValue(_) => self.eval_default_value(node, scope),
            _ => Err(UnresolvableKind::Misc(format!(
                "Unsupported AST method or statement type: {:?}",
                node.get_stmt()
            ))),
        }?;
        Ok(evaluated)
    }

    /// Evaluates an array literal from the AST.
    /// This function distinguishes between flat array lists and parenthesized
    /// expressions, which are typically used for struct or tuple initializers
    /// within an array in IEC 61131-3.
    fn eval_array_literal(
        &mut self,
        elements: &AstNode,
        scope: Option<&str>,
    ) -> Result<ConstValue, UnresolvableKind> {
        let evaluated = self.traverse_ast(elements, scope)?;

        let array_elements = match (elements.get_stmt(), evaluated) {
            // Hits for `[(x := 10, y := 20)]`: keep the struct initializer as one array element.
            (
                AstStatement::ParenExpression(inner),
                ConstValue::List { elements: values, kind: ListKind::ExpressionList, list_id },
            ) => {
                let inner_id = inner.get_id();
                let element_id = list_id.unwrap_or(inner_id);
                vec![(
                    ConstValue::List { elements: values, kind: ListKind::ExpressionList, list_id },
                    element_id,
                )]
            }
            // Hits for `[(x := 10)]`: wrap the single assignment back into a struct initializer.
            (AstStatement::ParenExpression(inner), ConstValue::Assignment(assign)) => {
                let inner_id = inner.get_id();
                let assign_id = assign.assign_id;
                vec![(
                    ConstValue::List {
                        elements: vec![(ConstValue::Assignment(assign), assign_id)],
                        kind: ListKind::ExpressionList,
                        list_id: Some(inner_id),
                    },
                    inner_id,
                )]
            }
            // Hits for `[(42)]`: keep the parenthesized scalar as one array element.
            (AstStatement::ParenExpression(inner), scalar) => vec![(scalar, inner.get_id())],
            // Hits for `[1, 2]`: use the expression list entries as the array elements.
            (.., ConstValue::List { elements: values, kind: ListKind::ExpressionList, .. }) => values,
            // Hits for `[1]`: turn the single non-list value into a one-element array.
            (.., scalar) => vec![(scalar, elements.get_id())],
        };

        Ok(ConstValue::List { elements: array_elements, kind: ListKind::ArrayLiteral, list_id: None })
    }

    /// Evaluates an array index access expression by first resolving the base value and index expression.
    /// e.g. myVar : INT := myArray[4];
    fn eval_index_reference(
        &mut self,
        base: &Option<Box<AstNode>>,
        index_node: &AstNode,
        scope: Option<&str>,
    ) -> Result<ConstValue, UnresolvableKind> {
        let base_node = base
            .as_ref()
            .ok_or_else(|| UnresolvableKind::Misc("Index access without a base expression".into()))?;

        let target_array = self.traverse_ast(base_node, scope)?;
        let index = self.traverse_ast(index_node, scope)?;

        let index_val = match index {
            ConstValue::Int(v) => usize::try_from(v).map_err(|_| {
                UnresolvableKind::Misc(format!(
                    "Invalid array index: {v} (must be positive and within bounds)"
                ))
            })?,
            _ => {
                return Err(UnresolvableKind::Misc(format!(
                    "Array index must be an integer, but got {}",
                    index.get_datatype()
                )))
            }
        };

        match target_array {
            ConstValue::List { elements, .. } => {
                let (element, _) = elements.get(index_val).ok_or_else(|| {
                    UnresolvableKind::Misc(format!(
                        "Array index {index_val} out of bounds (array contains {} number of initialized elements)",
                        elements.len()
                    ))
                })?;

                Ok(element.clone())
            }
            _ => Err(UnresolvableKind::Misc(format!(
                "Cannot use index access '[]' on non-array type {}",
                target_array.get_datatype()
            ))),
        }
    }

    fn eval_call_statement(
        &mut self,
        call: &CallStatement,
        scope: Option<&str>,
    ) -> Result<ConstValue, UnresolvableKind> {
        let func_name = Self::extract_call_statement_function_name(call)
            .ok_or_else(|| UnresolvableKind::Misc("Invalid function call".to_string()))?;

        let mut args = Vec::new();
        if let Some(params) = &call.parameters {
            match self.traverse_ast(params, scope)? {
                ConstValue::List { elements: values, kind: ListKind::ExpressionList, .. } => {
                    args.extend(values.into_iter().map(|(v, _)| v))
                }
                value => args.push(value),
            }
        }

        match func_name.as_str() {
            //Arithmetic and Trigonometric Functions
            "SQRT" => {
                if args.len() != 1 {
                    return Err(UnresolvableKind::Misc(format!(
                        "SQRT expects exactly 1 argument, but got {}",
                        args.len()
                    )));
                }

                let val = &args[0];

                match val {
                    ConstValue::Real(v) => {
                        if *v < 0.0 {
                            Err(UnresolvableKind::Misc(format!(
                                "SQRT argument must not be negative (got {})",
                                v
                            )))
                        } else {
                            Ok(ConstValue::Real(v.sqrt()))
                        }
                    }
                    _ => Err(UnresolvableKind::Misc(format!(
                        "SQRT expects a REAL value, but got {}",
                        val.get_datatype()
                    ))),
                }
            }
            "ABS" => {
                if args.len() != 1 {
                    return Err(UnresolvableKind::Misc(format!(
                        "ABS expects exactly 1 argument, but got {}",
                        args.len()
                    )));
                }

                let val = &args[0];
                match val {
                    ConstValue::Real(v) => Ok(ConstValue::Real(v.abs())),
                    ConstValue::Int(v) => Ok(ConstValue::Int(v.abs())),
                    _ => Err(UnresolvableKind::Misc(format!(
                        "ABS expects a REAL or INT value, but got {}",
                        val.get_datatype()
                    ))),
                }
            }
            "ADD" => {
                if args.is_empty() {
                    return Err(UnresolvableKind::Misc("ADD expects at least 1 argument".to_string()));
                }

                let has_real = args.iter().any(|arg| matches!(arg, ConstValue::Real(_)));

                if has_real {
                    let mut sum: f64 = 0.0;
                    for arg in args {
                        match arg {
                            ConstValue::Real(v) => sum += v,
                            ConstValue::Int(v) => sum += v as f64,
                            _ => return Err(UnresolvableKind::Misc("ADD: Wrong type".into())),
                        }
                    }
                    Ok(ConstValue::Real(sum))
                } else {
                    let mut sum: i128 = 0;
                    for arg in args {
                        match arg {
                            ConstValue::Int(v) => sum += v,
                            _ => return Err(UnresolvableKind::Misc("ADD: Only INT and REAL allowed".into())),
                        }
                    }
                    Ok(ConstValue::Int(sum))
                }
            }
            "MUL" => {
                if args.is_empty() {
                    return Err(UnresolvableKind::Misc("MUL expects at least 1 argument".to_string()));
                }

                let has_real = args.iter().any(|arg| matches!(arg, ConstValue::Real(_)));
                if has_real {
                    let mut product: f64 = 1.0;
                    for arg in args {
                        match arg {
                            ConstValue::Real(v) => product *= v,
                            ConstValue::Int(v) => product *= v as f64,
                            _ => return Err(UnresolvableKind::Misc("MUL: Wrong type".into())),
                        }
                    }
                    Ok(ConstValue::Real(product))
                } else {
                    let mut product: i128 = 1;
                    for arg in args {
                        match arg {
                            ConstValue::Int(v) => product *= v,
                            _ => return Err(UnresolvableKind::Misc("MUL: Only INT and REAL allowed".into())),
                        }
                    }
                    Ok(ConstValue::Int(product))
                }
            }
            "SUB" => {
                if args.len() != 2 {
                    return Err(UnresolvableKind::Misc("SUB expects exactly 2 arguments".to_string()));
                }

                let has_real = args.iter().any(|arg| matches!(arg, ConstValue::Real(_)));
                let (left, right) = (&args[0], &args[1]);

                if has_real {
                    let l_val = match left {
                        ConstValue::Real(v) => *v,
                        ConstValue::Int(v) => *v as f64,
                        _ => return Err(UnresolvableKind::Misc("SUB: Wrong type".into())),
                    };
                    let r_val = match right {
                        ConstValue::Real(v) => *v,
                        ConstValue::Int(v) => *v as f64,
                        _ => return Err(UnresolvableKind::Misc("SUB: Wrong type".into())),
                    };
                    Ok(ConstValue::Real(l_val - r_val))
                } else {
                    match (left, right) {
                        (ConstValue::Int(l), ConstValue::Int(r)) => Ok(ConstValue::Int(l - r)),
                        _ => Err(UnresolvableKind::Misc("SUB: Only INT and REAL allowed".into())),
                    }
                }
            }
            "DIV" => {
                if args.len() != 2 {
                    return Err(UnresolvableKind::Misc("DIV expects exactly 2 arguments".to_string()));
                }

                let has_real = args.iter().any(|arg| matches!(arg, ConstValue::Real(_)));
                let (left, right) = (&args[0], &args[1]);

                if has_real {
                    let l_val = match left {
                        ConstValue::Real(v) => *v,
                        ConstValue::Int(v) => *v as f64,
                        _ => return Err(UnresolvableKind::Misc("DIV: Wrong type".into())),
                    };
                    let r_val = match right {
                        ConstValue::Real(v) => *v,
                        ConstValue::Int(v) => *v as f64,
                        _ => return Err(UnresolvableKind::Misc("DIV: Wrong type".into())),
                    };
                    if r_val == 0.0 {
                        return Err(UnresolvableKind::Misc("DIV: Division by zero".into()));
                    }
                    Ok(ConstValue::Real(l_val / r_val))
                } else {
                    match (left, right) {
                        (ConstValue::Int(l), ConstValue::Int(r)) => {
                            if *r == 0 {
                                return Err(UnresolvableKind::Misc("DIV: Division by zero".into()));
                            }
                            Ok(ConstValue::Int(l / r))
                        }
                        _ => Err(UnresolvableKind::Misc("DIV: Only INT and REAL allowed".into())),
                    }
                }
            }
            //Min, Max and Limit Functions
            "MIN" => {
                if args.is_empty() {
                    return Err(UnresolvableKind::Misc("MIN expects at least 1 argument".to_string()));
                }

                let has_real = args.iter().any(|arg| matches!(arg, ConstValue::Real(_)));
                if has_real {
                    let mut min_val: f64 = f64::MAX;
                    for arg in args {
                        let val = match arg {
                            ConstValue::Real(v) => v,
                            ConstValue::Int(v) => v as f64,
                            _ => {
                                return Err(UnresolvableKind::Misc(
                                    "MIN: Unexpected type for numeric comparison".into(),
                                ))
                            }
                        };
                        if val < min_val {
                            min_val = val;
                        }
                    }
                    Ok(ConstValue::Real(min_val))
                } else {
                    let mut min_val: i128 = i128::MAX;
                    for arg in args {
                        match arg {
                            ConstValue::Int(v) => {
                                if v < min_val {
                                    min_val = v;
                                }
                            }
                            _ => return Err(UnresolvableKind::Misc("MIN: Only INT and REAL allowed".into())),
                        }
                    }
                    Ok(ConstValue::Int(min_val))
                }
            }
            "MAX" => {
                if args.is_empty() {
                    return Err(UnresolvableKind::Misc("MIN expects at least 1 argument".to_string()));
                }

                let has_real = args.iter().any(|arg| matches!(arg, ConstValue::Real(_)));
                if has_real {
                    let mut max_val: f64 = f64::MIN;
                    for arg in args {
                        let val = match arg {
                            ConstValue::Real(v) => v,
                            ConstValue::Int(v) => v as f64,
                            _ => {
                                return Err(UnresolvableKind::Misc(
                                    "MAX: Unexpected type for numeric comparison".into(),
                                ))
                            }
                        };
                        if val > max_val {
                            max_val = val;
                        }
                    }
                    Ok(ConstValue::Real(max_val))
                } else {
                    let mut max_val: i128 = i128::MIN;
                    for arg in args {
                        match arg {
                            ConstValue::Int(v) => {
                                if v > max_val {
                                    max_val = v;
                                }
                            }
                            _ => return Err(UnresolvableKind::Misc("MIN: Only INT and REAL allowed".into())),
                        }
                    }
                    Ok(ConstValue::Int(max_val))
                }
            }
            "LIMIT" => {
                if args.len() != 3 {
                    return Err(UnresolvableKind::Misc(
                        "LIMIT expects exactly 3 arguments (min, in, max)".into(),
                    ));
                }

                let has_real = args.iter().any(|arg| matches!(arg, ConstValue::Real(_)));
                if has_real {
                    let get_real = |idx: usize| match args[idx] {
                        ConstValue::Real(v) => Ok(v),
                        ConstValue::Int(v) => Ok(v as f64),
                        _ => Err(UnresolvableKind::Misc("LIMIT: Numeric value expected".into())),
                    };

                    let (min, input, max) = (get_real(0)?, get_real(1)?, get_real(2)?);
                    let res = if input < min {
                        min
                    } else if input > max {
                        max
                    } else {
                        input
                    };
                    Ok(ConstValue::Real(res))
                } else {
                    let get_int = |idx: usize| match args[idx] {
                        ConstValue::Int(v) => Ok(v),
                        _ => Err(UnresolvableKind::Misc("LIMIT: Integer expected".into())),
                    };

                    let (min, input, max) = (get_int(0)?, get_int(1)?, get_int(2)?);
                    let res = if input < min {
                        min
                    } else if input > max {
                        max
                    } else {
                        input
                    };
                    Ok(ConstValue::Int(res))
                }
            }
            //Comparison
            "GT" | "GE" | "EQ" | "LE" | "LT" | "NE" => {
                if args.len() != 2 {
                    return Err(UnresolvableKind::Misc(format!("{func_name} expects exactly 2 arguments")));
                }

                let has_real = args.iter().any(|arg| matches!(arg, ConstValue::Real(_)));

                if has_real {
                    let get_real = |idx: usize| match args[idx] {
                        ConstValue::Real(v) => Ok(v),
                        ConstValue::Int(v) => Ok(v as f64),
                        _ => Err(UnresolvableKind::Misc(format!("{func_name}: Numeric value expected"))),
                    };

                    let (left, right) = (get_real(0)?, get_real(1)?);

                    let res = match func_name.as_str() {
                        "GT" => left > right,
                        "GE" => left >= right,
                        "EQ" => left == right,
                        "LE" => left <= right,
                        "LT" => left < right,
                        "NE" => left != right,
                        _ => unreachable!(),
                    };
                    Ok(ConstValue::Bool(res))
                } else {
                    let get_int = |idx: usize| match args[idx] {
                        ConstValue::Int(v) => Ok(v),
                        _ => Err(UnresolvableKind::Misc(format!("{func_name}: Integer expected"))),
                    };

                    let (left, right) = (get_int(0)?, get_int(1)?);

                    let res = match func_name.as_str() {
                        "GT" => left > right,
                        "GE" => left >= right,
                        "EQ" => left == right,
                        "LE" => left <= right,
                        "LT" => left < right,
                        "NE" => left != right,
                        _ => unreachable!(),
                    };
                    Ok(ConstValue::Bool(res))
                }
            }
            _ => Err(UnresolvableKind::Misc(format!(
                "Unknown or not implemented standard function '{}'",
                func_name
            ))),
        }
    }

    /// Extracts and normalizes the function name from a CallStatement.
    fn extract_call_statement_function_name(call: &CallStatement) -> Option<String> {
        match call.operator.get_stmt() {
            AstStatement::ReferenceExpr(ReferenceExpr { access, .. }) => match access {
                ReferenceAccess::Member(reference) => {
                    reference.get_flat_reference_name().map(|n| n.to_uppercase())
                }
                _ => None,
            },
            AstStatement::Identifier(name) => Some(name.to_uppercase()),
            _ => None,
        }
    }

    fn eval_assignment(
        &mut self,
        data: &Assignment,
        scope: Option<&str>,
        node: &AstNode,
    ) -> Result<ConstValue, UnresolvableKind> {
        let right_value = self.traverse_ast(&data.right, scope)?;

        Ok(ConstValue::Assignment(AssignmentValue::new(data, right_value, node.get_id())))
    }

    fn eval_expression_list(
        &mut self,
        expressions: &[AstNode],
        scope: Option<&str>,
        node_id: usize,
    ) -> Result<ConstValue, UnresolvableKind> {
        let mut resolved = Vec::new();
        for expr in expressions {
            //traverse the AST to resolve every member
            let value = self.traverse_ast(expr, scope)?;
            resolved.push((value, expr.get_id()));
        }
        Ok(ConstValue::List { elements: resolved, kind: ListKind::ExpressionList, list_id: Some(node_id) })
    }

    fn eval_default_value(
        &mut self,
        node: &AstNode,
        scope: Option<&str>,
    ) -> Result<ConstValue, UnresolvableKind> {
        let dti = self
            .result_type_info(node)
            .cloned()
            .ok_or_else(|| UnresolvableKind::Misc("Cannot determine type for default value".to_string()))?;
        let type_name = dti.get_name().to_owned();

        // Check for Explicit Initial Values in the Type Definition
        let explicit_init_node = self
            .current_const_target_type
            .as_deref()
            .and_then(|name| self.index.get_initial_value_for_type(name))
            .or_else(|| self.index.get_initial_value_for_type(&type_name))
            .cloned();

        if let Some(init_node) = explicit_init_node {
            let resolved = self.traverse_ast(&init_node, scope)?;
            return self.cast_to_target_type(resolved, &type_name, node.get_id());
        }

        //The default value of an ENUM is implicitly its first defined identifier. In the absence of an explicit assignment,
        //the compiler must resolve the first variant to its underlying integer value (typically 0)
        if let Some(target_type_name) = self.current_const_target_type.as_deref() {
            if let Some(dt) = self.index.find_type(target_type_name) {
                if let DataTypeInformation::Enum { variants, .. } = dt.get_type_information() {
                    if let Some(first_init) = variants.first().and_then(|v| v.initial_value.as_ref()) {
                        let literal = self.evaluate(*first_init)?;
                        let resolved = self.traverse_ast(&literal, scope)?;
                        return self.cast_to_target_type(resolved, &type_name, node.get_id());
                    }
                }
            }
        }

        //Standard Values for Base Datatypes which need no further checks
        let default_val = if dti.is_pointer() {
            ConstValue::Null
        } else if dti.is_string_utf16() {
            ConstValue::Str(String::new(), true)
        } else if dti.is_string_utf8() {
            ConstValue::Str(String::new(), false)
        } else if Self::is_bool_type(&dti) {
            ConstValue::Bool(false)
        } else if dti.is_float() {
            ConstValue::Real(0.0)
        } else if dti.is_int() {
            ConstValue::Int(0)
        } else {
            return Err(UnresolvableKind::Misc(
                "Cannot determine default value because Datatype is not supported".to_string(),
            ));
        };
        Ok(default_val)
    }

    fn eval_unary_expression(
        &self,
        operator: Operator,
        value: ConstValue,
        node: &AstNode,
    ) -> Result<ConstValue, UnresolvableKind> {
        match (operator, value) {
            (Operator::Minus, ConstValue::Int(v)) => {
                let (result, overflowed) = v.overflowing_neg();
                if overflowed {
                    return Err(
                        self.overflow_error(node, "This will overflow: cannot negate the minimum i128 value")
                    );
                }
                self.ensure_int_result_in_expr_range(result, node)?;
                Ok(ConstValue::Int(result))
            }
            (Operator::Minus, ConstValue::Real(v)) => {
                let result = -v;
                let width = self.float_width_for_expr(node).unwrap_or(64);
                if Self::is_float_overflow(result, width) {
                    return Err(self.overflow_error(node, "Float result out of range"));
                }
                Ok(ConstValue::Real(result))
            }
            (Operator::Plus, v) => Ok(v),
            (Operator::Not, ConstValue::Bool(v)) => Ok(ConstValue::Bool(!v)),
            (Operator::Not, ConstValue::Int(v)) => Ok(ConstValue::Int(!v)),
            _ => Err(UnresolvableKind::Misc(format!(
                "Unsupported unary operation in const_evaluator_new: {operator:?}"
            ))),
        }
    }

    fn eval_binary_expression(
        &self,
        operator: Operator,
        left: ConstValue,
        right: ConstValue,
        node: &AstNode,
    ) -> Result<ConstValue, UnresolvableKind> {
        match (left, right) {
            (ConstValue::Bool(l), ConstValue::Bool(r)) => self.eval_bool_binary(operator, l, r),
            (ConstValue::Int(l), ConstValue::Int(r)) => self.eval_int_binary(operator, l, r, node),
            (ConstValue::Real(l), ConstValue::Real(r)) => self.eval_real_binary(operator, l, r, node),
            (ConstValue::Real(l), ConstValue::Int(r)) => self.eval_real_binary(operator, l, r as f64, node),
            (ConstValue::Int(l), ConstValue::Real(r)) => self.eval_real_binary(operator, l as f64, r, node),
            (ConstValue::Str(lv, _), ConstValue::Str(rv, _)) => self.eval_string_binary(operator, lv, rv),
            _ => Err(self.unsupported_binary(operator)),
        }
    }

    fn eval_int_binary(
        &self,
        operator: Operator,
        left: i128,
        right: i128,
        node: &AstNode,
    ) -> Result<ConstValue, UnresolvableKind> {
        let int_result = match operator {
            Operator::Plus => {
                let (result, overflowed) = left.overflowing_add(right);
                if overflowed {
                    return Err(self.overflow_error(node, "addition overflow"));
                }
                result
            }
            Operator::Minus => {
                let (result, overflowed) = left.overflowing_sub(right);
                if overflowed {
                    return Err(self.overflow_error(node, "subtraction overflow"));
                }
                result
            }
            Operator::Multiplication => {
                let (result, overflowed) = left.overflowing_mul(right);
                if overflowed {
                    return Err(self.overflow_error(node, "multiplication overflow"));
                }
                result
            }
            Operator::Division => {
                if right == 0 {
                    return Err(UnresolvableKind::Misc("Attempt to divide by zero".to_string()));
                }
                if left == i128::MIN && right == -1 {
                    return Err(self.overflow_error(node, "division overflow"));
                }
                left / right
            }
            Operator::Modulo => {
                if right == 0 {
                    return Err(UnresolvableKind::Misc(
                        "Attempt to calculate the remainder with a divisor of zero".to_string(),
                    ));
                }
                if left == i128::MIN && right == -1 {
                    return Err(self.overflow_error(node, "modulo overflow"));
                }
                left % right
            }
            Operator::And => left & right,
            Operator::Or => left | right,
            Operator::Xor => left ^ right,
            Operator::Equal => return Ok(ConstValue::Bool(left == right)),
            Operator::NotEqual => return Ok(ConstValue::Bool(left != right)),
            Operator::Greater => return Ok(ConstValue::Bool(left > right)),
            Operator::GreaterOrEqual => return Ok(ConstValue::Bool(left >= right)),
            Operator::Less => return Ok(ConstValue::Bool(left < right)),
            Operator::LessOrEqual => return Ok(ConstValue::Bool(left <= right)),
            _ => return Err(self.unsupported_binary(operator)),
        };

        self.ensure_int_result_in_expr_range(int_result, node)?;

        Ok(ConstValue::Int(int_result))
    }

    fn eval_real_binary(
        &self,
        operator: Operator,
        left: f64,
        right: f64,
        node: &AstNode,
    ) -> Result<ConstValue, UnresolvableKind> {
        let real_result = match operator {
            Operator::Plus => left + right,
            Operator::Minus => left - right,
            Operator::Multiplication => left * right,
            Operator::Division => {
                if right == 0.0 {
                    return Err(UnresolvableKind::Misc("Attempt to divide by zero".to_string()));
                }
                left / right
            }
            Operator::Modulo => {
                if right == 0.0 {
                    return Err(UnresolvableKind::Misc(
                        "Attempt to calculate the remainder with a divisor of zero".to_string(),
                    ));
                }
                left % right
            }
            Operator::Equal
            | Operator::NotEqual
            | Operator::Greater
            | Operator::GreaterOrEqual
            | Operator::Less
            | Operator::LessOrEqual => {
                return Err(UnresolvableKind::Misc("Cannot compare Reals without epsilon".to_string()))
            }
            _ => return Err(self.unsupported_binary(operator)),
        };

        let width = self.float_width_for_expr(node).unwrap_or(64);
        if Self::is_float_overflow(real_result, width) {
            return Err(self.overflow_error(node, "Float result out of range"));
        }

        Ok(ConstValue::Real(real_result))
    }

    fn eval_bool_binary(
        &self,
        operator: Operator,
        left: bool,
        right: bool,
    ) -> Result<ConstValue, UnresolvableKind> {
        match operator {
            Operator::And => Ok(ConstValue::Bool(left && right)),
            Operator::Or => Ok(ConstValue::Bool(left || right)),
            Operator::Xor => Ok(ConstValue::Bool(left ^ right)),
            Operator::Equal => Ok(ConstValue::Bool(left == right)),
            Operator::NotEqual => Ok(ConstValue::Bool(left != right)),
            _ => {
                Err(UnresolvableKind::Misc(format!("Operator {} is not supported for BOOL types", operator)))
            }
        }
    }

    fn eval_string_binary(
        &self,
        operator: Operator,
        left: String,
        right: String,
    ) -> Result<ConstValue, UnresolvableKind> {
        match operator {
            Operator::Equal => Ok(ConstValue::Bool(left == right)),
            Operator::NotEqual => Ok(ConstValue::Bool(left != right)),
            _ => Err(UnresolvableKind::Misc(format!(
                "Operator {} is not supported for STRING/WSTRING types",
                operator
            ))),
        }
    }

    /// Resolves a named constant reference by looking it up in the scope hierarchy
    fn eval_variable_reference(
        &mut self,
        name: &str,
        qualifier: Option<&str>,
        scope: Option<&str>,
    ) -> Result<ConstValue, UnresolvableKind> {
        let (initial_id, type_name) = {
            let variable =
                self.index.find_variable(qualifier.or(scope), std::slice::from_ref(&name)).ok_or_else(
                    || UnresolvableKind::Misc(format!("Cannot resolve constant reference `{name}`")),
                )?;

            if !variable.is_constant() {
                return Err(UnresolvableKind::Misc(format!("`{name}` is no const reference")));
            }

            let Some(initial_id) = variable.initial_value else {
                return Err(UnresolvableKind::Misc(format!(
                    "Constant reference `{name}` has no initial value"
                )));
            };

            (initial_id, variable.get_type_name().to_string())
        };

        if let Some(val) = self.const_cache.get(&initial_id) {
            return Ok(val.clone());
        }

        let resolved = match self.evaluate(initial_id) {
            Ok(resolved) => resolved,
            Err(kind) => {
                if kind.get_reason().contains("Cycle detected") {
                    return Err(kind);
                }
                return Err(UnresolvableKind::Misc(format!(
                    "Constant reference `{name}` is not resolvable: {}",
                    kind.get_reason()
                )));
            }
        };

        if resolved.is_default_value() {
            if let Some(dti) = self.index.find_effective_type_info(&type_name) {
                if dti.is_struct() || dti.is_array() {
                    return Err(UnresolvableKind::Misc(format!(
                        "Cannot reference {} with implicit default initialization",
                        if dti.is_struct() { "struct" } else { "array" }
                    )));
                }
            }
        }

        let value = ConstValue::from_ast(&resolved)?;
        Ok(value)
    }

    /// Handles explicit type casts (e.g., INT#5) and Enum variant access (e.g., Color#Red).
    /// It evaluates the target and converts it to the required bit-representation.
    fn eval_cast_reference(
        &mut self,
        target: &AstNode,
        base: Option<&AstNode>,
        scope: Option<&str>,
        node: &AstNode,
    ) -> Result<ConstValue, UnresolvableKind> {
        let Some(type_name) = base.and_then(|it| it.get_flat_reference_name()) else {
            return Err(UnresolvableKind::Misc("Cannot resolve unknown Type-Cast.".to_string()));
        };

        let Some(dti) = self.index.find_effective_type_info(type_name) else {
            return Err(UnresolvableKind::Misc("Cannot resolve unknown Type-Cast.".to_string()));
        };

        // ENUM SPECIAL CASE: Looking up variant values directly from the enum definition.
        if let DataTypeInformation::Enum { name: enum_name, .. } = dti {
            if let AstStatement::Identifier(ref_name) = target.get_stmt() {
                let Some(variant) = self.index.find_enum_variant(enum_name, ref_name) else {
                    return Err(UnresolvableKind::Misc(format!(
                        "Cannot resolve constant enum {enum_name}#{ref_name}."
                    )));
                };
                let Some(initial_id) = variant.initial_value else {
                    return Err(UnresolvableKind::Misc(format!(
                        "Cannot resolve constant enum {enum_name}#{ref_name}."
                    )));
                };

                // look for the value in the cache first
                if let Some(val) = self.const_cache.get(&initial_id) {
                    return Ok(val.clone());
                }

                let resolved = self.evaluate(initial_id)?;
                return ConstValue::from_ast(&resolved);
            }
            return Err(UnresolvableKind::Misc("Cannot resolve unknown constant.".to_string()));
        }

        // STANDARD CAST: Evaluating the value and then applying physical type conversion rules.
        let cast_target_type_name = dti.get_name().to_string();
        let casted = self.traverse_ast(target, scope)?;
        self.cast_to_target_type(casted, &cast_target_type_name, node.get_id())
    }

    /// Evaluates struct field access (e.g., myPoint.x) by resolving the base struct
    /// and extracting the requested field value.
    fn eval_struct_field_access(
        &mut self,
        base: &AstNode,
        field_name: &str,
        scope: Option<&str>,
        base_dti: &DataTypeInformation,
    ) -> Result<ConstValue, UnresolvableKind> {
        let base_type_name = base_dti.get_name().to_string();

        let base_value = self.traverse_ast(base, scope)?;

        // Extract the field value from the struct
        match base_value {
            // CASE 1: Struct represented as a list of assignments (e.g., from initialization)
            ConstValue::List { elements, kind: ListKind::ExpressionList, .. } => {
                for (element, _) in elements {
                    if let ConstValue::Assignment(assign) = element {
                        if let Some(assigned_field_name) = assign.left.get_flat_reference_name() {
                            if assigned_field_name.eq_ignore_ascii_case(field_name) {
                                return Ok(*assign.right);
                            }
                        }
                    }
                }
                Err(UnresolvableKind::Misc(format!(
                    "Field {field_name} not found in struct {base_type_name}"
                )))
            }
            // CASE 2: Single assignment
            ConstValue::Assignment(assign) => {
                if let Some(assigned_field_name) = assign.left.get_flat_reference_name() {
                    if assigned_field_name.eq_ignore_ascii_case(field_name) {
                        return Ok(*assign.right);
                    }
                }
                Err(UnresolvableKind::Misc(format!(
                    "Field {field_name} not found in struct {base_type_name}"
                )))
            }
            // Other cases: cannot extract field
            _ => Err(UnresolvableKind::Misc(format!(
                "Cannot access field on value of type {}",
                base_value.get_datatype()
            ))),
        }
    }

    fn effective_type_info_for_field_base(
        &self,
        base: &AstNode,
        scope: Option<&str>,
    ) -> Option<DataTypeInformation> {
        if matches!(
            self.annotations.get(base),
            Some(
                StatementAnnotation::Program { .. }
                    | StatementAnnotation::Type { .. }
                    | StatementAnnotation::Function { .. }
                    | StatementAnnotation::FunctionPointer { .. }
            )
        ) {
            return None;
        }

        if let Some(dti) = self.result_type_info(base) {
            return self
                .index
                .find_effective_type_info(dti.get_name())
                .cloned()
                .or_else(|| Some(dti.clone()));
        }

        base.get_flat_reference_name()
            .and_then(|name| self.index.find_variable(scope, std::slice::from_ref(&name)))
            .and_then(|var| self.index.find_effective_type_info(var.get_type_name()))
            .cloned()
    }

    /// This function acts as the "Gatekeeper" of the Constant Evaluator.
    /// It ensures that a calculated value (source) is physically and logically
    /// compatible with a defined target type (e.g., converting a generic Int to a WORD).
    fn cast_to_target_type(
        &self,
        value: ConstValue,
        target_type_name: &str,
        node_id: usize,
    ) -> Result<ConstValue, UnresolvableKind> {
        let Some(dti) = self.index.find_effective_type_info(target_type_name) else {
            return Err(UnresolvableKind::Misc(format!(
                "Cannot resolve target type `{target_type_name}` for const cast in const_evaluator_new.",
            )));
        };

        match (value, dti) {
            // Pass through MultipliedStatement without casting, nothing to cast here
            (ConstValue::Multiplied(element, multiplier, element_id), _) => {
                Ok(ConstValue::Multiplied(element, multiplier, element_id))
            }
            // If we have a list of values and the target is an array, we recursively
            // cast every element to the array's inner type (e.g., casting [1, 2] to ARRAY OF BYTE).
            (
                ConstValue::List { elements: values, kind, list_id },
                DataTypeInformation::Array { inner_type_name, .. },
            ) => {
                let casted_elements = values
                    .into_iter()
                    .map(|(value, id)| {
                        self.cast_to_target_type(value, inner_type_name, id).map(|cv| (cv, id))
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ConstValue::List { elements: casted_elements, kind, list_id })
            }
            // Subranges (e.g., INT(0..100)): We first cast to the base type,
            // then verify the result stays within the 0..100 boundary.
            (v, DataTypeInformation::SubRange { referenced_type, sub_range, .. }) => {
                let casted = self.cast_to_target_type(v, referenced_type, node_id)?;
                match casted {
                    ConstValue::Int(_) => self.enforce_subrange(casted, sub_range),
                    other => Err(UnresolvableKind::Misc(format!(
                        "SubRange expects an integer value but got {}",
                        other.get_datatype()
                    ))),
                }
            }
            // Type Alias (e.g., TYPE MyInt : INT): We simply unwrap the alias and cast to the base.
            (v, DataTypeInformation::Alias { referenced_type, .. }) => {
                self.cast_to_target_type(v, referenced_type, node_id)
            }
            // Struct Initializer (e.g., myPoint : Point := (x := 1, y := 2)):
            // We resolve each member's type and recursively cast the assigned values.
            (v, dti @ DataTypeInformation::Struct { .. }) => self.cast_struct_initializer(v, dti),
            (ConstValue::Null, dti) if dti.is_pointer() => Ok(ConstValue::Null),
            (ConstValue::Str(v, _), dti) if dti.is_string() => Ok(ConstValue::Str(v, dti.is_string_utf16())),
            (ConstValue::Bool(_), dti) if !Self::is_bool_type(dti) => Err(UnresolvableKind::Misc(format!(
                "Cannot convert value of type BOOL to {}",
                dti.get_name()
            ))),
            (ConstValue::Bool(v), dti) if Self::is_bool_type(dti) => Ok(ConstValue::Bool(v)),
            (ConstValue::Int(v), dti) if Self::is_bool_type(dti) => match v {
                0 => Ok(ConstValue::Bool(false)),
                1 => Ok(ConstValue::Bool(true)),
                _ => Err(UnresolvableKind::Misc(format!(
                    "Implicit conversion from INT to BOOL failed: Only 0 or 1 are allowed, but got {}",
                    v
                ))),
            },
            (v, dti) if Self::is_bool_type(dti) => Err(UnresolvableKind::Misc(format!(
                "Cannot convert value of type {} to BOOL",
                v.get_datatype()
            ))),
            (ConstValue::Int(v), DataTypeInformation::Enum { .. }) => Ok(ConstValue::Int(v)),
            (ConstValue::Int(v), DataTypeInformation::Integer { signed, size, semantic_size, .. })
                if !Self::is_bool_type(dti) =>
            {
                let width = semantic_size.unwrap_or(*size);
                if Self::is_integer_overflow(v, *signed, width) {
                    return Err(self.type_overflow(dti));
                }
                Ok(ConstValue::Int(v))
            }
            (ConstValue::Real(v), DataTypeInformation::Float { size, .. }) => {
                if Self::is_float_overflow(v, *size) {
                    return Err(self.type_overflow(dti));
                }
                Ok(ConstValue::Real(v))
            }
            (ConstValue::Int(v), DataTypeInformation::Float { size, .. }) => {
                let fv = v as f64;
                if Self::is_float_overflow(fv, *size) {
                    return Err(self.type_overflow(dti));
                }
                Ok(ConstValue::Real(fv))
            }
            (ConstValue::Real(v), DataTypeInformation::Integer { signed, size, semantic_size, .. })
                if !Self::is_bool_type(dti) =>
            {
                if !v.is_finite() {
                    return Err(self.type_overflow(dti));
                }
                let truncated = v.trunc();
                if truncated < i128::MIN as f64 || truncated > i128::MAX as f64 {
                    return Err(self.type_overflow(dti));
                }
                let int_value = truncated as i128;
                let width = semantic_size.unwrap_or(*size);
                if Self::is_integer_overflow(int_value, *signed, width) {
                    return Err(self.type_overflow(dti));
                }
                Ok(ConstValue::Int(int_value))
            }
            // let the validator handle it
            (v, _) => Ok(v),
        }
    }

    /// Verifies if a calculated integer value fits within the target bit-width
    /// and signedness defined for the expression at the given AST node.
    fn ensure_int_result_in_expr_range(&self, value: i128, node: &AstNode) -> Result<(), UnresolvableKind> {
        if let Some((signed, width)) = self.integer_layout_for_expr(node) {
            if Self::is_integer_overflow(value, signed, width) {
                return Err(self.overflow_error(node, "result out of range"));
            }
        }
        Ok(())
    }

    /// Processes struct initializers (e.g., (x := 1, y := 2.0)).
    /// Recursively casts each assigned value to the corresponding
    /// struct member's type and validates the subrange if given.
    fn cast_struct_initializer(
        &self,
        value: ConstValue,
        target_dti: &DataTypeInformation,
    ) -> Result<ConstValue, UnresolvableKind> {
        match (value, target_dti) {
            // CASE 1: Multi-member initialization list (e.g., '(x := 1, y := 2)')
            (
                ConstValue::List { elements, kind: ListKind::ExpressionList, list_id },
                DataTypeInformation::Struct { members, .. },
            ) => {
                let casted_elements = elements
                    .into_iter()
                    .map(|(val, id)| match val {
                        ConstValue::Assignment(assign) => {
                            let member_type = self.resolve_struct_member_type_name(&assign.left, members)?;
                            let casted_right =
                                self.cast_to_target_type(*assign.right, &member_type, assign.right_id)?;

                            Ok((
                                ConstValue::Assignment(AssignmentValue {
                                    left: assign.left,
                                    right: Box::new(casted_right),
                                    assign_id: assign.assign_id,
                                    right_id: assign.right_id,
                                    right_location: assign.right_location,
                                }),
                                id,
                            ))
                        }
                        other => Ok((other, id)),
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ConstValue::List { elements: casted_elements, kind: ListKind::ExpressionList, list_id })
            }

            // CASE 2: Single member assignment within a struct context
            (ConstValue::Assignment(assign), DataTypeInformation::Struct { members, .. }) => {
                let member_type = self.resolve_struct_member_type_name(&assign.left, members)?;
                let casted_right = self.cast_to_target_type(*assign.right, &member_type, assign.right_id)?;

                Ok(ConstValue::Assignment(AssignmentValue {
                    left: assign.left,
                    right: Box::new(casted_right),
                    assign_id: assign.assign_id,
                    right_id: assign.right_id,
                    right_location: assign.right_location,
                }))
            }

            // all others ok
            (v, _) => Ok(v),
        }
    }

    /// Resolves the data type name for a specific struct member based on the assignment's left side.
    /// It searches the struct's member list for a name match (case-insensitive).
    fn resolve_struct_member_type_name(
        &self,
        left: &AstNode,
        members: &[VariableIndexEntry],
    ) -> Result<String, UnresolvableKind> {
        let Some(member_name) = left.get_flat_reference_name() else {
            return Err(UnresolvableKind::Misc("Cannot resolve struct member assignment".to_string()));
        };

        members
            .iter()
            .find(|it| it.get_name().eq_ignore_ascii_case(member_name))
            .map(|it| it.get_type_name().to_string())
            .ok_or_else(|| UnresolvableKind::Misc(format!("Unknown struct member `{member_name}`")))
    }

    /// Validates that an integer constant falls within the mathematical
    /// boundaries of a defined SubRange
    fn enforce_subrange(
        &self,
        value: ConstValue,
        sub_range: &std::ops::Range<crate::typesystem::TypeSize>,
    ) -> Result<ConstValue, UnresolvableKind> {
        let ConstValue::Int(v) = value else {
            return Err(UnresolvableKind::Misc("SubRange expects an integer value".to_string()));
        };

        // Resolve the range bounds. Note that bounds themselves
        // can be other constants, requiring an index lookup.
        let start =
            sub_range.start.as_int_value(self.index).map_err(|err| {
                UnresolvableKind::Misc(format!("Cannot resolve SubRange lower bound: {err}"))
            })? as i128;

        let end =
            sub_range.end.as_int_value(self.index).map_err(|err| {
                UnresolvableKind::Misc(format!("Cannot resolve SubRange upper bound: {err}"))
            })? as i128;

        // Range Check
        if v < start || v > end {
            return Err(UnresolvableKind::Misc(format!("Value {v} is out of SubRange {start}..{end}")));
        }

        Ok(ConstValue::Int(v))
    }

    /// Helper function to create an UnresolvableKind for overflow/underflow errors.
    /// Uses type information if available, otherwise falls back to the provided reason.
    fn overflow_error(&self, node: &AstNode, reason: &str) -> UnresolvableKind {
        if let Some(dti) = self.result_type_info(node) {
            return self.type_overflow(dti);
        }
        UnresolvableKind::Misc(format!("Overflow: {reason}"))
    }

    /// Helper function to create an UnresolvableKind for unsupported binary errors.
    fn unsupported_binary(&self, operator: Operator) -> UnresolvableKind {
        UnresolvableKind::Misc(format!("Unsupported binary operation in const_evaluator_new: {operator:?}"))
    }

    fn result_type_info(&self, node: &AstNode) -> Option<&DataTypeInformation> {
        self.annotations
            .get_type_hint(node, self.index)
            .or_else(|| self.annotations.get_type(node, self.index))
            .map(|dt| dt.get_type_information())
    }

    fn integer_layout_for_expr(&self, node: &AstNode) -> Option<(bool, u32)> {
        match self.result_type_info(node) {
            Some(DataTypeInformation::Integer { signed, size, semantic_size, .. }) => {
                Some((*signed, semantic_size.unwrap_or(*size)))
            }
            _ => None,
        }
    }

    fn float_width_for_expr(&self, node: &AstNode) -> Option<u32> {
        match self.result_type_info(node) {
            Some(DataTypeInformation::Float { size, .. }) => Some(*size),
            _ => None,
        }
    }

    fn is_bool_type(dti: &DataTypeInformation) -> bool {
        matches!(dti, DataTypeInformation::Integer { semantic_size: Some(1), .. })
    }

    fn type_overflow(&self, dti: &DataTypeInformation) -> UnresolvableKind {
        UnresolvableKind::Misc(format!("Overflow: This will overflow/underflow for type {}", dti.get_name()))
    }

    fn integer_bounds(signed: bool, width: u32) -> Option<(i128, i128)> {
        match (signed, width) {
            (true, 8) => Some((i8::MIN as i128, i8::MAX as i128)),
            (true, 16) => Some((i16::MIN as i128, i16::MAX as i128)),
            (true, 32) => Some((i32::MIN as i128, i32::MAX as i128)),
            (true, 64) => Some((i64::MIN as i128, i64::MAX as i128)),
            (false, 8) => Some((0, u8::MAX as i128)),
            (false, 16) => Some((0, u16::MAX as i128)),
            (false, 32) => Some((0, u32::MAX as i128)),
            (false, 64) => Some((0, u64::MAX as i128)),
            _ => None,
        }
    }

    /// Checks whether an integer value exceeds the supported type bounds.
    fn is_integer_overflow(value: i128, signed: bool, width: u32) -> bool {
        let Some((min, max)) = Self::integer_bounds(signed, width) else {
            return width == 0;
        };

        value < min || value > max
    }

    /// Checks whether a floating-point value exceeds the supported type bounds.
    fn is_float_overflow(value: f64, width: u32) -> bool {
        if !value.is_finite() {
            return true;
        }

        match width {
            32 => value < f32::MIN as f64 || value > f32::MAX as f64,
            64 => false,
            _ => false,
        }
    }
}
