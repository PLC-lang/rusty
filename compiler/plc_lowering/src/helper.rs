//! Shared helpers for determining whether an expression tree is compile-time constant.

use plc::index::Index;
use plc_ast::ast::{AstNode, AstStatement, MultipliedStatement};

/// Returns `true` if every leaf in the expression tree can be evaluated at
/// compile time. Literals and references to constant variables (when `index`
/// and `pou_name` are provided) are considered constant. Function calls,
/// struct literal assignments, and non-constant variable references are not.
pub fn is_const_expression(node: &AstNode, index: Option<&Index>, pou_name: Option<&str>) -> bool {
    match node.get_stmt() {
        AstStatement::Literal(..) => true,
        AstStatement::ExpressionList(exprs) => exprs.iter().all(|e| is_const_expression(e, index, pou_name)),
        AstStatement::MultipliedStatement(MultipliedStatement { multiplier, element }) => {
            is_const_expression(multiplier, index, pou_name) && is_const_expression(element, index, pou_name)
        }
        AstStatement::ParenExpression(inner) => is_const_expression(inner, index, pou_name),
        AstStatement::Identifier(..) | AstStatement::ReferenceExpr(..) => {
            // If we have an index, check whether this is a reference to a constant variable.
            index.is_some_and(|idx| is_const_reference(node, idx, pou_name))
        }
        // Everything else: function calls, struct literal assignments, etc.
        _ => false,
    }
}

/// Returns `true` if the node is a reference to a constant variable.
fn is_const_reference(node: &AstNode, index: &Index, pou_name: Option<&str>) -> bool {
    let Some(name) = node.get_flat_reference_name() else { return false };

    // Check as POU-local member first, then as global.
    let variable =
        pou_name.and_then(|pou| index.find_member(pou, name)).or_else(|| index.find_global_variable(name));

    variable.is_some_and(|v| v.is_constant())
}
