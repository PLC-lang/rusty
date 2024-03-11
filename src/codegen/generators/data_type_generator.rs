// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::codegen::debug::Debug;
use crate::index::{Index, VariableIndexEntry, VariableType};
use crate::resolver::{AstAnnotations, Dependency};
use crate::typesystem::{self, DataTypeInformation, Dimension, StringEncoding, StructSource};
use crate::{
    codegen::{
        debug::DebugBuilderEnum,
        llvm_index::LlvmTypedIndex,
        llvm_typesystem::{get_llvm_float_type, get_llvm_int_type},
    },
    typesystem::DataType,
};
use indexmap::IndexSet;
use inkwell::{
    types::{BasicType, BasicTypeEnum},
    values::{BasicValue, BasicValueEnum},
    AddressSpace,
};
use plc_ast::ast::{AstNode, AstStatement};
use plc_ast::literals::AstLiteral;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
/// the data_type_generator generates user defined data-types
/// - Structures
/// - Enum types
/// - SubRange types
/// - Alias types
/// - sized Strings
use std::collections::{HashMap, VecDeque};

use super::ADDRESS_SPACE_GENERIC;
use super::{expression_generator::ExpressionCodeGenerator, llvm::Llvm};

pub struct DataTypeGenerator<'ink, 'b> {
    llvm: &'b Llvm<'ink>,
    debug: &'b mut DebugBuilderEnum<'ink>,
    index: &'b Index,
    annotations: &'b AstAnnotations,
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
    debug: &mut DebugBuilderEnum<'ink>,
    dependencies: &IndexSet<Dependency>,
    index: &Index,
    annotations: &AstAnnotations,
) -> Result<LlvmTypedIndex<'ink>, Diagnostic> {
    let mut types = vec![];
    let mut pou_types = vec![];

    // Always add the builtin types
    let builtins = typesystem::get_builtin_types();
    for builtin in &builtins {
        types.push((builtin.get_name(), builtin));
    }

    for dep in dependencies {
        if let Dependency::Datatype(name) = dep {
            if let Some(pou) = index.find_pou(name) {
                if !pou.is_generic() && !pou.is_action() {
                    pou_types.push((name.as_str(), pou.get_instance_struct_type_or_void(index)));
                }
            } else if let Some(datatype) = index.find_type(name) {
                if !datatype.get_type_information().is_generic(index) {
                    types.push((name, datatype))
                }
            }
        }
    }

    let mut generator =
        DataTypeGenerator { llvm, debug, index, annotations, types_index: LlvmTypedIndex::default() };

    // first create all STUBs for struct types (empty structs)
    // and associate them in the llvm index
    for (name, user_type) in &types {
        if let DataTypeInformation::Struct { name: struct_name, .. } = user_type.get_type_information() {
            generator.types_index.associate_type(name, llvm.create_struct_stub(struct_name).into())?;
        }
    }
    // pou_types will always be struct
    for (name, user_type) in &pou_types {
        if let DataTypeInformation::Struct { name: struct_name, .. } = user_type.get_type_information() {
            generator.types_index.associate_pou_type(name, llvm.create_struct_stub(struct_name).into())?;
        }
    }

    // now create all other types (enum's, arrays, etc.)
    for (name, user_type) in &types {
        let gen_type = generator.create_type(name, user_type)?;
        generator.types_index.associate_type(name, gen_type)?
        //Get and associate debug type
    }

    for (name, user_type) in &pou_types {
        let gen_type = generator.create_type(name, user_type)?;
        generator.types_index.associate_pou_type(name, gen_type)?
    }

    // Combine the types and pou_types into a single Vector
    let mut types_to_init = VecDeque::new();
    types_to_init.extend(types);
    types_to_init.extend(pou_types);
    // now since all types should be available in the llvm index, we can think about constructing and associating
    for (_, user_type) in &types_to_init {
        //Expand all types
        generator.expand_opaque_types(user_type)?;
    }

    let mut tries = 0;
    let mut errors = HashMap::new();
    // If the tries are equal to the number of types remaining, it means we failed to resolve
    // anything
    while tries < types_to_init.len() {
        //Take the current element,
        if let Some((name, user_type)) = types_to_init.pop_front() {
            errors.remove(name);
            //try to resolve it
            match generator.generate_initial_value(user_type) {
                Err(err) => {
                    tries += 1;
                    types_to_init.push_back((name, user_type));
                    errors.insert(name, err);
                }
                Ok(init_value) => {
                    if let Some(init_value) = init_value {
                        if let Err(err) = generator.types_index.associate_initial_value(name, init_value) {
                            //If it fails, push it back into the list
                            tries += 1;
                            types_to_init.push_back((name, user_type));
                            errors.insert(name, err);
                        } else {
                            tries = 0;
                        }
                    }
                }
            }
        }
    }
    //If we didn't resolve anything this cycle, report the remaining issues and exit
    if !types_to_init.is_empty() {
        //Report each error as a new diagnostic, add the type's location as related to the error
        let diags = types_to_init
            .into_iter()
            .map(|(name, ty)| {
                errors
                    .remove(name)
                    .map(|diag| diag.with_secondary_location(ty.location.clone()))
                    .unwrap_or_else(|| Diagnostic::cannot_generate_initializer(name, ty.location.clone()))
            })
            .collect::<Vec<_>>();
        //Report the operation failure
        return Err(Diagnostic::new("Some initial values were not generated")
            .with_error_code("E075")
            .with_sub_diagnostics(diags));
    }
    Ok(generator.types_index)
}

impl<'ink, 'b> DataTypeGenerator<'ink, 'b> {
    /// generates the members of an opaque struct and associates its initial values
    fn expand_opaque_types(&mut self, data_type: &DataType) -> Result<(), Diagnostic> {
        let information = data_type.get_type_information();
        if let DataTypeInformation::Struct { source, members, .. } = information {
            let members = members
                .iter()
                .filter(|it| !it.is_temp() && !it.is_return())
                .map(|m| self.types_index.get_associated_type(m.get_type_name()))
                .collect::<Result<Vec<BasicTypeEnum>, Diagnostic>>()?;

            let struct_type = match source {
                StructSource::Pou(..) => self.types_index.get_associated_pou_type(data_type.get_name()),
                StructSource::OriginalDeclaration | StructSource::Internal(_) => {
                    self.types_index.get_associated_type(data_type.get_name())
                }
            }
            .map(BasicTypeEnum::into_struct_type)?;

            struct_type.set_body(members.as_slice(), false);
        }
        Ok(())
    }

    /// Creates an llvm type to be associated with the given data type.
    /// Generates only an opaque type for structs.
    /// Eagerly generates but does not associate nested array and referenced aliased types
    fn create_type(&mut self, name: &str, data_type: &DataType) -> Result<BasicTypeEnum<'ink>, Diagnostic> {
        self.debug.register_debug_type(name, data_type, self.index)?;
        let information = data_type.get_type_information();
        match information {
            DataTypeInformation::Struct { source, .. } => match source {
                StructSource::Pou(..) => self.types_index.get_associated_pou_type(data_type.get_name()),
                StructSource::OriginalDeclaration => {
                    self.types_index.get_associated_type(data_type.get_name())
                }
                StructSource::Internal(_) => self.types_index.get_associated_type(data_type.get_name()),
            },

            // We distinguish between two types of arrays, normal and variable length ones.
            // Variable Length Arrays are defined by having unknown sizes at compile time, thus we handle
            // these two cases based on whether the {start,end}-offset equals `TypeSize::Undetermined`.
            DataTypeInformation::Array { inner_type_name, dimensions, .. } => {
                if dimensions.iter().any(|dimension| dimension.is_undetermined()) {
                    self.create_type(inner_type_name, self.index.get_type(inner_type_name)?)
                } else {
                    self.index
                        .get_effective_type_by_name(inner_type_name)
                        .and_then(|inner_type| self.create_type(inner_type_name, inner_type))
                        .and_then(|inner_type| self.create_nested_array_type(inner_type, dimensions))
                        .map(|it| it.as_basic_type_enum())
                }
            }
            DataTypeInformation::Integer { size, .. } => {
                Ok(get_llvm_int_type(self.llvm.context, *size, name).into())
            }
            DataTypeInformation::Enum { name, referenced_type, .. } => {
                let effective_type = self.index.get_effective_type_or_void_by_name(referenced_type);
                if let DataTypeInformation::Integer { .. } = effective_type.get_type_information() {
                    self.create_type(name, effective_type)
                } else {
                    Err(Diagnostic::new(format!(
                        "Invalid type nature for generic argument. {} is no `ANY_INT`.",
                        effective_type.get_name()
                    ))
                    .with_error_code("E062"))
                }
            }
            DataTypeInformation::Float { size, .. } => {
                Ok(get_llvm_float_type(self.llvm.context, *size, name).into())
            }
            DataTypeInformation::String { size, encoding } => {
                let base_type = if *encoding == StringEncoding::Utf8 {
                    self.llvm.context.i8_type()
                } else {
                    self.llvm.context.i16_type()
                };

                let string_size = size
                    .as_int_value(self.index)
                    .map_err(|it| Diagnostic::codegen_error(it.as_str(), SourceLocation::undefined()))?
                    as u32;
                Ok(base_type.array_type(string_size).into())
            }
            DataTypeInformation::SubRange { referenced_type, .. }
            | DataTypeInformation::Alias { referenced_type, .. } => self
                .index
                .get_effective_type_by_name(referenced_type)
                .and_then(|data_type| self.create_type(name, data_type)),
            DataTypeInformation::Void => Ok(get_llvm_int_type(self.llvm.context, 32, "Void").into()),
            DataTypeInformation::Pointer { inner_type_name, .. } => {
                let inner_type = self.create_type(inner_type_name, self.index.get_type(inner_type_name)?)?;
                Ok(inner_type.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)).into())
            }
            DataTypeInformation::Generic { .. } => {
                unreachable!("Generic types should not be generated")
            }
        }
    }

    fn generate_initial_value(
        &mut self,
        data_type: &DataType,
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        let information = data_type.get_type_information();
        match information {
            DataTypeInformation::Struct { source, members, .. } => {
                let member_names_and_initializers = members
                    .iter()
                    .filter(|it| it.get_variable_type() != VariableType::Temp)
                    .map(|it| {
                        self.generate_initial_value_for_variable(it).and_then(|v| match v {
                            Some(v) => Ok((it.get_qualified_name(), v)),
                            None => self
                                .types_index
                                .get_associated_type(it.get_type_name())
                                .map(get_default_for)
                                .map(|v| (it.get_qualified_name(), v)),
                        })
                    })
                    .collect::<Result<Vec<(&str, BasicValueEnum)>, Diagnostic>>()?;

                let mut member_values: Vec<BasicValueEnum> = Vec::new();
                for (name, v) in &member_names_and_initializers {
                    self.types_index.associate_initial_value(name, *v)?;
                    member_values.push(*v);
                }

                let struct_type = match source {
                    StructSource::Pou(..) => self.types_index.get_associated_pou_type(data_type.get_name()),
                    StructSource::OriginalDeclaration => {
                        self.types_index.get_associated_type(data_type.get_name())
                    }
                    StructSource::Internal(_) => self.types_index.get_associated_type(data_type.get_name()),
                }?
                .into_struct_type();

                Ok(Some(struct_type.const_named_struct(&member_values).as_basic_value_enum()))
            }
            DataTypeInformation::Array { .. } => self.generate_array_initializer(
                data_type,
                |stmt| matches!(stmt.stmt, AstStatement::Literal(AstLiteral::Array { .. })),
                "LiteralArray",
            ),
            DataTypeInformation::String { .. } => self.generate_array_initializer(
                data_type,
                |stmt| matches!(stmt.stmt, AstStatement::Literal(AstLiteral::String { .. })),
                "LiteralString",
            ),
            DataTypeInformation::SubRange { referenced_type, .. } => {
                self.generate_initial_value_for_type(data_type, referenced_type)
            }
            DataTypeInformation::Alias { referenced_type, .. } => {
                self.generate_initial_value_for_type(data_type, referenced_type)
            }
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
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        let initializer = variable
            .initial_value
            .and_then(|it| self.index.get_const_expressions().get_constant_statement(&it));
        self.generate_initializer(variable.get_qualified_name(), initializer, variable.get_type_name())
    }

    /// generates and returns an optional inital value at the given dataType
    /// if no initial value is defined, it returns  an optional initial value of
    /// the aliased type (referenced_type)
    fn generate_initial_value_for_type(
        &mut self,
        data_type: &DataType,
        referenced_type: &str,
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        self.generate_initializer(
            data_type.get_name(),
            self.index.get_const_expressions().maybe_get_constant_statement(&data_type.initial_value),
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
        initializer: Option<&AstNode>,
        data_type_name: &str,
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        if let Some(initializer) = initializer {
            let generator = ExpressionCodeGenerator::new_context_free(
                self.llvm,
                self.index,
                self.annotations,
                &self.types_index,
            );
            generator.generate_expression(initializer).map(Some).map_err(|_| {
                Diagnostic::cannot_generate_initializer(qualified_name, initializer.get_location())
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
        predicate: fn(&AstNode) -> bool,
        expected_ast: &str,
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        if let Some(initializer) =
            self.index.get_const_expressions().maybe_get_constant_statement(&data_type.initial_value)
        {
            if predicate(initializer) {
                let generator = ExpressionCodeGenerator::new_context_free(
                    self.llvm,
                    self.index,
                    self.annotations,
                    &self.types_index,
                );
                Ok(Some(generator.generate_literal(initializer)?.get_basic_value_enum()))
            } else {
                Err(Diagnostic::codegen_error(
                    format!("Expected {expected_ast} but found {initializer:?}"),
                    initializer.get_location(),
                ))
            }
        } else {
            Ok(None)
        }
    }

    /// creates the llvm types for a multi-dimensional array
    ///
    /// an array with multiple dimensions will be flattened into a long
    /// 1-dimensional array.
    /// e.g. `arr: ARRAY[0..2, 0..2] OF INT` produces the same result like
    /// `arr: ARRAY[0..3] OF INT`.
    fn create_nested_array_type(
        &self,
        inner_type: BasicTypeEnum<'ink>,
        dimensions: &[Dimension],
    ) -> Result<BasicTypeEnum<'ink>, Diagnostic> {
        let len = dimensions
            .iter()
            .map(|dimension| {
                dimension
                    .get_length(self.index)
                    .map_err(|it| Diagnostic::codegen_error(it.as_str(), SourceLocation::undefined()))
            })
            .collect::<Result<Vec<u32>, Diagnostic>>()?
            .into_iter()
            .reduce(|a, b| a * b)
            .ok_or_else(|| {
                Diagnostic::codegen_error("Invalid array dimensions", SourceLocation::undefined())
            })?;

        let result = match inner_type {
            BasicTypeEnum::IntType(ty) => ty.array_type(len),
            BasicTypeEnum::FloatType(ty) => ty.array_type(len),
            BasicTypeEnum::StructType(ty) => ty.array_type(len),
            BasicTypeEnum::ArrayType(ty) => ty.array_type(len),
            BasicTypeEnum::PointerType(ty) => ty.array_type(len),
            BasicTypeEnum::VectorType(ty) => ty.array_type(len),
        }
        .as_basic_type_enum();

        Ok(result)
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
