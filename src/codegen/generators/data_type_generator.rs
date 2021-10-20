// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
/// the data_type_generator generates user defined data-types
/// - Structures
/// - Enum types
/// - SubRange types
/// - Alias types
/// - sized Strings
use crate::ast::SourceRange;
use crate::index::{Index, VariableIndexEntry};
use crate::resolver::AnnotationMap;
use crate::typesystem::Dimension;
use crate::{ast::AstStatement, compile_error::CompileError, typesystem::DataTypeInformation};
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

pub struct DataTypeGenerator<'ink, 'b> {
    llvm: &'b Llvm<'ink>,
    index: &'b Index,
    annotations: &'b AnnotationMap,
    types_index: LlvmTypedIndex<'ink>,
}

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
    annotations: &AnnotationMap,
) -> Result<LlvmTypedIndex<'ink>, CompileError> {
    let mut generator = DataTypeGenerator {
        llvm,
        index,
        annotations,
        types_index: LlvmTypedIndex::new(),
    };

    let types = generator.index.get_types();
    for (name, user_type) in types {
        if let DataTypeInformation::Struct {
            name: struct_name, ..
        } = user_type.get_type_information()
        {
            generator
                .types_index
                .associate_type(name, llvm.create_struct_stub(struct_name).into())?;
        }
    }
    for (name, user_type) in types {
        let gen_type = generator.create_type(name, user_type)?;
        generator.types_index.associate_type(name, gen_type)?
    }
    for (name, user_type) in types {
        generator.expand_opaque_types(user_type)?;
        if let Some(initial_value) = generator.generate_initial_value(user_type) {
            generator
                .types_index
                .associate_initial_value(name, initial_value)?
        }
    }
    Ok(generator.types_index)
}

impl<'ink, 'b> DataTypeGenerator<'ink, 'b> {
    /// generates the members of an opaque struct and associates its initial values
    fn expand_opaque_types(&mut self, data_type: &DataType) -> Result<(), CompileError> {
        let information = data_type.get_type_information();
        if let DataTypeInformation::Struct { member_names, .. } = information {
            let mut struct_generator =
                StructGenerator::new(self.llvm, self.index, self.annotations, &self.types_index);
            let members: Vec<&VariableIndexEntry> = member_names
                .iter()
                .map(|variable_name| {
                    self.index
                        .find_member(data_type.get_name(), variable_name)
                        .unwrap()
                })
                .filter(|var| !var.is_return())
                //Avoid generating temp variables in llvm
                .filter(|var| !var.is_temp()) 
                .collect();
            let ((_, initial_value), member_values) =
                struct_generator.generate_struct_type(&members, data_type.get_name())?;
            for (member, value) in member_values {
                let qualified_name = format!("{}.{}", data_type.get_name(), member);
                self.types_index
                    .associate_initial_value(&qualified_name, value)?;
            }
            self.types_index
                .associate_initial_value(data_type.get_name(), initial_value)?;
        }
        Ok(())
    }

    /// Creates an llvm type to be associated with the given data type.
    /// Generates only an opaque type for structs.
    /// Eagerly generates but does not associate nested array and referenced aliased types
    fn create_type(
        &self,
        name: &str,
        data_type: &DataType,
    ) -> Result<BasicTypeEnum<'ink>, CompileError> {
        let information = data_type.get_type_information();
        match information {
            DataTypeInformation::Struct { .. } => {
                self.types_index.get_associated_type(data_type.get_name())
            }
            DataTypeInformation::Array {
                inner_type_name,
                dimensions,
                ..
            } => {
                let inner_type = self
                    .create_type(
                        inner_type_name,
                        self.index.find_type(inner_type_name).unwrap(),
                    )
                    .unwrap();

                self.create_nested_array_type(inner_type, dimensions)
                    .map(|it| it.as_basic_type_enum())
                    .map_err(|err| CompileError::codegen_error(err, SourceRange::undefined()))
                //TODO error location
            }
            DataTypeInformation::Integer { size, .. } => {
                get_llvm_int_type(self.llvm.context, *size, name).map(|it| it.into())
            }
            DataTypeInformation::Enum { name, .. } => {
                let enum_size = information.get_size();
                get_llvm_int_type(self.llvm.context, enum_size, name).map(|it| it.into())
            }
            DataTypeInformation::Float { size, .. } => {
                get_llvm_float_type(self.llvm.context, *size, name).map(|it| it.into())
            }
            DataTypeInformation::String { size, encoding } => {
                let string_size = size
                    .as_int_value(self.index)
                    .map_err(|it| CompileError::codegen_error(it, SourceRange::undefined()))?
                    as u32;
                Ok(self
                    .llvm
                    .context
                    .i8_type()
                    .array_type(string_size * encoding.get_bytes_per_char())
                    .into())
            }
            DataTypeInformation::SubRange {
                referenced_type, ..
            } => {
                let ref_type =
                    self.create_type(name, self.index.find_type(referenced_type).unwrap())?;
                Ok(ref_type)
            }
            DataTypeInformation::Alias {
                referenced_type, ..
            } => {
                let ref_type =
                    self.create_type(name, self.index.find_type(referenced_type).unwrap())?;
                Ok(ref_type)
            }
            // REVIEW: Void types are not basic type enums, so we return an int here
            DataTypeInformation::Void => {
                get_llvm_int_type(self.llvm.context, 32, "Void").map(Into::into)
            }
            DataTypeInformation::Pointer {
                inner_type_name, ..
            } => {
                let inner_type =
                    self.create_type(inner_type_name, self.index.get_type(inner_type_name)?)?;
                Ok(inner_type.ptr_type(AddressSpace::Generic).into())
            }
        }
    }

    fn generate_initial_value(&self, data_type: &DataType) -> Option<BasicValueEnum<'ink>> {
        let information = data_type.get_type_information();
        match information {
            DataTypeInformation::Struct { .. } => None, //Done elsewhere
            DataTypeInformation::Array { .. } => self
                .generate_array_initializer(
                    data_type,
                    data_type.get_name(),
                    |stmt| matches!(stmt, AstStatement::LiteralArray { .. }),
                    "LiteralArray",
                )
                .unwrap(),
            DataTypeInformation::Integer { .. } => None,
            DataTypeInformation::Enum { .. } => None,
            DataTypeInformation::Float { .. } => None,
            DataTypeInformation::String { .. } => self
                .generate_array_initializer(
                    data_type,
                    data_type.get_name(),
                    |stmt| matches!(stmt, AstStatement::LiteralString { .. }),
                    "LiteralString",
                )
                .unwrap(),
            DataTypeInformation::SubRange {
                referenced_type, ..
            } => self.register_aliased_initial_value(data_type, referenced_type),
            DataTypeInformation::Alias {
                referenced_type, ..
            } => self.register_aliased_initial_value(data_type, referenced_type),
            // Void types are not basic type enums, so we return an int here
            DataTypeInformation::Void => None, //get_llvm_int_type(llvm.context, 32, "Void").map(Into::into),
            DataTypeInformation::Pointer { .. } => None,
        }
    }

    /// generates and returns an optional inital value at the given dataType
    /// if no initial value is defined, it returns  an optional initial value of
    /// the aliased type (referenced_type)
    fn register_aliased_initial_value(
        &self,
        data_type: &DataType,
        referenced_type: &str,
    ) -> Option<BasicValueEnum<'ink>> {
        if let Some(initializer) = self
            .index
            .get_const_expressions()
            .maybe_get_constant_statement(&data_type.initial_value)
        {
            let generator = ExpressionCodeGenerator::new_context_free(
                self.llvm,
                self.index,
                self.annotations,
                &self.types_index,
                None,
            );
            let (_, initial_value) = generator.generate_expression(initializer).unwrap();
            Some(initial_value)
        } else {
            // if there's no initializer defined for this alias, we go and check the aliased type for an initial value
            self.index
                .get_type(referenced_type)
                .ok()
                .and_then(|referenced_data_type| self.generate_initial_value(referenced_data_type))
        }
    }

    /// generates and associates the given array-datatype (used for arrays and strings)
    fn generate_array_initializer(
        &self,
        data_type: &DataType,
        name: &str,
        predicate: fn(&AstStatement) -> bool,
        expected_ast: &str,
    ) -> Result<Option<BasicValueEnum<'ink>>, CompileError> {
        if let Some(initializer) = self
            .index
            .get_const_expressions()
            .maybe_get_constant_statement(&data_type.initial_value)
        {
            if predicate(initializer) {
                let array_type = self.index.get_type_information(name)?;
                let generator = ExpressionCodeGenerator::new_context_free(
                    self.llvm,
                    self.index,
                    self.annotations,
                    &self.types_index,
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
    fn create_nested_array_type(
        &self,
        end_type: BasicTypeEnum<'ink>,
        dimensions: &[Dimension],
    ) -> Result<ArrayType<'ink>, String> {
        let mut result: Option<ArrayType> = None;
        let mut current_type = end_type;

        for dimension in dimensions.iter().rev() {
            let len = dimension.get_length(self.index)?;
            result = Some(match current_type {
                BasicTypeEnum::IntType(..) => current_type.into_int_type().array_type(len),
                BasicTypeEnum::FloatType(..) => current_type.into_float_type().array_type(len),
                BasicTypeEnum::StructType(..) => current_type.into_struct_type().array_type(len),
                BasicTypeEnum::ArrayType(..) => current_type.into_array_type().array_type(len),
                BasicTypeEnum::PointerType(..) => current_type.into_pointer_type().array_type(len),
                BasicTypeEnum::VectorType(..) => current_type.into_vector_type().array_type(len),
            });
            current_type = result.unwrap().into();
        }
        Ok(result.unwrap())
    }
}
