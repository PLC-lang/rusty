use indexmap::IndexMap;
use plc::ast::{AstStatement, Operator};

use crate::model::{
    fbd::{Node, NodeIndex},
    variables::{BlockVariable, FunctionBlockVariable},
};

use super::ParseSession;

impl BlockVariable {
    pub(crate) fn transform(
        &self,
        session: &ParseSession,
        index: &NodeIndex,
        ast_association: &mut IndexMap<usize, AstStatement>,
    ) {
        let Some(ref_id) = &self.ref_local_id else {
            // param not provided/passed
            return;
        };

        if ast_association.get(ref_id).is_some() {
            // we have already transformed the referenced element
            return;
        }

        match index.get(ref_id) {
            Some(Node::Block(block)) => {
                // XXX: chaining blocks happens here. we might solve this with
                // temp variables in future
                block.transform(session, index, ast_association);
            }
            Some(Node::FunctionBlockVariable(var)) => var.transform(session, ast_association),
            Some(Node::Control(_)) => todo!(),
            Some(Node::Connector(_)) => todo!(),
            None => unreachable!(),
        }
    }
}

impl FunctionBlockVariable {
    pub(crate) fn transform(
        &self,
        session: &ParseSession,
        ast_association: &mut IndexMap<usize, AstStatement>,
    ) {
        let stmt = if self.negated {
            let ident = session.parse_expression(&self.expression);
            let location = ident.get_location();
            AstStatement::UnaryExpression {
                operator: Operator::Not,
                value: Box::new(ident),
                location,
                id: session.next_id(),
            }
        } else {
            session.parse_expression(&self.expression)
        };

        ast_association.insert(self.local_id, stmt);
    }
}
