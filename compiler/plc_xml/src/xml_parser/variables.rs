use indexmap::IndexMap;
use plc::ast::{AstStatement, Operator};

use crate::model::{
    fbd::{Node, NodeId, NodeIndex},
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

    fn as_param(
        &self,
        _block_id: NodeId,
        _session: &mut ParseSession,
        _index: &IndexMap<usize, Node>,
        ast_association: &mut IndexMap<NodeId, AstStatement>,
    ) -> Option<AstStatement> {
        if let Some(ref_id) = self.ref_local_id {
            // let param = if matches!(index.get(&ref_id).unwrap(), Node::Block(_)) {
            //     // we are directly chaining blocks -> temp var needed
            //     if let Some(previous) = ast_association.get(&ref_id) {
            //         let temp_var = if matches!(previous, AstStatement::Reference { .. }) {
            //             previous.clone()
            //         } else {
            //             AstStatement::Reference { name: format!("__{}", previous.get_id()), location: SourceRange::undefined(), id: session.next_id() }
            //         };
            //     } else {
            //         panic!("unhandled missing block")
            //     };
            //     todo!()
            // } else {
            //     ast_association.remove(&ref_id)
            // };

            // param
            ast_association.remove(&ref_id)
        } else {
            None
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
