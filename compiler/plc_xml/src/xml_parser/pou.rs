use ast::ast::{AstNode, Implementation};

use crate::model::pou::Pou;

use super::ParseSession;

impl<'xml> Pou<'xml> {
    fn transform(&self, session: &mut ParseSession) -> Vec<AstNode> {
        let Some(fbd) = &self.body.function_block_diagram else {
            // empty body
            return vec![];
        };

        if cfg!(feature = "debug") {
            let statements = fbd.transform(session);
            println!("{statements:#?}");

            return statements;
        }

        fbd.transform(session)
    }

    pub fn build_implementation(&self, session: &mut ParseSession) -> Implementation {
        let statements = self.transform(session);

        Implementation {
            name: self.name.to_string(),
            type_name: self.name.to_string(),
            linkage: session.linkage,
            pou_type: self.pou_type.into(),
            statements,
            location: session.create_file_only_location(),
            name_location: session.create_file_only_location(),
            overriding: false,
            generic: false,
            access: None,
        }
    }
}
