// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use generational_arena::{Arena, Iter};
use plc_ast::{
    ast::{AstNode, AstStatement},
    literals::AstLiteral,
};

use plc_source::source_location::SourceLocation;
pub type ConstId = generational_arena::Index;

/// wrapper around ConstExpression stored in the arena
/// changing expr allows to change the referenced const-expression
/// without aquiring a new ID in the arena
#[derive(Debug)]
struct ConstWrapper {
    /// the constant expression
    expr: ConstExpression,
    /// the name of the data_type that this should resolve to (is this really always known?)
    target_type_name: String,
}

impl ConstWrapper {
    pub fn get_statement(&self) -> &AstNode {
        self.expr.get_statement()
    }
}

/// constant expressions registered here are wrapped behind this enum to indicate
/// whether this expression was already (potentially) resolved or not, or if a
/// resolving failed.
#[derive(Debug)]
pub enum ConstExpression {
    Unresolved {
        statement: AstNode,
        /// optional qualifier used when evaluating this expression
        /// e.g. a const-expression inside a POU would use this POU's name as a
        /// qualifier.
        scope: Option<String>,
    },
    Resolved(AstNode),
    Unresolvable {
        statement: AstNode,
        reason: UnresolvableKind,
    },
}

impl ConstExpression {
    /// returns the const-expression represented as an AST-element
    pub fn get_statement(&self) -> &AstNode {
        match &self {
            ConstExpression::Unresolved { statement, .. }
            | ConstExpression::Resolved(statement)
            | ConstExpression::Unresolvable { statement, .. } => statement,
        }
    }

    /// returns an optional qualifier that should be used as a scope when
    /// resolving this ConstExpression (e.g. the host's POU-name)
    pub fn get_qualifier(&self) -> Option<&str> {
        match &self {
            ConstExpression::Unresolved { scope, .. } => scope.as_ref().map(|it| it.as_str()),
            _ => None,
        }
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self, ConstExpression::Resolved(_))
    }

    pub(crate) fn is_default(&self) -> bool {
        self.get_statement().is_default_value()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnresolvableKind {
    /// Indicates that the const expression was not resolvable for any reason not listed in [`UnresolvableKind`].
    Misc(String),

    /// Indicates that the const expression was not resolvable because it would yield an overflow.
    Overflow(String, SourceLocation),

    /// Indicates that the const expression is not resolvable before initialization during codegen
    InitLater { initializer: Box<AstNode>, scope: Option<String>, /* XXX: target type name might be needed here */ },
}

impl UnresolvableKind {
    pub fn get_reason(&self) -> &str {
        match self {
            UnresolvableKind::Misc(val) | UnresolvableKind::Overflow(val, ..) => val,
            UnresolvableKind::InitLater { .. } => "Init later",
        }
    }

    pub fn is_misc(&self) -> bool {
        matches!(self, UnresolvableKind::Misc(..))
    }

    pub fn is_overflow(&self) -> bool {
        matches!(self, UnresolvableKind::Overflow(..))
    }
}

#[derive(Default, Debug)]
pub struct ConstExpressions {
    expressions: Arena<ConstWrapper>,
}

impl ConstExpressions {
    pub fn new() -> ConstExpressions {
        ConstExpressions { expressions: Arena::new() }
    }

    /// adds the const expression `statement`
    /// - `statement`: the const expression to add
    /// - `target_type_name`: the datatype this expression will be assigned to
    /// - `scope`: the scope this expression needs to be resolved in (e.g. a POU's name)
    pub fn add_expression(
        &mut self,
        statement: AstNode,
        target_type_name: String,
        scope: Option<String>,
    ) -> ConstId {
        self.expressions
            .insert(ConstWrapper { expr: ConstExpression::Unresolved { statement, scope }, target_type_name })
    }

    /// returns the expression associated with the given `id` together with an optional
    /// `qualifier` that represents the expressions scope  (e.g. the host's POU-name)
    pub fn find_expression(&self, id: &ConstId) -> (Option<&AstNode>, Option<&str>) {
        self.expressions
            .get(*id)
            .filter(|it| !it.expr.is_default())
            .map(|it| (Some(it.expr.get_statement()), it.expr.get_qualifier()))
            .unwrap_or((None, None))
    }

    pub fn find_expression_target_type(&self, id: &ConstId) -> Option<&str> {
        self.expressions.get(*id).map(|it| it.target_type_name.as_str())
    }

    /// similar to `find_expression` but it does not return the `AstStatement` directly.
    /// it returns a ConstExpression wrapper that indicates whether this expression
    /// was successfully resolved yet or not
    pub fn find_const_expression(&self, id: &ConstId) -> Option<&ConstExpression> {
        self.expressions.get(*id).map(|it| &it.expr)
    }

    /// clones the expression in the ConstExpressions and returns all of its elements
    pub fn clone(&self, id: &ConstId) -> Option<(AstNode, String, Option<String>)> {
        self.expressions.get(*id).map(|it| match &it.expr {
            ConstExpression::Unresolved { statement, scope } => {
                (statement.clone(), it.target_type_name.clone(), scope.clone())
            }
            ConstExpression::Resolved(s) | ConstExpression::Unresolvable { statement: s, .. } => {
                (s.clone(), it.target_type_name.clone(), None)
            }
        })
    }

    /// marks the const-expression represented by the given `id` as resolvend and stores the the
    /// given `new_statement` as it's resolved value.
    pub fn mark_resolved(&mut self, id: &ConstId, new_statement: AstNode) -> Result<(), String> {
        let wrapper = self
            .expressions
            .get_mut(*id)
            .ok_or_else(|| format!("Cannot find constant expression with id: {id:?}"))?;

        wrapper.expr = ConstExpression::Resolved(new_statement);
        Ok(())
    }

    /// marks the const-expression represented by the given `id` as unresolvable with a given
    /// `reason`.
    pub fn mark_unresolvable(&mut self, id: &ConstId, reason: UnresolvableKind) -> Result<(), String> {
        let wrapper = self
            .expressions
            .get_mut(*id)
            .ok_or_else(|| format!("Cannot find constant expression with id: {id:?}"))?;

        wrapper.expr = ConstExpression::Unresolvable { statement: wrapper.get_statement().clone(), reason };

        Ok(())
    }

    /// adds the given constant expression to the constants arena and returns the ID to reference it
    /// - `expr`: the const expression to add
    /// - `target_type`: the datatype this expression will be assigned to
    /// - `scope`: the scope this expression needs to be resolved in (e.g. a POU's name)
    pub fn add_constant_expression(
        &mut self,
        expr: AstNode,
        target_type: String,
        scope: Option<String>,
    ) -> ConstId {
        self.add_expression(expr, target_type, scope)
    }

    /// convinience-method to add the constant exression if there is some, otherwhise not
    /// use this only as a shortcut if you have an Option<AstStatement> - e.g. an optional initializer.
    /// otherwhise use `add_constant_expression`
    pub fn maybe_add_constant_expression(
        &mut self,
        expr: Option<AstNode>,
        target_type_name: &str,
        scope: Option<String>,
    ) -> Option<ConstId> {
        expr.map(|it| self.add_constant_expression(it, target_type_name.to_string(), scope))
    }

    /// convinience-method to query for an optional constant expression.
    /// if the given `id` is `None`, this method returns `None`
    /// use this only as a shortcut if you have an Option<ConstId> - e.g. an optional initializer.
    /// otherwhise use `get_constant_expression`
    pub fn maybe_get_constant_statement(&self, id: &Option<ConstId>) -> Option<&AstNode> {
        id.as_ref().and_then(|it| self.get_constant_statement(it))
    }

    /// query the constants arena for an expression associated with the given `id`
    pub fn get_constant_statement(&self, id: &ConstId) -> Option<&AstNode> {
        self.find_expression(id).0
    }

    /// query the constants arena for a resolved expression associated with the given `id`.
    /// this operation returns None, if an unresolved/unresolvable expression was registered
    /// for the given id (for different behavior see `get_constant_statement`)
    pub fn get_resolved_constant_statement(&self, id: &ConstId) -> Option<&AstNode> {
        self.find_const_expression(id).filter(|it| it.is_resolved()).map(ConstExpression::get_statement)
    }

    /// query the constants arena for an expression that can be evaluated to an i128.
    /// returns an Err if no expression was associated, or the associated expression is a
    /// complex one (not a LiteralInteger)
    pub fn get_constant_int_statement_value(&self, id: &ConstId) -> Result<i128, String> {
        self.get_constant_statement(id).ok_or_else(|| "Cannot find constant expression".into()).and_then(
            |it| match it.get_stmt() {
                AstStatement::Literal(AstLiteral::Integer(i)) => Ok(*i),
                _ => Err(format!("Cannot extract int constant from {it:#?}")),
            },
        )
    }

    pub fn import(&mut self, other: ConstExpressions) {
        self.expressions.extend(other.expressions)
    }
}

impl<'a> IntoIterator for &'a ConstExpressions {
    type Item = (ConstId, &'a AstNode);
    type IntoIter = IntoStatementIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IntoStatementIter { inner: self.expressions.iter() }
    }
}

pub struct IntoStatementIter<'a> {
    inner: Iter<'a, ConstWrapper>,
}

impl<'a> Iterator for IntoStatementIter<'a> {
    type Item = (ConstId, &'a AstNode);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(idx, expr)| (idx, expr.get_statement()))
    }
}
