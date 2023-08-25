use ast::ast::{AstStatement, Operator, SourceRange};

use crate::{
    error::Error,
    model::{
        control::{Control, ControlKind},
        fbd::{Node, NodeIndex},
    },
};

use super::ParseSession;

impl Control {
    pub(crate) fn transform(&self, session: &ParseSession, index: &NodeIndex) -> AstStatement {
        match self.kind {
            ControlKind::Jump => unimplemented!(),
            ControlKind::Label => unimplemented!(),
            ControlKind::Return => transform_return(self, session, index),
        }
    }
}

/// Takes a [`ControlKind::Return`] variant and transforms it into a [`AstStatement::ReturnStatement`] with
/// its [`AstStatement::ReturnStatement::condition`] field populated.
fn transform_return(control: &Control, session: &ParseSession, index: &NodeIndex) -> AstStatement {
    let Some(ref_local_id) = control.ref_local_id else { todo!("error") };
    let Some(node) = index.get(&ref_local_id) else { todo!("error") };

    let condition = match node {
        Node::FunctionBlockVariable(variable) => variable.transform(session),
        _ => todo!("error"),
    };

    // XXX: Introduce trait for negation, because we'll probably need it more often
    let possibly_negated_condition = if control.negated {
        AstStatement::UnaryExpression {
            operator: Operator::Not,
            location: condition.get_location(),
            value: Box::new(condition),
            id: session.next_id(),
        }
    } else {
        condition
    };

    AstStatement::ReturnStatement {
        condition: Some(Box::new(possibly_negated_condition)),
        location: SourceRange::undefined(),
        id: session.next_id(),
    }
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
