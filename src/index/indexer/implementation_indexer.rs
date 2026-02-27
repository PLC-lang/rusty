use plc_ast::ast::{Implementation, PouType, TypeNature};

use crate::{
    index::{Index, PouIndexEntry},
    typesystem::{self, DataTypeInformation},
};

pub struct ImplementationIndexer<'i> {
    index: &'i mut Index,
}

impl<'i> ImplementationIndexer<'i> {
    pub fn new(index: &'i mut Index) -> Self {
        Self { index }
    }

    pub fn index_implementation(&mut self, implementation: &Implementation) {
        let pou_type = &implementation.pou_type;
        let start_location = implementation
            .statements
            .first()
            .map(|it| it.get_location())
            .as_ref()
            .or(Some(&implementation.location))
            .cloned()
            .unwrap();
        self.index.register_implementation(
            &implementation.name,
            &implementation.type_name,
            pou_type.get_optional_owner_class().as_ref(),
            pou_type.into(),
            implementation.generic,
            start_location,
        );
        //if we are registering an action, also register a datatype for it
        if pou_type == &PouType::Action {
            let datatype = typesystem::DataType {
                name: implementation.name.to_string(),
                initial_value: None,
                information: DataTypeInformation::Alias {
                    name: implementation.name.clone(),
                    referenced_type: implementation.type_name.clone(),
                },
                nature: TypeNature::Derived,
                location: implementation.name_location.clone(),
                linkage: implementation.linkage,
            };

            self.index.register_pou(PouIndexEntry::create_action_entry(
                implementation.name.as_str(),
                implementation.type_name.as_str(),
                implementation.linkage,
                implementation.name_location.clone(),
            ));
            self.index.register_pou_type(datatype);
        }
    }
}
