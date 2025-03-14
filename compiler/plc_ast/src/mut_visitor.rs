//! This module defines the `AstVisitorMut` trait and its associated macros.
//! The `AstVisitorMut` trait provides a set of methods for mutably traversing and visiting ASTs

use std::borrow::BorrowMut;

use crate::ast::{
    flatten_expression_list, Assignment, AstNode, AstStatement, BinaryExpression, CallStatement,
    CompilationUnit, DataType, DataTypeDeclaration, DirectAccess, HardwareAccess, Implementation, Interface,
    JumpStatement, MultipliedStatement, Pou, PropertyBlock, RangeStatement, ReferenceAccess, ReferenceExpr,
    UnaryExpression, UserTypeDeclaration, Variable, VariableBlock,
};
use crate::control_statements::{AstControlStatement, ConditionalBlock, ReturnStatement};
use crate::literals::AstLiteral;
use crate::try_from_mut;

#[macro_export]
macro_rules! visit_all_nodes_mut {
    ($visitor:expr, $iter:expr) => {
        // Note: The `allow` is needed to suppress warnings about `while let Some(...)` warnings
        // because `visit_all_nodes_mut!` is used for both Option and Non-Option types
        #[allow(warnings)]
        {
            for node in $iter {
                $visitor.visit(node);
            }
        }
    };
}

/// Macro that calls the visitor's `visit` method for every AstNode in the passed sequence of nodes.
macro_rules! visit_nodes_mut {
    ($visitor:expr, $($node:expr),*) => {
        $(
            $visitor.visit($node);
        )*
    };
}

pub trait WalkerMut {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut;
}

pub trait AstVisitorMut: Sized {
    fn visit(&mut self, node: &mut AstNode) {
        node.walk(self)
    }

    //Takes ownership of the node, manipulates it and returns a new node
    fn map(&mut self, mut node: AstNode) -> AstNode {
        node.borrow_mut().walk(self);
        node
    }

    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self)
    }

    fn visit_implementation(&mut self, implementation: &mut Implementation) {
        implementation.walk(self);
    }

    fn visit_variable_block(&mut self, block: &mut VariableBlock) {
        block.walk(self)
    }

    fn visit_variable(&mut self, variable: &mut Variable) {
        variable.walk(self);
    }

    fn visit_enum_element(&mut self, element: &mut AstNode) {
        element.walk(self);
    }

    fn visit_data_type_declaration(&mut self, data_type_declaration: &mut DataTypeDeclaration) {
        data_type_declaration.walk(self);
    }

    fn visit_user_type_declaration(&mut self, user_type: &mut UserTypeDeclaration) {
        user_type.walk(self);
    }

    fn visit_data_type(&mut self, data_type: &mut DataType) {
        data_type.walk(self);
    }

    fn visit_pou(&mut self, pou: &mut Pou) {
        pou.walk(self);
    }

    fn visit_empty_statement(&mut self, _node: &mut AstNode) {}

    fn visit_default_value(&mut self, _node: &mut AstNode) {}

    fn visit_literal(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, AstLiteral).expect("Is a literal");
        stmt.walk(self)
    }

    fn visit_multiplied_statement(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, MultipliedStatement).expect("MultipliedStatement");
        stmt.walk(self)
    }

    fn visit_reference_expr(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, ReferenceExpr).expect("ReferenceExpr");
        stmt.walk(self)
    }

    fn visit_identifier(&mut self, _node: &mut AstNode) {}

    fn visit_direct_access(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, DirectAccess).expect("DirectAccess");
        stmt.walk(self)
    }

    fn visit_hardware_access(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, HardwareAccess).expect("HardwareAccess");
        stmt.walk(self)
    }

    fn visit_binary_expression(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, BinaryExpression).expect("BinaryExpression");
        stmt.walk(self)
    }

    fn visit_unary_expression(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, UnaryExpression).expect("UnaryExpression");
        stmt.walk(self)
    }

    fn visit_expression_list(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, Vec<AstNode>).expect("Vec<AstNode>");
        visit_all_nodes_mut!(self, stmt);
    }

    fn visit_paren_expression(&mut self, node: &mut AstNode) {
        let AstStatement::ParenExpression(inner) = node.get_stmt_mut() else {
            unreachable!("Must be ParenExpression");
        };
        inner.walk(self)
    }

    fn visit_range_statement(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, RangeStatement).expect("RangeStatement");
        stmt.walk(self)
    }

    fn visit_vla_range_statement(&mut self, _node: &mut AstNode) {}

    fn visit_assignment(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, Assignment).expect("Assignment");
        stmt.walk(self)
    }

    fn visit_output_assignment(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, Assignment).expect("Assignment");
        stmt.walk(self)
    }

    fn visit_ref_assignment(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, Assignment).expect("Assignment");
        stmt.walk(self)
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, CallStatement).expect("CallStatement");
        stmt.walk(self)
    }

    fn visit_control_statement(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, AstControlStatement).expect("AstControlStatement");
        stmt.walk(self)
    }

    fn visit_case_condition(&mut self, node: &mut AstNode) {
        let AstStatement::CaseCondition(child) = node.get_stmt_mut() else {
            unreachable!("CaseCondition");
        };
        child.walk(self)
    }

    fn visit_exit_statement(&mut self, _node: &mut AstNode) {}

    fn visit_continue_statement(&mut self, _node: &mut AstNode) {}

    fn visit_return_statement(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, ReturnStatement).expect("ReturnStatement");
        stmt.walk(self)
    }

    fn visit_jump_statement(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, JumpStatement).expect("CallStatement");
        stmt.walk(self)
    }

    /// Visits a `LabelStatement` node.
    /// # Arguments
    /// * `stmt` - The unwrapedyped `LabelStatement` node to visit.
    /// * `node` - The wrapped `AstNode` node to visit. Offers access to location information and AstId
    fn visit_label_statement(&mut self, _node: &mut AstNode) {}

    /// Visits a `Allocation` node.
    /// # Arguments
    /// * `stmt` - The unwrapedyped `Allocation` node to visit.
    /// * `node` - The wrapped `AstNode` node to visit. Offers access to location information and AstId
    fn visit_allocation(&mut self, _node: &mut AstNode) {}

    fn visit_interface(&mut self, _interface: &mut Interface) {}

    fn visit_property(&mut self, _property: &mut PropertyBlock) {}
}

impl WalkerMut for AstLiteral {
    fn walk<V>(&mut self, _visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        // do nothing
    }
}

impl WalkerMut for MultipliedStatement {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visitor.visit(&mut self.element)
    }
}

impl WalkerMut for ReferenceExpr {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        if let Some(base) = &mut self.base {
            visitor.visit(base);
        }

        match &mut self.access {
            ReferenceAccess::Member(t) | ReferenceAccess::Index(t) | ReferenceAccess::Cast(t) => {
                visitor.visit(t)
            }
            _ => {}
        }
    }
}

impl WalkerMut for DirectAccess {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes_mut!(visitor, &mut self.index);
    }
}

impl WalkerMut for HardwareAccess {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_all_nodes_mut!(visitor, &mut self.address);
    }
}

impl WalkerMut for BinaryExpression {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes_mut!(visitor, &mut self.left, &mut self.right);
    }
}

impl WalkerMut for UnaryExpression {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes_mut!(visitor, &mut self.value);
    }
}

impl WalkerMut for Assignment {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes_mut!(visitor, &mut self.left, &mut self.right);
    }
}

impl WalkerMut for RangeStatement {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes_mut!(visitor, &mut self.start, &mut self.end);
    }
}

impl WalkerMut for CallStatement {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes_mut!(visitor, &mut self.operator);
        if let Some(params) = &mut self.parameters {
            visit_nodes_mut!(visitor, params);
        }
    }
}

impl WalkerMut for Vec<ConditionalBlock> {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        for b in self {
            visit_nodes_mut!(visitor, &mut b.condition);
            visit_all_nodes_mut!(visitor, &mut b.body);
        }
    }
}

impl WalkerMut for AstControlStatement {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        match self {
            AstControlStatement::If(stmt) => {
                stmt.blocks.walk(visitor);
                visit_all_nodes_mut!(visitor, &mut stmt.else_block);
            }
            AstControlStatement::WhileLoop(stmt) | AstControlStatement::RepeatLoop(stmt) => {
                visit_nodes_mut!(visitor, &mut stmt.condition);
                visit_all_nodes_mut!(visitor, &mut stmt.body);
            }
            AstControlStatement::ForLoop(stmt) => {
                visit_nodes_mut!(visitor, &mut stmt.counter, &mut stmt.start, &mut stmt.end);
                visit_all_nodes_mut!(visitor, &mut stmt.by_step);
                visit_all_nodes_mut!(visitor, &mut stmt.body);
            }
            AstControlStatement::Case(stmt) => {
                visit_nodes_mut!(visitor, &mut stmt.selector);
                stmt.case_blocks.walk(visitor);
                visit_all_nodes_mut!(visitor, &mut stmt.else_block);
            }
        }
    }
}

impl WalkerMut for ReturnStatement {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_all_nodes_mut!(visitor, &mut self.condition);
    }
}

impl WalkerMut for JumpStatement {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes_mut!(visitor, &mut self.condition, &mut self.target);
    }
}

impl WalkerMut for AstNode {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        match self.stmt {
            AstStatement::EmptyStatement(_) => visitor.visit_empty_statement(self),
            AstStatement::DefaultValue(_) => visitor.visit_default_value(self),
            AstStatement::Literal(_) => visitor.visit_literal(self),
            AstStatement::MultipliedStatement(_) => visitor.visit_multiplied_statement(self),
            AstStatement::ReferenceExpr(_) => visitor.visit_reference_expr(self),
            AstStatement::Identifier(_) => visitor.visit_identifier(self),
            AstStatement::DirectAccess(_) => visitor.visit_direct_access(self),
            AstStatement::HardwareAccess(_) => visitor.visit_hardware_access(self),
            AstStatement::BinaryExpression(_) => visitor.visit_binary_expression(self),
            AstStatement::UnaryExpression(_) => visitor.visit_unary_expression(self),
            AstStatement::ExpressionList(_) => visitor.visit_expression_list(self),
            AstStatement::ParenExpression(_) => visitor.visit_paren_expression(self),
            AstStatement::RangeStatement(_) => visitor.visit_range_statement(self),
            AstStatement::VlaRangeStatement => visitor.visit_vla_range_statement(self),
            AstStatement::Assignment(_) => visitor.visit_assignment(self),
            AstStatement::OutputAssignment(_) => visitor.visit_output_assignment(self),
            AstStatement::RefAssignment(_) => visitor.visit_ref_assignment(self),
            AstStatement::CallStatement(_) => visitor.visit_call_statement(self),
            AstStatement::ControlStatement(_) => visitor.visit_control_statement(self),
            AstStatement::CaseCondition(_) => visitor.visit_case_condition(self),
            AstStatement::ExitStatement(_) => visitor.visit_exit_statement(self),
            AstStatement::ContinueStatement(_) => visitor.visit_continue_statement(self),
            AstStatement::ReturnStatement(_) => visitor.visit_return_statement(self),
            AstStatement::JumpStatement(_) => visitor.visit_jump_statement(self),
            AstStatement::LabelStatement(_) => visitor.visit_label_statement(self),
            AstStatement::AllocationStatement(_) => visitor.visit_allocation(self),
        }
    }
}

impl WalkerMut for CompilationUnit {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        for block in &mut self.global_vars {
            visitor.visit_variable_block(block);
        }

        for user_type in &mut self.user_types {
            visitor.visit_user_type_declaration(user_type);
        }

        for pou in &mut self.units {
            visitor.visit_pou(pou);
        }

        for i in &mut self.implementations {
            visitor.visit_implementation(i);
        }

        for i in &mut self.interfaces {
            visitor.visit_interface(i);
        }
    }
}

impl WalkerMut for UserTypeDeclaration {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visitor.visit_data_type(&mut self.data_type);
        visit_all_nodes_mut!(visitor, &mut self.initializer);
    }
}

impl WalkerMut for VariableBlock {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        for v in self.variables.iter_mut() {
            visitor.visit_variable(v);
        }
    }
}

impl WalkerMut for Variable {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_all_nodes_mut!(visitor, &mut self.address);
        visitor.visit_data_type_declaration(&mut self.data_type_declaration);
        visit_all_nodes_mut!(visitor, &mut self.initializer);
    }
}

impl WalkerMut for DataType {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        match self {
            DataType::StructType { variables, .. } => {
                for v in variables.iter_mut() {
                    visitor.visit_variable(v);
                }
            }
            DataType::EnumType { elements, .. } => {
                flatten_expression_list(elements).iter_mut().map(|it| it.clone()).for_each(|mut ele| {
                    visitor.visit_enum_element(&mut ele);
                });
            }
            DataType::SubRangeType { bounds, .. } => {
                visit_all_nodes_mut!(visitor, bounds);
            }
            DataType::ArrayType { bounds, referenced_type, .. } => {
                visitor.visit(bounds);
                visitor.visit_data_type_declaration(referenced_type);
            }
            DataType::PointerType { referenced_type, .. } => {
                visitor.visit_data_type_declaration(referenced_type);
            }
            DataType::StringType { size, .. } => {
                visit_all_nodes_mut!(visitor, size);
            }
            DataType::VarArgs { referenced_type, .. } => {
                if let Some(data_type_declaration) = referenced_type {
                    visitor.visit_data_type_declaration(data_type_declaration);
                }
            }
            DataType::GenericType { .. } => {
                //no further visits
            }
        }
    }
}

impl WalkerMut for DataTypeDeclaration {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        if let DataTypeDeclaration::Definition { data_type, .. } = self {
            visitor.visit_data_type(data_type);
        }
    }
}

impl<T: WalkerMut> WalkerMut for Option<T> {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        if let Some(node) = self {
            node.walk(visitor);
        }
    }
}

impl WalkerMut for Pou {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        for block in &mut self.variable_blocks {
            visitor.visit_variable_block(block);
        }

        for property in &mut self.properties {
            visitor.visit_property(property);
        }

        if let Some(rt) = self.return_type.as_mut() {
            visitor.visit_data_type_declaration(rt)
        }
    }
}

impl WalkerMut for Implementation {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        for n in &mut self.statements {
            visitor.visit(n);
        }
    }
}
