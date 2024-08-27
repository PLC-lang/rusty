//! This module defines the `AstVisitorMut` trait and its associated macros.
//! The `AstVisitorMut` trait provides a set of methods for mutably traversing and visiting ASTs

use crate::ast::{
    flatten_expression_list, Assignment, AstNode, AstStatement, BinaryExpression, CallStatement,
    CompilationUnit, DataType, DataTypeDeclaration, DefaultValue, DirectAccess, EmptyStatement,
    HardwareAccess, Implementation, JumpStatement, LabelStatement, MultipliedStatement, Pou, RangeStatement,
    ReferenceAccess, ReferenceExpr, UnaryExpression, UserTypeDeclaration, Variable, VariableBlock,
};
use crate::control_statements::{AstControlStatement, ConditionalBlock, ReturnStatement};
use crate::literals::AstLiteral;

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
macro_rules! visit_nodes {
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

    fn visit_empty_statement(&mut self, _stmt: &mut EmptyStatement, _node: &mut AstNode) {}

    fn visit_default_value(&mut self, _stmt: &mut DefaultValue, _node: &mut AstNode) {}

    fn visit_literal(&mut self, stmt: &mut AstLiteral, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_multiplied_statement(&mut self, stmt: &mut MultipliedStatement, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_reference_expr(&mut self, stmt: &mut ReferenceExpr, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_identifier(&mut self, _stmt: &mut str, _node: &mut AstNode) {}

    fn visit_direct_access(&mut self, stmt: &mut DirectAccess, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_hardware_access(&mut self, stmt: &mut HardwareAccess, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_binary_expression(&mut self, stmt: &mut BinaryExpression, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_unary_expression(&mut self, stmt: &mut UnaryExpression, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_expression_list(&mut self, stmt: &mut Vec<AstNode>, _node: &mut AstNode) {
        visit_all_nodes_mut!(self, stmt);
    }

    fn visit_paren_expression(&mut self, inner: &mut AstNode, _node: &mut AstNode) {
        inner.walk(self)
    }

    fn visit_range_statement(&mut self, stmt: &mut RangeStatement, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_vla_range_statement(&mut self, _node: &mut AstNode) {}

    fn visit_assignment(&mut self, stmt: &mut Assignment, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_output_assignment(&mut self, stmt: &mut Assignment, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_ref_assignment(&mut self, stmt: &mut Assignment, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_call_statement(&mut self, stmt: &mut CallStatement, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_control_statement(&mut self, stmt: &mut AstControlStatement, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_case_condition(&mut self, child: &mut AstNode, _node: &mut AstNode) {
        child.walk(self)
    }

    fn visit_exit_statement(&mut self, _node: &mut AstNode) {}

    fn visit_continue_statement(&mut self, _node: &mut AstNode) {}

    fn visit_return_statement(&mut self, stmt: &mut ReturnStatement, _node: &mut AstNode) {
        stmt.walk(self)
    }

    fn visit_jump_statement(&mut self, stmt: &mut JumpStatement, _node: &mut AstNode) {
        stmt.walk(self)
    }

    /// Visits a `LabelStatement` node.
    /// # Arguments
    /// * `stmt` - The unwrapedyped `LabelStatement` node to visit.
    /// * `node` - The wrapped `AstNode` node to visit. Offers access to location information and AstId
    fn visit_label_statement(&mut self, _stmt: &mut LabelStatement, _node: &mut AstNode) {}
}

/// Helper method that walks through a slice of `ConditionalBlock` and applies the visitor's `walk` method to each node.
fn walk_conditional_blocks<V>(visitor: &mut V, blocks: &mut [ConditionalBlock])
where
    V: AstVisitorMut,
{
    for b in blocks {
        visit_nodes!(visitor, &mut b.condition);
        visit_all_nodes_mut!(visitor, &mut b.body);
    }
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
        visit_nodes!(visitor, &mut self.index);
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
        visit_nodes!(visitor, &mut self.left, &mut self.right);
    }
}

impl WalkerMut for UnaryExpression {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes!(visitor, &mut self.value);
    }
}

impl WalkerMut for Assignment {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes!(visitor, &mut self.left, &mut self.right);
    }
}

impl WalkerMut for RangeStatement {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes!(visitor, &mut self.start, &mut self.end);
    }
}

impl WalkerMut for CallStatement {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        visit_nodes!(visitor, &mut self.operator);
        if let Some(params) = &mut self.parameters {
            visit_nodes!(visitor, params);
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
                walk_conditional_blocks(visitor, &mut stmt.blocks);
                visit_all_nodes_mut!(visitor, &mut stmt.else_block);
            }
            AstControlStatement::WhileLoop(stmt) | AstControlStatement::RepeatLoop(stmt) => {
                visit_nodes!(visitor, &mut stmt.condition);
                visit_all_nodes_mut!(visitor, &mut stmt.body);
            }
            AstControlStatement::ForLoop(stmt) => {
                visit_nodes!(visitor, &mut stmt.counter, &mut stmt.start, &mut stmt.end);
                visit_all_nodes_mut!(visitor, &mut stmt.by_step);
                visit_all_nodes_mut!(visitor, &mut stmt.body);
            }
            AstControlStatement::Case(stmt) => {
                visit_nodes!(visitor, &mut stmt.selector);
                walk_conditional_blocks(visitor, &mut stmt.case_blocks);
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
        visit_nodes!(visitor, &mut self.condition, &mut self.target);
    }
}

impl WalkerMut for AstNode {
    fn walk<V>(&mut self, visitor: &mut V)
    where
        V: AstVisitorMut,
    {
        match self.stmt.clone() {
            AstStatement::EmptyStatement(ref mut stmt) => visitor.visit_empty_statement(stmt, self),
            AstStatement::DefaultValue(ref mut stmt) => visitor.visit_default_value(stmt, self),
            AstStatement::Literal(ref mut stmt) => visitor.visit_literal(stmt, self),
            AstStatement::MultipliedStatement(ref mut stmt) => visitor.visit_multiplied_statement(stmt, self),
            AstStatement::ReferenceExpr(ref mut stmt) => visitor.visit_reference_expr(stmt, self),
            AstStatement::Identifier(ref mut stmt) => visitor.visit_identifier(stmt, self),
            AstStatement::DirectAccess(ref mut stmt) => visitor.visit_direct_access(stmt, self),
            AstStatement::HardwareAccess(ref mut stmt) => visitor.visit_hardware_access(stmt, self),
            AstStatement::BinaryExpression(ref mut stmt) => visitor.visit_binary_expression(stmt, self),
            AstStatement::UnaryExpression(ref mut stmt) => visitor.visit_unary_expression(stmt, self),
            AstStatement::ExpressionList(ref mut stmt) => visitor.visit_expression_list(stmt, self),
            AstStatement::ParenExpression(ref mut stmt) => visitor.visit_paren_expression(stmt, self),
            AstStatement::RangeStatement(ref mut stmt) => visitor.visit_range_statement(stmt, self),
            AstStatement::VlaRangeStatement => visitor.visit_vla_range_statement(self),
            AstStatement::Assignment(ref mut stmt) => visitor.visit_assignment(stmt, self),
            AstStatement::OutputAssignment(ref mut stmt) => visitor.visit_output_assignment(stmt, self),
            AstStatement::RefAssignment(ref mut stmt) => visitor.visit_ref_assignment(stmt, self),
            AstStatement::CallStatement(ref mut stmt) => visitor.visit_call_statement(stmt, self),
            AstStatement::ControlStatement(ref mut stmt) => visitor.visit_control_statement(stmt, self),
            AstStatement::CaseCondition(ref mut stmt) => visitor.visit_case_condition(stmt, self),
            AstStatement::ExitStatement(ref mut _stmt) => visitor.visit_exit_statement(self),
            AstStatement::ContinueStatement(ref mut _stmt) => visitor.visit_continue_statement(self),
            AstStatement::ReturnStatement(ref mut stmt) => visitor.visit_return_statement(stmt, self),
            AstStatement::JumpStatement(ref mut stmt) => visitor.visit_jump_statement(stmt, self),
            AstStatement::LabelStatement(ref mut stmt) => visitor.visit_label_statement(stmt, self),
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
        if let DataTypeDeclaration::DataTypeDefinition { data_type, .. } = self {
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
