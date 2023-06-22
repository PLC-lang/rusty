use plc::ast::{AstStatement, Implementation, PouType as AstPouType, SourceRange};

use crate::model::action::Action;

use super::ParseSession;

impl Action {
    pub(crate) fn transform(&self, session: &mut ParseSession) -> Vec<AstStatement> {
        todo!()
    }

    // TODO: sourcerange
    pub(crate) fn build_implementation(&self, session: &mut ParseSession) -> Implementation {
        Implementation {
            name: self.name.to_owned(),
            type_name: self.type_name.to_owned(),
            linkage: session.linkage,
            pou_type: AstPouType::Action,
            statements: self.transform(session),
            location: SourceRange::undefined(),
            name_location: SourceRange::undefined(),
            overriding: false,
            generic: false,
            access: None,
        }
    }
}
