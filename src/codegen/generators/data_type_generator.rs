/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// the data_type_generator generates user defined data-types
/// - Structures
/// - Enum types
/// - SubRange types
/// - Alias types
use inkwell::{module::Module, types::{ArrayType, BasicType, BasicTypeEnum}, values::{BasicValue}};
use crate::{ast::{DataType, Statement, UserTypeDeclaration, Variable}, compile_error::CompileError, index::{DataTypeInformation, Dimension, Index}};

use super::{expression_generator::ExpressionCodeGenerator, llvm::LLVM, struct_generator::StructGenerator};

/// generates a stub type (e.g. an opaque struct type) for the given type. We generate stubs for all types
/// before we generate the type-details so we avoid ordering-problems or problems with circular dependencies
///
/// generated type-stubs are registerd in the index
/// array types are generated right away
pub fn generate_data_type_stubs<'a>(llvm: &LLVM<'a>, index: &mut Index<'a>, data_types: &Vec<UserTypeDeclaration>) -> Result<(), CompileError>{
    for user_type in data_types {
        match &user_type.data_type {
            DataType::StructType { name, variables } => {
                let member_names :Vec<String> = variables.iter().map(|it| it.name.to_string()).collect();
                index.associate_type(
                    name.as_ref().unwrap().as_str(),
                    DataTypeInformation::Struct {
                        name: name.clone().unwrap(),
                        generated_type: llvm
                            .create_struct_stub(name.as_ref().unwrap())
                            .into(),
                        member_names
                    },
                );
            }
            DataType::EnumType { name, elements: _ } => index.associate_type(
                name.as_ref().unwrap().as_str(),
                DataTypeInformation::Integer {
                    signed: true,
                    size: 32,
                    generated_type: llvm.i32_type().as_basic_type_enum(),
                },
            ),
            DataType::SubRangeType {
                name,
                referenced_type: type_ref_name,
            } => {
                let alias_name = name.as_ref().unwrap();
                index.associate_type(
                    alias_name,
                    DataTypeInformation::Alias {
                        name: alias_name.clone(),
                        referenced_type: type_ref_name.clone(),
                    },
                );
            }
            DataType::ArrayType {
                name,
                bounds,
                referenced_type,
            } => {
                let dimensions = get_array_dimensions(bounds)?;

                let referenced_type_name = referenced_type.get_name().unwrap();
                let target_type = index.get_type_information(referenced_type_name)?.get_type();
                let internal_type = index
                    .find_type_information(referenced_type_name)
                    .unwrap();
                index.associate_type(
                    name.as_ref().unwrap().as_str(),
                    DataTypeInformation::Array {
                        inner_type_name: referenced_type_name.to_string(),
                        inner_type_hint: Box::new(internal_type),
                        generated_type: create_nested_array_type(
                            target_type,
                            dimensions.clone(),
                        )
                        .as_basic_type_enum(),
                        dimensions,
                    },
                )
            }
            DataType::StringType {..} => {
                unimplemented!();
            }
        };
    }
    Ok(())
}

/// generates the llvm-type for the given data-type and registers it at the index
/// this function may create and register a ... 
/// - Struct type for a STRUCT
/// - global variables for enum-elements
/// - an alias index entry for sub-range types
/// - TODO array
pub fn generate_data_type<'a>(
    module: &Module<'a>,
    llvm: &LLVM<'a>,
    index: &mut Index<'a>,
    data_types: &Vec<UserTypeDeclaration>,
) -> Result<(), CompileError> {
    for data_type in data_types {
        match &data_type.data_type {
            DataType::StructType { name, variables } => {
                let name = name.as_ref().unwrap();
                let mut struct_generator = StructGenerator::new(llvm, index);
                let members: Vec<&Variable> = variables.iter().collect();
                let (_, initial_value) = struct_generator.generate_struct_type(&members, name)?;
                index.associate_type_initial_value(name, initial_value);
                
            }
            DataType::EnumType { name: _, elements } => {
                // generate a global variable for every enum element
                for (i, element) in elements.iter().enumerate() {
                    let int_type = llvm.i32_type();
                    let element_variable = llvm.create_global_variable(
                        module, 
                        element,
                        int_type.as_basic_type_enum(),
                        Some(int_type.const_int(i as u64, false).as_basic_value_enum()),
                    );

                    //associate the enum element's global variable with the enume-element's name
                    index.associate_global_variable(element, element_variable.as_pointer_value());
                }
            }
            DataType::SubRangeType {
                name,
                referenced_type,
            } => {
                let alias_name = name.as_ref().map(|it| it.as_str()).unwrap();
                index.associate_type_alias(alias_name, referenced_type.as_str());
                if let Some(initializer) = &data_type.initializer {
                    let generator = ExpressionCodeGenerator::new_context_free(llvm, index, None);
                    let (_, initial_value) = generator.generate_expression(initializer)?;
                    index.associate_type_initial_value(alias_name, initial_value);
                }
            }
            DataType::ArrayType { name, .. } => {
                if let Some(initializer) = &data_type.initializer {
                    if let Statement::LiteralArray{ .. } = initializer {
                        let name = name.as_ref().ok_or_else(|| CompileError::codegen_error("Expected named datatype but found none".to_string(), initializer.get_location()))?;
                        let array_type = index.get_type_information(name)?;
                        let generator = ExpressionCodeGenerator::new_context_free(llvm, index, Some(array_type));
                        let (_, initial_value) = generator.generate_literal(initializer)?;
                        index.associate_type_initial_value(name, initial_value)
                    } else {
                        return Err(CompileError::codegen_error(
                            format!("Expected LiteralArray but found {:?}", initializer), initializer.get_location()));
                    }
                }
            }
            DataType::StringType { .. } => {unimplemented!()}
        }
    }
    Ok(())
}

/// constructs a vector with all dimensions for the given bounds-statement
/// e.g. [0..10, 0..5]
fn get_array_dimensions(bounds: &Statement) -> Result<Vec<Dimension>, CompileError> {
    let mut result = vec![];
    for statement in bounds.get_as_list() {
        result.push(get_single_array_dimension(statement)?);
    }
    Ok(result)
}

/// constructs a Dimension for the given RangeStatement
/// throws an error if the given statement is no RangeStatement
fn get_single_array_dimension(bounds: &Statement) -> Result<Dimension, CompileError> {
    if let Statement::RangeStatement { start, end } = bounds {
        let start_offset = evaluate_constant_int(start).unwrap_or(0);
        let end_offset = evaluate_constant_int(end).unwrap_or(0);
        Ok(Dimension {
            start_offset,
            end_offset,
        })
    } else {
        Err(CompileError::codegen_error(format!("Unexpected Statement {:?}, expected range", bounds), bounds.get_location()))
    }
}

/// extracts the compile-time value of the given statement.
/// returns an error if no value can be derived at compile-time
fn extract_value(s: &Statement) -> Result<String, CompileError> {
    match s {
        Statement::UnaryExpression {
            operator, value, ..
        } => extract_value(value).map(|result| format!("{}{}", operator, result)),
        Statement::LiteralInteger { value, .. } => Ok(value.to_string()),
        //TODO constatnts
        _ => Err(CompileError::codegen_error("Unsupported Statement. Cannot evaluate expression.".to_string() , s.get_location())),
    }
}

/// evaluate the given statemetn as i32
fn evaluate_constant_int(s: &Statement) -> Result<i32, CompileError> {
    let value = extract_value(s);
    value.map(|v| v.parse().unwrap_or(0))
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