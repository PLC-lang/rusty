use std::{fmt::Display, vec};

use plc_ast::ast::{AstNode, AstStatement, ConfigVariable, ReferenceAccess, ReferenceExpr};
use plc_diagnostics::diagnostics::Diagnostic;

use crate::{index::Index, typesystem::Dimension};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionPathElement<'idx> {
    Name(&'idx str),
    ArrayDimensions(&'idx [Dimension]),
    ArrayAccess(Vec<i128>),
}

impl Display for ExpressionPathElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionPathElement::Name(name) => write!(f, "{name}"),
            ExpressionPathElement::ArrayDimensions(_) => unimplemented!(),
            ExpressionPathElement::ArrayAccess(_) => unimplemented!(),
        }
    }
}

impl<'idx> From<&'idx str> for ExpressionPathElement<'idx> {
    fn from(name: &'idx str) -> Self {
        ExpressionPathElement::Name(name)
    }
}

impl<'idx> From<&'idx [Dimension]> for ExpressionPathElement<'idx> {
    fn from(dims: &'idx [Dimension]) -> Self {
        ExpressionPathElement::ArrayDimensions(dims)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExpressionPath<'idx> {
    names: Vec<ExpressionPathElement<'idx>>,
}

impl<'idx> ExpressionPath<'idx> {
    pub fn join(&mut self, name: &mut ExpressionPath<'idx>) {
        self.names.append(&mut name.names)
    }

    pub fn append(&self, element: ExpressionPathElement<'idx>) -> ExpressionPath<'idx> {
        let mut res = self.clone();
        res.names.push(element);
        res
    }

    /// Expands the given name to reference all underlying instances
    /// This implementation will create an element for every contained array
    pub fn expand(&self, index: &Index) -> Vec<String> {
        let mut levels: Vec<Vec<String>> = vec![];
        for seg in self.names.iter() {
            let level = match seg {
                ExpressionPathElement::Name(s) => vec![s.to_string()],
                ExpressionPathElement::ArrayDimensions(dimensions) => {
                    let mut array = dimensions.iter().map(|it| it.get_range_inclusive(index).unwrap()).fold(
                        vec![],
                        |curr, it| {
                            let mut res = vec![];
                            it.into_iter().for_each(|next| {
                                if curr.is_empty() {
                                    res.push(next.to_string());
                                } else {
                                    for item in curr.iter() {
                                        res.push(format!("{item},{next}"));
                                    }
                                }
                            });
                            res
                        },
                    );

                    //Add array brackets
                    array.iter_mut().for_each(|s| *s = format!("[{s}]"));
                    array
                }
                ExpressionPathElement::ArrayAccess(idx) => {
                    let Some(first) = idx.first() else {
                        unreachable!("Caught at the parsing stage")
                    };

                    let access =
                        idx.iter().skip(1).fold(format!("{first}"), |acc, idx| format!("{acc},{idx}"));

                    vec![format!("[{access}]")]
                }
            };
            levels.push(level);
        }
        levels.into_iter().fold(vec![], |curr, it| {
            let mut res = vec![];
            it.into_iter().for_each(|next| {
                if curr.is_empty() {
                    res.push(next);
                } else {
                    let separator = if next.starts_with('[') { "" } else { "." };
                    for ele in &curr {
                        res.push(format!("{ele}{separator}{next}"));
                    }
                }
            });
            res
        })
    }
}

impl<'a> TryFrom<&'a ConfigVariable> for ExpressionPath<'a> {
    type Error = Vec<Diagnostic>;

    fn try_from(value: &'a ConfigVariable) -> Result<Self, Self::Error> {
        let mut names = get_expression_path_segments(&value.reference)?;
        names.reverse();
        Ok(Self { names })
    }
}

// Transforms a `ConfigVariable`'s 'AstNode' into a collection of corresponding `ExpressionPathElement`s.
// This function will traverse the AST top-to-bottom, collecting segments along the way, which means the order of the collection 
// needs to be reversed by the caller to match the written expression.
fn get_expression_path_segments(node: &AstNode) -> Result<Vec<ExpressionPathElement>, Vec<Diagnostic>> {
    let mut paths = vec![];
    let mut diagnostics = vec![];
    let mut add_diagnostic = |location| {
        diagnostics.push(
            Diagnostic::new("VAR_CONFIG array access must be a literal integer").with_location(location),
        );
    };
    match &node.stmt {
        AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Member(reference), base }) => {
            paths.push(ExpressionPathElement::Name(reference.get_flat_reference_name().unwrap_or_default()));
            if let Some(base) = base {
                match get_expression_path_segments(base) {
                    Ok(v) => paths.extend(v),
                    Err(e) => diagnostics.extend(e),
                };
            }
        }
        AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Index(idx), base }) => {
            match &idx.as_ref().stmt {
                AstStatement::Literal(_) => {
                    if let Some(v) = idx.get_literal_integer_value().map(|it| vec![it]) {
                        paths.push(ExpressionPathElement::ArrayAccess(v))
                    } else {
                        add_diagnostic(&idx.location);
                    }
                }
                AstStatement::ExpressionList(vec) => {
                    let mut res = vec![];
                    vec.iter().for_each(|idx: &AstNode| {
                        if let Some(v) = idx.get_literal_integer_value() {
                            res.push(v);
                        } else {
                            add_diagnostic(&idx.location);
                        }
                    });
                    paths.push(ExpressionPathElement::ArrayAccess(res));
                }
                _ => add_diagnostic(&idx.location),
            }
            if let Some(base) = base {
                match get_expression_path_segments(base) {
                    Ok(v) => paths.extend(v),
                    Err(e) => diagnostics.extend(e),
                };
            }
        }
        _ => add_diagnostic(&node.location),
    };

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    Ok(paths)
}

impl<'a> From<Vec<ExpressionPathElement<'a>>> for ExpressionPath<'a> {
    fn from(v: Vec<ExpressionPathElement<'a>>) -> Self {
        ExpressionPath { names: v }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expression_path::{ExpressionPath, ExpressionPathElement},
        index::Index,
        typesystem::{Dimension, TypeSize},
    };

    #[test]
    fn expand_single() {
        let name = ExpressionPath { names: vec![ExpressionPathElement::Name("Test")] };
        let index = Index::default();
        assert_eq!(name.expand(&index), vec!["Test".to_string()])
    }

    #[test]
    fn expand_qualifed() {
        let name = ExpressionPath {
            names: vec![ExpressionPathElement::Name("a"), ExpressionPathElement::Name("b")],
        };
        let index = Index::default();
        assert_eq!(name.expand(&index), vec!["a.b".to_string()])
    }

    #[test]
    fn expand_array() {
        let dims = vec![Dimension {
            start_offset: TypeSize::LiteralInteger(-1),
            end_offset: TypeSize::LiteralInteger(1),
        }];

        let name = ExpressionPath {
            names: vec![ExpressionPathElement::Name("a"), ExpressionPathElement::ArrayDimensions(&dims)],
        };
        let index = Index::default();
        assert_eq!(name.expand(&index), vec!["a[-1]".to_string(), "a[0]".to_string(), "a[1]".to_string(),])
    }

    #[test]
    fn expand_array_single_element() {
        let dims = vec![Dimension {
            start_offset: TypeSize::LiteralInteger(1),
            end_offset: TypeSize::LiteralInteger(1),
        }];

        let name = ExpressionPath {
            names: vec![ExpressionPathElement::Name("a"), ExpressionPathElement::ArrayDimensions(&dims)],
        };
        let index = Index::default();
        assert_eq!(name.expand(&index), vec!["a[1]".to_string(),])
    }

    #[test]
    fn expand_multidim_array() {
        let dims = vec![
            Dimension { start_offset: TypeSize::LiteralInteger(-1), end_offset: TypeSize::LiteralInteger(1) },
            Dimension { start_offset: TypeSize::LiteralInteger(0), end_offset: TypeSize::LiteralInteger(1) },
            Dimension { start_offset: TypeSize::LiteralInteger(1), end_offset: TypeSize::LiteralInteger(1) },
        ];

        let name = ExpressionPath {
            names: vec![ExpressionPathElement::Name("a"), ExpressionPathElement::ArrayDimensions(&dims)],
        };
        let index = Index::default();
        let mut res = name.expand(&index);
        res.sort();
        assert_eq!(
            res,
            vec![
                "a[-1,0,1]".to_string(),
                "a[-1,1,1]".to_string(),
                "a[0,0,1]".to_string(),
                "a[0,1,1]".to_string(),
                "a[1,0,1]".to_string(),
                "a[1,1,1]".to_string(),
            ]
        )
    }

    #[test]
    fn expand_nested_array() {
        let dims1 = vec![Dimension {
            start_offset: TypeSize::LiteralInteger(-1),
            end_offset: TypeSize::LiteralInteger(1),
        }];
        let dims2 = vec![Dimension {
            start_offset: TypeSize::LiteralInteger(0),
            end_offset: TypeSize::LiteralInteger(1),
        }];
        let dims3 = vec![Dimension {
            start_offset: TypeSize::LiteralInteger(1),
            end_offset: TypeSize::LiteralInteger(1),
        }];

        let name = ExpressionPath {
            names: vec![
                ExpressionPathElement::Name("a"),
                ExpressionPathElement::ArrayDimensions(&dims1),
                ExpressionPathElement::ArrayDimensions(&dims2),
                ExpressionPathElement::ArrayDimensions(&dims3),
            ],
        };
        let index = Index::default();
        let mut res = name.expand(&index);
        res.sort();
        assert_eq!(
            res,
            vec![
                "a[-1][0][1]".to_string(),
                "a[-1][1][1]".to_string(),
                "a[0][0][1]".to_string(),
                "a[0][1][1]".to_string(),
                "a[1][0][1]".to_string(),
                "a[1][1][1]".to_string(),
            ]
        )
    }
}
