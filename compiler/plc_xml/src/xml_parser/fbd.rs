use indexmap::IndexMap;
use plc::ast::AstStatement;

use crate::model::fbd::{FunctionBlockDiagram, NodeId, Node};

use super::ParseSession;

impl FunctionBlockDiagram {
    pub(crate) fn transform(&self, session: &ParseSession) -> Vec<AstStatement> {
        let mut ast_association = IndexMap::new();
        self.nodes.iter().for_each(|(id, _)| self.transform_node(*id, session, &mut ast_association));
        ast_association
            .into_iter()
            .filter(|(k, _)| self.nodes.get(k).map(|it| it.get_exec_id()).is_some())
            .map(|(_, v)| v)
            .collect()
    }

    fn transform_node(
        &self,
        id: NodeId,
        session: &ParseSession,
        ast_association: &mut IndexMap<NodeId, AstStatement>,
    ) {
        let Some(current_node) = self.nodes.get(&id) else {
            unreachable!()
        };
        match current_node {
            Node::Block(block) => block.transform(session, &self.nodes, ast_association),
            Node::FunctionBlockVariable(var) => {
                var.transform(session, ast_association);

                // if we are not being assigned to, we can return here
                let Some(ref_id) = var.ref_local_id else {
                    // ast_association.insert(id, stmt, ast_association);
                    return;
                };

                let Some(rhs) = ast_association.remove(&ref_id).or_else(|| { 
                    self.transform_node(id, session, ast_association); 
                    ast_association.remove(&ref_id)
                }) else {
                    unreachable!()
                };

                let Some(lhs) = ast_association.remove(&id) else {
                    unreachable!()
                };

                ast_association.insert(
                    id,
                    AstStatement::Assignment {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        id: session.next_id(),
                    },
                );
            }
            Node::Control(_) => todo!(),
            Node::Connector(_) => todo!(),
        }
    }
}
