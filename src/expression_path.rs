use std::fmt::Display;

use crate::{index::Index, typesystem::Dimension};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressionPathElement<'idx> {
    Name(&'idx str),
    ArrayAccess(&'idx [Dimension]),
}

impl Display for ExpressionPathElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionPathElement::Name(name) => write!(f, "{name}"),
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
        ExpressionPathElement::ArrayAccess(dims)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExpressionPath<'idx> {
    names: Vec<ExpressionPathElement<'idx>>,
}

impl<'idx> ExpressionPath<'idx> {
    pub fn names(&self) -> Vec<String> {
        let mut out = Vec::new();
        for name in &self.names {
            match name {
                ExpressionPathElement::Name(val) => out.push(val.to_string()),
                _ => panic!("oops"),
            }
        }

        out
    }

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
                crate::expression_path::ExpressionPathElement::Name(s) => vec![s.to_string()],
                crate::expression_path::ExpressionPathElement::ArrayAccess(dimensions) => {
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
