use plc::ast::{AstStatement, Implementation, PouType as AstPouType, SourceRange};

use crate::model::action::Action;

use super::ParseSession;

impl Action {
    pub(crate) fn transform(&self, _session: &ParseSession) -> Vec<AstStatement> {
        todo!()
    }

    // TODO: sourcerange
    pub(crate) fn build_implementation(&self, session: &ParseSession) -> Implementation {
        let statements = self.transform(session);

        Implementation {
            name: self.name.to_owned(),
            type_name: self.type_name.to_owned(),
            linkage: session.linkage,
            pou_type: AstPouType::Action,
            statements,
            location: SourceRange::undefined(),
            name_location: SourceRange::undefined(),
            overriding: false,
            generic: false,
            access: None,
        }
    }
}
