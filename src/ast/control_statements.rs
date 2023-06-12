use super::{AstId, AstStatement, ConditionalBlock, SourceRange};

#[derive(Clone, PartialEq)]
pub struct IfStatement {
    pub blocks: Vec<ConditionalBlock>,
    pub else_block: Vec<AstStatement>,
}

#[derive(Clone, PartialEq)]
pub struct ForLoopStatement {
    pub counter: Box<AstStatement>,
    pub start: Box<AstStatement>,
    pub end: Box<AstStatement>,
    pub by_step: Option<Box<AstStatement>>,
    pub body: Vec<AstStatement>,
}

#[derive(Clone, PartialEq)]
/// used for While and Repeat loops
pub struct LoopStatement {
    pub condition: Box<AstStatement>,
    pub body: Vec<AstStatement>,
}

#[derive(Clone, PartialEq)]
pub enum AstControlStatement {
    IfStatement(IfStatement),
    ForLoop(ForLoopStatement),
    WhileLoop(LoopStatement),
    RepeatLoop(LoopStatement),
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

    pub fn for_loop(
        counter: AstStatement,
        start: AstStatement,
        end: AstStatement,
        by_step: Option<AstStatement>,
        body: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::ForLoop(ForLoopStatement {
                counter: Box::new(counter),
                start: Box::new(start),
                end: Box::new(end),
                by_step: by_step.map(Box::new),
                body,
            }),
            location,
            id,
        }
    }

    pub fn while_statement(
        condition: AstStatement,
        body: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::WhileLoop(LoopStatement { condition: Box::new(condition), body }),
            id,
            location,
        }
    }

    pub fn repeat_statement(
        condition: AstStatement,
        body: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::RepeatLoop(LoopStatement { condition: Box::new(condition), body }),
            id,
            location,
        }
    }
}
