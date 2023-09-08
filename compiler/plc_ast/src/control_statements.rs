use std::fmt::{Debug, Formatter};

use crate::ast::AstNode;

#[derive(Clone, PartialEq)]
pub struct IfStatement {
    pub blocks: Vec<ConditionalBlock>,
    pub else_block: Vec<AstNode>,
}

#[derive(Clone, PartialEq)]
pub struct ForLoopStatement {
    pub counter: Box<AstNode>,
    pub start: Box<AstNode>,
    pub end: Box<AstNode>,
    pub by_step: Option<Box<AstNode>>,
    pub body: Vec<AstNode>,
}

#[derive(Clone, PartialEq)]
/// used for While and Repeat loops
pub struct LoopStatement {
    pub condition: Box<AstNode>,
    pub body: Vec<AstNode>,
}

#[derive(Clone, PartialEq)]
pub struct CaseStatement {
    pub selector: Box<AstNode>,
    pub case_blocks: Vec<ConditionalBlock>,
    pub else_block: Vec<AstNode>,
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
    pub condition: Box<AstNode>,
    pub body: Vec<AstNode>,
}

impl Debug for ConditionalBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConditionalBlock")
            .field("condition", &self.condition)
            .field("body", &self.body)
            .finish()
    }
}
