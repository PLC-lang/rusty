use std::path::PathBuf;

use crate::index::Index;
use plc_ast::{
    ast::{
        Assignment, AstFactory, AstId, AstNode, AstStatement, CompilationUnit, DataTypeDeclaration,
        Implementation, LinkageType, Pou, PouType, ReferenceExpr, Variable, VariableBlock,
    },
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

/// Creates a new AstNode with the same content but with an internal location.
/// This is used for generated code that should skip source-location-based validation.
fn with_internal_location(node: &AstNode, id_provider: &mut IdProvider) -> AstNode {
    let mut new_node = node.clone();
    new_node.location = SourceLocation::internal();
    new_node.id = id_provider.next_id();
    new_node
}

pub fn create_member_reference_with_location(
    ident: &str,
    mut id_provider: IdProvider,
    base: Option<AstNode>,
    location: SourceLocation,
) -> AstNode {
    AstFactory::create_member_reference(
        AstFactory::create_identifier(ident, location, id_provider.next_id()),
        base,
        id_provider.next_id(),
    )
}

pub fn create_member_reference(ident: &str, id_provider: IdProvider, base: Option<AstNode>) -> AstNode {
    create_member_reference_with_location(ident, id_provider, base, SourceLocation::internal())
}

/// Takes some expression such as `bar := (baz := (qux := ADR(val)), baz2 := (qux := ADR(val)))` returning all final
/// assignment paths such as [`bar.baz.qux := ADR(val)`, `bar.baz2.qux := ADR(val)`].
pub fn create_assignment_paths(node: &AstNode, id_provider: IdProvider) -> Vec<Vec<AstNode>> {
    match node.get_stmt() {
        AstStatement::Assignment(Assignment { left, right }) => {
            let mut result = create_assignment_paths(right, id_provider.clone());
            for inner in result.iter_mut() {
                inner.insert(0, left.as_ref().clone());
            }
            result
        }
        AstStatement::ExpressionList(nodes) => {
            let mut result = vec![];
            for node in nodes {
                let inner = create_assignment_paths(node, id_provider.clone());
                result.extend(inner);
            }
            result
        }
        AstStatement::ParenExpression(node) => create_assignment_paths(node, id_provider),
        _ => vec![vec![node.clone()]],
    }
}

/// Checks if the given node is a `REF(...)` call and returns the argument if so.
/// This is used to detect reference assignments that should use `REF=` syntax.
pub fn extract_ref_call_argument(node: &AstNode) -> Option<&AstNode> {
    if let AstStatement::CallStatement(call) = node.get_stmt_peeled() {
        // Check if the operator is a reference to "REF"
        if let Some(name) = call.operator.get_flat_reference_name() {
            if name == "REF" {
                // Return the first parameter (the argument to REF)
                if let Some(params) = &call.parameters {
                    let param = match params.get_stmt_peeled() {
                        AstStatement::ExpressionList(list) => list.first().unwrap_or(params.as_ref()),
                        _ => params.as_ref(),
                    };
                    let actual_param = match param.get_stmt_peeled() {
                        AstStatement::Assignment(data) | AstStatement::OutputAssignment(data) => {
                            data.right.as_ref()
                        }
                        _ => param,
                    };
                    return Some(actual_param);
                }
            }
        }
    }
    None
}

/// Takes some expression such as `foo : FooStruct := (bar := (baz := (qux := ADR(val)), baz2 := (qux := ADR(val))));`
/// and returns assignments of form [`foo.bar.baz.qux := ADR(val)`, `foo.bar.baz2.qux := ADR(val)`].
///
/// For REF() calls on the RHS (e.g., `foo := REF(bar)`), this generates `REF=` assignments instead,
/// so `foo REF= bar` which properly sets up reference semantics.
pub fn create_assignments_from_initializer(
    var_ident: &str,
    self_ident: Option<&str>,
    rhs: &Option<AstNode>,
    id_provider: IdProvider,
) -> Vec<AstNode> {
    create_assignments_from_initializer_with_index(var_ident, self_ident, rhs, id_provider, None, None)
}

pub fn create_assignments_from_initializer_with_index(
    var_ident: &str,
    self_ident: Option<&str>,
    rhs: &Option<AstNode>,
    mut id_provider: IdProvider,
    index: Option<&Index>,
    current_pou: Option<&str>,
) -> Vec<AstNode> {
    let Some(initializer) = rhs else {
        return Vec::new();
    };

    let mut result = vec![];
    for mut path in create_assignment_paths(initializer, id_provider.clone()) {
        path.insert(0, create_member_reference(var_ident, id_provider.clone(), None));
        if self_ident.is_some() {
            path.insert(0, create_member_reference("self", id_provider.clone(), None));
        }

        let right_node = path.pop().expect("must have at least one node in the path");
        let mut left = path.pop().expect("must have at least one node in the path");

        for node in path.into_iter().rev() {
            insert_base_node(&mut left, node);
        }

        let right_node = maybe_qualify_rhs(&right_node, index, self_ident, current_pou, id_provider.clone());

        // Check if the RHS is a REF() call - if so, use REF= assignment
        let assignment = if let Some(ref_arg) = extract_ref_call_argument(&right_node) {
            // Use internal location for the RHS to avoid validation issues on generated code
            let ref_arg_internal = with_internal_location(ref_arg, &mut id_provider);
            AstFactory::create_ref_assignment(left, ref_arg_internal, id_provider.next_id())
        } else {
            AstFactory::create_assignment(left, right_node, id_provider.next_id())
        };

        result.push(assignment);
    }

    result
}

/// Inserts a new base node into the member reference chain. For example a call such as `insert_base_node("b.c", a")`
/// will yield `a.b.c`.
pub fn insert_base_node(member: &mut AstNode, new_base: AstNode) {
    match &mut member.stmt {
        AstStatement::ReferenceExpr(ReferenceExpr { base, .. }) => match base {
            Some(inner) => insert_base_node(inner, new_base),
            None => {
                // We hit the end of the chain, simply replace the base (which must be None) with the new one
                base.replace(Box::new(new_base));
            }
        },

        _ => unreachable!("invalid function call, expected a member reference"),
    }
}

pub fn create_ref_assignment(
    lhs_ident: &str,
    base_ident: Option<&str>,
    rhs: &AstNode,
    id_provider: IdProvider,
) -> AstNode {
    create_ref_assignment_with_index(lhs_ident, base_ident, rhs, id_provider, None, None)
}

pub fn create_ref_assignment_with_index(
    lhs_ident: &str,
    base_ident: Option<&str>,
    rhs: &AstNode,
    mut id_provider: IdProvider,
    index: Option<&Index>,
    current_pou: Option<&str>,
) -> AstNode {
    let lhs = create_member_reference(
        lhs_ident,
        id_provider.clone(),
        base_ident.map(|id| create_member_reference(id, id_provider.clone(), None)),
    );

    // Process the RHS to qualify local variable references with base_ident
    let processed_rhs = maybe_qualify_rhs(rhs, index, base_ident, current_pou, id_provider.clone());

    AstFactory::create_ref_assignment(lhs, processed_rhs, id_provider.next_id())
}

pub fn create_assignment(
    lhs_ident: &str,
    base_ident: Option<&str>,
    rhs: &AstNode,
    id_provider: IdProvider,
) -> AstNode {
    create_assignment_with_index(lhs_ident, base_ident, rhs, id_provider, None, None)
}

pub fn create_assignment_with_index(
    lhs_ident: &str,
    base_ident: Option<&str>,
    rhs: &AstNode,
    mut id_provider: IdProvider,
    index: Option<&Index>,
    current_pou: Option<&str>,
) -> AstNode {
    let lhs = create_member_reference(
        lhs_ident,
        id_provider.clone(),
        base_ident.map(|id| create_member_reference(id, id_provider.clone(), None)),
    );

    // Process the RHS to qualify local variable references with base_ident
    let processed_rhs = maybe_qualify_rhs(rhs, index, base_ident, current_pou, id_provider.clone());

    AstFactory::create_assignment(lhs, processed_rhs, id_provider.next_id())
}

/// Walks through an AST expression and qualifies any unqualified member references
/// that resolve to local variables of the given POU with the base identifier (typically "self").
fn qualify_local_references(
    node: &AstNode,
    base_ident: &str,
    pou_name: &str,
    index: &Index,
    mut id_provider: IdProvider,
) -> AstNode {
    match node.get_stmt() {
        // Handle CallStatement: REF(i) where i needs qualification
        AstStatement::CallStatement(call) => {
            let qualified_params = call.parameters.as_ref().map(|params| {
                Box::new(qualify_local_references(params, base_ident, pou_name, index, id_provider.clone()))
            });

            AstNode::new(
                AstStatement::CallStatement(plc_ast::ast::CallStatement {
                    operator: call.operator.clone(),
                    parameters: qualified_params,
                }),
                id_provider.next_id(),
                node.get_location(),
            )
        }

        // Handle Identifier: check if it's an unqualified reference to a local variable
        AstStatement::Identifier(_) => {
            if let Some(var_name) = node.get_flat_reference_name() {
                // Check if this resolves to a local variable in the current POU
                if let Some(variable) = index.find_variable(Some(pou_name), &[var_name]) {
                    if variable.is_local() {
                        // Qualify it with the base identifier
                        let qualified_base = create_member_reference(base_ident, id_provider.clone(), None);
                        return create_member_reference_with_location(
                            var_name,
                            id_provider.clone(),
                            Some(qualified_base),
                            node.get_location(),
                        );
                    }
                }
            }
            node.clone()
        }

        // Handle ReferenceExpr: check if it's an unqualified reference to a local variable
        AstStatement::ReferenceExpr(ReferenceExpr { access, base }) => {
            // Only process unqualified references (base is None)
            if base.is_none() {
                if let Some(var_name) = node.get_flat_reference_name() {
                    // Check if this resolves to a local variable in the current POU
                    if let Some(variable) = index.find_variable(Some(pou_name), &[var_name]) {
                        if variable.is_local() {
                            // Qualify it with the base identifier
                            let qualified_base =
                                create_member_reference(base_ident, id_provider.clone(), None);
                            return AstNode::new(
                                AstStatement::ReferenceExpr(ReferenceExpr {
                                    access: access.clone(),
                                    base: Some(Box::new(qualified_base)),
                                }),
                                id_provider.next_id(),
                                node.get_location(),
                            );
                        }
                    }
                }
            }
            node.clone()
        }

        // Handle ExpressionList: qualify each element
        AstStatement::ExpressionList(nodes) => {
            let qualified_nodes = nodes
                .iter()
                .map(|n| qualify_local_references(n, base_ident, pou_name, index, id_provider.clone()))
                .collect();
            AstNode::new(
                AstStatement::ExpressionList(qualified_nodes),
                id_provider.next_id(),
                node.get_location(),
            )
        }

        // For other statements, recurse if they contain sub-expressions
        AstStatement::ParenExpression(inner) => {
            let qualified_inner =
                qualify_local_references(inner, base_ident, pou_name, index, id_provider.clone());
            AstNode::new(
                AstStatement::ParenExpression(Box::new(qualified_inner)),
                id_provider.next_id(),
                node.get_location(),
            )
        }

        // For other statement types, return as-is
        _ => node.clone(),
    }
}

fn maybe_qualify_rhs(
    rhs: &AstNode,
    index: Option<&Index>,
    base_ident: Option<&str>,
    current_pou: Option<&str>,
    id_provider: IdProvider,
) -> AstNode {
    match (index, base_ident, current_pou) {
        (Some(idx), Some(base), Some(pou)) => qualify_local_references(rhs, base, pou, idx, id_provider),
        _ => rhs.to_owned(),
    }
}

pub fn create_call_statement(
    operator: &str,
    member: &str,
    base: Option<&str>,
    mut id_provider: IdProvider,
    location: &SourceLocation,
) -> AstNode {
    let op = create_member_reference(operator, id_provider.clone(), None);
    let param = create_member_reference(
        member,
        id_provider.clone(),
        base.map(|it| create_member_reference(it, id_provider.clone(), None)),
    );
    AstFactory::create_call_statement(op, Some(param), id_provider.next_id(), location.clone())
}

pub fn new_constructor(
    base_name: &str,
    linkage: LinkageType,
    pou_type: PouType,
    statements: Vec<AstNode>,
    mut id_provider: IdProvider,
) -> (Pou, Implementation) {
    let ctor_name = format!("{base_name}__ctor");
    // Create a VAR_IN_OUT block with self as parameter
    let self_block = VariableBlock::default()
        .with_block_type(plc_ast::ast::VariableBlockType::InOut)
        .with_variables(vec![new_variable("self", base_name)]);
    let pou = new_pou(
        &ctor_name,
        id_provider.next_id(),
        vec![self_block],
        pou_type.clone(),
        linkage,
        &SourceLocation::internal(),
    );
    let implementation =
        new_implementation(&ctor_name, statements, pou_type, linkage, SourceLocation::internal());
    (pou, implementation)
}

pub fn new_unit_constructor(
    unit_name: &str,
    statements: Vec<AstNode>,
    mut id_provider: IdProvider,
) -> (Pou, Implementation) {
    let ctor_name = format!("__unit_{unit_name}__ctor");
    let pou = new_pou(
        &ctor_name,
        id_provider.next_id(),
        vec![],
        PouType::ProjectInit,
        LinkageType::Internal,
        &SourceLocation::internal(),
    );
    let implementation = new_implementation(
        &ctor_name,
        statements,
        PouType::ProjectInit,
        LinkageType::Internal,
        SourceLocation::internal(),
    );
    (pou, implementation)
}

pub fn new_variable(name: &str, data_type_name: &str) -> Variable {
    Variable {
        name: name.into(),
        data_type_declaration: DataTypeDeclaration::Reference {
            referenced_type: data_type_name.into(),
            location: SourceLocation::internal(),
        },
        initializer: None,
        address: None,
        location: SourceLocation::internal(),
    }
}

pub fn new_pou(
    name: &str,
    id: AstId,
    variable_blocks: Vec<VariableBlock>,
    kind: PouType,
    linkage: LinkageType,
    location: &SourceLocation,
) -> Pou {
    Pou {
        name: name.into(),
        id,
        variable_blocks,
        kind,
        return_type: None,
        location: location.clone(),
        name_location: location.to_owned(),
        poly_mode: None,
        generics: vec![],
        linkage,
        super_class: None,
        interfaces: vec![],
        properties: vec![],
        is_const: false,
    }
}

pub fn new_implementation(
    name: &str,
    statements: Vec<AstNode>,
    pou_type: PouType,
    linkage: LinkageType,
    location: SourceLocation,
) -> Implementation {
    Implementation {
        name: name.into(),
        type_name: name.into(),
        linkage,
        pou_type,
        statements,
        location: location.clone(),
        name_location: location.clone(),
        end_location: location,
        overriding: false,
        generic: false,
        access: None,
    }
}

/// Returns a sanitized unit name suitable for use as an identifier (e.g. in generated code)
pub fn get_unit_name(unit: &CompilationUnit) -> String {
    let path: PathBuf = (&unit.file).into();
    let name = path.file_name().map(|it| it.to_string_lossy()).unwrap_or_default();
    name.replace(['*', '.'], "_")
}
