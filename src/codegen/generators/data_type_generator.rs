// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// the data_type_generator generates user defined data-types
/// - Structures
/// - Enum types
/// - SubRange types
/// - Alias types
/// - sized Strings
use crate::index::{Index, VariableIndexEntry};
use crate::{
    ast::{AstStatement, Dimension},
    compile_error::CompileError,
    typesystem::DataTypeInformation,
};
use crate::{
    codegen::{
        llvm_index::LlvmTypedIndex,
        llvm_typesystem::{get_llvm_float_type, get_llvm_int_type},
    },
    typesystem::DataType,
};
use inkwell::{
    types::{ArrayType, BasicType, BasicTypeEnum},
    values::BasicValueEnum,
    AddressSpace,
};

use super::{
    expression_generator::ExpressionCodeGenerator, llvm::Llvm, struct_generator::StructGenerator,
};

/// generates the llvm-type for the given data-type and registers it at the index
/// this function may create and register a ...
/// - Struct type for a STRUCT
/// - global variables for enum-elements
/// - an alias index entry for sub-range types
/// - Array type for arrays
/// - array type for sized Strings
pub fn generate_data_types<'ink>(
    llvm: &Llvm<'ink>,
    index: &Index,
) -> Result<LlvmTypedIndex<'ink>, CompileError> {
    let mut types_index = LlvmTypedIndex::new();
    let types = index.get_types();
    for (name, user_type) in types {
        if let DataTypeInformation::Struct {
            name: struct_name, ..
        } = user_type.get_type_information()
        {
            types_index.associate_type(name, llvm.create_struct_stub(struct_name).into())?;
        }
    }
    for (name, user_type) in types {
        let gen_type = create_type(llvm, index, &types_index, name, user_type)?;
        types_index.associate_type(name, gen_type)?
    }
    for (name, user_type) in types {
        expand_opaque_types(llvm, index, &mut types_index, user_type)?;
        if let Some(initial_value) = generate_initial_value(index, &types_index, llvm, user_type) {
            types_index.associate_initial_value(name, initial_value)?
        }
    }
    Ok(types_index)
}

/// generates the members of an opaque struct and associates its initial values
fn expand_opaque_types<'ink>(
    llvm: &Llvm<'ink>,
    index: &Index,
    types_index: &mut LlvmTypedIndex<'ink>,
    data_type: &DataType,
) -> Result<(), CompileError> {
    let information = data_type.get_type_information();
    if let DataTypeInformation::Struct { member_names, .. } = information {
        let mut struct_generator = StructGenerator::new(llvm, index, types_index);
        let members: Vec<&VariableIndexEntry> = member_names
            .iter()
            .map(|variable_name| {
                index
                    .find_member(data_type.get_name(), variable_name)
                    .unwrap()
            })
            .filter(|var| !var.is_return())
            .collect();
        let ((_, initial_value), member_values) =
            struct_generator.generate_struct_type(&members, data_type.get_name())?;
        for (member, value) in member_values {
            let qualified_name = format!("{}.{}", data_type.get_name(), member);
            types_index.associate_initial_value(&qualified_name, value)?;
        }
        types_index.associate_initial_value(data_type.get_name(), initial_value)?;
    }
    Ok(())
}

/// Creates an llvm type to be associated with the given data type.
/// Generates only an opaque type for structs.
/// Eagerly generates but does not associate nested array and referenced aliased types
fn create_type<'ink>(
    llvm: &Llvm<'ink>,
    index: &Index,
    types_index: &LlvmTypedIndex<'ink>,
    name: &str,
    data_type: &DataType,
) -> Result<BasicTypeEnum<'ink>, CompileError> {
    let information = data_type.get_type_information();
    match information {
        DataTypeInformation::Struct { .. } => types_index.get_associated_type(data_type.get_name()),
        DataTypeInformation::Array {
            inner_type_name,
            dimensions,
            ..
        } => {
            let inner_type = create_type(
                llvm,
                index,
                types_index,
                inner_type_name,
                index.find_type(inner_type_name).unwrap(),
            )
            .unwrap();
            let array_type = create_nested_array_type(inner_type, dimensions.clone()).into();
            Ok(array_type)
        }
        DataTypeInformation::Integer { size, .. } => {
            get_llvm_int_type(llvm.context, *size, name).map(|it| it.into())
        }
        DataTypeInformation::Float { size, .. } => {
            get_llvm_float_type(llvm.context, *size, name).map(|it| it.into())
        }
        DataTypeInformation::String { size, encoding } => Ok(llvm
            .context
            .i8_type()
            .array_type(*size * encoding.get_bytes_per_char())
            .into()),
        DataTypeInformation::SubRange {
            referenced_type, ..
        } => {
            let ref_type = create_type(
                llvm,
                index,
                types_index,
                name,
                index.find_type(referenced_type).unwrap(),
            )?;
            Ok(ref_type)
        }
        DataTypeInformation::Alias {
            referenced_type, ..
        } => {
            let ref_type = create_type(
                llvm,
                index,
                types_index,
                name,
                index.find_type(referenced_type).unwrap(),
            )?;
            Ok(ref_type)
        }
        // REVIEW: Void types are not basic type enums, so we return an int here
        DataTypeInformation::Void => get_llvm_int_type(llvm.context, 32, "Void").map(Into::into),
        DataTypeInformation::Pointer {
            inner_type_name, ..
        } => {
            let inner_type = create_type(
                llvm,
                index,
                types_index,
                inner_type_name,
                index.get_type(inner_type_name)?,
            )?;
            Ok(inner_type.ptr_type(AddressSpace::Generic).into())
        }
    }
}

fn generate_initial_value<'ink>(
    index: &Index,
    types_index: &LlvmTypedIndex<'ink>,
    llvm: &Llvm<'ink>,
    data_type: &DataType,
) -> Option<BasicValueEnum<'ink>> {
    let information = data_type.get_type_information();
    match information {
        DataTypeInformation::Struct { .. } => None, //Done elsewhere
        DataTypeInformation::Array { .. } => generate_array_initializer(
            data_type,
            data_type.get_name(),
            index,
            types_index,
            llvm,
            |stmt| matches!(stmt, AstStatement::LiteralArray { .. }),
            "LiteralArray",
        )
        .unwrap(),
        DataTypeInformation::Integer { .. } => None,
        DataTypeInformation::Float { .. } => None,
        DataTypeInformation::String { .. } => generate_array_initializer(
            data_type,
            data_type.get_name(),
            index,
            types_index,
            llvm,
            |stmt| matches!(stmt, AstStatement::LiteralString { .. }),
            "LiteralString",
        )
        .unwrap(),
        DataTypeInformation::SubRange {
            referenced_type, ..
        } => register_aliased_initial_value(index, types_index, llvm, data_type, referenced_type),
        DataTypeInformation::Alias {
            referenced_type, ..
        } => register_aliased_initial_value(index, types_index, llvm, data_type, referenced_type),
        // Void types are not basic type enums, so we return an int here
        DataTypeInformation::Void => None, //get_llvm_int_type(llvm.context, 32, "Void").map(Into::into),
        DataTypeInformation::Pointer { .. } => None,
    }
}

/// generates and returns an optional inital value at the given dataType
/// if no initial value is defined, it returns  an optional initial value of
/// the aliased type (referenced_type)
fn register_aliased_initial_value<'ink>(
    index: &Index,
    types_index: &LlvmTypedIndex<'ink>,
    llvm: &Llvm<'ink>,
    data_type: &DataType,
    referenced_type: &str,
) -> Option<BasicValueEnum<'ink>> {
    if let Some(initializer) = &data_type.initial_value {
        let generator = ExpressionCodeGenerator::new_context_free(llvm, index, types_index, None);
        let (_, initial_value) = generator.generate_expression(initializer).unwrap();
        Some(initial_value)
    } else {
        // if there's no initializer defined for this alias, we go and check the aliased type for an initial value
        index
            .get_type(referenced_type)
            .ok()
            .and_then(|referenced_data_type| {
                generate_initial_value(index, types_index, llvm, referenced_data_type)
            })
    }
}

/// generates and associates the given array-datatype (used for arrays and strings)
fn generate_array_initializer<'ink>(
    data_type: &DataType,
    name: &str,
    index: &Index,
    types_index: &LlvmTypedIndex<'ink>,
    llvm: &Llvm<'ink>,
    predicate: fn(&AstStatement) -> bool,
    expected_ast: &str,
) -> Result<Option<BasicValueEnum<'ink>>, CompileError> {
    if let Some(initializer) = &data_type.initial_value {
        if predicate(initializer) {
            let array_type = index.get_type_information(name)?;
            let generator = ExpressionCodeGenerator::new_context_free(
                llvm,
                index,
                types_index,
                Some(array_type),
            );
            let (_, initial_value) = generator.generate_literal(initializer)?;
            Ok(Some(initial_value))
        } else {
            Err(CompileError::codegen_error(
                format!("Expected {} but found {:?}", expected_ast, initializer),
                initializer.get_location(),
            ))
        }
    } else {
        Ok(None)
    }
}

/// creates the llvm types for a multi-dimensional array
fn create_nested_array_type(end_type: BasicTypeEnum, dimensions: Vec<Dimension>) -> ArrayType {
    let mut result: Option<ArrayType> = None;
    let mut current_type = end_type;
    for dimension in dimensions.iter().rev() {
        result = Some(match current_type {
            BasicTypeEnum::IntType(..) => current_type
                .into_int_type()
                .array_type(dimension.get_length()),
            BasicTypeEnum::FloatType(..) => current_type
                .into_float_type()
                .array_type(dimension.get_length()),
            BasicTypeEnum::StructType(..) => current_type
                .into_struct_type()
                .array_type(dimension.get_length()),
            BasicTypeEnum::ArrayType(..) => current_type
                .into_array_type()
                .array_type(dimension.get_length()),
            BasicTypeEnum::PointerType(..) => current_type
                .into_pointer_type()
                .array_type(dimension.get_length()),
            BasicTypeEnum::VectorType(..) => current_type
                .into_vector_type()
                .array_type(dimension.get_length()),
        });
        current_type = result.unwrap().into();
    }
    result.unwrap()
}
