//! This module is responsible for lowering inherited references.
//! Derived POUs can access members of their base class using the `__SUPER` qualifier,
//! which is an implicit reference to an instance of the base class which
//! is added as a member variable of the derived class in the 'pre_index' pass.
//!
//! For example, given the following code:
//! ```iec61131st
//! FUNCTION_BLOCK foo
//! END_FUNCTION_BLOCK
//!
//! FUNCTION_BLOCK bar EXTENDS foo
//! END_FUNCTION_BLOCK
//! ```
//! After the `pre-index` pass, the `bar`-function block will be transformed to:
//! ```iec61131st
//! FUNCTION_BLOCK bar
//! VAR
//!   __foo : foo;
//! END_VAR
//! END_FUNCTION_BLOCK
//! ```
//!
//! During the `post_annotate` pass, the inheritance hierarchy is resolved and the `__SUPER` references are added to
//! each `ReferenceExpression` that references a member of the base class.
//!
//! For example, given the following code:
//! ```iec61131st
//! FUNCTION_BLOCK foo
//! VAR
//!     x: INT;
//! END_VAR
//! END_FUNCTION_BLOCK
//!
//! FUNCTION_BLOCK bar EXTENDS foo
//! END_FUNCTION_BLOCK
//!
//! FUNCTION_BLOCK baz
//! VAR
//!    myFb : bar;
//! END_VAR
//!     myFb.x := 1;
//! END_FUNCTION_BLOCK
//! ```
//! After the `post_annotate` pass, the `baz`-function block will be transformed to:
//! ```iec61131st
//! FUNCTION_BLOCK baz
//! VAR
//!   myFb : bar;
//! END_VAR
//!    myFb.__foo.x := 1;
//! END_FUNCTION_BLOCK
//! ```

use plc::{index::Index, lowering::create_call_statement, resolver::AnnotationMap};
use plc_ast::{
    ast::{
        Assignment, AstFactory, AstNode, AstStatement, CallStatement, CompilationUnit, DataTypeDeclaration, LinkageType, Pou, PouType, ReferenceAccess, ReferenceExpr, Variable, VariableBlock, VariableBlockType
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    try_from_mut,
};
use plc_source::source_location::SourceLocation;

#[derive(Debug, Clone)]
struct Context {
    base_type_name: Option<String>,
    pou: Option<String>,
    access_kind: Option<AccessKind>,
    in_call: bool,
    id_provider: IdProvider,
}

#[derive(Debug, Copy, Clone)]
enum AccessKind {
    MemberOrIndex,
    Cast,
    Global,
}

impl Context {
    fn new(id_provider: IdProvider) -> Self {
        Self { base_type_name: None, pou: None, access_kind: None, in_call: false, id_provider }
    }

    fn with_base(&self, base_type_name: impl Into<String>) -> Self {
        Self { base_type_name: Some(base_type_name.into()), ..self.clone() }
    }

    fn try_with_base(&self, implementation_name: &str, index: &Index) -> Option<Self> {
        index
            .find_pou(implementation_name)
            .and_then(|it| it.get_super_class())
            .map(|base| self.with_base(base))
    }

    fn with_pou(&self, pou: impl Into<String>) -> Self {
        Self { pou: Some(pou.into()), ..self.clone() }
    }

    fn provider(&self) -> IdProvider {
        self.id_provider.clone()
    }
}

pub struct InheritanceLowerer {
    pub index: Option<Index>,
    pub annotations: Option<Box<dyn AnnotationMap>>,
    ctx: Context,
}

impl InheritanceLowerer {
    pub fn new(id_provider: IdProvider) -> Self {
        Self { index: None, annotations: None, ctx: Context::new(id_provider) }
    }

    pub fn provider(&self) -> IdProvider {
        self.ctx.provider()
    }

    pub fn visit_unit(&mut self, unit: &mut CompilationUnit) {
        // before we start walking the unit, we need to lower the `super` keyword to the corresponding `ReferenceExpr`
        let mut super_lowerer =
            SuperKeywordLowerer::new(self.provider(), self.index.as_ref(), self.annotations.as_mut());
        super_lowerer.visit_compilation_unit(unit);
        self.visit_compilation_unit(unit);
    }

    fn walk_with_context<T: WalkerMut>(&mut self, t: &mut T, ctx: Context) {
        let old_ctx = std::mem::replace(&mut self.ctx, ctx);
        t.walk(self);
        self.ctx = old_ctx;
    }

    // Updates the base to reflect the inheritance chain of the node and returns it
    fn update_inheritance_chain(
        &self,
        node: &AstNode,
        base: Option<Box<AstNode>>,
        base_type: &str,
    ) -> Option<Box<AstNode>> {
        if self.index.is_none() || self.annotations.is_none() {
            return base;
        }

        let annotations = self.annotations.as_ref().expect("Annotations exist");
        let Some(qualified_name) = annotations.get_qualified_name(node) else {
            return base;
        };

        let segment = qualified_name.split('.').next().expect("Must have a name");

        if base_type == segment {
            return base;
        }

        let index = self.index.as_ref().expect("Index exists");
        let inheritance_chain = index.get_inheritance_chain(base_type, segment);
        if inheritance_chain.len() <= 1 {
            return base;
        }

        // add a `__SUPER` qualifier for each element in the inheritance chain, exluding `self`
        inheritance_chain.iter().rev().skip(1).fold(base, |base, pou| {
            Some(Box::new(AstFactory::create_member_reference(
                            AstFactory::create_identifier(
                                format!("__{}", pou.get_name()),
                                SourceLocation::internal(),
                                self.provider().next_id(),
                            ),
                            base.map(|it: Box<AstNode>| *it),
                            self.provider().next_id(),
                        )))
        })
    }
}

impl AstVisitorMut for InheritanceLowerer {
    fn visit_pou(&mut self, pou: &mut Pou) {
        if self.index.is_some() {
            // methods need to be walked in the context of its container
            let pou_name = if let PouType::Method { parent, .. } = &pou.kind { parent } else { &pou.name };
            return self.walk_with_context(pou, self.ctx.with_pou(pou_name));
        }
        if !matches!(pou.kind, PouType::FunctionBlock | PouType::Class) {
            return;
        }

        let Some(base_name) = pou.super_class.as_ref() else {
            return;
        };

        let base_var = Variable {
            name: format!("__{}", &base_name.name),
            data_type_declaration: DataTypeDeclaration::Reference {
                referenced_type: base_name.name.clone(),
                location: SourceLocation::internal(),
            },
            location: SourceLocation::internal(),
            initializer: None,
            address: None,
        };

        let block = VariableBlock {
            variables: vec![base_var],
            kind: VariableBlockType::Local,
            linkage: LinkageType::Internal,
            location: SourceLocation::internal(),
            ..Default::default()
        };

        // To ensure that the `__SUPER` variable is consistently the first variable in the struct,
        // we insert it as the first variable block. This simplifies writing FFI bindings.
        pou.variable_blocks.insert(0, block);
        self.walk_with_context(pou, self.ctx.with_pou(&pou.name));
    }

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        //Only go through the implementation if we have the index and annotations
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }

        let type_name = if let PouType::Method { parent, .. } = &implementation.pou_type {
            parent
        } else {
            &implementation.type_name
        };
        let ctx = self.ctx.with_pou(type_name);
        let ctx = self
            .index
            .as_ref()
            .and_then(|it| ctx.try_with_base(&implementation.type_name, it))
            .unwrap_or(ctx);

        self.walk_with_context(implementation, ctx);
    }

    fn visit_reference_expr(&mut self, node: &mut AstNode) {
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }
        // Find the base type of the expression before walking the node
        log::trace!("Looking for base name in expression {:?}", node.get_stmt());
        let base_type_name =
            if let AstStatement::ReferenceExpr(ReferenceExpr { base: Some(base), .. }) = node.get_stmt() {
                let index = self.index.as_ref().expect("Index exists");
                let annotations = self.annotations.as_ref().expect("Annotations exist");
                annotations.get_type(base, index).map(|it| it.get_name().to_string())
            } else {
                self.ctx.pou.clone()
            };
        log::trace!("Found base type name: {base_type_name:?}");
        // First walk the statement itself so we make sure any base is correctly added
        let stmt = try_from_mut!(node, ReferenceExpr).expect("ReferenceExpr");
        stmt.walk(self);
        // If the reference is to a member of the base class, we need to add a reference to the
        // base class
        if let ReferenceExpr { base, access: ReferenceAccess::Member(access) } = stmt {
            if let Some(base_type_name) = base_type_name {
                let mut owned_base =
                    self.update_inheritance_chain(access, std::mem::take(base), &base_type_name);
                std::mem::swap(base, &mut owned_base);
            }
        };
    }

    fn visit_assignment(&mut self, node: &mut AstNode) {
        let Assignment { left, right } = try_from_mut!(node, Assignment).expect("Assignment");
        // If we are in a call statement, don't walk the left side
        if !self.ctx.in_call {
            left.walk(self);
        }
        right.walk(self);
    }

    fn visit_output_assignment(&mut self, node: &mut AstNode) {
        let Assignment { left, right } = try_from_mut!(node, Assignment).expect("OutputAssignment");
        // If we are in a call statement, don't walk the left side
        if !self.ctx.in_call {
            left.walk(self);
        }
        right.walk(self);
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, CallStatement).expect("CallStatement");
        let mut ctx = self.ctx.clone();
        ctx.in_call = true;
        self.walk_with_context(stmt, ctx);
    }

}

struct SuperKeywordLowerer<'sup> {
    index: Option<&'sup Index>,
    annotations: Option<&'sup mut Box<dyn AnnotationMap>>,
    ctx: Context,
}

impl<'sup> SuperKeywordLowerer<'sup> {
    pub fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }
        unit.walk(self);
    }

    fn new(
        id_provider: IdProvider,
        index: Option<&'sup Index>,
        annotations: Option<&'sup mut Box<dyn AnnotationMap>>,
    ) -> Self {
        Self { index, ctx: Context::new(id_provider), annotations }
    }

    fn provider(&self) -> IdProvider {
        self.ctx.provider()
    }

    fn walk_with_context<T: WalkerMut>(&mut self, t: &mut T, ctx: Context) {
        let old_ctx = std::mem::replace(&mut self.ctx, ctx);
        t.walk(self);
        self.ctx = old_ctx;
    }
}

impl AstVisitorMut for SuperKeywordLowerer<'_> {
    fn visit_pou(&mut self, pou: &mut Pou) {
        if self.index.is_some() {
            // methods need to be walked in the context of its container
            let pou_name = if let PouType::Method { parent, .. } = &pou.kind { parent } else { &pou.name };
            return self.walk_with_context(pou, self.ctx.with_pou(pou_name));
        }
        if !matches!(pou.kind, PouType::FunctionBlock | PouType::Class) {
            return;
        }

        if pou.super_class.is_none() {
            return;
        };

        self.walk_with_context(pou, self.ctx.with_pou(&pou.name));
    }
    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        //Only go through the implementation if we have the index and annotations
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }

        let type_name = if let PouType::Method { parent, .. } = &implementation.pou_type {
            parent
        } else {
            &implementation.type_name
        };
        let ctx = self.ctx.with_pou(type_name);
        let ctx = self
            .index
            .as_ref()
            .and_then(|it| ctx.try_with_base(&implementation.type_name, it))
            .unwrap_or(ctx);

        self.walk_with_context(implementation, ctx);
    }

    fn visit_reference_expr(&mut self, node: &mut AstNode) {
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }
        let ReferenceExpr { base, access } = try_from_mut!(node, ReferenceExpr).expect("ReferenceExpr");
        if let Some(base) = base {
            self.visit(base);
        }

        match access {
            ReferenceAccess::Member(t) | ReferenceAccess::Index(t) => {
                if !t.is_super() {
                    return self.visit(t);
                };
                self.ctx.access_kind = Some(AccessKind::MemberOrIndex);
                self.visit(t);
                self.ctx.access_kind = None;

                // if we encountered a `super` reference and were able to lower it, we need to add the original base
                // to the new `ReferenceExpr` that was created
                if !t.is_super() {
                    match t.get_stmt_mut() {
                        AstStatement::ReferenceExpr(ReferenceExpr { base: super_base, .. }) => {
                            std::mem::swap(base, super_base);
                        }
                        _ => {
                            if cfg!(debug_assertions) {
                                unreachable!("Edge-case of `SUPER` usage we didn't expect");
                            }
                        }
                    }
                };
            }
            ReferenceAccess::Global(t) => {
                self.ctx.access_kind = Some(AccessKind::Global);
                self.visit(t);
                self.ctx.access_kind = None;
            }
            ReferenceAccess::Cast(t) => {
                if !t.is_super() {
                    return self.visit(t);
                };
                self.ctx.access_kind = Some(AccessKind::Cast);
                self.visit(t);
                self.ctx.access_kind = None;
            }
            ReferenceAccess::Deref => {}
            _ => {
                // We don't need to do anything for other access types (famous last words)
                log::trace!("Unsupported access type: {access:?}");
                if cfg!(debug_assertions) {
                    unimplemented!("Unsupported access type: {access:?}");
                }
            }
        };
    }

    fn visit_super(&mut self, node: &mut AstNode) {
        let Some(base_type_name) = self
            .ctx
            .base_type_name
            .as_deref()
            .or_else(|| {
                self.ctx
                    .pou
                    .as_ref()
                    .and_then(|it| self.index.unwrap().find_pou(it))
                    .and_then(|it| it.get_super_class())
            })
            .map(|it| format!("__{it}"))
        else {
            return;
        };

        let mut old_node = std::mem::take(node);
        let location = old_node.get_location();
        let AstStatement::Super(deref_marker) = old_node.get_stmt() else {
            unreachable!("Must be a super statement")
        };

        let mut new_node = match self.ctx.access_kind {
            Some(AccessKind::MemberOrIndex) | Some(AccessKind::Global) | Some(AccessKind::Cast)=> {
                // Neither of these cases are valid code:
                //      1. `myFb.SUPER`     - this is a qualified access to `SUPER` outside of the derived POU
                //      2. `myFb.SUPER.x`   - this is a member access to a parent variable through the pointer instead of the instance on top of an outside access
                //      3. `.SUPER^.x`      - this is an access to super through the global namespace operator
                //      4. `superTy#SUPER`  - this is an attempted cast of super to some type
                // Return the original node and let the validator handle it
                std::mem::swap(node, &mut old_node);
                return;
            }
            None if deref_marker.is_some() => {
                // If the super statement is dereferenced, we can just use the existing base-class instance
                AstFactory::create_member_reference(
                    AstFactory::create_identifier(&base_type_name, location, self.provider().next_id())
                        .with_metadata(old_node.into()),
                    None,
                    self.provider().next_id(),
                )
            }
            None => {
                // If the super statement is not dereferenced, we need to bitcast the base-class instance
                create_call_statement("REF", &base_type_name, None, self.provider().clone(), &location)
                    .with_metadata(old_node.into())
            }
        };

        std::mem::swap(node, &mut new_node);
        let resolver = super::LoweringResolver::new(self.index.unwrap(), self.provider())
            .with_pou(self.ctx.pou.as_deref().unwrap_or_default());
        self.annotations.as_mut().unwrap().import(resolver.resolve_statement(node));
    }
}
