use std::path::PathBuf;

use plc_ast::{
    ast::{
        Assignment, AstFactory, AstId, AstNode, AstStatement, CompilationUnit, DataTypeDeclaration,
        Implementation, LinkageType, Pou, PouType, ReferenceExpr, Variable, VariableBlock,
    },
    provider::IdProvider,
};
use plc_source::source_location::{FileMarker, SourceLocation};

#[derive(Clone, Default)]
pub struct Context {
    /// optional context for references (e.g. `x` may mean `POU.x` if used inside `POU` body or `STRUCT.x` if `x` is a member of `STRUCT`)
    scope: Option<String>,

    pub id_provider: IdProvider,
}

// TODO: use &str with lifetimes, requires loads of changes to the visitor/walker traits
impl Context {
    pub fn new(id_provider: IdProvider) -> Self {
        Self { scope: None, id_provider }
    }

    /// updates the context's scope and returns the previous value
    pub fn scope(&mut self, pou: Option<String>) -> Option<String> {
        std::mem::replace(&mut self.scope, pou)
    }

    pub fn get_scope(&self) -> &Option<String> {
        &self.scope
    }

    pub fn get_id_provider(&self) -> IdProvider {
        self.id_provider.clone()
    }

    pub fn next_id(&mut self) -> usize {
        self.id_provider.next_id()
    }
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

/// Takes some expression such as `foo : FooStruct := (bar := (baz := (qux := ADR(val)), baz2 := (qux := ADR(val))));`
/// and returns assignments of form [`foo.bar.baz.qux := ADR(val)`, `foo.bar.baz2.qux := ADR(val)`].
pub fn create_assignments_from_initializer(
    var_ident: &str,
    self_ident: Option<&str>,
    rhs: &Option<AstNode>,
    mut id_provider: IdProvider,
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

        let right = path.pop().expect("must have at least one node in the path");
        let mut left = path.pop().expect("must have at least one node in the path");

        for node in path.into_iter().rev() {
            insert_base_node(&mut left, node);
        }

        result.push(AstFactory::create_assignment(left, right, id_provider.next_id()));
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

        _ => panic!("invalid function call, expected a member reference"),
    }
}

pub fn create_ref_assignment(
    lhs_ident: &str,
    base_ident: Option<&str>,
    rhs: &AstNode,
    mut id_provider: IdProvider,
) -> AstNode {
    let lhs = create_member_reference(
        lhs_ident,
        id_provider.clone(),
        base_ident.map(|id| create_member_reference(id, id_provider.clone(), None)),
    );
    AstFactory::create_ref_assignment(lhs, rhs.to_owned(), id_provider.next_id())
}

pub fn create_assignment(
    lhs_ident: &str,
    base_ident: Option<&str>,
    rhs: &AstNode,
    mut id_provider: IdProvider,
) -> AstNode {
    let lhs = create_member_reference(
        lhs_ident,
        id_provider.clone(),
        base_ident.map(|id| create_member_reference(id, id_provider.clone(), None)),
    );
    AstFactory::create_assignment(lhs, rhs.to_owned(), id_provider.next_id())
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
    let ctor_name = format!("{base_name}_ctor");
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
    let ctor_name = format!("__{unit_name}_ctor");
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
