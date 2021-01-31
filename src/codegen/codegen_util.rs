use inkwell::{types::BasicTypeEnum, values::PointerValue};

use crate::{ast::DataTypeDeclaration, index::{DataTypeIndexEntry, DataTypeInformation, Index}};

pub fn find_type_information_in_index<'a, 'b>(data_type: &DataTypeDeclaration, index: &'b Index<'a>) -> Option<&'b DataTypeInformation<'a>> {
        data_type.get_name()
            .and_then(|it| index.find_type(it))
            .and_then(DataTypeIndexEntry::get_type_information)
    }



pub fn find_type_in_index<'a>(data_type: &DataTypeDeclaration, index: &Index<'a>) -> Option<BasicTypeEnum<'a>> {
        data_type.get_name()
            .and_then(|it| index.find_type(it))
            .and_then(DataTypeIndexEntry::get_type)
    }


pub fn find_variable_in_index<'a>(name: &[String], scope: Option<&str>, index: &Index<'a>) -> Option<PointerValue<'a>> {
        index
            .find_variable(scope, name)
            .map(|e| e.get_generated_reference())
            .flatten()
    }

