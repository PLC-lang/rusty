// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
/// the data_type_generator generates user defined data-types
/// - Structures
/// - Enum types
/// - SubRange types
/// - Alias types
/// - sized Strings
use crate::ast::SourceRange;
use crate::codegen::generators::struct_generator;
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

    // extend the stubbed structs with the member fields
    for (_, user_type) in types {
        generator.expand_opaque_types(user_type)?;
        /*if let Some(initial_value) = generator.generate_initial_value(user_type) {
            generator
                .types_index
                .associate_initial_value(name, initial_value)?
        }*/
    }

    // now since all types should be available in the llvm index, we can think about constructing and associating
    // initial values for the types
    for (name, user_type) in types {
        //TODO err handling
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
            // let (initial_value, member_values) =
            struct_generator.generate_struct_type(&members, data_type.get_name())?;
            // for (member, value) in member_values {
            //     let qualified_name = format!("{}.{}", data_type.get_name(), member);
            //     self.types_index
            //         .associate_initial_value(&qualified_name, value)?;
            // }
            // self.types_index
            //     .associate_initial_value(data_type.get_name(), initial_value)?;
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
                        self.index.find_effective_type(inner_type_name).unwrap(),
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
            } => {
                let ref_type = self.create_type(
                    name,
                    self.index.find_effective_type(referenced_type).unwrap(),
                )?;
                Ok(ref_type)
            }
            DataTypeInformation::Alias {
                referenced_type, ..
            } => {
                let ref_type = self.create_type(
                    name,
                    self.index.find_effective_type(referenced_type).unwrap(),
                )?;
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

    fn generate_initial_value(
        &mut self,
        data_type: &DataType,
    ) -> Result<Option<BasicValueEnum<'ink>>, CompileError> {
        let information = data_type.get_type_information();
        match information {
            DataTypeInformation::Struct { .. } => {
                let members = self.index.get_container_members(data_type.get_name());

                let member_pairs = members
                    .iter()
                    .filter(|it| it.get_variable_type() != VariableType::Temp)
                    .map(|it| {
                        self.generate_initial_value_for_variable(it)
                            .and_then(|v| match v {
                                Some(v) => Ok((it.get_qualified_name(), v)),
                                None => self
                                    .types_index
                                    .get_associated_type(it.get_type_name())
                                    .map(struct_generator::get_default_for)
                                    .map(|v| (it.get_qualified_name(), v)),
                            })
                    })
                    .collect::<Result<Vec<(&str, BasicValueEnum)>, CompileError>>()?;

                for (name, v) in &member_pairs {
                    self.types_index.associate_initial_value(name, *v)?;
                }

                let member_values = member_pairs
                    .iter()
                    .map(|(_, v)| *v)
                    .collect::<Vec<BasicValueEnum>>();
                let st = self
                    .types_index
                    .get_associated_type(data_type.get_name())?
                    .into_struct_type();
                Ok(Some(
                    st.const_named_struct(&member_values).as_basic_value_enum(),
                ))
            }
            DataTypeInformation::Array { .. } => self.generate_array_initializer(
                data_type,
                |stmt| matches!(stmt, AstStatement::LiteralArray { .. }),
                "LiteralArray",
            ),
            DataTypeInformation::Integer { .. } => Ok(None),
            DataTypeInformation::Enum { .. } => Ok(None),
            DataTypeInformation::Float { .. } => Ok(None),
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
            // Void types are not basic type enums, so we return an int here
            DataTypeInformation::Void => Ok(None), //get_llvm_int_type(llvm.context, 32, "Void").map(Into::into),
            DataTypeInformation::Pointer { .. } => Ok(None),
        }
    }

    fn generate_initial_value_for_variable(
        &mut self,
        variable: &VariableIndexEntry,
    ) -> Result<Option<BasicValueEnum<'ink>>, CompileError> {
        //is there an initializer defined?
        let initializer = variable.initial_value.and_then(|it| {
            self.index
                .get_const_expressions()
                .get_constant_statement(&it)
        });
        self.generate_initializer(initializer, variable.get_type_name())
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
            self.index
                .get_const_expressions()
                .maybe_get_constant_statement(&data_type.initial_value),
            referenced_type,
        )
    }

    fn generate_initializer(
        &mut self,
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
            generator.generate_expression(initializer).map(Some)
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
