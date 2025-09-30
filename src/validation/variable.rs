use plc_ast::ast::{
    ArgumentProperty, AstNode, AstStatement, CallStatement, ConfigVariable, Pou, PouType, Variable,
    VariableBlock, VariableBlockType,
};
use plc_diagnostics::diagnostics::Diagnostic;

use super::{
    array::validate_array_assignment,
    statement::{validate_assignment_mismatch, visit_statement},
    types::{data_type_is_fb_or_class_instance, visit_data_type_declaration},
    ValidationContext, Validator, Validators,
};
use crate::{index::const_expressions::ConstExpression, resolver::AnnotationMap};
use crate::{index::const_expressions::UnresolvableKind, typesystem::DataTypeInformation};
use crate::{index::PouIndexEntry, validation::statement::validate_enum_variant_assignment};
use crate::{index::VariableIndexEntry, resolver::StatementAnnotation};

pub fn visit_config_variable<T: AnnotationMap>(
    validator: &mut Validator,
    var_config: &ConfigVariable,
    context: &ValidationContext<T>,
) {
    let Some(StatementAnnotation::Variable { qualified_name, .. }) =
        context.annotations.get(&var_config.reference)
    else {
        // The template variable referenced in the VAR_CONFIG block does not exist
        validator.push_diagnostic(
            Diagnostic::new(format!(
                "Template variable `{}` does not exist",
                var_config.reference.get_flat_reference_name().unwrap_or_default(),
            ))
            .with_error_code("E101")
            .with_location(&var_config.location),
        );
        return;
    };

    let Some(var_template) = context.index.find_fully_qualified_variable(qualified_name) else {
        return;
    };

    // The template variable does exist, check
    // (1) if the types of the config and template variable are the same
    // (2) if the template variable has a hardware binding (`.. AT ... : ...`)
    // (3) if the config variable has specified a full hardware address
    // (4) if the template variable has specified a incomplete hardware address

    // (1)
    let (var_config_ty, var_config_ty_info) = {
        let ty_name = &var_config.data_type.get_name().unwrap_or_default();
        let ty = context.index.get_effective_type_or_void_by_name(ty_name);
        let ty_info = ty.get_type_information();

        (ty, ty_info)
    };

    let (var_template_ty, var_template_ty_info) = {
        let ty = context.index.get_effective_type_or_void_by_name(&var_template.data_type_name);
        let ty_info = context.index.find_elementary_pointer_type(ty.get_type_information());

        (ty, ty_info)
    };

    if var_template_ty_info != var_config_ty_info {
        validator.push_diagnostic(
            Diagnostic::new(format!(
                "Config and Template variable types differ ({} and {})",
                validator.get_type_name_or_slice(var_config_ty),
                validator.get_type_name_or_slice(var_template_ty)
            ))
            .with_error_code("E001")
            .with_location(var_config.location.span(&var_config.data_type.get_location()))
            .with_secondary_location(&var_template.source_location),
        )
    }

    // (2)
    if var_template.get_hardware_binding().is_none() {
        validator.push_diagnostic(
            Diagnostic::new(format!(
                "`{}` is missing a hardware binding",
                var_config
                    .reference
                    .get_parent_name_of_reference()
                    .or_else(|| var_config.reference.get_flat_reference_name())
                    .unwrap_or_default(),
            ))
            .with_error_code("E102")
            .with_location(&var_template.source_location)
            .with_secondary_location(var_config.location.span(&var_config.data_type.get_location())),
        );

        // Early return, because we may get further false-positive errors due to incorrect declaration
        return;
    }

    // (3)
    if var_config.address.is_template() {
        validator.push_diagnostic(
            Diagnostic::new("Variables defined in a VAR_CONFIG block must have a complete address")
                .with_error_code("E104")
                .with_location(&var_config.address.location),
        )
    }

    // (4)
    if !var_template.is_template() {
        validator.push_diagnostic(
                    Diagnostic::new("The configured variable is not a template, overriding non-template hardware addresses is not allowed")
                        .with_error_code("E103")
                        .with_location(&var_config.location)
                        .with_secondary_location(&var_template.source_location)
                )
    }
}

pub fn visit_variable_block<T: AnnotationMap>(
    validator: &mut Validator,
    pou: Option<&Pou>,
    block: &VariableBlock,
    context: &ValidationContext<T>,
) {
    validate_variable_block(validator, block);

    for variable in &block.variables {
        visit_variable(validator, variable, context);

        if let Some(referenced_type) = variable.data_type_declaration.get_referenced_type() {
            if context.index.get_type_information_or_void(&referenced_type).is_vla() {
                validate_vla(validator, pou, block, variable);
            }
        }
    }
}

fn validate_variable_block(validator: &mut Validator, block: &VariableBlock) {
    if matches!(block.kind, VariableBlockType::External) {
        validator.push_diagnostic(
            Diagnostic::new("VAR_EXTERNAL blocks have no effect")
                .with_error_code("E106")
                .with_location(&block.location),
        );
    }

    if block.constant
        && !matches!(
            block.kind,
            VariableBlockType::Global | VariableBlockType::Local | VariableBlockType::External
        )
    {
        validator.push_diagnostic(
            Diagnostic::new("This variable block does not support the CONSTANT modifier")
                .with_error_code("E034")
                .with_location(&block.location),
        )
    }
}

pub fn visit_variable<T: AnnotationMap>(
    validator: &mut Validator,
    variable: &Variable,
    context: &ValidationContext<T>,
) {
    validate_variable(validator, variable, context);
    visit_data_type_declaration(validator, &variable.data_type_declaration, context);
}

/// Validates Variable Length Arrays as specified in the IEC61131-3, i.e. VLAs are only allowed to be defined
/// inside the following Variable Block and POU combinations
/// - Input, Output and InOut within a Function or Method or
/// - InOut within Function-Block
fn validate_vla(validator: &mut Validator, pou: Option<&Pou>, block: &VariableBlock, variable: &Variable) {
    let Some(pou) = pou else {
        if matches!(block.kind, VariableBlockType::Global) {
            validator.push_diagnostic(
                Diagnostic::new("VLAs can not be defined as global variables")
                    .with_error_code("E044")
                    .with_location(&variable.location),
            )
        }

        return;
    };

    match (&pou.kind, block.kind) {
        (PouType::Function, VariableBlockType::Input(ArgumentProperty::ByVal)) => validator.push_diagnostic(
            Diagnostic::new(
                "Variable Length Arrays are always by-ref, even when declared in a by-value block",
            )
            .with_error_code("E047")
            .with_location(&variable.location),
        ),

        (PouType::Program, _) => validator.push_diagnostic(
            Diagnostic::new("Variable Length Arrays are not allowed to be defined inside a Program")
                .with_error_code("E044")
                .with_location(&variable.location),
        ),

        (
            PouType::Function | PouType::Method { .. },
            VariableBlockType::Input(ArgumentProperty::ByRef)
            | VariableBlockType::Output
            | VariableBlockType::InOut,
        )
        | (PouType::FunctionBlock, VariableBlockType::InOut) => (),

        _ => validator.push_diagnostic(
            Diagnostic::new(format!(
                "Variable Length Arrays are not allowed to be defined as {} variables inside a {}",
                block.kind, pou.kind
            ))
            .with_error_code("E044")
            .with_location(&variable.location),
        ),
    }
}

fn validate_array_ranges<T>(validator: &mut Validator, variable: &Variable, context: &ValidationContext<T>)
where
    T: AnnotationMap,
{
    let ty_name = variable.data_type_declaration.get_name().unwrap_or_default();
    let ty_info = context.index.get_effective_type_or_void_by_name(ty_name).get_type_information();

    if ty_info.is_array() {
        let mut types = vec![];
        ty_info.get_inner_array_types(&mut types, context.index);

        for ty in types {
            let DataTypeInformation::Array { dimensions, .. } = ty else {
                unreachable!("`get_inner_types()` only operates on Arrays");
            };

            for dimension in dimensions.iter().filter_map(|dim| dim.get_range(context.index).ok()) {
                let std::ops::Range { start, end } = dimension;

                if start > end {
                    validator.push_diagnostic(
                        Diagnostic::new(format!(
                            "Invalid range `{start}..{end}`, did you mean `{end}..{start}`?"
                        ))
                        .with_location(variable.location.clone())
                        .with_error_code("E097"),
                    );
                }
            }
        }
    }
}

fn validate_variable<T: AnnotationMap>(
    validator: &mut Validator,
    variable: &Variable,
    context: &ValidationContext<T>,
) {
    validate_variable_redeclaration(validator, variable, context);

    validate_array_ranges(validator, variable, context);

    if let Some(v_entry) = context.index.find_variable(context.qualifier, &[&variable.name]) {
        validate_reference_to_declaration(validator, context, variable, v_entry);

        if let Some(initializer) = &variable.initializer {
            // Assume `foo : ARRAY[1..5] OF DINT := [...]`, here the first function call validates the
            // assignment as a whole whereas the second function call (`visit_statement`) validates the
            // initializer in case it has further sub-assignments.
            validate_array_assignment(validator, context, variable);
            visit_statement(validator, initializer, context);
        }

        match v_entry
            .initial_value
            .and_then(|initial_id| context.index.get_const_expressions().find_const_expression(&initial_id))
        {
            Some(ConstExpression::Unresolvable { reason, statement }) => {
                match reason.as_ref() {
                    UnresolvableKind::Misc(reason) => validator.push_diagnostic(
                        Diagnostic::new(format!(
                            "Unresolved constant `{}` variable: {}",
                            variable.name.as_str(),
                            reason
                        ))
                        .with_error_code("E033")
                        .with_location(statement.get_location()),
                    ),
                    UnresolvableKind::Overflow(..) => (),
                    UnresolvableKind::Address(init) => {
                        let Some(node) = init.initializer.as_ref() else {
                            return;
                        };

                        let Some(rhs_ty) = context.annotations.get_type(node, context.index) else {
                            return;
                        };

                        if context.index.find_elementary_pointer_type(rhs_ty.get_type_information()).is_void()
                        {
                            // we could not find the type in the index, a validation for this exists elsewhere
                            return;
                        };

                        report_temporary_address_in_pointer_initializer(validator, context, v_entry, node);

                        validate_assignment_mismatch(
                            context,
                            validator,
                            context.index.get_effective_type_or_void_by_name(v_entry.get_type_name()),
                            rhs_ty,
                            &node.get_location(),
                        );
                    }
                };
            }
            Some(ConstExpression::Unresolved { statement, .. }) => {
                validator.push_diagnostic(
                    Diagnostic::new(format!("Unresolved constant `{}` variable", variable.name.as_str(),))
                        .with_error_code("E033")
                        .with_location(statement.get_location()),
                );
            }
            None if v_entry.is_constant() && !v_entry.is_var_external() => {
                validator.push_diagnostic(
                    Diagnostic::new(format!("Unresolved constant `{}` variable", variable.name.as_str(),))
                        .with_error_code("E033")
                        .with_location(&variable.location),
                );
            }
            _ => {
                if let Some(rhs) = variable.initializer.as_ref() {
                    validate_enum_variant_assignment(
                        context,
                        validator,
                        v_entry.get_qualified_name(),
                        context.index.get_effective_type_or_void_by_name(v_entry.get_type_name()),
                        rhs,
                    )
                }
            }
        }

        // check if we declared a constant fb-instance or class-instance
        if v_entry.is_constant() && data_type_is_fb_or_class_instance(v_entry.get_type_name(), context.index)
        {
            validator.push_diagnostic(
                Diagnostic::new(format!(
                    "Invalid constant {}, FUNCTION_BLOCK- and CLASS-instances cannot be declared constant",
                    v_entry.get_name()
                ))
                .with_error_code("E035")
                .with_location(&variable.location),
            );
        }
    }
}

/// Validates if a variable present in a parent POU has been redeclared in a child POU
fn validate_variable_redeclaration<T: AnnotationMap>(
    validator: &mut Validator,
    variable: &Variable,
    context: &ValidationContext<T>,
) {
    let Some(child_pou) = context.index.find_pou(context.qualifier.unwrap_or_default()) else {
        return;
    };

    let mut super_class = child_pou.get_super_class();
    while let Some(parent_str) = super_class {
        let Some(parent_pou) = context.index.find_pou(parent_str) else {
            return;
        };

        if let Some(shadowed_variable) =
            context.index.find_member(parent_pou.get_name(), variable.name.as_str()).filter(|v| !v.is_temp())
        {
            (!variable.location.is_internal()).then(|| {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Variable `{}` is already declared in parent POU `{}`",
                        variable.get_name(),
                        shadowed_variable.get_qualifier().unwrap_or_default()
                    ))
                    .with_error_code("E021")
                    .with_location(&variable.location)
                    .with_secondary_location(&shadowed_variable.source_location),
                )
            });
            break;
        }

        super_class = parent_pou.get_super_class();
    }
}

fn report_temporary_address_in_pointer_initializer<T: AnnotationMap>(
    validator: &mut Validator<'_>,
    context: &ValidationContext<'_, T>,
    v_entry: &VariableIndexEntry,
    initializer: &AstNode,
) {
    if v_entry.is_temp() {
        return;
    }

    if let Some(pou) = context.qualifier.and_then(|q| context.index.find_pou(q)) {
        match pou {
            PouIndexEntry::Program { .. }
            | PouIndexEntry::FunctionBlock { .. }
            | PouIndexEntry::Class { .. } => (),
            // PouIndexEntry::Method { .. } => {
            //     unimplemented!("We'll worry about this once we get around to OOP")
            // }
            _ => return,
        }
    }

    let (Some(flat_ref), Some(location)) = (match &initializer.get_stmt() {
        AstStatement::ReferenceExpr(_) => {
            (initializer.get_flat_reference_name(), Some(initializer.get_location()))
        }
        AstStatement::CallStatement(CallStatement { parameters, .. }) => (
            parameters.as_ref().and_then(|it| it.as_ref().get_flat_reference_name()),
            parameters.as_ref().map(|it| it.get_location()),
        ),
        _ => (None, None),
    }) else {
        return;
    };

    let Some(rhs_entry) = context.index.find_member(context.qualifier.unwrap_or_default(), flat_ref) else {
        return;
    };

    if !rhs_entry.is_temp() {
        return;
    }

    validator.diagnostics.push(
        Diagnostic::new("Cannot assign address of temporary variable to a member-variable")
            .with_location(location)
            .with_error_code("E109"),
    );
}

/// Returns a diagnostic if a `REFERENCE TO` variable is incorrectly declared.
fn validate_reference_to_declaration<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    variable: &Variable,
    variable_entry: &VariableIndexEntry,
) {
    let Some(variable_ty) = context.index.find_effective_type_by_name(variable_entry.get_type_name()) else {
        return;
    };

    if !(variable_ty.get_type_information().is_reference_to()
        || variable_ty.get_type_information().is_alias())
    {
        return;
    }

    let Some(inner_ty_name) = variable_ty.get_type_information().get_inner_pointer_type_name() else {
        unreachable!("`REFERENCE TO` is defined as a pointer, hence this must exist")
    };

    // Assert that the referenced type is no variable reference
    let qualifier = context.qualifier.unwrap_or_default();
    let inner_ty_is_local_var = context.index.find_member(qualifier, inner_ty_name).is_some();
    let inner_ty_is_global_var = context.index.find_global_variable(inner_ty_name).is_some();

    if inner_ty_is_local_var || inner_ty_is_global_var {
        validator.push_diagnostic(
            Diagnostic::new("REFERENCE TO variables can not reference other variables")
                .with_location(&variable_ty.location)
                .with_error_code("E099"),
        );
    }

    if let Some(ref initializer) = variable.initializer {
        report_temporary_address_in_pointer_initializer(validator, context, variable_entry, initializer);

        let Some(type_lhs) = context.annotations.get_type_hint(initializer, context.index) else { return };
        let Some(type_rhs) = context.annotations.get_type(initializer, context.index) else { return };

        validate_assignment_mismatch(context, validator, type_lhs, type_rhs, &initializer.location);
    }
}

#[cfg(test)]
mod variable_validator_tests {
    use insta::assert_snapshot;

    use crate::test_utils::tests::parse_and_validate_buffered;

    #[test]
    fn validate_empty_struct_declaration() {
        let diagnostics = parse_and_validate_buffered(
            "
        TYPE the_struct : STRUCT END_STRUCT END_TYPE

        PROGRAM prg
            VAR
                my_struct : STRUCT
                END_STRUCT
            END_VAR
        END_PROGRAM
        ",
        );
        assert_snapshot!(diagnostics);
    }

    #[test]
    fn validate_empty_enum_declaration() {
        let diagnostics = parse_and_validate_buffered(
            "
        TYPE my_enum : (); END_TYPE

        PROGRAM prg
            VAR
                my_enum : ();
            END_VAR
        END_PROGRAM
        ",
        );
        assert_snapshot!(diagnostics);
    }
}
