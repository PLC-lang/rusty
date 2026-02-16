mod ast;
mod syntax_error;
mod syntax_node;
mod parsing;

use std::{marker::PhantomData, ops::Range, sync::Arc};

pub use rowan::{
    api::Preorder, Direction, GreenNode, NodeOrToken, SyntaxText, TextRange, TextSize, TokenAtOffset,
    WalkEvent,
};

pub use crate::syntax_node::{
    PreorderWithTokens, StructuredTextLanguage, SyntaxElement, SyntaxElementChildren, SyntaxNode,
    SyntaxNodeChildren, SyntaxToken, SyntaxTreeBuilder,
};

pub use plc_rowan_parser::{SyntaxKind, T};

use crate::{ast::{AstNode, CompilationUnit}, syntax_error::SyntaxError};

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: Option<GreenNode>,
    errors: Option<Arc<[SyntaxError]>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse { green: self.green.clone(), errors: self.errors.clone(), _ty: PhantomData }
    }
}

impl<T> Parse<T> {
    fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
        Parse {
            green: Some(green),
            errors: if errors.is_empty() { None } else { Some(errors.into()) },
            _ty: PhantomData,
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.as_ref().unwrap().clone())
    }

    pub fn errors(&self) -> Vec<SyntaxError> {
        let mut errors = if let Some(e) = self.errors.as_deref() { e.to_vec() } else { vec![] };
        //TODO: reactivate
        // validation::validate(&self.syntax_node(), &mut errors);
        errors
    }
}

impl<T: AstNode> Parse<T> {
    /// Converts this parse result into a parse result for an untyped syntax tree.
    pub fn to_syntax(mut self) -> Parse<SyntaxNode> {
        let green = self.green.take();
        let errors = self.errors.take();
        Parse { green, errors, _ty: PhantomData }
    }

    /// Gets the parsed syntax tree as a typed ast node.
    ///
    /// # Panics
    ///
    /// Panics if the root node cannot be casted into the typed ast node
    /// (e.g. if it's an `ERROR` node).
    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    /// Converts from `Parse<T>` to [`Result<T, Vec<SyntaxError>>`].
    pub fn ok(self) -> Result<T, Vec<SyntaxError>> {
        match self.errors() {
            errors if !errors.is_empty() => Err(errors),
            _ => Ok(self.tree()),
        }
    }
}

impl Parse<SyntaxNode> {
    pub fn cast<N: AstNode>(mut self) -> Option<Parse<N>> {
        if N::cast(self.syntax_node()).is_some() {
            Some(Parse { green: self.green.take(), errors: self.errors.take(), _ty: PhantomData })
        } else {
            None
        }
    }
}

impl Parse<CompilationUnit> {
    // pub fn reparse(&self, delete: TextRange, insert: &str) -> Parse<CompilationUnit> {
    //     self.incremental_reparse(delete, insert )
    //         .unwrap_or_else(|| self.full_reparse(delete, insert ))
    // }

    // fn incremental_reparse(
    //     &self,
    //     delete: TextRange,
    //     insert: &str,
    // ) -> Option<Parse<CompilationUnit>> {
    //     // FIXME: validation errors are not handled here
    //     parsing::incremental_reparse(
    //         self.tree().syntax(),
    //         delete,
    //         insert,
    //         self.errors.as_deref().unwrap_or_default().iter().cloned(),
    //         edition,
    //     )
    //     .map(|(green_node, errors, _reparsed_range)| Parse {
    //         green: Some(green_node),
    //         errors: if errors.is_empty() { None } else { Some(errors.into()) },
    //         _ty: PhantomData,
    //     })
    // }

    fn full_reparse(&self, delete: TextRange, insert: &str) -> Parse<CompilationUnit> {
        let mut text = self.tree().syntax().text().to_string();
        text.replace_range(Range::<usize>::from(delete), insert);
        CompilationUnit::parse(&text)
    }
}

/// `CompilationUnit` represents a parse tree for a single st file.
impl CompilationUnit {
    pub fn parse(text: &str) -> Parse<CompilationUnit> {
        let (green, errors) = parsing::parse_text(text );
        let root = SyntaxNode::new_root(green.clone());

        assert_eq!(root.kind(), SyntaxKind::COMPILATION_UNIT);
        Parse::new(green, errors)
    }
}