use ast::ast::{AstStatement, SourceRange};

use crate::model::{block::Block, fbd::NodeIndex};

use super::ParseSession;

impl Block {
    pub(crate) fn transform(&self, session: &ParseSession, index: &NodeIndex) -> AstStatement {
        let operator = Box::new(AstStatement::Reference {
            name: self.type_name.clone(),
            location: SourceRange::undefined(),
            id: session.next_id(),
        });

        let parameters = if !self.variables.is_empty() {
            Box::new(Some(AstStatement::ExpressionList {
                expressions: self
                    .variables
                    .iter()
                    .filter_map(|var| {
                        // try to transform the element this block variable points to
                        var.transform(session, index)
                    })
                    .collect(),
                id: session.next_id(),
            }))
        } else {
            Box::new(None)
        };

        AstStatement::CallStatement {
            operator,
            parameters,
            location: SourceRange::undefined(),
            id: session.next_id(),
        }
    }
}
