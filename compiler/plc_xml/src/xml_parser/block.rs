use ast::ast::{AstFactory, AstNode};
use std::borrow::Cow;

use crate::model::{block::Block, fbd::NodeIndex};

use super::ParseSession;

impl<'xml> Block<'xml> {
    pub(crate) fn transform(&self, session: &ParseSession, index: &NodeIndex) -> AstNode {
        let parameters = self
            .variables
            .iter()
            .filter_map(|var| {
                // try to transform the element this block variable points to
                var.transform(session, index)
            })
            .collect();

        AstFactory::create_call_to(
            self.instance_name.as_ref().map(Cow::to_string).unwrap_or(self.type_name.to_string()),
            parameters,
            session.next_id(),
            session.next_id(),
            &session.create_block_location(self.local_id, self.execution_order_id),
        )
    }
}
