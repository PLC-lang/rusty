use std::fmt::{Debug, Formatter, Result};

use super::{AstId, AstStatement, SourceRange};

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
pub struct CaseStatement {
    pub selector: Box<AstStatement>,
    pub case_blocks: Vec<ConditionalBlock>,
    pub else_block: Vec<AstStatement>,
}

#[derive(Clone, PartialEq)]
pub enum AstControlStatement {
    If(IfStatement),
    ForLoop(ForLoopStatement),
    WhileLoop(LoopStatement),
    RepeatLoop(LoopStatement),
    Case(CaseStatement),
}

#[derive(Clone, PartialEq)]
pub struct ConditionalBlock {
    pub condition: Box<AstStatement>,
    pub body: Vec<AstStatement>,
}

impl Debug for ConditionalBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ConditionalBlock")
            .field("condition", &self.condition)
            .field("body", &self.body)
            .finish()
    }
}

impl AstControlStatement {
    pub fn new_if_statement(
        blocks: Vec<ConditionalBlock>,
        else_block: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::If(IfStatement { blocks, else_block }),
            location,
            id,
        }
    }

    pub fn new_for_loop(
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

    pub fn new_while_statement(
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

    pub fn new_repeat_statement(
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

    pub fn new_case_statement(
        selector: AstStatement,
        case_blocks: Vec<ConditionalBlock>,
        else_block: Vec<AstStatement>,
        location: SourceRange,
        id: AstId,
    ) -> AstStatement {
        AstStatement::ControlStatement {
            kind: AstControlStatement::Case(CaseStatement {
                selector: Box::new(selector),
                case_blocks,
                else_block,
            }),
            id,
            location,
        }
    }
}
