use crate::{
    builtins,
    index::{FxIndexSet, Index},
    resolver::{
        const_evaluator::UnresolvableConstant, AnnotationMap, AnnotationMapImpl, Dependency, StringLiterals,
    },
};
use initializers::{Init, Initializers, GLOBAL_SCOPE};
use plc_ast::{
    ast::{CompilationUnit, DataType, LinkageType}, mut_visitor::{AstVisitorMut, WalkerMut}, provider::IdProvider, visit_all_nodes,
};

mod initializers;

pub struct AstLowerer {
    index: Index,
    annotation_map: AnnotationMapImpl,
    unresolved_initializers: Initializers,
    units: Vec<(CompilationUnit, FxIndexSet<Dependency>, StringLiterals)>,
    ctx: LoweringContext,
}

impl AstLowerer {
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
        let mut units = annotated_units;
        for unit in units.iter_mut().map(|(unit, _, _)| unit) {
            lowerer.visit_compilation_unit(unit)
        }

        let lowered = lowerer.with_units(units).lower_init_functions(init_symbol_name);
        dbg!(&lowered.annotation_map);

        (lowered.units, lowered.index, lowered.annotation_map)
    }

    fn new(
        index: Index,
        annotation_map: AnnotationMapImpl,
        unresolved_initializers: Vec<UnresolvableConstant>,
        id_provider: IdProvider,
    ) -> Self {
        Self {
            index,
            annotation_map,
            unresolved_initializers: Initializers::new(&unresolved_initializers),
            units: vec![],
            ctx: LoweringContext::new(id_provider),
        }
    }

    fn with_units(self, units: Vec<(CompilationUnit, FxIndexSet<Dependency>, StringLiterals)>) -> Self {
        Self {
            index: self.index,
            annotation_map: self.annotation_map,
            unresolved_initializers: self.unresolved_initializers,
            units,
            ctx: self.ctx,
        }
    }
}

impl AstVisitorMut for AstLowerer {
    fn visit(&mut self, node: &mut plc_ast::ast::AstNode) {
        node.walk(self)
    }

    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self)
    }

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        implementation.walk(self);
    }

    fn visit_variable_block(&mut self, block: &mut plc_ast::ast::VariableBlock) {
        block.walk(self)
    }

    fn visit_variable(&mut self, variable: &mut plc_ast::ast::Variable) {
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
                    &self
                        .ctx
                        .pou
                        .as_ref()
                        .or(self.ctx.qualifier.as_ref())
                        .unwrap_or(&GLOBAL_SCOPE.to_owned()),
                    Some(&variable.name),
                    &Some(initializer.clone()),
                );
            }
        }
        variable.walk(self);
    }

    fn visit_enum_element(&mut self, element: &mut plc_ast::ast::AstNode) {
        element.walk(self);
    }

    fn visit_data_type_declaration(&mut self, data_type_declaration: &mut plc_ast::ast::DataTypeDeclaration) {
        data_type_declaration.walk(self);
    }

    fn visit_user_type_declaration(&mut self, user_type: &mut plc_ast::ast::UserTypeDeclaration) {
        if let DataType::StructType { name, .. } = &user_type.data_type {
            let Some(name) = name else {
                return user_type.walk(self);
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
        user_type.walk(self);
    }

    fn visit_data_type(&mut self, data_type: &mut DataType) {
        if let plc_ast::ast::DataType::StructType { name, .. } = data_type {
            name.as_ref().map(|it| self.ctx.with_qualifier(it.to_owned()));
        };
        data_type.walk(self);
    }

    fn visit_pou(&mut self, pou: &mut plc_ast::ast::Pou) {
        if !matches!(pou.linkage, LinkageType::External | LinkageType::BuiltIn) {
            self.unresolved_initializers.maybe_insert_initializer(&pou.name, None, &None);
        }
        self.ctx.with_pou(pou.name.to_owned());
        pou.walk(self);
    }

    fn visit_empty_statement(&mut self, _stmt: &mut plc_ast::ast::EmptyStatement, _node: &mut plc_ast::ast::AstNode) {}

    fn visit_default_value(&mut self, _stmt: &mut plc_ast::ast::DefaultValue, _node: &mut plc_ast::ast::AstNode) {}

    fn visit_literal(&mut self, stmt: &mut plc_ast::literals::AstLiteral, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_multiplied_statement(&mut self, stmt: &mut plc_ast::ast::MultipliedStatement, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_reference_expr(&mut self, stmt: &mut plc_ast::ast::ReferenceExpr, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_identifier(&mut self, _stmt: &mut str, _node: &mut plc_ast::ast::AstNode) {}

    fn visit_direct_access(&mut self, stmt: &mut plc_ast::ast::DirectAccess, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_hardware_access(&mut self, stmt: &mut plc_ast::ast::HardwareAccess, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_binary_expression(&mut self, stmt: &mut plc_ast::ast::BinaryExpression, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_unary_expression(&mut self, stmt: &mut plc_ast::ast::UnaryExpression, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_expression_list(&mut self, stmt: &mut Vec<plc_ast::ast::AstNode>, _node: &mut plc_ast::ast::AstNode) {
        visit_all_nodes!(self, stmt);
    }

    fn visit_paren_expression(&mut self, inner: &mut plc_ast::ast::AstNode, _node: &mut plc_ast::ast::AstNode) {
        inner.walk(self)
    }

    fn visit_range_statement(&mut self, stmt: &mut plc_ast::ast::RangeStatement, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_vla_range_statement(&mut self, _node: &mut plc_ast::ast::AstNode) {}

    fn visit_assignment(&mut self, stmt: &mut plc_ast::ast::Assignment, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_output_assignment(&mut self, stmt: &mut plc_ast::ast::Assignment, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_ref_assignment(&mut self, stmt: &mut plc_ast::ast::Assignment, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_call_statement(&mut self, stmt: &mut plc_ast::ast::CallStatement, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_control_statement(&mut self, stmt: &mut plc_ast::control_statements::AstControlStatement, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_case_condition(&mut self, child: &mut plc_ast::ast::AstNode, _node: &mut plc_ast::ast::AstNode) {
        child.walk(self)
    }

    fn visit_exit_statement(&mut self, _node: &mut plc_ast::ast::AstNode) {}

    fn visit_continue_statement(&mut self, _node: &mut plc_ast::ast::AstNode) {}

    fn visit_return_statement(&mut self, stmt: &mut plc_ast::control_statements::ReturnStatement, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_jump_statement(&mut self, stmt: &mut plc_ast::ast::JumpStatement, _node: &mut plc_ast::ast::AstNode) {
        stmt.walk(self)
    }

    fn visit_label_statement(&mut self, _stmt: &mut plc_ast::ast::LabelStatement, _node: &mut plc_ast::ast::AstNode) {}
}

// impl AstVisitor for AstLowerer {
//     fn visit(&mut self, node: &plc_ast::ast::AstNode) {
//         node.walk(self)
//     }

//     fn visit_compilation_unit(&mut self, unit: &plc_ast::ast::CompilationUnit) {
//         unit.walk(self)
//     }

//     fn visit_implementation(&mut self, implementation: &plc_ast::ast::Implementation) {
//         implementation.walk(self);
//     }

//     fn visit_variable_block(&mut self, block: &plc_ast::ast::VariableBlock) {
//         block.walk(self)
//     }

//     fn visit_variable(&mut self, variable: &plc_ast::ast::Variable) {
        
//         variable.walk(self);
//     }

//     fn visit_enum_element(&mut self, element: &plc_ast::ast::AstNode) {
//         element.walk(self);
//     }

//     fn visit_data_type_declaration(&mut self, data_type_declaration: &plc_ast::ast::DataTypeDeclaration) {
//         data_type_declaration.walk(self);
//     }

//     fn visit_user_type_declaration(&mut self, user_type: &plc_ast::ast::UserTypeDeclaration) {
        
//         user_type.walk(self);
//     }

//     fn visit_data_type(&mut self, data_type: &plc_ast::ast::DataType) {

//         data_type.walk(self);
//     }

//     fn visit_pou(&mut self, pou: &plc_ast::ast::Pou) {

//         pou.walk(self);
//     }

//     fn visit_empty_statement(&mut self, _stmt: &plc_ast::ast::EmptyStatement, _node: &plc_ast::ast::AstNode) {
//     }

//     fn visit_default_value(&mut self, _stmt: &plc_ast::ast::DefaultValue, _node: &plc_ast::ast::AstNode) {}

//     fn visit_literal(&mut self, stmt: &plc_ast::literals::AstLiteral, _node: &plc_ast::ast::AstNode) {
//         stmt.walk(self)
//     }

//     fn visit_multiplied_statement(
//         &mut self,
//         stmt: &plc_ast::ast::MultipliedStatement,
//         _node: &plc_ast::ast::AstNode,
//     ) {
//         stmt.walk(self)
//     }

//     fn visit_reference_expr(&mut self, stmt: &plc_ast::ast::ReferenceExpr, _node: &plc_ast::ast::AstNode) {
//         stmt.walk(self)
//     }

//     fn visit_identifier(&mut self, _stmt: &str, _node: &plc_ast::ast::AstNode) {}

//     fn visit_direct_access(&mut self, stmt: &plc_ast::ast::DirectAccess, _node: &plc_ast::ast::AstNode) {
//         stmt.walk(self)
//     }

//     fn visit_hardware_access(&mut self, stmt: &plc_ast::ast::HardwareAccess, _node: &plc_ast::ast::AstNode) {
//         stmt.walk(self)
//     }

//     fn visit_binary_expression(
//         &mut self,
//         stmt: &plc_ast::ast::BinaryExpression,
//         _node: &plc_ast::ast::AstNode,
//     ) {
//         stmt.walk(self)
//     }

//     fn visit_unary_expression(
//         &mut self,
//         stmt: &plc_ast::ast::UnaryExpression,
//         _node: &plc_ast::ast::AstNode,
//     ) {
//         stmt.walk(self)
//     }

//     fn visit_expression_list(&mut self, stmt: &Vec<plc_ast::ast::AstNode>, _node: &plc_ast::ast::AstNode) {
//         visit_all_nodes!(self, stmt);
//     }

//     fn visit_paren_expression(&mut self, inner: &plc_ast::ast::AstNode, _node: &plc_ast::ast::AstNode) {
//         inner.walk(self)
//     }

//     fn visit_range_statement(&mut self, stmt: &plc_ast::ast::RangeStatement, _node: &plc_ast::ast::AstNode) {
//         stmt.walk(self)
//     }

//     fn visit_vla_range_statement(&mut self, _node: &plc_ast::ast::AstNode) {}

//     fn visit_assignment(&mut self, stmt: &plc_ast::ast::Assignment, _node: &plc_ast::ast::AstNode) {
//         stmt.walk(self)
//     }

//     fn visit_output_assignment(&mut self, stmt: &plc_ast::ast::Assignment, _node: &plc_ast::ast::AstNode) {
//         stmt.walk(self)
//     }

//     fn visit_ref_assignment(&mut self, stmt: &plc_ast::ast::Assignment, _node: &plc_ast::ast::AstNode) {
//         stmt.walk(self)
//     }

//     fn visit_call_statement(&mut self, stmt: &plc_ast::ast::CallStatement, _node: &plc_ast::ast::AstNode) {
//         stmt.walk(self)
//     }

//     fn visit_control_statement(
//         &mut self,
//         stmt: &plc_ast::control_statements::AstControlStatement,
//         _node: &plc_ast::ast::AstNode,
//     ) {
//         stmt.walk(self)
//     }

//     fn visit_case_condition(&mut self, child: &plc_ast::ast::AstNode, _node: &plc_ast::ast::AstNode) {
//         child.walk(self)
//     }

//     fn visit_exit_statement(&mut self, _node: &plc_ast::ast::AstNode) {}

//     fn visit_continue_statement(&mut self, _node: &plc_ast::ast::AstNode) {}

//     fn visit_return_statement(
//         &mut self,
//         stmt: &plc_ast::control_statements::ReturnStatement,
//         _node: &plc_ast::ast::AstNode,
//     ) {
//         stmt.walk(self)
//     }

//     fn visit_jump_statement(&mut self, stmt: &plc_ast::ast::JumpStatement, _node: &plc_ast::ast::AstNode) {
//         stmt.walk(self)
//     }

//     fn visit_label_statement(&mut self, _stmt: &plc_ast::ast::LabelStatement, _node: &plc_ast::ast::AstNode) {
//     }
// }

#[derive(Clone, Default)]
pub struct LoweringContext {
    /// the type_name of the context for a reference (e.g. `a.b` where `a`'lwr type is the context of `b`)
    qualifier: Option<String>,
    /// optional context for references (e.g. `x` may mean `POU.x` if used inside `POU`'lwr body)
    pou: Option<String>,

    pub id_provider: IdProvider,
}

impl LoweringContext {
    /// returns a copy of the current context and changes the `current_qualifier` to the given qualifier
    fn with_qualifier(&mut self, qualifier: String) {
        self.qualifier = Some(qualifier);
    }

    /// returns a copy of the current context and changes the `current_pou` to the given pou
    fn with_pou(&mut self, pou: String) {
        self.pou = Some(pou);
    }

    fn new(id_provider: IdProvider) -> Self {
        let mut ctx = Self::default();
        ctx.id_provider = id_provider;
        ctx
    }
}
