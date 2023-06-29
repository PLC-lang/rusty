use indexmap::IndexMap;
use plc::ast::AstStatement;

use crate::model::fbd::{FunctionBlockDiagram, Node, NodeId};

use super::ParseSession;

impl FunctionBlockDiagram {
    /// Transforms the body of a function block diagram to their AST-equivalent, in order of execution.
    /// Only statements that are necessary for execution logic will be selected.
    pub(crate) fn transform(&self, session: &ParseSession) -> Vec<AstStatement> {
        let mut ast_association = IndexMap::new();
        // transform each node to an ast-statement. since we might see and transform a node multiple times, we use an
        // ast-association map to keep track of the latest statement for each id
        self.nodes.iter().for_each(|(id, _)| {
            ast_association.insert(*id, self.transform_node(*id, session));
        });

        // filter the map for each statement belonging to a node with an execution id or a temp-var, discard the rest -> these have no impact
        ast_association
            .into_iter()
            .filter(|(key, _)| {
                self.nodes.get(key).is_some_and(|node| node.get_exec_id().is_some() || node.is_temp_var())
            })
            .map(|(_, v)| v)
            .collect()
    }

    fn transform_node(&self, id: NodeId, session: &ParseSession) -> AstStatement {
        let Some(current_node) = self.nodes.get(&id) else {
            unreachable!()
        };
        match current_node {
            Node::Block(block) => block.transform(session, &self.nodes),
            Node::FunctionBlockVariable(var) => {
                let lhs = var.transform(session);

                // if we are not being assigned to, we can return here
                let Some(ref_id) = var.ref_local_id else {
                    return lhs;
                };

                let rhs = self.transform_node(ref_id, session);

                AstStatement::Assignment { left: Box::new(lhs), right: Box::new(rhs), id: session.next_id() }
            }
            Node::Control(_) => todo!(),
            Node::Connector(_) => todo!(),
        }
    }
}
