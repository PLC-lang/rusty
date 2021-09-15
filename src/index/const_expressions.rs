use generational_arena::Arena;

use crate::ast::AstStatement;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

pub type ConstId = generational_arena::Index;
pub struct ConstExpressions{
    expressions: Arena::<AstStatement>,
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

    pub fn add_maybe(&mut self, statement: &Option<AstStatement>) -> Option<ConstId> {
        statement.as_ref().map(|it| self.add_expression(it.clone()))
    }

    pub fn find_expression(&self, id: &ConstId) -> Option<&AstStatement> {
        self.expressions.get(*id)
    }

    pub fn remove(&mut self, id: &ConstId) -> Option<AstStatement> {
        self.expressions.remove(*id)
    }
}

