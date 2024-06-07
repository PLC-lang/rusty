/// This module defines the `AstVisitor` trait and its associated macros.
/// The `AstVisitor` trait provides a set of methods for traversing and visiting ASTs
use crate::ast::AstNode;
use crate::ast::*;
use crate::control_statements::{AstControlStatement, ConditionalBlock, ReturnStatement};
use crate::literals::AstLiteral;

/// Macro that walks through all nodes in an iterator and applies the visitor's `walk` method to each node.
macro_rules! visit_all_nodes {
    ($visitor:expr, $iter:expr) => {
        for node in $iter {
            $visitor.visit(node);
        }
    };
}

/// Macro that walks through a list of nodes and applies the visitor's `walk` method to each node.
macro_rules! visit_nodes {
    ($visitor:expr, $($node:expr),*) => {
        $(
            $visitor.visit($node);
        )*
    };
}

pub trait Walker {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor;
}

/// The `AstVisitor` trait provides a set of methods for traversing and visiting different types of AST nodes.
/// Implementors can individually override the methods they are interested in. When overriding a method,
/// make sure to call `walk` on the visited statement to visit its children. DO NOT call walk on
/// the node itself (last parameter).
///
/// # Example
/// ```
/// use plc_ast::{
///    ast::{Assignment, AstNode},
///    visitor::{AstVisitor, Walker},
/// };
///
/// struct AssignmentCounter {
///    count: usize,
/// }
///
/// impl AstVisitor for AssignmentCounter {
///    fn visit_assignment(&mut self, stmt: &Assignment, _node: &AstNode) {
///        self.count += 1;
///        // visit child nodes
///        stmt.walk(self);
///    }
///
///    fn visit_output_assignment(&mut self, stmt: &Assignment, _node: &AstNode) {
///        self.count += 1;
///        // visit child nodes
///        stmt.walk(self);
///    }
/// }
/// ```
pub trait AstVisitor: Sized {
    /// visits through an `AstNode` and applies the visitor's `walk` method to each node.
    fn visit(&mut self, node: &AstNode) {
        node.walk(self)
    }

    /// Visits a `CompilationUnit` node.
    /// Make sure to call `walk` on the `CompilationUnit` node to visit its children.
    fn visit_compilation_unit(&mut self, unit: &CompilationUnit) {
        unit.walk(self)
    }

    /// Visits an `Implementation` node.
    /// Make sure to call `walk` on the `Implementation` node to visit its children.
    fn visit_implementation(&mut self, implementation: &Implementation) {
        implementation.walk(self);
    }

    /// Visits a `DataTypeDeclaration` node.
    /// Make sure to call `walk` on the `VariableBlock` node to visit its children.
    fn visit_variable_block(&mut self, block: &VariableBlock) {
        block.walk(self)
    }

    /// Visits a `Variable` node.
    /// Make sure to call `walk` on the `Variable` node to visit its children.
    fn visit_variable(&mut self, variable: &Variable) {
        variable.walk(self);
    }

    /// Visits an enum element `AstNode` node.
    /// Make sure to call `walk` on the `AstNode` node to visit its children.
    fn visit_enum_element(&mut self, element: &AstNode) {
        element.walk(self);
    }

    /// Visits a `DataTypeDeclaration` node.
    /// Make sure to call `walk` on the `DataTypeDeclaration` node to visit its children.
    fn visit_data_type_declaration(&mut self, data_type_declaration: &DataTypeDeclaration) {
        data_type_declaration.walk(self);
    }

    /// Visits a `UserTypeDeclaration` node.
    /// Make sure to call `walk` on the `UserTypeDeclaration` node to visit its children.
    fn visit_user_type_declaration(&mut self, user_type: &UserTypeDeclaration) {
        user_type.walk(self);
    }

    /// Visits a `UserTypeDeclaration` node.
    /// Make sure to call `walk` on the `DataType` node to visit its children.
    fn visit_data_type(&mut self, data_type: &DataType) {
        data_type.walk(self);
    }

    /// Visits a `Pou` node.
    /// Make sure to call `walk` on the `Pou` node to visit its children.
    fn visit_pou(&mut self, pou: &Pou) {
        pou.walk(self);
    }

    /// Visits an `EmptyStatement` node.
    fn visit_empty_statement(&mut self, _stmt: &EmptyStatement, _node: &AstNode) {}

    /// Visits a `DefaultValue` node.
    fn visit_default_value(&mut self, _stmt: &DefaultValue, _node: &AstNode) {}

    /// Visits an `AstLiteral` node.
    /// Make sure to call `walk` on the `AstLiteral` node to visit its children.
    fn visit_literal(&mut self, stmt: &AstLiteral, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits a `MultipliedStatement` node.
    /// Make sure to call `walk` on the `MultipliedStatement` node to visit its children.
    fn visit_multiplied_statement(&mut self, stmt: &MultipliedStatement, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits a `ReferenceExpr` node.
    /// Make sure to call `walk` on the `ReferenceExpr` node to visit its children.
    fn visit_reference_expr(&mut self, stmt: &ReferenceExpr, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits an `Identifier` node.
    /// Make sure to call `walk` on the `Identifier` node to visit its children.
    fn visit_identifier(&mut self, _stmt: &str, _node: &AstNode) {}

    /// Visits a `DirectAccess` node.
    /// Make sure to call `walk` on the `DirectAccess` node to visit its children.
    fn visit_direct_access(&mut self, stmt: &DirectAccess, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits a `HardwareAccess` node.
    /// Make sure to call `walk` on the `HardwareAccess` node to visit its children.
    fn visit_hardware_access(&mut self, stmt: &HardwareAccess, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits a `BinaryExpression` node.
    /// Make sure to call `walk` on the `BinaryExpression` node to visit its children.
    fn visit_binary_expression(&mut self, stmt: &BinaryExpression, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits a `UnaryExpression` node.
    /// Make sure to call `walk` on the `UnaryExpression` node to visit its children.
    fn visit_unary_expression(&mut self, stmt: &UnaryExpression, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits an `ExpressionList` node.
    /// Make sure to call `walk` on the `Vec<AstNode>` node to visit its children.
    fn visit_expression_list(&mut self, stmt: &Vec<AstNode>, _node: &AstNode) {
        visit_all_nodes!(self, stmt);
    }

    /// Visits a `ParenExpression` node.
    /// Make sure to call `walk` on the inner `AstNode` node to visit its children.
    fn visit_paren_expression(&mut self, inner: &AstNode, _node: &AstNode) {
        inner.walk(self)
    }

    /// Visits a `RangeStatement` node.
    /// Make sure to call `walk` on the `RangeStatement` node to visit its children.
    fn visit_range_statement(&mut self, stmt: &RangeStatement, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits a `VlaRangeStatement` node.
    fn visit_vla_range_statement(&mut self, _node: &AstNode) {}

    /// Visits an `Assignment` node.
    /// Make sure to call `walk` on the `Assignment` node to visit its children.
    fn visit_assignment(&mut self, stmt: &Assignment, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits an `OutputAssignment` node.
    /// Make sure to call `walk` on the `Assignment` node to visit its children.
    fn visit_output_assignment(&mut self, stmt: &Assignment, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits a `CallStatement` node.
    /// Make sure to call `walk` on the `CallStatement` node to visit its children.
    fn visit_call_statement(&mut self, stmt: &CallStatement, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits an `AstControlStatement` node.
    /// Make sure to call `walk` on the `AstControlStatement` node to visit its children.
    fn visit_control_statement(&mut self, stmt: &AstControlStatement, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits a `CaseCondition` node.
    /// Make sure to call `walk` on the child-`AstNode` node to visit its children.
    fn visit_case_condition(&mut self, child: &AstNode, _node: &AstNode) {
        child.walk(self)
    }

    /// Visits an `ExitStatement` node.
    fn visit_exit_statement(&mut self, _node: &AstNode) {}

    /// Visits a `ContinueStatement` node.
    fn visit_continue_statement(&mut self, _node: &AstNode) {}

    /// Visits a `ReturnStatement` node.
    /// Make sure to call `walk` on the `ReturnStatement` node to visit its children.
    fn visit_return_statement(&mut self, stmt: &ReturnStatement, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits a `JumpStatement` node.
    /// Make sure to call `walk` on the `JumpStatement` node to visit its children.
    fn visit_jump_statement(&mut self, stmt: &JumpStatement, _node: &AstNode) {
        stmt.walk(self)
    }

    /// Visits a `LabelStatement` node.
    fn visit_label_statement(&mut self, _stmt: &LabelStatement, _node: &AstNode) {}
}

/// Walks through a slice of `ConditionalBlock` and applies the visitor's `walk` method to each node.
fn walk_conditional_blocks<V>(visitor: &mut V, blocks: &[ConditionalBlock])
where
    V: AstVisitor,
{
    for b in blocks {
        visit_nodes!(visitor, &b.condition);
        visit_all_nodes!(visitor, &b.body);
    }
}

impl Walker for AstLiteral {
    fn walk<V>(&self, _visitor: &mut V)
    where
        V: AstVisitor,
    {
        // do nothing
    }
}

impl Walker for MultipliedStatement {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visitor.visit(&self.element)
    }
}

impl Walker for ReferenceExpr {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        if let Some(base) = &self.base {
            visitor.visit(base);
        }

        match &self.access {
            ReferenceAccess::Member(t) | ReferenceAccess::Index(t) | ReferenceAccess::Cast(t) => {
                visitor.visit(t)
            }
            _ => {}
        }
    }
}

impl Walker for DirectAccess {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visit_nodes!(visitor, &self.index);
    }
}

impl Walker for HardwareAccess {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visit_all_nodes!(visitor, &self.address);
    }
}

impl Walker for BinaryExpression {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visit_nodes!(visitor, &self.left, &self.right);
    }
}

impl Walker for UnaryExpression {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visit_nodes!(visitor, &self.value);
    }
}

impl Walker for Assignment {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visit_nodes!(visitor, &self.left, &self.right);
    }
}

impl Walker for RangeStatement {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visit_nodes!(visitor, &self.start, &self.end);
    }
}

impl Walker for CallStatement {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visit_nodes!(visitor, &self.operator);
        if let Some(params) = &self.parameters {
            visit_nodes!(visitor, params);
        }
    }
}

impl Walker for AstControlStatement {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        match self {
            AstControlStatement::If(stmt) => {
                walk_conditional_blocks(visitor, &stmt.blocks);
                visit_all_nodes!(visitor, &stmt.else_block);
            }
            AstControlStatement::WhileLoop(stmt) | AstControlStatement::RepeatLoop(stmt) => {
                visit_nodes!(visitor, &stmt.condition);
                visit_all_nodes!(visitor, &stmt.body);
            }
            AstControlStatement::ForLoop(stmt) => {
                visit_nodes!(visitor, &stmt.counter, &stmt.start, &stmt.end);
                visit_all_nodes!(visitor, &stmt.by_step);
                visit_all_nodes!(visitor, &stmt.body);
            }
            AstControlStatement::Case(stmt) => {
                visit_nodes!(visitor, &stmt.selector);
                walk_conditional_blocks(visitor, &stmt.case_blocks);
                visit_all_nodes!(visitor, &stmt.else_block);
            }
        }
    }
}

impl Walker for ReturnStatement {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visit_all_nodes!(visitor, &self.condition);
    }
}

impl Walker for JumpStatement {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visit_nodes!(visitor, &self.condition, &self.target);
    }
}

impl Walker for AstNode {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        let node = self;
        match &self.stmt {
            AstStatement::EmptyStatement(stmt) => visitor.visit_empty_statement(stmt, node),
            AstStatement::DefaultValue(stmt) => visitor.visit_default_value(stmt, node),
            AstStatement::Literal(stmt) => visitor.visit_literal(stmt, node),
            AstStatement::MultipliedStatement(stmt) => visitor.visit_multiplied_statement(stmt, node),
            AstStatement::ReferenceExpr(stmt) => visitor.visit_reference_expr(stmt, node),
            AstStatement::Identifier(stmt) => visitor.visit_identifier(stmt, node),
            AstStatement::DirectAccess(stmt) => visitor.visit_direct_access(stmt, node),
            AstStatement::HardwareAccess(stmt) => visitor.visit_hardware_access(stmt, node),
            AstStatement::BinaryExpression(stmt) => visitor.visit_binary_expression(stmt, node),
            AstStatement::UnaryExpression(stmt) => visitor.visit_unary_expression(stmt, node),
            AstStatement::ExpressionList(stmt) => visitor.visit_expression_list(stmt, node),
            AstStatement::ParenExpression(stmt) => visitor.visit_paren_expression(stmt, node),
            AstStatement::RangeStatement(stmt) => visitor.visit_range_statement(stmt, node),
            AstStatement::VlaRangeStatement => visitor.visit_vla_range_statement(node),
            AstStatement::Assignment(stmt) => visitor.visit_assignment(stmt, node),
            AstStatement::OutputAssignment(stmt) => visitor.visit_output_assignment(stmt, node),
            AstStatement::CallStatement(stmt) => visitor.visit_call_statement(stmt, node),
            AstStatement::ControlStatement(stmt) => visitor.visit_control_statement(stmt, node),
            AstStatement::CaseCondition(stmt) => visitor.visit_case_condition(stmt, node),
            AstStatement::ExitStatement(_stmt) => visitor.visit_exit_statement(node),
            AstStatement::ContinueStatement(_stmt) => visitor.visit_continue_statement(node),
            AstStatement::ReturnStatement(stmt) => visitor.visit_return_statement(stmt, node),
            AstStatement::JumpStatement(stmt) => visitor.visit_jump_statement(stmt, node),
            AstStatement::LabelStatement(stmt) => visitor.visit_label_statement(stmt, node),
        }
    }
}

impl Walker for CompilationUnit {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        for block in &self.global_vars {
            visitor.visit_variable_block(block);
        }

        for user_type in &self.user_types {
            visitor.visit_user_type_declaration(user_type);
        }

        for pou in &self.units {
            visitor.visit_pou(pou);
        }

        for i in &self.implementations {
            visitor.visit_implementation(i);
        }
    }
}

impl Walker for UserTypeDeclaration {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visitor.visit_data_type(&self.data_type);
        visit_all_nodes!(visitor, &self.initializer);
    }
}

impl Walker for VariableBlock {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        for v in self.variables.iter() {
            visitor.visit_variable(v);
        }
    }
}

impl Walker for Variable {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        visit_all_nodes!(visitor, &self.address);
        visitor.visit_data_type_declaration(&self.data_type_declaration);
        visit_all_nodes!(visitor, &self.initializer);
    }
}

impl Walker for DataType {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        match self {
            DataType::StructType { variables, .. } => {
                for v in variables.iter() {
                    visitor.visit_variable(v);
                }
            }
            DataType::EnumType { elements, .. } => {
                for ele in flatten_expression_list(elements) {
                    visitor.visit_enum_element(ele);
                }
            }
            DataType::SubRangeType { bounds, .. } => {
                visit_all_nodes!(visitor, bounds);
            }
            DataType::ArrayType { bounds, referenced_type, .. } => {
                visitor.visit(bounds);
                visitor.visit_data_type_declaration(referenced_type);
            }
            DataType::PointerType { referenced_type, .. } => {
                visitor.visit_data_type_declaration(referenced_type);
            }
            DataType::StringType { size, .. } => {
                visit_all_nodes!(visitor, size);
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

impl Walker for DataTypeDeclaration {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        if let DataTypeDeclaration::DataTypeDefinition { data_type, .. } = self {
            visitor.visit_data_type(data_type);
        }
    }
}

impl<T> Walker for Option<T>
where
    T: Walker,
{
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        if let Some(node) = self {
            node.walk(visitor);
        }
    }
}

impl Walker for Pou {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        for block in &self.variable_blocks {
            visitor.visit_variable_block(block);
        }

        self.return_type.as_ref().inspect(|rt| visitor.visit_data_type_declaration(rt));
    }
}

impl Walker for Implementation {
    fn walk<V>(&self, visitor: &mut V)
    where
        V: AstVisitor,
    {
        for n in &self.statements {
            visitor.visit(n);
        }
    }
}
