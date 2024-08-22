use crate::{
    builtins,
    index::{FxIndexSet, Index},
    resolver::{
        const_evaluator::UnresolvableConstant, AnnotationMap, AnnotationMapImpl, Dependency, StringLiterals,
    },
};
use initializers::{Init, Initializers, GLOBAL_SCOPE};
use plc_ast::{
    ast::{CompilationUnit, DataType, LinkageType},
    mut_visitor::{AstVisitorMut, VisitorContext, WalkerMut},
    provider::IdProvider,
    visit_all_nodes_mut,
};

mod initializers;

pub struct AstLowerer {
    index: Index,
    annotation_map: AnnotationMapImpl,
    unresolved_initializers: Initializers,
    units: Vec<CompilationUnit>,
}

impl AstLowerer {
    pub fn lower(
        index: Index,
        annotations: AnnotationMapImpl,
        annotated_units: Vec<(CompilationUnit, FxIndexSet<Dependency>, StringLiterals)>,
        unresolvables: Vec<UnresolvableConstant>,
        id_provider: IdProvider,
        init_symbol_name: &str,
    ) -> Vec<CompilationUnit> {
        let mut lowerer = Self::new(index, annotations, unresolvables);
        // visit all units
        let ctxt = LoweringContext::new(id_provider.clone());
        let units = annotated_units
            .into_iter()
            .map(|(mut unit, _, _)| {
                lowerer.visit_compilation_unit(&mut unit, &ctxt);
                unit
            })
            .collect::<Vec<_>>();

        let lowered = lowerer.with_units(units).lower_init_functions(init_symbol_name, &ctxt);

        lowered.units
    }

    fn new(
        index: Index,
        annotation_map: AnnotationMapImpl,
        unresolved_initializers: Vec<UnresolvableConstant>,
    ) -> Self {
        Self {
            index,
            annotation_map,
            unresolved_initializers: Initializers::new(&unresolved_initializers),
            units: vec![],
        }
    }

    fn with_units(self, units: Vec<CompilationUnit>) -> Self {
        Self {
            index: self.index,
            annotation_map: self.annotation_map,
            unresolved_initializers: self.unresolved_initializers,
            units,
        }
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
        // TODO: refactor alias/reference to lowering to be done right after parsing in a de-sugaring step
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

            let initializer_is_not_wrapped_in_ref_call = {
                !(initializer.is_call()
                    && self
                        .annotation_map
                        .get_type(initializer, &self.index)
                        .is_some_and(|opt| opt.is_pointer()))
            };

            if variable_is_auto_deref_pointer && initializer_is_not_wrapped_in_ref_call {
                debug_assert!(builtins::get_builtin("REF").is_some(), "REF must exist for this use-case");
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
                self.unresolved_initializers.maybe_insert_initializer(name, Some(lhs), &init);
            }
            // XXX: necessary?
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
            ctxt
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
}
