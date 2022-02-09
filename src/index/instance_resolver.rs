use crate::{typesystem::DataTypeInformation, qualifed_name::{QualifiedName, QualifiedNameElement}};

use super::{Index, VariableIndexEntry};
pub type Instance<'idx> = (QualifiedName<'idx>, &'idx VariableIndexEntry);
type InstanceEntry<'idx> = (QualifiedNameElement<'idx>, &'idx VariableIndexEntry);


pub struct InstanceIterator<'idx> {
    index: &'idx Index,
    iterator: Box<dyn Iterator<Item = InstanceEntry<'idx>> + 'idx>,
    inner: Option<Box<InstanceIterator<'idx>>>,
    current_prefix: QualifiedName<'idx>,
}

impl<'idx> Iterator for InstanceIterator<'idx> {
    type Item = Instance<'idx>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_some() {
            self.get_from_inner()
        } else {
            self.get()
        }
    }
}
impl<'idx> InstanceIterator<'idx> {
    pub fn new(index: &'idx Index) -> InstanceIterator<'idx> {
        InstanceIterator {
            index,
            current_prefix: QualifiedName::default(),
            iterator: (Box::new(index.get_globals().iter().map(|(_, it)| {
                (
                    it.get_qualified_name()
                        .split('.')
                        .last()
                        .expect("Variable needs a name")
                        .into(),
                    it,
                )
            }))) as Box<dyn Iterator<Item = InstanceEntry<'idx>>>,
            inner: None,
        }
    }

    fn inner(
        index: &'idx Index,
        container: &str,
        current_prefix: &QualifiedName<'idx>,
    ) -> Option<InstanceIterator<'idx>> {
        //If the container is an array, build a new iterator for that datatype with the iterations of that array as variables
        let inner_type = index.find_effective_type_info(container);
        let (container, prefix) = if let Some(DataTypeInformation::Array {
            inner_type_name,
            dimensions,
            ..
        }) = inner_type
        {
            (
                inner_type_name.as_str(),
                current_prefix.append(dimensions.as_slice().into()),
            )
        } else {
            (container, current_prefix.clone())
        };
        index
            .get_members(container)
            .map(|it| {
                it.iter().map(|(_, it)| {
                    (
                        it.get_qualified_name()
                            .split('.')
                            .last()
                            .expect("Variable needs a name")
                            .into(),
                        it,
                    )
                })
            })
            .map(|iterator| InstanceIterator {
                index,
                current_prefix: prefix,
                iterator: (Box::new(iterator)) as Box<dyn Iterator<Item = InstanceEntry<'idx>>>,
                inner: None,
            })
    }

    fn get(&mut self) -> Option<Instance<'idx>> {
        if let Some((entry, variable)) = self.iterator.next() {
            self.inner = InstanceIterator::inner(self.index, variable.get_type_name(), &vec![entry].into())
                .map(Box::new);
            let name = self.current_prefix.append(entry);
            Some((name, variable))
        } else {
            None
        }
    }

    fn get_from_inner(&mut self) -> Option<Instance<'idx>> {
        let res = if let Some(inner) = self.inner.as_deref_mut() {
            let res = inner.next();
            if let Some((mut name, variable)) = res {
                let mut v = self.current_prefix.clone();
                v.join(&mut name);
                Some((v, variable))
            } else {
                self.inner = None;
                res
            }
        } else {
            None
        };
        res.or_else(|| self.get())
    }
}

