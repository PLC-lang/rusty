use ast::ast::{AstFactory, AstNode, Operator};

use crate::model::{
    fbd::{Node, NodeIndex},
    variables::{BlockVariable, FunctionBlockVariable},
};

use super::ParseSession;

impl BlockVariable {
    pub(crate) fn transform(&self, session: &ParseSession, index: &NodeIndex) -> Option<AstNode> {
        let Some(ref_id) = &self.ref_local_id else {
            // param not provided/passed
            return None;
        };

        // XXX: data-recursion?
        match index.get(ref_id) {
            Some(Node::Block(block)) => Some(block.transform(session, index)),
            Some(Node::FunctionBlockVariable(var)) => Some(var.transform(session)),
            Some(Node::Control(_)) => todo!(),
            Some(Node::Connector(_)) => unreachable!(),
            None => unreachable!(),
        }
    }
}

// variables, parameters -> more readable names?
impl<'xml> FunctionBlockVariable<'xml> {
    pub(crate) fn transform(&self, session: &ParseSession) -> AstNode {
        if self.negated {
            let ident = session.parse_expression(&self.expression, self.local_id, self.execution_order_id);

            AstFactory::create_unary_expression(
                Operator::Not,
                ident,
                session.create_block_location(self.local_id, self.execution_order_id),
                session.next_id(),
            )
        } else {
            session.parse_expression(&self.expression, self.local_id, self.execution_order_id)
        }
    }
}
