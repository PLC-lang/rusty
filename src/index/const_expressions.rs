// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::ast::AstStatement;
use generational_arena::Arena;

pub type ConstId = generational_arena::Index;

#[derive(Default)]
pub struct ConstExpressions {
    expressions: Arena<AstStatement>,
}

impl ConstExpressions {
    pub fn new() -> ConstExpressions {
        ConstExpressions {
            expressions: Arena::new(),
        }
    }

    pub fn add_expression(&mut self, statement: AstStatement) -> ConstId {
        self.expressions.insert(statement)
    }

    pub fn find_expression(&self, id: &ConstId) -> Option<&AstStatement> {
        self.expressions.get(*id)
    }

    pub fn remove(&mut self, id: &ConstId) -> Option<AstStatement> {
        self.expressions.remove(*id)
    }
}
