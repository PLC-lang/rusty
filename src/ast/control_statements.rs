use super::{AstId, AstStatement, ConditionalBlock, SourceRange};

#[derive(Clone, PartialEq)]
pub struct IfStatement {
    pub blocks: Vec<ConditionalBlock>,
    pub else_block: Vec<AstStatement>,
}

#[derive(Clone, PartialEq)]
pub enum AstControlStatement {
    IfStatement(IfStatement),
}

impl AstControlStatement {
    pub fn if_statement(
        blocks: Vec<ConditionalBlock>,
        else_block: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::IfStatement(IfStatement { blocks, else_block }),
            location,
            id,
        }
    }
}
