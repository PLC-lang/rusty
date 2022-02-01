use crate::{index::Index, typesystem::Dimension};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QualifiedNameElement<'idx> {
    Name(&'idx str),
    ArrayAccess(&'idx [Dimension]),
}

impl ToString for QualifiedNameElement<'_> {
    fn to_string(&self) -> String {
        match self {
            QualifiedNameElement::Name(name) => name.to_string(),
            QualifiedNameElement::ArrayAccess(_) => todo!(),
        }
    }
}

impl<'idx> From<&'idx str> for QualifiedNameElement<'idx> {
    fn from(name: &'idx str) -> Self {
        QualifiedNameElement::Name(name)
    }
}

impl<'idx> From<&'idx [Dimension]> for QualifiedNameElement<'idx> {
    fn from(dims: &'idx [Dimension]) -> Self {
        QualifiedNameElement::ArrayAccess(dims)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct QualifiedName<'idx> {
    names: Vec<QualifiedNameElement<'idx>>,
}

impl<'idx> QualifiedName<'idx> {
    pub fn join(&mut self, name: &mut QualifiedName<'idx>) {
        self.names.append(&mut name.names)
    }

    pub fn append(&self, element: QualifiedNameElement<'idx>) -> QualifiedName<'idx> {
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
                crate::qualifed_name::QualifiedNameElement::Name(s) => vec![s.to_string()],
                crate::qualifed_name::QualifiedNameElement::ArrayAccess(dimensions) => {
                    let mut array = dimensions
                        .iter()
                        .map(|it| it.get_range_inclusive(index).unwrap())
                        .fold(vec![], |curr, it| {
                            let mut res = vec![];
                            it.into_iter().for_each(|next| {
                                if curr.is_empty() {
                                    res.push(next.to_string());
                                } else {
                                    for item in curr.iter() {
                                        res.push(format!("{},{}", item, next));
                                    }
                                }
                            });
                            res
                        });

                    //Add array brackets
                    array.iter_mut().for_each(|s| *s = format!("[{}]", s));
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
                        res.push(format!("{}{}{}", ele, separator, next));
                    }
                }
            });
            res
        })
    }
}

impl<'a> From<Vec<QualifiedNameElement<'a>>> for QualifiedName<'a> {
    fn from(v: Vec<QualifiedNameElement<'a>>) -> Self {
        QualifiedName { names: v }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        index::Index,
        qualifed_name::{QualifiedName, QualifiedNameElement},
        typesystem::{Dimension, TypeSize},
    };

    #[test]
    fn expand_single() {
        let name = QualifiedName {
            names: vec![QualifiedNameElement::Name("Test")],
        };
        let index = Index::default();
        assert_eq!(name.expand(&index), vec!["Test".to_string()])
    }

    #[test]
    fn expand_qualifed() {
        let name = QualifiedName {
            names: vec![
                QualifiedNameElement::Name("a"),
                QualifiedNameElement::Name("b"),
            ],
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

        let name = QualifiedName {
            names: vec![
                QualifiedNameElement::Name("a"),
                QualifiedNameElement::ArrayAccess(&dims),
            ],
        };
        let index = Index::default();
        assert_eq!(
            name.expand(&index),
            vec!["a[-1]".to_string(), "a[0]".to_string(), "a[1]".to_string(),]
        )
    }

    #[test]
    fn expand_array_single_element() {
        let dims = vec![Dimension {
            start_offset: TypeSize::LiteralInteger(1),
            end_offset: TypeSize::LiteralInteger(1),
        }];

        let name = QualifiedName {
            names: vec![
                QualifiedNameElement::Name("a"),
                QualifiedNameElement::ArrayAccess(&dims),
            ],
        };
        let index = Index::default();
        assert_eq!(name.expand(&index), vec!["a[1]".to_string(),])
    }

    #[test]
    fn expand_multidim_array() {
        let dims = vec![
            Dimension {
                start_offset: TypeSize::LiteralInteger(-1),
                end_offset: TypeSize::LiteralInteger(1),
            },
            Dimension {
                start_offset: TypeSize::LiteralInteger(0),
                end_offset: TypeSize::LiteralInteger(1),
            },
            Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(1),
            },
        ];

        let name = QualifiedName {
            names: vec![
                QualifiedNameElement::Name("a"),
                QualifiedNameElement::ArrayAccess(&dims),
            ],
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

        let name = QualifiedName {
            names: vec![
                QualifiedNameElement::Name("a"),
                QualifiedNameElement::ArrayAccess(&dims1),
                QualifiedNameElement::ArrayAccess(&dims2),
                QualifiedNameElement::ArrayAccess(&dims3),
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
