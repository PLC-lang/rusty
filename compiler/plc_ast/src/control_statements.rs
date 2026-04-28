use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use plc_source::source_location::SourceLocation;

use crate::ast::AstNode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct IfStatement {
    pub blocks: Vec<ConditionalBlock>,
    pub else_block: Vec<AstNode>,
    pub end_location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct ForLoopStatement {
    pub counter: Box<AstNode>,
    pub start: Box<AstNode>,
    pub end: Box<AstNode>,
    pub by_step: Option<Box<AstNode>>,
    pub body: Vec<AstNode>,
    pub end_location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
/// used for While and Repeat loops
pub struct LoopStatement {
    pub condition: Box<AstNode>,
    pub body: Vec<AstNode>,
    pub end_location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct CaseStatement {
    pub selector: Box<AstNode>,
    pub case_blocks: Vec<ConditionalBlock>,
    pub else_block: Vec<AstNode>,
    pub end_location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub enum AstControlStatement {
    If(IfStatement),
    ForLoop(ForLoopStatement),
    WhileLoop(LoopStatement),
    RepeatLoop(LoopStatement),
    Case(CaseStatement),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct ConditionalBlock {
    pub condition: Box<AstNode>,
    pub body: Vec<AstNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct ReturnStatement {
    /// Indicates that the given condition must evaluate to true in order for the return to take place.
    /// Only used in CFC where the condition may be [`Some`] and [`None`] otherwise.
    pub condition: Option<Box<AstNode>>,
}

impl ForLoopStatement {
    pub fn get_conditionals(&self) -> Vec<&AstNode> {
        let mut conditionals = Vec::new();

        conditionals.push(self.counter.as_ref());
        conditionals.push(self.start.as_ref());
        conditionals.push(self.end.as_ref());
        if let Some(ref step) = self.by_step {
            conditionals.push(step);
        }

        conditionals
    }
}
