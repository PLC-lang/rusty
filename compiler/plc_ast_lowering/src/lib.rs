use ast::{
    ast::{CompilationUnit, DataType, LinkageType},
    provider::IdProvider,
    visit_all_nodes,
    visitor::{AstVisitor, Walker},
};
use initializers::{Init, Initializers};
use plc::{
    builtins, index::{FxIndexSet, Index}, resolver::{const_evaluator::UnresolvableConstant, AnnotationMap, AnnotationMapImpl, Dependency, StringLiterals}
};

mod initializers;

pub struct AstLowerer<'lwr> {
    index: Index,
    annotation_map: AnnotationMapImpl,
    unresolved_initializers: Initializers,
    units: Vec<(CompilationUnit, FxIndexSet<Dependency>, StringLiterals)>,
    ctx: LoweringContext<'lwr>
}

impl<'lwr> AstLowerer<'lwr>{
    pub fn lower(
        index: Index,
        annotations: AnnotationMapImpl,
        annotated_units: Vec<(CompilationUnit, FxIndexSet<Dependency>, StringLiterals)>,
        unresolvables: Vec<UnresolvableConstant>,
        id_provider: IdProvider,
        init_symbol_name: &str,
    ) -> (Vec<(CompilationUnit, FxIndexSet<Dependency>, StringLiterals)>, Index, AnnotationMapImpl) {
        let mut lowerer = Self::new(index, annotations, unresolvables, id_provider);
        
        // visit all units
        for unit in annotated_units.iter().map(|(unit, _, _)| unit) {
            lowerer.visit_compilation_unit(unit)
        }

        let lowered = lowerer.with_units(annotated_units).lower_init_functions(init_symbol_name);

        (lowered.units, lowered.index, lowered.annotation_map)
    }

    fn new(
        index: Index,
        annotation_map: AnnotationMapImpl,
        unresolved_initializers: Vec<UnresolvableConstant>,
        id_provider: IdProvider
    ) -> Self {
        Self {
            index,
            annotation_map,
            unresolved_initializers: Initializers::new(&unresolved_initializers),
            units: vec![],
            ctx: LoweringContext::new(id_provider)
        }
    }

    fn with_units(self, units: Vec<(CompilationUnit, FxIndexSet<Dependency>, StringLiterals)>) -> Self {
        Self { index: self.index, annotation_map: self.annotation_map, unresolved_initializers: self.unresolved_initializers, units, ctx: self.ctx }
    }
}

impl<'lwr> AstVisitor for AstLowerer<'lwr> {
    fn visit(&mut self, node: &ast::ast::AstNode) {
        node.walk(self)
    }

    fn visit_compilation_unit(&mut self, unit: &ast::ast::CompilationUnit) {
        unit.walk(self)
    }

    fn visit_implementation(&mut self, implementation: &ast::ast::Implementation) {
        implementation.walk(self);
    }

    fn visit_variable_block(&mut self, block: &ast::ast::VariableBlock) {
        block.walk(self)
    }

    fn visit_variable(&mut self, variable: &ast::ast::Variable) {
        if let Some(initializer) = variable.initializer.as_ref() {
            let Some(variable_ty) = variable.data_type_declaration.get_referenced_type().and_then(|it| self.index.find_effective_type_by_name(&it)) else {
                return variable.walk(self);
            };

            let variable_is_auto_deref_pointer = {
                variable_ty.get_type_information().is_alias() || variable_ty.get_type_information().is_reference_to()
            };
        
            let initializer_is_not_wrapped_in_ref_call = {
                !(initializer.is_call()
                    && self.annotation_map.get_type(initializer, &self.index).is_some_and(|opt| opt.is_pointer()))
            };
        
            if variable_is_auto_deref_pointer && initializer_is_not_wrapped_in_ref_call {
                debug_assert!(builtins::get_builtin("REF").is_some(), "REF must exist for this use-case");
        
                self.unresolved_initializers.maybe_insert_initializer(
                    self.ctx.pou.or(self.ctx.qualifier.as_deref()).as_ref().unwrap_or(&"__global"),
                    Some(&variable.name),
                    &Some(initializer.clone()),
                );
            }
        }
        variable.walk(self);
    }

    fn visit_enum_element(&mut self, element: &ast::ast::AstNode) {
        element.walk(self);
    }

    fn visit_data_type_declaration(&mut self, data_type_declaration: &ast::ast::DataTypeDeclaration) {
        data_type_declaration.walk(self);
    }

    fn visit_user_type_declaration(&mut self, user_type: &ast::ast::UserTypeDeclaration) {
        if let DataType::StructType { name, .. } = &user_type.data_type {
            let Some(name) = name else {
                return user_type.walk(self);
            };

            let member_inits = self.index
                .get_container_members(name)
                .iter()
                .filter_map(|var| {
                    // struct member initializers don't have a qualifier/scope while evaluated in `const_evaluator.rs` and are registered as globals under their data-type name;
                    // look for member initializers for this struct in the global initializers, remove them and add new entries with the correct qualifier and left-hand-side
                    self.unresolved_initializers
                        .get_mut("__global")
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
        user_type.walk(self);
    }

    fn visit_data_type(&mut self, data_type: &ast::ast::DataType) {
        if let ast::ast::DataType::StructType { name, .. } = data_type {
            name.as_ref().map(|it| self.ctx.with_qualifier(it.to_owned()));
        };
        data_type.walk(self);
    }

    fn visit_pou(&mut self, pou: &ast::ast::Pou) {
        if !matches!(pou.linkage, LinkageType::External | LinkageType::BuiltIn) {
            self.unresolved_initializers.maybe_insert_initializer(&pou.name, None, &None);
        }
        self.ctx.with_pou(pou.name.to_owned());
        pou.walk(self);
    }

    fn visit_empty_statement(&mut self, _stmt: &ast::ast::EmptyStatement, _node: &ast::ast::AstNode) {}

    fn visit_default_value(&mut self, _stmt: &ast::ast::DefaultValue, _node: &ast::ast::AstNode) {}

    fn visit_literal(&mut self, stmt: &ast::literals::AstLiteral, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_multiplied_statement(
        &mut self,
        stmt: &ast::ast::MultipliedStatement,
        _node: &ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_reference_expr(&mut self, stmt: &ast::ast::ReferenceExpr, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_identifier(&mut self, _stmt: &str, _node: &ast::ast::AstNode) {}

    fn visit_direct_access(&mut self, stmt: &ast::ast::DirectAccess, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_hardware_access(&mut self, stmt: &ast::ast::HardwareAccess, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_binary_expression(&mut self, stmt: &ast::ast::BinaryExpression, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_unary_expression(&mut self, stmt: &ast::ast::UnaryExpression, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_expression_list(&mut self, stmt: &Vec<ast::ast::AstNode>, _node: &ast::ast::AstNode) {
        visit_all_nodes!(self, stmt);
    }

    fn visit_paren_expression(&mut self, inner: &ast::ast::AstNode, _node: &ast::ast::AstNode) {
        inner.walk(self)
    }

    fn visit_range_statement(&mut self, stmt: &ast::ast::RangeStatement, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_vla_range_statement(&mut self, _node: &ast::ast::AstNode) {}

    fn visit_assignment(&mut self, stmt: &ast::ast::Assignment, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_output_assignment(&mut self, stmt: &ast::ast::Assignment, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_ref_assignment(&mut self, stmt: &ast::ast::Assignment, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_call_statement(&mut self, stmt: &ast::ast::CallStatement, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_control_statement(
        &mut self,
        stmt: &ast::control_statements::AstControlStatement,
        _node: &ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_case_condition(&mut self, child: &ast::ast::AstNode, _node: &ast::ast::AstNode) {
        child.walk(self)
    }

    fn visit_exit_statement(&mut self, _node: &ast::ast::AstNode) {}

    fn visit_continue_statement(&mut self, _node: &ast::ast::AstNode) {}

    fn visit_return_statement(
        &mut self,
        stmt: &ast::control_statements::ReturnStatement,
        _node: &ast::ast::AstNode,
    ) {
        stmt.walk(self)
    }

    fn visit_jump_statement(&mut self, stmt: &ast::ast::JumpStatement, _node: &ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_label_statement(&mut self, _stmt: &ast::ast::LabelStatement, _node: &ast::ast::AstNode) {}
}

#[derive(Clone, Default)]
pub struct LoweringContext<'lwr> {
    /// the type_name of the context for a reference (e.g. `a.b` where `a`'lwr type is the context of `b`)
    qualifier: Option<String>,
    /// optional context for references (e.g. `x` may mean `POU.x` if used inside `POU`'lwr body)
    pou: Option<&'lwr str>,

    pub id_provider: IdProvider,
}

impl<'lwr> LoweringContext<'lwr> {
    /// returns a copy of the current context and changes the `current_qualifier` to the given qualifier
    fn with_qualifier(&mut self, qualifier: String) {
        let mut ctx = self.clone();
        ctx.qualifier = Some(qualifier);
    }

    /// returns a copy of the current context and changes the `current_pou` to the given pou
    fn with_pou(&mut self, pou: String) {
        let mut ctx = self.clone();
        ctx.pou = Some(&pou);
    }

    fn new(id_provider: IdProvider) -> Self {
        let mut ctx = Self::default();
        ctx.id_provider = id_provider;
        ctx
    }
}