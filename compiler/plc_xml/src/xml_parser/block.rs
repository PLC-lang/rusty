use ast::ast::{AstFactory, AstStatement, SourceRange};

use crate::model::{block::Block, fbd::NodeIndex};

use super::ParseSession;

impl Block {
    pub(crate) fn transform(&self, session: &ParseSession, index: &NodeIndex) -> AstStatement {
        let parameters = self
            .variables
            .iter()
            .filter_map(|var| {
                // try to transform the element this block variable points to
                var.transform(session, index)
            })
            .collect();

        AstFactory::create_call_to(
            self.type_name.clone(),
            parameters,
            session.next_id(),
            session.next_id(),
            &SourceRange::undefined(),
        )
    }
}
