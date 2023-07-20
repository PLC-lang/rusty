use std::fmt::{Debug, Formatter, Result};

use super::AstStatement;

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
