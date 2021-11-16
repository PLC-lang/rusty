use std::convert::TryInto;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
/// the data_type_generator generates user defined data-types
/// - Structures
/// - Enum types
/// - SubRange types
/// - Alias types
/// - sized Strings
use crate::ast::SourceRange;
use crate::index::{Index, VariableIndexEntry, VariableType};
use crate::resolver::AnnotationMap;
use crate::typesystem::{Dimension, StringEncoding};
use crate::{ast::AstStatement, compile_error::CompileError, typesystem::DataTypeInformation};
use crate::{
    codegen::{
        llvm_index::LlvmTypedIndex,
        llvm_typesystem::{get_llvm_float_type, get_llvm_int_type},
    },
    typesystem::DataType,
};
use inkwell::values::BasicValue;
use inkwell::{
    types::{ArrayType, BasicType, BasicTypeEnum},
    values::BasicValueEnum,
    AddressSpace,
};

use super::{expression_generator::ExpressionCodeGenerator, llvm::Llvm};

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

    // first create all STUBs for struct types (empty structs)
    // and associate them in the llvm index
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
    // now create all other types (enum's, arrays, etc.)
    for (name, user_type) in types {
        let gen_type = generator.create_type(name, user_type)?;
        generator.types_index.associate_type(name, gen_type)?
    }

    // now since all types should be available in the llvm index, we can think about constructing and associating
    // initial values for the types
    for (name, user_type) in types {
        generator.expand_opaque_types(user_type)?;
        if let Some(init_value) = generator.generate_initial_value(user_type)? {
            generator
                .types_index
                .associate_initial_value(name, init_value)?;
        }
    }

    Ok(generator.types_index)
}

impl<'ink, 'b> DataTypeGenerator<'ink, 'b> {
    /// generates the members of an opaque struct and associates its initial values
    fn expand_opaque_types(&mut self, data_type: &DataType) -> Result<(), CompileError> {
        let information = data_type.get_type_information();
        if let DataTypeInformation::Struct { .. } = information {
            let members = self
                .index
                .get_container_members(data_type.get_name())
                .into_iter()
                .filter(|it| !it.is_temp() && !it.is_return())
                .map(|m| self.types_index.get_associated_type(m.get_type_name()))
                .collect::<Result<Vec<BasicTypeEnum>, CompileError>>()?;

            let struct_type = self
                .types_index
                .get_associated_type(data_type.get_name())
                .map(BasicTypeEnum::into_struct_type)?;

            struct_type.set_body(members.as_slice(), false);
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
            } => self
                .index
                .get_effective_type(inner_type_name)
                .and_then(|inner_type| self.create_type(inner_type_name, inner_type))
                .and_then(|inner_type| self.create_nested_array_type(inner_type, dimensions))
                .map(|it| it.as_basic_type_enum()),
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
                let base_type = if *encoding == StringEncoding::Utf8 {
                    self.llvm.context.i8_type()
                } else {
                    self.llvm.context.i16_type()
                };

                let string_size = size
                    .as_int_value(self.index)
                    .map_err(|it| CompileError::codegen_error(it, SourceRange::undefined()))?
                    as u32;
                Ok(base_type.array_type(string_size).into())
            }
            DataTypeInformation::SubRange {
                referenced_type, ..
            }
            | DataTypeInformation::Alias {
                referenced_type, ..
            } => self
                .index
                .get_effective_type(referenced_type)
                .and_then(|data_type| self.create_type(name, data_type)),
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

    fn generate_initial_value(
        &mut self,
        data_type: &DataType,
    ) -> Result<Option<BasicValueEnum<'ink>>, CompileError> {
        let information = data_type.get_type_information();
        match information {
            DataTypeInformation::Struct { .. } => {
                let members = self.index.get_container_members(data_type.get_name());
                let member_names_and_initializers = members
                    .iter()
                    .filter(|it| it.get_variable_type() != VariableType::Temp)
                    .map(|it| {
                        self.generate_initial_value_for_variable(it)
                            .and_then(|v| match v {
                                Some(v) => Ok((it.get_qualified_name(), v)),
                                None => self
                                    .types_index
                                    .get_associated_type(it.get_type_name())
                                    .map(get_default_for)
                                    .map(|v| (it.get_qualified_name(), v)),
                            })
                    })
                    .collect::<Result<Vec<(&str, BasicValueEnum)>, CompileError>>()?;

                let mut member_values: Vec<BasicValueEnum> = Vec::new();
                for (name, v) in &member_names_and_initializers {
                    self.types_index.associate_initial_value(name, *v)?;
                    member_values.push(*v);
                }

                let struct_type = self
                    .types_index
                    .get_associated_type(data_type.get_name())?
                    .into_struct_type();

                Ok(Some(
                    struct_type
                        .const_named_struct(&member_values)
                        .as_basic_value_enum(),
                ))
            }
            DataTypeInformation::Array { .. } => self.generate_array_initializer(
                data_type,
                |stmt| matches!(stmt, AstStatement::LiteralArray { .. }),
                "LiteralArray",
            ),
            DataTypeInformation::String { .. } => self.generate_array_initializer(
                data_type,
                |stmt| matches!(stmt, AstStatement::LiteralString { .. }),
                "LiteralString",
            ),
            DataTypeInformation::SubRange {
                referenced_type, ..
            } => self.generate_initial_value_for_type(data_type, referenced_type),
            DataTypeInformation::Alias {
                referenced_type, ..
            } => self.generate_initial_value_for_type(data_type, referenced_type),
            //all other types (scalars, pointer and void)
            _ => Ok(None),
        }
    }

    /// generates and returns an optional inital value at the given declared variable
    /// if no initial value is defined, it returns the initial value of the variable's
    /// datatype or Ok(None) if the type also has no declared default value
    fn generate_initial_value_for_variable(
        &mut self,
        variable: &VariableIndexEntry,
    ) -> Result<Option<BasicValueEnum<'ink>>, CompileError> {
        let initializer = variable.initial_value.and_then(|it| {
            self.index
                .get_const_expressions()
                .get_constant_statement(&it)
        });
        self.generate_initializer(
            variable.get_qualified_name(),
            initializer,
            variable.get_type_name(),
        )
    }

    /// generates and returns an optional inital value at the given dataType
    /// if no initial value is defined, it returns  an optional initial value of
    /// the aliased type (referenced_type)
    fn generate_initial_value_for_type(
        &mut self,
        data_type: &DataType,
        referenced_type: &str,
    ) -> Result<Option<BasicValueEnum<'ink>>, CompileError> {
        self.generate_initializer(
            data_type.get_name(),
            self.index
                .get_const_expressions()
                .maybe_get_constant_statement(&data_type.initial_value),
            referenced_type,
        )
    }

    /// generates the given initializer-statement if one is present
    /// if no initializer is present, it returns an optional default value
    /// of the given datatype.
    /// Errors will be reported using the given qualified_name
    fn generate_initializer(
        &mut self,
        qualified_name: &str,
        initializer: Option<&AstStatement>,
        data_type_name: &str,
    ) -> Result<Option<BasicValueEnum<'ink>>, CompileError> {
        if let Some(initializer) = initializer {
            let generator = ExpressionCodeGenerator::new_context_free(
                self.llvm,
                self.index,
                self.annotations,
                &self.types_index,
            );
            generator
                .generate_expression(initializer)
                .map(Some)
                .map_err(|_| {
                    CompileError::cannot_generate_initializer(
                        qualified_name,
                        initializer.get_location(),
                    )
                })
        } else {
            // if there's no initializer defined for this alias, we go and check the aliased type for an initial value
            self.index
                .get_type(data_type_name)
                .and_then(|referenced_data_type| self.generate_initial_value(referenced_data_type))
        }
    }

    /// generates and associates the given array-datatype (used for arrays and strings)
    fn generate_array_initializer(
        &self,
        data_type: &DataType,
        predicate: fn(&AstStatement) -> bool,
        expected_ast: &str,
    ) -> Result<Option<BasicValueEnum<'ink>>, CompileError> {
        if let Some(initializer) = self
            .index
            .get_const_expressions()
            .maybe_get_constant_statement(&data_type.initial_value)
        {
            if predicate(initializer) {
                let generator = ExpressionCodeGenerator::new_context_free(
                    self.llvm,
                    self.index,
                    self.annotations,
                    &self.types_index,
                );
                Ok(Some(generator.generate_literal(initializer)?))
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
    ) -> Result<ArrayType<'ink>, CompileError> {
        let dimensions: Vec<u32> = dimensions
            .iter()
            .map(|dimension| {
                dimension
                    .get_length(self.index)
                    .map_err(|it| CompileError::codegen_error(it, SourceRange::undefined()))
            })
            .collect::<Result<Vec<u32>, CompileError>>()?;

        let result = dimensions.iter().rev().fold(end_type, |current_type, len| {
            match current_type {
                BasicTypeEnum::IntType(..) => current_type.into_int_type().array_type(*len),
                BasicTypeEnum::FloatType(..) => current_type.into_float_type().array_type(*len),
                BasicTypeEnum::StructType(..) => current_type.into_struct_type().array_type(*len),
                BasicTypeEnum::ArrayType(..) => current_type.into_array_type().array_type(*len),
                BasicTypeEnum::PointerType(..) => current_type.into_pointer_type().array_type(*len),
                BasicTypeEnum::VectorType(..) => current_type.into_vector_type().array_type(*len),
            }
            .as_basic_type_enum()
        });

        let array_result: Result<ArrayType, _> = result.try_into();
        array_result.map_err(|_| {
            CompileError::codegen_error(
                format!("Expected ArrayType but found {:#?}", result),
                SourceRange::undefined(),
            )
        })
    }
}

pub fn get_default_for(basic_type: BasicTypeEnum) -> BasicValueEnum {
    match basic_type {
        BasicTypeEnum::ArrayType(t) => t.const_zero().into(),
        BasicTypeEnum::FloatType(t) => t.const_zero().into(),
        BasicTypeEnum::IntType(t) => t.const_zero().into(),
        BasicTypeEnum::PointerType(t) => t.const_zero().into(),
        BasicTypeEnum::StructType(t) => t.const_zero().into(),
        BasicTypeEnum::VectorType(t) => t.const_zero().into(),
    }
}
