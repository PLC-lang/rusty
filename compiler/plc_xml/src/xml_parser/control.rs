use ast::ast::{AstStatement, SourceRange};

use crate::model::{
    control::{Control, ControlKind},
    fbd::{Node, NodeIndex},
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

// TODO: Describe what's happening
fn transform_return(control: &Control, session: &ParseSession, index: &NodeIndex) -> AstStatement {
    let Some(ref_local_id) = control.ref_local_id else { unreachable!() };

    let Some(node) = index.get(&ref_local_id) else { todo!("error") };
    let condition = match node {
        Node::FunctionBlockVariable(variable) => variable.transform(session),

        Node::Block(_) => todo!(),
        Node::Control(control) => match control.kind {
            ControlKind::Return => todo!("error, returns can not be chained"),
            _ => todo!("..."), // TODO: Can we chain other control elements with a return?
        },
        Node::Connector(_) => todo!(),
    };

    AstStatement::ReturnStatement {
        condition: Some(Box::new(condition)),
        location: SourceRange::undefined(),
        id: session.next_id(),
    }
}
