use ast::ast::{AstStatement, Implementation, PouType as AstPouType};
use plc_source::source_location::SourceLocation;

use crate::model::action::Action;

use super::ParseSession;

impl Action {
    pub(crate) fn transform(&self, _session: &ParseSession) -> Vec<AstStatement> {
        todo!()
    }

    pub(crate) fn build_implementation(&self, session: &ParseSession) -> Implementation {
        let statements = self.transform(session);

        Implementation {
            name: self.name.to_owned(),
            type_name: self.type_name.to_owned(),
            linkage: session.linkage,
            pou_type: AstPouType::Action,
            statements,
            location: SourceLocation::undefined(),
            name_location: SourceLocation::undefined(),
            overriding: false,
            generic: false,
            access: None,
        }
    }
}
