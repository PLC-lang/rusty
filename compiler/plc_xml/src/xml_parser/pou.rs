use plc::ast::{AstStatement, Implementation, SourceRange};

use crate::model::pou::Pou;

use super::ParseSession;

impl Pou {
    fn transform(&self, session: &mut ParseSession) -> Vec<AstStatement> {
        let Some(fbd) = &self.body.function_block_diagram else {
            // empty body
            return vec![]
        };

        let statements = fbd.transform(session);

        #[cfg(feature = "debug")]
        println!("{statements:#?}");

        statements
    }

    // TODO: sourcerange
    pub fn build_implementation(&self, session: &mut ParseSession) -> Implementation {
        Implementation {
            name: self.name.to_owned(),
            type_name: self.name.to_owned(),
            linkage: session.linkage,
            pou_type: self.pou_type.into(),
            statements: self.transform(session),
            location: SourceRange::undefined(),
            name_location: SourceRange::undefined(),
            overriding: false,
            generic: false,
            access: None,
        }
    }
}
