use plc_ast::ast::{AstNode, AstStatement, DataType, DataTypeDeclaration, PouType, UserTypeDeclaration};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    resolver::AnnotationMap,
    typesystem::{DataTypeInformation, StructSource},
};

use super::{variable::visit_variable, ValidationContext, Validator, Validators};

pub fn visit_data_type_declaration<T: AnnotationMap>(
    validator: &mut Validator,
    declaration: &DataTypeDeclaration,
    context: &ValidationContext<T>,
) {
    if let DataTypeDeclaration::DataTypeDefinition { data_type, location, .. } = declaration {
        visit_data_type(validator, data_type, location, context);
    }
}

pub fn visit_data_type<T: AnnotationMap>(
    validator: &mut Validator,
    data_type: &DataType,
    location: &SourceLocation,
    context: &ValidationContext<T>,
) {
    validate_data_type(validator, data_type, location);

    let context = &context.with_optional_qualifier(data_type.get_name());
    match data_type {
        DataType::StructType { variables, .. } => {
            variables.iter().for_each(|v| visit_variable(validator, v, context))
        }
        DataType::ArrayType { referenced_type, .. } => {
            visit_data_type_declaration(validator, referenced_type, context)
        }
        DataType::VarArgs { referenced_type: Some(referenced_type), .. } => {
            visit_data_type_declaration(validator, referenced_type.as_ref(), context);
        }
        _ => {}
    }
}

fn validate_data_type(validator: &mut Validator, data_type: &DataType, location: &SourceLocation) {
    match data_type {
        DataType::StructType { variables, .. } => {
            if variables.is_empty() {
                validator.push_diagnostic(
                    Diagnostic::new("Variable block is empty")
                        .with_error_code("E028")
                        .with_location(location.clone()),
                );
            }
        }
        DataType::EnumType {
            elements: AstNode { stmt: AstStatement::ExpressionList(expressions), .. },
            ..
        } if expressions.is_empty() => {
            validator.push_diagnostic(
                Diagnostic::new("Variable block is empty")
                    .with_error_code("E028")
                    .with_location(location.clone()),
            );
        }
        DataType::VarArgs { referenced_type: None, sized: true } => validator.push_diagnostic(
            Diagnostic::new("Missing datatype: Sized Variadics require a known datatype.")
                .with_error_code("E038")
                .with_location(location.clone()),
        ),
        _ => {}
    }
}

pub fn visit_user_type_declaration<T: AnnotationMap>(
    validator: &mut Validator,
    user_type: &UserTypeDeclaration,
    context: &ValidationContext<T>,
) {
    visit_data_type(validator, &user_type.data_type, &user_type.location, context);
}

pub fn data_type_is_fb_or_class_instance(type_name: &str, index: &Index) -> bool {
    let data_type_info = index.find_effective_type_by_name(type_name).map_or_else(
        || index.get_void_type().get_type_information(),
        crate::typesystem::DataType::get_type_information,
    );

    if let DataTypeInformation::Struct {
        source: StructSource::Pou(PouType::FunctionBlock) | StructSource::Pou(PouType::Class),
        ..
    } = data_type_info
    {
        return true;
    }

    match data_type_info {
        DataTypeInformation::Struct { members, .. } =>
        //see if any member is fb or class intance
        {
            members.iter().any(|member| data_type_is_fb_or_class_instance(member.get_type_name(), index))
        }
        DataTypeInformation::Array { inner_type_name, .. } => {
            data_type_is_fb_or_class_instance(inner_type_name.as_str(), index)
        }
        DataTypeInformation::Pointer { inner_type_name, .. } => {
            data_type_is_fb_or_class_instance(inner_type_name.as_str(), index)
        }
        DataTypeInformation::Alias { referenced_type, .. } => {
            data_type_is_fb_or_class_instance(referenced_type.as_str(), index)
        }
        _ => false,
    }
}
