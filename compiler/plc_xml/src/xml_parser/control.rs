use plc::ast::{AstStatement, SourceRange};

use crate::model::{
    control::{Control, ControlKind},
    fbd::NodeIndex,
};

use super::ParseSession;

impl Control {
    pub(crate) fn transform(&self, session: &ParseSession, index: &NodeIndex) -> AstStatement {
        if self.kind != ControlKind::Return {
            unimplemented!()
        }

        let Some(ref_local_id) = self.ref_local_id else { unreachable!() };
        let ref_ast = match index.get(&ref_local_id) {
            Some(val) => match val {
                crate::model::fbd::Node::Block(val) => val.transform(session, index),
                crate::model::fbd::Node::FunctionBlockVariable(val) => val.transform(session),
                // crate::model::fbd::Node::Control(val) => val.transform(session, index),
                crate::model::fbd::Node::Control(_) => todo!(),
                crate::model::fbd::Node::Connector(_) => todo!(),
            },
            None => todo!(),
        };

        AstStatement::ReturnStatement {
            condition: Some(Box::new(ref_ast.to_owned())),
            location: SourceRange::undefined(),
            id: session.next_id(),
        }
    }
}
