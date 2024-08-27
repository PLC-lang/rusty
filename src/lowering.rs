use crate::{
    index::{get_init_fn_name, Index, VariableIndexEntry},
    resolver::const_evaluator::UnresolvableConstant,
};
use initializers::{Init, Initializers, GLOBAL_SCOPE};
use plc_ast::{
    ast::{AstFactory, AstNode, CompilationUnit, DataType, LinkageType, PouType},
    mut_visitor::{AstVisitorMut, VisitorContext, WalkerMut},
    provider::IdProvider,
    visit_all_nodes_mut,
};
use plc_source::source_location::SourceLocation;

mod initializers;

pub struct AstLowerer {
    index: Index,
    unresolved_initializers: Initializers,
    units: Vec<CompilationUnit>,
}

impl AstLowerer {
    pub fn lower(
        index: Index,
        mut units: Vec<CompilationUnit>,
        unresolvables: Vec<UnresolvableConstant>,
        id_provider: IdProvider,
        init_symbol_name: &str,
    ) -> Vec<CompilationUnit> {
        let mut lowerer = Self::new(index, unresolvables);
        // visit all units
        let ctxt = LoweringContext::new(id_provider.clone());
        units.iter_mut().for_each(|unit| {
            lowerer.visit_compilation_unit(unit, &ctxt);
        });

        let lowered = lowerer.with_units(units).lower_init_functions(init_symbol_name, &ctxt);

        lowered.units
    }

    fn new(index: Index, unresolved_initializers: Vec<UnresolvableConstant>) -> Self {
        Self { index, unresolved_initializers: Initializers::new(&unresolved_initializers), units: vec![] }
    }

    fn with_units(self, units: Vec<CompilationUnit>) -> Self {
        Self { index: self.index, unresolved_initializers: self.unresolved_initializers, units }
    }
}

impl AstVisitorMut for AstLowerer {
    fn visit<T: plc_ast::mut_visitor::VisitorContext>(&mut self, node: &mut plc_ast::ast::AstNode, ctxt: &T) {
        node.walk(self, ctxt)
    }

    fn visit_compilation_unit<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        unit: &mut CompilationUnit,
        ctxt: &T,
    ) {
        unit.walk(self, ctxt)
    }

    fn visit_implementation<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        implementation: &mut plc_ast::ast::Implementation,
        ctxt: &T,
    ) {
        let predicate = |var: &VariableIndexEntry| {
            var.is_temp() || (implementation.pou_type == PouType::Function && var.is_local())
        };
        // get unresolved inits
        if let Some(mut stmts) = self.unresolved_initializers.get(&implementation.name).map(|it| {
            let assignments = it
                .iter()
                .filter(|(id, _)| {
                    self.index.find_member(&implementation.name, id).is_some_and(|it| predicate(it))
                })
                .filter_map(|(lhs, init)| {
                    init.as_ref().map(|it| create_ref_assignment(lhs, None, &it, ctxt.get_id_provider()))
                });

            let delegated_calls = self
                .index
                .get_pou_members(&implementation.name)
                .iter()
                .filter(|it| predicate(it))
                .filter_map(|it| {
                    let dti = self.index.get_type_information_or_void(it.get_type_name());
                    if dti.is_struct() {
                        Some(create_call_statement(
                            &get_init_fn_name(&dti.get_name()),
                            it.get_name(),
                            None,
                            ctxt.get_id_provider(),
                            &implementation.name_location,
                        ))
                    } else {
                        None
                    }
                });

            assignments.chain(delegated_calls).collect::<Vec<_>>()
        }) {
            stmts.extend(std::mem::take(&mut implementation.statements));
            implementation.statements = stmts;
        }
        dbg!(&implementation);
        implementation.walk(self, ctxt);
    }

    fn visit_variable_block<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        block: &mut plc_ast::ast::VariableBlock,
        ctxt: &T,
    ) {
        block.walk(self, ctxt)
    }

    fn visit_variable<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        variable: &mut plc_ast::ast::Variable,
        ctxt: &T,
    ) {
        if let Some(initializer) = variable.initializer.as_ref() {
            let Some(variable_ty) = variable
                .data_type_declaration
                .get_referenced_type()
                .and_then(|it| self.index.find_effective_type_by_name(&it))
            else {
                return variable.walk(self, ctxt);
            };

            let variable_is_auto_deref_pointer = {
                variable_ty.get_type_information().is_alias()
                    || variable_ty.get_type_information().is_reference_to()
            };

            if variable_is_auto_deref_pointer {
                self.unresolved_initializers.insert_initializer(
                    ctxt.get_pou()
                        .as_ref()
                        .or(ctxt.get_qualifier().as_ref())
                        .unwrap_or(&GLOBAL_SCOPE.to_owned()),
                    Some(&variable.name),
                    &Some(initializer.clone()),
                );
            }
        }
        variable.walk(self, ctxt);
    }

    fn visit_enum_element<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        element: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        element.walk(self, ctxt);
    }

    fn visit_data_type_declaration<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        data_type_declaration: &mut plc_ast::ast::DataTypeDeclaration,
        ctxt: &T,
    ) {
        data_type_declaration.walk(self, ctxt);
    }

    fn visit_user_type_declaration<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        user_type: &mut plc_ast::ast::UserTypeDeclaration,
        ctxt: &T,
    ) {
        if let DataType::StructType { name, .. } = &user_type.data_type {
            let Some(name) = name else {
                return user_type.walk(self, ctxt);
            };

            let member_inits = self
                .index
                .get_container_members(name)
                .iter()
                .filter_map(|var| {
                    // struct member initializers don't have a qualifier/scope while evaluated in `const_evaluator.rs` and are registered as globals under their data-type name;
                    // look for member initializers for this struct in the global initializers, remove them and add new entries with the correct qualifier and left-hand-side
                    self.unresolved_initializers
                        .get_mut(GLOBAL_SCOPE)
                        .and_then(|it| it.swap_remove(var.get_type_name()))
                        .map(|node| (var.get_name(), node))
                })
                .collect::<Vec<_>>();

            for (lhs, init) in member_inits {
                // update struct member initializers
                self.unresolved_initializers.maybe_insert_initializer(name, Some(lhs), &init);
            }
            // add struct-type initializer
            self.unresolved_initializers.maybe_insert_initializer(name, None, &user_type.initializer);
        }
        user_type.walk(self, ctxt);
    }

    fn visit_data_type<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        data_type: &mut DataType,
        ctxt: &T,
    ) {
        let ctxt = if let plc_ast::ast::DataType::StructType { name, .. } = data_type {
            if let Some(it) = name.as_ref() {
                &ctxt.with_qualifier(it)
            } else {
                ctxt
            }
        } else {
            ctxt // yuck
        };

        data_type.walk(self, ctxt);
    }

    fn visit_pou<T: plc_ast::mut_visitor::VisitorContext>(&mut self, pou: &mut plc_ast::ast::Pou, ctxt: &T) {
        if !matches!(pou.linkage, LinkageType::External | LinkageType::BuiltIn) {
            self.unresolved_initializers.maybe_insert_initializer(&pou.name, None, &None);
        }

        pou.walk(self, &ctxt.with_pou(&pou.name));
    }

    fn visit_empty_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        _stmt: &mut plc_ast::ast::EmptyStatement,
        _node: &mut plc_ast::ast::AstNode,
        _ctxt: &T,
    ) {
    }

    fn visit_default_value<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        _stmt: &mut plc_ast::ast::DefaultValue,
        _node: &mut plc_ast::ast::AstNode,
        _ctxt: &T,
    ) {
    }

    fn visit_literal<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::literals::AstLiteral,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_multiplied_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::MultipliedStatement,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_reference_expr<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::ReferenceExpr,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_identifier<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        _stmt: &mut str,
        _node: &mut plc_ast::ast::AstNode,
        _ctxt: &T,
    ) {
    }

    fn visit_direct_access<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::DirectAccess,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_hardware_access<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::HardwareAccess,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_binary_expression<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::BinaryExpression,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_unary_expression<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::UnaryExpression,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_expression_list<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut Vec<plc_ast::ast::AstNode>,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        visit_all_nodes_mut!(self, stmt, ctxt);
    }

    fn visit_paren_expression<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        inner: &mut plc_ast::ast::AstNode,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        inner.walk(self, ctxt)
    }

    fn visit_range_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::RangeStatement,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_vla_range_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        _node: &mut plc_ast::ast::AstNode,
        _ctxt: &T,
    ) {
    }

    fn visit_assignment<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::Assignment,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_output_assignment<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::Assignment,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_ref_assignment<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::Assignment,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_call_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::CallStatement,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_control_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::control_statements::AstControlStatement,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_case_condition<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        child: &mut plc_ast::ast::AstNode,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        child.walk(self, ctxt)
    }

    fn visit_exit_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        _node: &mut plc_ast::ast::AstNode,
        _ctxt: &T,
    ) {
    }

    fn visit_continue_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        _node: &mut plc_ast::ast::AstNode,
        _ctxt: &T,
    ) {
    }

    fn visit_return_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::control_statements::ReturnStatement,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_jump_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        stmt: &mut plc_ast::ast::JumpStatement,
        _node: &mut plc_ast::ast::AstNode,
        ctxt: &T,
    ) {
        stmt.walk(self, ctxt)
    }

    fn visit_label_statement<T: plc_ast::mut_visitor::VisitorContext>(
        &mut self,
        _stmt: &mut plc_ast::ast::LabelStatement,
        _node: &mut plc_ast::ast::AstNode,
        _ctxt: &T,
    ) {
    }
}

#[derive(Clone, Default)]
pub struct LoweringContext {
    /// the type_name of the context for a reference (e.g. `a.b` where `a`'lwr type is the context of `b`)
    qualifier: Option<String>,
    /// optional context for references (e.g. `x` may mean `POU.x` if used inside `POU`'lwr body)
    pou: Option<String>,

    pub id_provider: IdProvider,
}

// TODO: use &str with lifetimes, requires loads of changes to the visitor/walker traits
impl VisitorContext for LoweringContext {
    fn new(id_provider: IdProvider) -> Self {
        Self { qualifier: None, pou: None, id_provider }
    }

    /// returns a copy of the current context and changes the `current_qualifier` to the given qualifier
    fn with_qualifier(&self, qualifier: &str) -> Self {
        let mut ctx = self.clone();
        ctx.qualifier = Some(qualifier.into());
        ctx
    }

    /// returns a copy of the current context and changes the `current_pou` to the given pou   
    fn with_pou(&self, pou: &str) -> Self {
        let mut ctx = self.clone();
        ctx.pou = Some(pou.into());
        ctx
    }

    fn get_qualifier(&self) -> &Option<String> {
        &self.qualifier
    }

    fn get_pou(&self) -> &Option<String> {
        &self.pou
    }

    fn get_id_provider(&self) -> IdProvider {
        self.id_provider.clone()
    }
}

fn create_member_reference(ident: &str, mut id_provider: IdProvider, base: Option<AstNode>) -> AstNode {
    AstFactory::create_member_reference(
        AstFactory::create_identifier(ident, SourceLocation::internal(), id_provider.next_id()),
        base,
        id_provider.next_id(),
    )
}

fn create_assignment_if_necessary(
    lhs_ident: &str,
    base_ident: Option<&str>,
    rhs: &Option<AstNode>,
    mut id_provider: IdProvider,
) -> Option<AstNode> {
    let lhs = create_member_reference(
        lhs_ident,
        id_provider.clone(),
        base_ident.map(|id| create_member_reference(id, id_provider.clone(), None)),
    );
    rhs.as_ref().map(|node| AstFactory::create_assignment(lhs, node.to_owned(), id_provider.next_id()))
}

fn create_ref_assignment(
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

fn create_call_statement(
    operator: &str,
    member_id: &str,
    base_id: Option<&str>,
    mut id_provider: IdProvider,
    location: &SourceLocation,
) -> AstNode {
    let op = create_member_reference(&operator, id_provider.clone(), None);
    let param = create_member_reference(
        member_id,
        id_provider.clone(),
        base_id.map(|it| create_member_reference(it, id_provider.clone(), None)),
    );
    AstFactory::create_call_statement(op, Some(param), id_provider.next_id(), location.clone())
}
