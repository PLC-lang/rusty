use ast::ast::{AstStatement, Operator};

use crate::model::{
    fbd::{Node, NodeIndex},
    variables::{BlockVariable, FunctionBlockVariable},
};

use super::ParseSession;

impl BlockVariable {
    pub(crate) fn transform(&self, session: &ParseSession, index: &NodeIndex) -> Option<AstStatement> {
        let Some(ref_id) = &self.ref_local_id else {
            // param not provided/passed
            return None;
        };

        // XXX: data-recursion?
        match index.get(ref_id) {
            Some(Node::Block(block)) => Some(block.transform(session, index)),
            Some(Node::FunctionBlockVariable(var)) => Some(var.transform(session)),
            Some(Node::Control(_)) => todo!(),
            Some(Node::Connector(_)) => todo!(),
            None => unreachable!(),
        }
    }
}

// variables, parameters -> more readable names?
impl FunctionBlockVariable {
    pub(crate) fn transform(&self, session: &ParseSession) -> AstStatement {
        if self.negated {
            let ident = session.parse_expression(&self.expression, self.local_id);

            AstStatement::UnaryExpression {
                operator: Operator::Not,
                value: Box::new(ident),
                location: session.create_id_location(self.local_id),
                id: session.next_id(),
            }
        } else {
            session.parse_expression(&self.expression, self.local_id)
        }
    }
}
