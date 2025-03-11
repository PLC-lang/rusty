use ast::ast::{AstNode, Implementation, PouType as AstPouType};

use crate::model::action::Action;

use super::ParseSession;

impl Action<'_> {
    pub(crate) fn transform(&self, session: &mut ParseSession) -> Vec<AstNode> {
        let fbd = &self.body.function_block_diagram;

        if cfg!(feature = "debug") {
            let statements = fbd.transform(session);
            println!("{statements:#?}");

            return statements;
        }

        fbd.transform(session)
    }

    pub(crate) fn build_implementation(&self, session: &mut ParseSession) -> Implementation {
        let statements = self.transform(session);

        Implementation {
            name: format!("{}.{}", self.type_name, self.name),
            type_name: self.type_name.to_string(),
            linkage: session.linkage,
            pou_type: AstPouType::Action,
            statements,
            location: session.create_file_only_location(),
            name_location: session.create_file_only_location(),
            end_location: session.create_file_only_location(),
            overriding: false,
            generic: false,
            access: None,
        }
    }
}
