//! This module defines the `AstVisitorMut` trait and its associated macros.
//! The `AstVisitorMut` trait provides a set of methods for mutably traversing and visiting ASTs

use crate::ast::{flatten_expression_list, Assignment, AstNode, AstStatement, BinaryExpression, CallStatement, CompilationUnit, DataType, DataTypeDeclaration, DefaultValue, DirectAccess, EmptyStatement, HardwareAccess, Implementation, JumpStatement, LabelStatement, MultipliedStatement, Pou, RangeStatement, ReferenceAccess, ReferenceExpr, UnaryExpression, UserTypeDeclaration, Variable, VariableBlock};
use crate::control_statements::{AstControlStatement, ConditionalBlock, ReturnStatement};
use crate::literals::AstLiteral;
use crate::provider::IdProvider;

#[macro_export]
macro_rules! visit_all_nodes_mut {
    ($visitor:expr, $iter:expr, $ctxt:expr) => {
        // Note: The `allow` is needed to suppress warnings about `while let Some(...)` warnings
        // because `visit_all_nodes_mut!` is used for both Option and Non-Option types
        #[allow(warnings)]
        {
            for node in $iter {
                $visitor.visit(node, $ctxt);
            }
        }
    };
}

/// Macro that calls the visitor's `visit` method for every AstNode in the passed sequence of nodes.
macro_rules! visit_nodes {
    ($visitor:expr, $ctxt:expr, $($node:expr),*) => {
        $(
            $visitor.visit($node, $ctxt);
        )*
    };
}

pub trait VisitorContext {
    fn with_qualifier(&self, qualifier: &str) -> Self;

    /// returns a copy of the current context and changes the `current_pou` to the given pou
    fn with_pou(&self, pou: &str) -> Self;

    fn new(id_provider: IdProvider) -> Self;

    fn get_qualifier(&self) -> &Option<String> {
        &None
    }

    fn get_pou(&self) -> &Option<String> {
        &None
    }
}

pub trait WalkerMut {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext;
}

pub trait AstVisitorMut: Sized {
    fn visit<T: VisitorContext>(&mut self, node: &mut AstNode, ctxt: &T) {
        node.walk(self, ctxt)
    }

    fn visit_compilation_unit<T: VisitorContext>(&mut self, unit: &mut CompilationUnit, ctxt: &T) {
        unit.walk(self, ctxt)
    }

    fn visit_implementation<T: VisitorContext>(&mut self, implementation: &mut Implementation, ctxt: &T) {
        implementation.walk(self, ctxt);
    }

    fn visit_variable_block<T: VisitorContext>(&mut self, block: &mut VariableBlock, ctxt: &T) {
        block.walk(self, ctxt)
    }

    fn visit_variable<T: VisitorContext>(&mut self, variable: &mut Variable, ctxt: &T) {
        variable.walk(self, ctxt);
    }

    fn visit_enum_element<T: VisitorContext>(&mut self, element: &mut AstNode, ctxt: &T) {
        element.walk(self, ctxt);
    }

    fn visit_data_type_declaration<T: VisitorContext>(&mut self, data_type_declaration: &mut DataTypeDeclaration, ctxt: &T) {
        data_type_declaration.walk(self, ctxt);
    }

    fn visit_user_type_declaration<T: VisitorContext>(&mut self, user_type: &mut UserTypeDeclaration, ctxt: &T) {
        user_type.walk(self, ctxt);
    }

    fn visit_data_type<T: VisitorContext>(&mut self, data_type: &mut DataType, ctxt: &T) {
        data_type.walk(self, ctxt);
    }

    fn visit_pou<T: VisitorContext>(&mut self, pou: &mut Pou, ctxt: &T) {
        pou.walk(self, ctxt);
    }

    fn visit_empty_statement<T: VisitorContext>(&mut self, _stmt: &mut EmptyStatement, _node: &mut AstNode, _ctxt: &T) {}

    fn visit_default_value<T: VisitorContext>(&mut self, _stmt: &mut DefaultValue, _node: &mut AstNode, _ctxt: &T) {}

    fn visit_literal<T: VisitorContext>(&mut self, stmt: &mut AstLiteral, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_multiplied_statement<T: VisitorContext>(&mut self, stmt: &mut MultipliedStatement, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_reference_expr<T: VisitorContext>(&mut self, stmt: &mut ReferenceExpr, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_identifier<T: VisitorContext>(&mut self, _stmt: &mut str, _node: &mut AstNode, _ctxt: &T) {}

    fn visit_direct_access<T: VisitorContext>(&mut self, stmt: &mut DirectAccess, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_hardware_access<T: VisitorContext>(&mut self, stmt: &mut HardwareAccess, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_binary_expression<T: VisitorContext>(&mut self, stmt: &mut BinaryExpression, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_unary_expression<T: VisitorContext>(&mut self, stmt: &mut UnaryExpression, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_expression_list<T: VisitorContext>(&mut self, stmt: &mut Vec<AstNode>, _node: &mut AstNode, ctxt: &T) {
        visit_all_nodes_mut!(self, stmt, ctxt);
    }

    fn visit_paren_expression<T: VisitorContext>(&mut self, inner: &mut AstNode, _node: &mut AstNode, ctxt: &T) {
        inner.walk(self, ctxt)
    }

    fn visit_range_statement<T: VisitorContext>(&mut self, stmt: &mut RangeStatement, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_vla_range_statement<T: VisitorContext>(&mut self, _node: &mut AstNode, _ctxt: &T) {}

    fn visit_assignment<T: VisitorContext>(&mut self, stmt: &mut Assignment, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_output_assignment<T: VisitorContext>(&mut self, stmt: &mut Assignment, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_ref_assignment<T: VisitorContext>(&mut self, stmt: &mut Assignment, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_call_statement<T: VisitorContext>(&mut self, stmt: &mut CallStatement, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_control_statement<T: VisitorContext>(&mut self, stmt: &mut AstControlStatement, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_case_condition<T: VisitorContext>(&mut self, child: &mut AstNode, _node: &mut AstNode, ctxt: &T) {
        child.walk(self, ctxt)
    }

    fn visit_exit_statement<T: VisitorContext>(&mut self, _node: &mut AstNode, _ctxt: &T) {}

    fn visit_continue_statement<T: VisitorContext>(&mut self, _node: &mut AstNode, _ctxt: &T) {}

    fn visit_return_statement<T: VisitorContext>(&mut self, stmt: &mut ReturnStatement, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    fn visit_jump_statement<T: VisitorContext>(&mut self, stmt: &mut JumpStatement, _node: &mut AstNode, ctxt: &T) {
        stmt.walk(self, ctxt)
    }

    /// Visits a `LabelStatement` node.
    /// # Arguments
    /// * `stmt` - The unwraped, typed `LabelStatement` node to visit.
    /// * `node` - The wrapped `AstNode` node to visit. Offers access to location information and AstId
    fn visit_label_statement<T: VisitorContext>(&mut self, _stmt: &mut LabelStatement, _node: &mut AstNode, _ctxt: &T) {}
}

/// Helper method that walks through a slice of `ConditionalBlock` and applies the visitor's `walk` method to each node.
fn walk_conditional_blocks<V, T>(visitor: &mut V, blocks: &mut [ConditionalBlock], ctxt: &T)
where
    V: AstVisitorMut,
    T: VisitorContext
{
    for b in blocks {
        visit_nodes!(visitor, ctxt, &mut b.condition);
        visit_all_nodes_mut!(visitor, &mut b.body, ctxt);
    }
}

impl WalkerMut for AstLiteral {
    fn walk<V, T>(&mut self, _visitor: &mut V, _ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        // do nothing
    }
}

impl WalkerMut for MultipliedStatement {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visitor.visit(&mut self.element, ctxt)
    }
}

impl WalkerMut for ReferenceExpr {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        if let Some(base) = &mut self.base {
            visitor.visit(base, ctxt);
        }

        match &mut self.access {
            ReferenceAccess::Member(t) | ReferenceAccess::Index(t) | ReferenceAccess::Cast(t) => {
                visitor.visit(t, ctxt)
            }
            _ => {}
        }
    }
}

impl WalkerMut for DirectAccess {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visit_nodes!(visitor, ctxt, &mut self.index);
    }
}

impl WalkerMut for HardwareAccess {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visit_all_nodes_mut!(visitor, &mut self.address, ctxt);
    }
}

impl WalkerMut for BinaryExpression {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visit_nodes!(visitor, ctxt, &mut self.left, &mut self.right);
    }
}

impl WalkerMut for UnaryExpression {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visit_nodes!(visitor, ctxt, &mut self.value);
    }
}

impl WalkerMut for Assignment {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visit_nodes!(visitor, ctxt, &mut self.left, &mut self.right);
    }
}

impl WalkerMut for RangeStatement {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visit_nodes!(visitor, ctxt, &mut self.start, &mut self.end);
    }
}

impl WalkerMut for CallStatement {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visit_nodes!(visitor, ctxt, &mut self.operator);
        if let Some(params) = &mut self.parameters {
            visit_nodes!(visitor, ctxt, params);
        }
    }
}

impl WalkerMut for AstControlStatement {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        match self {
            AstControlStatement::If(stmt) => {
                walk_conditional_blocks(visitor, &mut stmt.blocks, ctxt);
                visit_all_nodes_mut!(visitor, &mut stmt.else_block, ctxt);
            }
            AstControlStatement::WhileLoop(stmt) | AstControlStatement::RepeatLoop(stmt) => {
                visit_nodes!(visitor, ctxt, &mut stmt.condition);
                visit_all_nodes_mut!(visitor, &mut stmt.body, ctxt);
            }
            AstControlStatement::ForLoop(stmt) => {
                visit_nodes!(visitor, ctxt, &mut stmt.counter, &mut stmt.start, &mut stmt.end);
                visit_all_nodes_mut!(visitor, &mut stmt.by_step, ctxt);
                visit_all_nodes_mut!(visitor, &mut stmt.body, ctxt);
            }
            AstControlStatement::Case(stmt) => {
                visit_nodes!(visitor, ctxt, &mut stmt.selector);
                walk_conditional_blocks(visitor, &mut stmt.case_blocks, ctxt);
                visit_all_nodes_mut!(visitor, &mut stmt.else_block, ctxt);
            }
        }
    }
}

impl WalkerMut for ReturnStatement {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visit_all_nodes_mut!(visitor, &mut self.condition, ctxt);
    }
}

impl WalkerMut for JumpStatement {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visit_nodes!(visitor, ctxt, &mut self.condition, &mut self.target);
    }
}

impl WalkerMut for AstNode {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        match self.stmt.clone() {
            AstStatement::EmptyStatement(ref mut stmt) => visitor.visit_empty_statement(stmt, self, ctxt),
            AstStatement::DefaultValue(ref mut stmt) => visitor.visit_default_value(stmt, self, ctxt),
            AstStatement::Literal(ref mut stmt) => visitor.visit_literal(stmt, self, ctxt),
            AstStatement::MultipliedStatement(ref mut stmt) => visitor.visit_multiplied_statement(stmt, self, ctxt),
            AstStatement::ReferenceExpr(ref mut stmt) => visitor.visit_reference_expr(stmt, self, ctxt),
            AstStatement::Identifier(ref mut stmt) => visitor.visit_identifier(stmt, self, ctxt),
            AstStatement::DirectAccess(ref mut stmt) => visitor.visit_direct_access(stmt, self, ctxt),
            AstStatement::HardwareAccess(ref mut stmt) => visitor.visit_hardware_access(stmt, self, ctxt),
            AstStatement::BinaryExpression(ref mut stmt) => visitor.visit_binary_expression(stmt, self, ctxt),
            AstStatement::UnaryExpression(ref mut stmt) => visitor.visit_unary_expression(stmt, self, ctxt),
            AstStatement::ExpressionList(ref mut stmt) => visitor.visit_expression_list(stmt, self, ctxt),
            AstStatement::ParenExpression(ref mut stmt) => visitor.visit_paren_expression(stmt, self, ctxt),
            AstStatement::RangeStatement(ref mut stmt) => visitor.visit_range_statement(stmt, self, ctxt),
            AstStatement::VlaRangeStatement => visitor.visit_vla_range_statement(self, ctxt),
            AstStatement::Assignment(ref mut stmt) => visitor.visit_assignment(stmt, self, ctxt),
            AstStatement::OutputAssignment(ref mut stmt) => visitor.visit_output_assignment(stmt, self, ctxt),
            AstStatement::RefAssignment(ref mut stmt) => visitor.visit_ref_assignment(stmt, self, ctxt),
            AstStatement::CallStatement(ref mut stmt) => visitor.visit_call_statement(stmt, self, ctxt),
            AstStatement::ControlStatement(ref mut stmt) => visitor.visit_control_statement(stmt, self, ctxt),
            AstStatement::CaseCondition(ref mut stmt) => visitor.visit_case_condition(stmt, self, ctxt),
            AstStatement::ExitStatement(ref mut _stmt) => visitor.visit_exit_statement(self, ctxt),
            AstStatement::ContinueStatement(ref mut _stmt) => visitor.visit_continue_statement(self, ctxt),
            AstStatement::ReturnStatement(ref mut stmt) => visitor.visit_return_statement(stmt, self, ctxt),
            AstStatement::JumpStatement(ref mut stmt) => visitor.visit_jump_statement(stmt, self, ctxt),
            AstStatement::LabelStatement(ref mut stmt) => visitor.visit_label_statement(stmt, self, ctxt),
        }
    }
}

impl WalkerMut for CompilationUnit {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        for block in &mut self.global_vars {
            visitor.visit_variable_block(block, ctxt);
        }

        for user_type in &mut self.user_types {
            visitor.visit_user_type_declaration(user_type, ctxt);
        }

        for pou in &mut self.units {
            visitor.visit_pou(pou, ctxt);
        }

        for i in &mut self.implementations {
            visitor.visit_implementation(i, ctxt);
        }
    }
}

impl WalkerMut for UserTypeDeclaration {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visitor.visit_data_type(&mut self.data_type, ctxt);
        visit_all_nodes_mut!(visitor, &mut self.initializer, ctxt);
    }
}

impl WalkerMut for VariableBlock {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        for v in self.variables.iter_mut() {
            visitor.visit_variable(v, ctxt);
        }
    }
}

impl WalkerMut for Variable {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        visit_all_nodes_mut!(visitor, &mut self.address, ctxt);
        visitor.visit_data_type_declaration(&mut self.data_type_declaration, ctxt);
        visit_all_nodes_mut!(visitor, &mut self.initializer, ctxt);
    }
}

impl WalkerMut for DataType {
    fn walk<V, T>(&mut self, visitor: &mut V, ctxt: &T)
    where
        V: AstVisitorMut,
        T: VisitorContext
    {
        match self {
            DataType::StructType { variables, .. } => {
                for v in variables.iter_mut() {
                    visitor.visit_variable(v, ctxt);
                }
            }
            DataType::EnumType { elements, .. } => {
                flatten_expression_list(elements).iter_mut().map(|it| it.clone()).for_each(|mut ele| {
                    visitor.visit_enum_element(&mut ele, ctxt);
                });
            }
            DataType::SubRangeType { bounds, .. } => {
                visit_all_nodes_mut!(visitor, bounds, ctxt);
            }
            DataType::ArrayType { bounds, referenced_type, .. } => {
                visitor.visit(bounds, ctxt);
                visitor.visit_data_type_declaration(referenced_type, ctxt);
            }
            DataType::PointerType { referenced_type, .. } => {
                visitor.visit_data_type_declaration(referenced_type, ctxt);
            }
            DataType::StringType { size, .. } => {
                visit_all_nodes_mut!(visitor, size, ctxt);
            }
            DataType::VarArgs { referenced_type, .. } => {
                if let Some(data_type_declaration) = referenced_type {
                    visitor.visit_data_type_declaration(data_type_declaration, ctxt);
                }
            }
            DataType::GenericType { .. } => {
                //no further visits
            }
        }
    }
}

impl WalkerMut for DataTypeDeclaration {
    fn walk<V, C>(&mut self, visitor: &mut V, ctxt: &C)
    where
        V: AstVisitorMut,
        C: VisitorContext
    {
        if let DataTypeDeclaration::DataTypeDefinition { data_type, .. } = self {
            visitor.visit_data_type(data_type, ctxt);
        }
    }
}

impl<T: WalkerMut> WalkerMut for Option<T>
{
    fn walk<V, C>(&mut self, visitor: &mut V, ctxt: &C)
    where
        V: AstVisitorMut,
        C: VisitorContext
    {
        if let Some(node) = self {
            node.walk(visitor, ctxt);
        }
    }
}

impl WalkerMut for Pou {
    fn walk<V, C>(&mut self, visitor: &mut V, ctxt: &C)
    where
        V: AstVisitorMut,
        C: VisitorContext
    {
        for block in &mut self.variable_blocks {
            visitor.visit_variable_block(block, ctxt);
        }

        if let Some(rt) = self.return_type.as_mut() {
            visitor.visit_data_type_declaration(rt, ctxt)
        }
    }
}

impl WalkerMut for Implementation {
    fn walk<V, C>(&mut self, visitor: &mut V, ctxt: &C)
    where
        V: AstVisitorMut,
        C: VisitorContext
    {
        for n in &mut self.statements {
            visitor.visit(n, ctxt);
        }
    }
}
