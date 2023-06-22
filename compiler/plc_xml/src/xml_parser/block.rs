use indexmap::IndexMap;
use plc::ast::{AstStatement, SourceRange};

use crate::model::{block::Block, fbd::NodeIndex};

use super::ParseSession;

impl Block {
    pub(crate) fn transform(
        &self,
        session: &ParseSession,
        index: &NodeIndex,
        ast_association: &mut IndexMap<usize, AstStatement>,
    ) {
        let operator = Box::new(AstStatement::Reference {
            name: self.type_name.clone(),
            location: SourceRange::undefined(),
            id: session.next_id(),
        });

        let parameters = if self.variables.len() > 0 {
            Box::new(Some(AstStatement::ExpressionList {
                expressions: self
                    .variables
                    .iter()
                    .filter_map(|var| {
                        var.transform(session, index, ast_association);
                        // this might be a problem if multiple blocks reference the same var
                        var.ref_local_id.map(|id| ast_association.remove(&id)).unwrap_or(None)
                    })
                    .collect(),
                id: session.next_id(),
            }))
        } else {
            Box::new(None)
        };

        ast_association.insert(
            self.local_id,
            AstStatement::CallStatement {
                operator,
                parameters,
                location: SourceRange::undefined(),
                id: session.next_id(),
            },
        );
    }
}
