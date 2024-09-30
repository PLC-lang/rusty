use std::{fmt::Display, vec};

use plc_ast::ast::{AstNode, AstStatement, ConfigVariable, ReferenceAccess, ReferenceExpr};
use plc_diagnostics::diagnostics::Diagnostic;

use crate::{
    index::Index,
    typesystem::{Dimension, TypeSize},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionPathElement<'idx> {
    Name(&'idx str),
    ArrayAccess(&'idx [Dimension]),
    Foo(Vec<TypeSize>),
}

impl Display for ExpressionPathElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionPathElement::Name(name) => write!(f, "{name}"),
            ExpressionPathElement::ArrayAccess(_) => unimplemented!(),
            ExpressionPathElement::Foo(_) => unimplemented!(),
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
        ExpressionPathElement::ArrayAccess(dims)
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
                ExpressionPathElement::ArrayAccess(dimensions) => {
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
                ExpressionPathElement::Foo(idx) => {
                    let Some(first) = idx.first().map(|it| it.as_int_value(index).ok()).flatten() else {
                        return vec![];
                    };
                    let mut res = format!("{first}");
                    idx.iter().skip(1).for_each(|it| {
                        if let Ok(i) = it.as_int_value(index) {
                            res = format!("{res},{i}");
                        };
                    });
                    vec![format!("[{res}]")]
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
        let (mut names, diags) = get_expression_path_segments(&value.reference);

        if !diags.is_empty() {
            return Err(diags);
        };
        names.reverse();
        Ok(Self { names })
    }
}

fn get_expression_path_segments<'a>(node: &'a AstNode) -> (Vec<ExpressionPathElement<'a>>, Vec<Diagnostic>) {
    let mut paths = vec![];
    let mut diagnostics = vec![];
    match &node.stmt {
        AstStatement::ReferenceExpr(
            ReferenceExpr { access: ReferenceAccess::Member(reference), base },
            ..,
        ) => {
            paths.push(ExpressionPathElement::Name(reference.get_flat_reference_name().unwrap_or_default()));
            if let Some(base) = base {
                let (vals, errs) = get_expression_path_segments(base);
                paths.extend(vals);
                diagnostics.extend(errs);
            }
        }
        AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Index(idx), base }) => {
            match &idx.as_ref().stmt {
                AstStatement::Literal(_) => {
                    if let Some(v) =
                        idx.get_literal_integer_value().map(|it| vec![TypeSize::LiteralInteger(it as i64)])
                    {
                        paths.push(ExpressionPathElement::Foo(v))
                    }
                }
                AstStatement::ExpressionList(vec) => {
                    let mut res = vec![];
                    vec.iter().for_each(|idx: &AstNode| {
                        if let Some(v) = idx.get_literal_integer_value() {
                            res.push(TypeSize::LiteralInteger(v as i64));
                        } else {
                            diagnostics.push(
                                Diagnostic::new("VAR_CONFIG array access must be a literal integer")
                                    .with_location(&idx.location),
                            );
                        }
                    });
                    paths.push(ExpressionPathElement::Foo(res));
                }
                _ => {
                    diagnostics.push(
                        Diagnostic::new("VAR_CONFIG array access must be a literal integer")
                            .with_location(&idx.location),
                    );
                }
            }
            if let Some(base) = base {
                let (vals, errs) = get_expression_path_segments(base);
                paths.extend(vals);
                diagnostics.extend(errs);
            }
        }
        _ => {
            unimplemented!()
        }
    };
    (paths, diagnostics)
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
            names: vec![ExpressionPathElement::Name("a"), ExpressionPathElement::ArrayAccess(&dims)],
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
            names: vec![ExpressionPathElement::Name("a"), ExpressionPathElement::ArrayAccess(&dims)],
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
            names: vec![ExpressionPathElement::Name("a"), ExpressionPathElement::ArrayAccess(&dims)],
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
                ExpressionPathElement::ArrayAccess(&dims1),
                ExpressionPathElement::ArrayAccess(&dims2),
                ExpressionPathElement::ArrayAccess(&dims3),
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
