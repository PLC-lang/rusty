use serde::Serialize;

use crate::typesystem::Dimension;

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
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

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct QualifiedName<'idx> {
    names: Vec<QualifiedNameElement<'idx>>,
}

impl<'idx> QualifiedName<'idx> {
    pub fn join(&mut self, name: &mut QualifiedName<'idx>) {
        self.names.append(&mut name.names)
    }

    pub fn append(
        &self,
        element: QualifiedNameElement<'idx>,
    ) -> QualifiedName<'idx> {
        let mut res = self.clone();
        res.names.push(element);
        res
    }
}

impl<'a> IntoIterator for QualifiedName<'a> {
    type Item = QualifiedNameElement<'a>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

impl<'a> From<Vec<QualifiedNameElement<'a>>> for QualifiedName<'a> {
    fn from(v: Vec<QualifiedNameElement<'a>>) -> Self {
        QualifiedName { names: v }
    }
}