use crate::{
    index::{get_init_fn_name, Index, VariableIndexEntry},
    resolver::{const_evaluator::UnresolvableConstant, AstAnnotations},
};
use initializers::{Init, InitAssignments, Initializers, GLOBAL_SCOPE};
use plc_ast::{
    ast::{AstFactory, AstNode, CompilationUnit, ConfigVariable, DataType, LinkageType, PouType},
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    visit_all_nodes_mut,
};
use plc_source::source_location::SourceLocation;

mod initializers;

pub struct AstLowerer {
    index: Index,
    annotations: AstAnnotations,
    unresolved_initializers: Initializers,
    var_config_initializers: Vec<AstNode>,
    units: Vec<CompilationUnit>,
    ctxt: LoweringContext,
}

impl AstLowerer {
    pub fn lower(
        mut units: Vec<CompilationUnit>,
        index: Index,
        annotations: AstAnnotations,
        unresolvables: Vec<UnresolvableConstant>,
        id_provider: IdProvider,
        init_symbol_name: &str,
    ) -> Vec<CompilationUnit> {
        let mut lowerer = Self::new(index, annotations, unresolvables, id_provider);
        // visit all units
        units.iter_mut().for_each(|unit| {
            lowerer.visit_compilation_unit(unit);
        });

        let lowered = lowerer.with_units(units).lower_init_functions(init_symbol_name);

        lowered.units
    }

    fn new(
        index: Index,
        annotations: AstAnnotations,
        unresolved_initializers: Vec<UnresolvableConstant>,
        id_provider: IdProvider,
    ) -> Self {
        Self {
            index,
            annotations,
            unresolved_initializers: Initializers::new(&unresolved_initializers),
            var_config_initializers: vec![],
            units: vec![],
            ctxt: LoweringContext::new(id_provider),
        }
    }

    fn with_units(self, units: Vec<CompilationUnit>) -> Self {
        Self {
            index: self.index,
            annotations: self.annotations,
            unresolved_initializers: self.unresolved_initializers,
            var_config_initializers: self.var_config_initializers,
            units,
            ctxt: self.ctxt,
        }
    }

    fn walk_with_scope<T>(&mut self, t: &mut T, pou_name: Option<impl Into<String>>)
    where
        T: WalkerMut,
    {
        let old = self.ctxt.scope(pou_name.map(Into::into));
        t.walk(self);
        self.ctxt.scope(old);
    }

    fn collect_alias_and_reference_to_inits(&mut self, variable: &mut plc_ast::ast::Variable) {
        if let Some(initializer) = variable.initializer.as_ref() {
            let Some(variable_ty) = variable
                .data_type_declaration
                .get_referenced_type()
                .and_then(|it| self.index.find_effective_type_by_name(&it))
            else {
                return variable.walk(self);
            };

            let variable_is_auto_deref_pointer = {
                variable_ty.get_type_information().is_alias()
                    || variable_ty.get_type_information().is_reference_to()
            };

            if variable_is_auto_deref_pointer {
                self.unresolved_initializers.insert_initializer(
                    self.ctxt.get_scope().as_ref().map(|it| it.as_str()).unwrap_or(GLOBAL_SCOPE),
                    Some(&variable.name),
                    &Some(initializer.clone()),
                );
            }
        }
    }

    fn add_init_statements(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        let predicate = |var: &VariableIndexEntry| {
            var.is_temp() || (implementation.pou_type == PouType::Function && var.is_local())
        };
        let strip_temporaries = |inits: &mut InitAssignments| {
            let mut temps = InitAssignments::default();
            let ids = inits
                .iter()
                .filter(|(id, _)| self.index.find_member(&implementation.name, id).is_some_and(predicate))
                .map(|(id, _)| id.to_owned())
                .collect::<Vec<_>>();

            for id in ids {
                if let Some(init) = inits.swap_remove(&id) {
                    temps.insert(id.to_owned(), init);
                };
            }

            temps
        };

        // remove all initializers for the current implementation/pou
        // XXX: this changes the order of init-statements in init-pous. but since assignments should still be ordered before call-statements,
        // this should be fine => edge cases?
        let Some(mut inits) = self.unresolved_initializers.swap_remove(&implementation.name) else {
            return;
        };

        // remove all non-stateful variable entries
        let temps = strip_temporaries(&mut inits);
        // re-enter the remaining initializers
        self.unresolved_initializers.insert(implementation.name.to_owned(), inits);

        // collect simple assignments
        let assignments = temps.into_iter().filter_map(|(lhs, init)| {
            init.as_ref().map(|it| {
                let func = if it.is_call() { create_assignment } else { create_ref_assignment };
                func(&lhs, None, it, self.ctxt.get_id_provider())
            })
        });

        // collect necessary call statements to init-functions
        let delegated_calls = self
            .index
            .get_pou_members(&implementation.name)
            .iter()
            .filter(|var| predicate(var))
            .filter_map(|var| {
                let dti =
                    self.index.get_effective_type_or_void_by_name(var.get_type_name()).get_type_information();
                if dti.is_struct() {
                    Some(create_call_statement(
                        &get_init_fn_name(dti.get_name()),
                        var.get_name(),
                        None,
                        self.ctxt.get_id_provider(),
                        &implementation.name_location,
                    ))
                } else {
                    None
                }
            });

        let stmts = assignments
            .chain(delegated_calls)
            .chain(std::mem::take(&mut implementation.statements))
            .collect::<Vec<_>>();
        implementation.statements = stmts;
    }

    /// Updates the scope and initialized variable for struct types. Adds entries for each encountered struct
    /// (this includes POU-structs, i.e. programs, ...) to the initializer map if no entry is present
    fn update_struct_initializers(&mut self, user_type: &mut plc_ast::ast::UserTypeDeclaration) {
        let effective_type =
            user_type.data_type.get_name().and_then(|it| self.index.find_effective_type_by_name(it));
        if let DataType::StructType { .. } = &user_type.data_type {
            let Some(ty) = effective_type else {
                return user_type.walk(self);
            };
            let name = ty.get_name();

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
            // add container to keys if not already present
            self.unresolved_initializers.maybe_insert_initializer(name, None, &user_type.initializer);
        }
    }

    fn maybe_add_global_instance_initializer(&mut self, variable: &plc_ast::ast::Variable) {
        let Some(global) = self.index.find_global_variable(variable.get_name()) else {
            return;
        };

        let info =
            self.index.get_effective_type_or_void_by_name(global.get_type_name()).get_type_information();

        if !info.is_struct()
            || self
                .index
                .find_pou(info.get_name())
                .is_some_and(|it| it.get_linkage() == &LinkageType::External)
        {
            return;
        }

        self.unresolved_initializers.maybe_insert_initializer(GLOBAL_SCOPE, Some(variable.get_name()), &None);
    }

    fn collect_var_config_assignments(&mut self, var_config: &[ConfigVariable]) {
        var_config.iter().for_each(|var| {
            if let Some(lhs) = var.name_segments.iter().fold(None, |qualifier, ident| {
                Some(create_member_reference(&ident, self.ctxt.get_id_provider(), qualifier))
            }) {
                self.var_config_initializers.push(AstFactory::create_assignment(
                    lhs,
                    var.address.clone(),
                    self.ctxt.next_id(),
                ));
            }
        });
    }
}

impl AstVisitorMut for AstLowerer {
    fn visit(&mut self, node: &mut plc_ast::ast::AstNode) {
        node.walk(self)
    }

    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        self.collect_var_config_assignments(&mut unit.var_config);
        unit.walk(self)
    }

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        self.add_init_statements(implementation);
        implementation.walk(self);
    }

    fn visit_variable_block(&mut self, block: &mut plc_ast::ast::VariableBlock) {
        block.walk(self)
    }

    fn visit_variable(&mut self, variable: &mut plc_ast::ast::Variable) {
        self.maybe_add_global_instance_initializer(variable);
        self.collect_alias_and_reference_to_inits(variable);
        variable.walk(self);
    }

    fn visit_enum_element(&mut self, element: &mut plc_ast::ast::AstNode) {
        element.walk(self);
    }

    fn visit_data_type_declaration(&mut self, data_type_declaration: &mut plc_ast::ast::DataTypeDeclaration) {
        data_type_declaration.walk(self);
    }

    fn visit_user_type_declaration(&mut self, user_type: &mut plc_ast::ast::UserTypeDeclaration) {
        self.update_struct_initializers(user_type);
        user_type.walk(self);
    }

    fn visit_data_type(&mut self, data_type: &mut DataType) {
        if matches!(data_type, plc_ast::ast::DataType::StructType { .. }) {
            self.walk_with_scope(data_type, data_type.get_name().map(ToOwned::to_owned))
        } else {
            data_type.walk(self)
        }
    }

    fn visit_pou(&mut self, pou: &mut plc_ast::ast::Pou) {
        if !matches!(pou.linkage, LinkageType::External | LinkageType::BuiltIn) {
            self.unresolved_initializers.maybe_insert_initializer(&pou.name, None, &None);
        }

        self.walk_with_scope(pou, Some(&pou.name.to_owned()));
    }

    fn visit_empty_statement(
        &mut self,
        _stmt: &mut plc_ast::ast::EmptyStatement,
        _node: &mut plc_ast::ast::AstNode,
    ) {
    }

    fn visit_default_value(
        &mut self,
        _stmt: &mut plc_ast::ast::DefaultValue,
        _node: &mut plc_ast::ast::AstNode,
    ) {
    }

    fn visit_literal(&mut self, stmt: &mut plc_ast::literals::AstLiteral, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_multiplied_statement(
        &mut self,
        stmt: &mut plc_ast::ast::MultipliedStatement,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_reference_expr(
        &mut self,
        stmt: &mut plc_ast::ast::ReferenceExpr,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_identifier(&mut self, _stmt: &mut str, _node: &mut plc_ast::ast::AstNode) {}

    fn visit_direct_access(
        &mut self,
        stmt: &mut plc_ast::ast::DirectAccess,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_hardware_access(
        &mut self,
        stmt: &mut plc_ast::ast::HardwareAccess,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_binary_expression(
        &mut self,
        stmt: &mut plc_ast::ast::BinaryExpression,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_unary_expression(
        &mut self,
        stmt: &mut plc_ast::ast::UnaryExpression,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_expression_list(
        &mut self,
        stmt: &mut Vec<plc_ast::ast::AstNode>,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        visit_all_nodes_mut!(self, stmt);
    }

    fn visit_paren_expression(
        &mut self,
        inner: &mut plc_ast::ast::AstNode,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        inner.walk(self)
    }

    fn visit_range_statement(
        &mut self,
        stmt: &mut plc_ast::ast::RangeStatement,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_vla_range_statement(&mut self, _node: &mut plc_ast::ast::AstNode) {}

    fn visit_assignment(&mut self, stmt: &mut plc_ast::ast::Assignment, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_output_assignment(
        &mut self,
        stmt: &mut plc_ast::ast::Assignment,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_ref_assignment(
        &mut self,
        stmt: &mut plc_ast::ast::Assignment,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_call_statement(
        &mut self,
        stmt: &mut plc_ast::ast::CallStatement,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_control_statement(
        &mut self,
        stmt: &mut plc_ast::control_statements::AstControlStatement,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_case_condition(&mut self, child: &mut plc_ast::ast::AstNode, _node: &mut plc_ast::ast::AstNode) {
        child.walk(self)
    }

    fn visit_exit_statement(&mut self, _node: &mut plc_ast::ast::AstNode) {}

    fn visit_continue_statement(&mut self, _node: &mut plc_ast::ast::AstNode) {}

    fn visit_return_statement(
        &mut self,
        stmt: &mut plc_ast::control_statements::ReturnStatement,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_jump_statement(
        &mut self,
        stmt: &mut plc_ast::ast::JumpStatement,
        _node: &mut plc_ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_label_statement(
        &mut self,
        _stmt: &mut plc_ast::ast::LabelStatement,
        _node: &mut plc_ast::ast::AstNode,
    ) {
    }
}

#[derive(Clone, Default)]
pub struct LoweringContext {
    /// optional context for references (e.g. `x` may mean `POU.x` if used inside `POU` body or `STRUCT.x` if `x` is a member of `STRUCT`)
    scope: Option<String>,

    pub id_provider: IdProvider,
}

// TODO: use &str with lifetimes, requires loads of changes to the visitor/walker traits
impl LoweringContext {
    fn new(id_provider: IdProvider) -> Self {
        Self { scope: None, id_provider }
    }

    /// updates the context's scope and returns the previous value
    fn scope(&mut self, pou: Option<String>) -> Option<String> {
        std::mem::replace(&mut self.scope, pou)
    }

    fn get_scope(&self) -> &Option<String> {
        &self.scope
    }

    fn get_id_provider(&self) -> IdProvider {
        self.id_provider.clone()
    }

    fn next_id(&mut self) -> usize {
        self.id_provider.next_id()
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

fn create_assignment(
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

fn create_call_statement(
    operator: &str,
    member_id: &str,
    base_id: Option<&str>,
    mut id_provider: IdProvider,
    location: &SourceLocation,
) -> AstNode {
    let op = create_member_reference(operator, id_provider.clone(), None);
    let param = create_member_reference(
        member_id,
        id_provider.clone(),
        base_id.map(|it| create_member_reference(it, id_provider.clone(), None)),
    );
    AstFactory::create_call_statement(op, Some(param), id_provider.next_id(), location.clone())
}
