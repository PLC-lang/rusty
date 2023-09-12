use ast::ast::{AstFactory, AstNode};
use plc_diagnostics::diagnostics::Diagnostic;

use crate::model::{
    control::{Control, ControlKind},
    fbd::{Node, NodeIndex},
};

use super::ParseSession;

impl<'xml> Control<'xml> {
    pub(crate) fn transform(&self, session: &ParseSession, index: &NodeIndex) -> Result<AstNode, Diagnostic> {
        match self.kind {
            ControlKind::Jump => unimplemented!(),
            ControlKind::Label => unimplemented!(),
            ControlKind::Return => transform_return(self, session, index),
        }
    }
}

fn transform_return(
    control: &Control,
    session: &ParseSession,
    index: &NodeIndex,
) -> Result<AstNode, Diagnostic> {
    let Some(ref_local_id) = control.ref_local_id else {
        let location = session.range_factory.create_block_location(control.local_id, None);
        return Err(Diagnostic::empty_control_statement(location));
    };

    let Some(node) = index.get(&ref_local_id) else {
        let location = session.range_factory.create_block_location(ref_local_id, None);
        return Err(Diagnostic::undefined_node(control.local_id, ref_local_id, location));
    };

    let condition = match node {
        Node::FunctionBlockVariable(variable) => Ok(variable.transform(session)),
        Node::Block(block) => Ok(block.transform(session, index)),

        _ => {
            let location_control = session.range_factory.create_block_location(control.local_id, None);
            let location_other = session.range_factory.create_block_location(ref_local_id, None);

            Err(Diagnostic::unexpected_nodes(location_control.span(&location_other)))
        }
    }?;

    // XXX: Introduce trait / helper-function for negation, because we'll probably need it more often
    let possibly_negated_condition = if control.negated {
        let location = condition.get_location();
        AstFactory::create_not_expression(condition, location, session.next_id())
    } else {
        condition
    };

    Ok(AstFactory::create_return_statement(
        Some(possibly_negated_condition),
        session.create_block_location(control.local_id, control.execution_order_id),
        session.next_id(),
    ))
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{
        model::control::Control,
        reader::PeekableReader,
        serializer::{XAddData, XConnectionPointIn, XReturn},
        xml_parser::Parseable,
    };

    #[test]
    fn simple_return() {
        let content = XReturn::new()
            .with_local_id("1")
            .with_execution_order_id("2")
            .with_connection_point_in(XConnectionPointIn::with_ref("3"))
            .with_add_data(XAddData::negated(false))
            .serialize();

        let reader = &mut PeekableReader::new(&content);
        assert_debug_snapshot!(Control::visit(reader).unwrap());
    }

    #[test]
    fn simple_negated_return() {
        let content = XReturn::new()
            .with_local_id("1")
            .with_execution_order_id("2")
            .with_connection_point_in(XConnectionPointIn::with_ref("3"))
            .with_add_data(XAddData::negated(true))
            .serialize();

        let reader = &mut PeekableReader::new(&content);
        assert_debug_snapshot!(Control::visit(reader).unwrap());
    }
}
