mod ast;
mod parsing;
mod syntax_error;
mod syntax_node;

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

use crate::{
    ast::{AstNode, CompilationUnit},
    syntax_error::SyntaxError,
};

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
        let (green, errors) = parsing::parse_text(text);
        let root = SyntaxNode::new_root(green.clone());

        assert_eq!(root.kind(), SyntaxKind::COMPILATION_UNIT);
        Parse::new(green, errors)
    }
}

#[cfg(test)]
mod tests {

    fn print(element: &SyntaxNode) {
        let mut indent = 0;
        for event in element.preorder_with_tokens() {
            match event {
                WalkEvent::Enter(node) => {
                    let text = match &node {
                        NodeOrToken::Node(it) => it.text().to_string(),
                        NodeOrToken::Token(it) => it.text().to_owned(),
                    };
                    println!("{:indent$}{:?} {:?}", " ", text, node.kind(), indent = indent);
                    indent += 2;
                }
                WalkEvent::Leave(_) => indent -= 2,
            }
        }
    }

    use super::*;
    pub use crate::ast::traits::*;
    use crate::ast::Expression;

    #[test]
    fn parse_roundtrip() {
        let text = "PROGRAM PRG
            VAR 
                x, y  : INT; 
                xx    : BOOL := TRUE;
            END_VAR
            VAR_INPUT y : BOOL; END_VAR
        
        END_PROGRAM";
        let cu = CompilationUnit::parse(text).ok().unwrap();

        let pou = cu.pous().next().unwrap();
        assert_eq!(pou.name().unwrap().ident_token().unwrap().text(), "PRG");

        let blocks = pou.var_declaration_blocks().unwrap().var_declaration_blocks().collect::<Vec<_>>();
        let vb1 = &blocks[0];
        let declarations = vb1.var_declarations().collect::<Vec<_>>();
        {
            let names = declarations[0].identifier_list().unwrap().names().collect::<Vec<_>>();
            assert_eq!(names[0].ident_token().unwrap().text(), "x");
            assert_eq!(names[1].ident_token().unwrap().text(), "y");
        }
        {
            let xx_node =
                declarations[1].identifier_list().unwrap().names().nth(0).unwrap().ident_token().unwrap();
            let xx = xx_node.text();
            assert_eq!(xx, "xx");

            let Expression::Literal(value) = declarations[1].init_value().unwrap() else {
                panic!("Expected a literal");
            };
            assert_eq!(value.syntax().text(), "TRUE");
        }
    }
}
