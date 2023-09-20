use ast::ast::{AstFactory, AstNode, AstStatement};
use indexmap::IndexMap;
use plc_source::source_location::SourceLocation;

use crate::model::fbd::{FunctionBlockDiagram, Node, NodeId};

use super::ParseSession;

impl<'xml> FunctionBlockDiagram<'xml> {
    /// Transforms the body of a function block diagram to their AST-equivalent, in order of execution.
    /// Only statements that are necessary for execution logic will be selected.
    pub(crate) fn transform(&self, session: &mut ParseSession) -> Vec<AstNode> {
        let mut ast_association = IndexMap::new();

        // transform each node to an ast-statement. since we might see and transform a node multiple times, we use an
        // ast-association map to keep track of the latest statement for each id
        self.nodes.iter().for_each(|(id, _)| {
            let (insert, remove_id) = self.transform_node(*id, session, &ast_association);

            if let Some(id) = remove_id {
                ast_association.remove(&id);
            };

            ast_association.insert(*id, insert);
        });

        // filter the map for each statement belonging to a node with an execution id or a temp-var, discard the rest -> these have no impact
        ast_association
            .into_iter()
            .filter(|(key, _)| self.nodes.get(key).is_some_and(|node| node.get_exec_id().is_some()))
            .map(|(_, value)| value)
            .collect()
    }

    fn transform_node(
        &self,
        id: NodeId,
        session: &mut ParseSession,
        ast_association: &IndexMap<usize, AstNode>,
    ) -> (AstNode, Option<NodeId>) {
        let Some(current_node) = self.nodes.get(&id) else { unreachable!() };

        match current_node {
            Node::Block(block) => (block.transform(session, &self.nodes), None),
            Node::FunctionBlockVariable(var) => {
                let lhs = var.transform(session);

                // if we are not being assigned to, we can return here
                let Some(ref_id) = var.ref_local_id else {
                    return (lhs, None);
                };

                let (rhs, remove_id) = ast_association
                    .get(&ref_id)
                    .map(|stmt| {
                        if matches!(stmt.get_stmt(), AstStatement::CallStatement(..)) {
                            (stmt.clone(), Some(ref_id))
                        } else {
                            self.transform_node(ref_id, session, ast_association)
                        }
                    })
                    .expect("Expected AST statement, found None");

                (AstFactory::create_assignment(lhs, rhs, session.next_id()), remove_id)
            }

            Node::Control(control) => match control.transform(session, &self.nodes) {
                Ok(value) => (value, None),
                Err(why) => {
                    session.diagnostics.push(why);
                    (AstFactory::create_empty_statement(SourceLocation::undefined(), session.next_id()), None)
                }
            },
            Node::Connector(_) => todo!(),
        }
    }
}
