use crate::{ast::*, control_statements::{AstControlStatement, ReturnStatement}, literals::AstLiteral};

pub trait AstVisitor<T : Default> {
    fn visit(&mut self, node: &AstNode) -> T {
        match node.get_stmt() {
            AstStatement::ReferenceExpr(expr) => self.visit_reference_expr(expr, node),
            AstStatement::Identifier(id) => self.visit_identifier(id, node),
            AstStatement::DirectAccess(access) => self.visit_direct_access(access, node),
            AstStatement::HardwareAccess(access) => self.visit_hardware_access(access, node),
            AstStatement::BinaryExpression(expr) => self.visit_binary_expression(expr, node),
            AstStatement::UnaryExpression(expr) => self.visit_unary_expression(expr, node),
            AstStatement::ExpressionList(list) => {
                Default::default()
            }
            AstStatement::ParenExpression(expr) => self.visit_paren_expression(expr.as_ref(), node),
            AstStatement::RangeStatement(stmt) => self.visit_range_statement(stmt, node),
            AstStatement::VlaRangeStatement => self.visit_vla_range_statement(node),
            AstStatement::Assignment(assign) => self.visit_assignment(assign, node),
            AstStatement::OutputAssignment(assign) => self.visit_output_assignment(assign, node),
            AstStatement::CallStatement(call) => self.visit_call_statement(call, node),
            AstStatement::ControlStatement(ctrl) => self.visit_control_statement(ctrl, node),
            AstStatement::CaseCondition(cond) => self.visit_case_condition(cond, node),
            AstStatement::ExitStatement(_) => self.visit_exit_statement(node),
            AstStatement::ContinueStatement(_) => self.visit_continue_statement(node),
            AstStatement::ReturnStatement(ret) => self.visit_return_statement(ret, node),
            AstStatement::JumpStatement(jump) => self.visit_jump_statement(jump, node),
            AstStatement::LabelStatement(label) => self.visit_label_statement(label, node),
            AstStatement::EmptyStatement(_) => self.visit_empty_statement(node),
            AstStatement::DefaultValue(value) => self.visit_default_value(value, node),
            AstStatement::Literal(literal) => self.visit_literal(literal, node),
            AstStatement::CastStatement(call) => self.visit_cast_statement(call, node),
            AstStatement::MultipliedStatement(multi) => self.visit_multiplied_statement(multi, node),
        }
    }

    // Define a method for each variant of AstStatement
    // Each method now returns a value of type T
    fn visit_reference_expr(&mut self, _expr: &ReferenceExpr, _node: &AstNode) -> T;
    fn visit_identifier(&mut self, _id: &str, _node: &AstNode) -> T;
    fn visit_direct_access(&mut self, _access: &DirectAccess, _node: &AstNode) -> T;
    fn visit_hardware_access(&mut self, _access: &HardwareAccess, _node: &AstNode) -> T;
    fn visit_binary_expression(&mut self, _expr: &BinaryExpression, _node: &AstNode) -> T;
    fn visit_unary_expression(&mut self, _expr: &UnaryExpression, _node: &AstNode) -> T;
    fn visit_range_statement(&mut self, _stmt: &RangeStatement, _node: &AstNode) -> T;
    fn visit_expression_list(&mut self, _list: &[AstNode], _node: &AstNode) -> T;
    fn visit_paren_expression(&mut self, _expr: &AstNode, _node: &AstNode) -> T;
    fn visit_vla_range_statement(&mut self, _node: &AstNode) -> T;
    fn visit_assignment(&mut self, _assign: &Assignment, _node: &AstNode) -> T;
    fn visit_output_assignment(&mut self, _assign: &Assignment, _node: &AstNode) -> T;
    fn visit_call_statement(&mut self, _call: &CallStatement, _node: &AstNode) -> T;
    fn visit_control_statement(&mut self, _ctrl: &AstControlStatement, _node: &AstNode) -> T;
    fn visit_case_condition(&mut self, _cond: &AstNode, _node: &AstNode) -> T;
    fn visit_exit_statement(&mut self, _node: &AstNode) -> T;
    fn visit_continue_statement(&mut self, _node: &AstNode) -> T;
    fn visit_return_statement(&mut self, _ret: &ReturnStatement, _node: &AstNode) -> T;
    fn visit_jump_statement(&mut self, _jump: &JumpStatement, _node: &AstNode) -> T;
    fn visit_label_statement(&mut self, _label: &LabelStatement, _node: &AstNode) -> T;
    fn visit_empty_statement(&mut self, _node: &AstNode) -> T;
    fn visit_default_value(&mut self, _value: &DefaultValue, _node: &AstNode) -> T;
    fn visit_literal(&mut self, _literal: &AstLiteral, _node: &AstNode) -> T;
    fn visit_cast_statement(&mut self, _call: &CastStatement, _node: &AstNode) -> T;
    fn visit_multiplied_statement(&mut self, _multi: &MultipliedStatement, _node: &AstNode) -> T;
}