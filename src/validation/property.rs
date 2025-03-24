use std::collections::HashSet;

use plc_ast::ast::{Interface, PropertyKind, VariableBlockType};
use plc_diagnostics::diagnostics::Diagnostic;

use crate::{index::PouIndexEntry, resolver::AnnotationMap};

use super::{ValidationContext, Validator, Validators};

pub fn visit_property<T>(validator: &mut Validator, context: &ValidationContext<T>)
where
    T: AnnotationMap,
{
    let Some(pou) = context.index.find_pou(context.qualifier.unwrap_or_default()) else {
        return;
    };

    validate_definition(validator, &pou);
    validate_name_clashes(validator, context, &pou);
    validate_overridden_signatures(validator, context, &pou);
}

fn validate_definition(validator: &mut Validator, pou: &PouIndexEntry) {
    for property in pou.get_properties_vec() {
        let mut count_get = 0;
        let mut count_set = 0;

        if !pou.is_stateful() {
            validator.push_diagnostic(
                Diagnostic::new(format!(
                    "Property `{}` must be defined in a stateful POU type (PROGRAM, CLASS or FUNCTION_BLOCK)",
                    property.ident.name,
                ))
                .with_location(pou.get_location())
                .with_error_code("E115"),
            );
        }

        for implementation in &property.implementations {
            for variable in &implementation.variable_blocks {
                if variable.location.is_internal() {
                    continue;
                }

                if !matches!(variable.kind, VariableBlockType::Local | VariableBlockType::Temp) {
                    validator.push_diagnostic(
                        Diagnostic::new("Properties only allow variable blocks of type VAR")
                            .with_location(&property.ident.location)
                            .with_secondary_location(&variable.location)
                            .with_error_code("E116"),
                    );
                }
            }

            match implementation.kind {
                PropertyKind::Get => count_get += 1,
                PropertyKind::Set => count_set += 1,
            }
        }

        if count_set + count_get == 0 {
            validator.push_diagnostic(
                Diagnostic::new("Property has neither a GET nor a SET block")
                    .with_location(&property.ident.location)
                    .with_error_code("E117"),
            );
            continue;
        }

        if count_get > 1 {
            validator.push_diagnostic(
                Diagnostic::new("Property has more than one GET block")
                    .with_location(&property.ident.location)
                    .with_error_code("E117"),
            );
        }

        if count_set > 1 {
            validator.push_diagnostic(
                Diagnostic::new("Property has more than one SET block")
                    .with_location(&property.ident.location)
                    .with_error_code("E117"),
            );
        }
    }
}

fn validate_name_clashes<T>(validator: &mut Validator, context: &ValidationContext<T>, pou: &PouIndexEntry)
where
    T: AnnotationMap,
{
    // TODO: See if we can improve this
    let mut seen = HashSet::new();
    let mut super_class = pou.get_super_class();
    while let Some(parent_str) = super_class {
        if !seen.insert(parent_str) {
            break;
        }

        let Some(pou_parent) = context.index.find_pou(parent_str) else {
            break;
        };

        // Check if any property in the current POU clashes with a variable in the parent POU
        for property in pou.get_properties_vec() {
            if let Some(variable) = context.index.find_member(parent_str, &property.ident.name) {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Name conflict between property and variable `{}` defined in POU `{}`",
                        property.ident.name,
                        pou.get_name()
                    ))
                    .with_error_code("E021")
                    .with_location(&property.ident.location)
                    .with_secondary_location(&variable.source_location),
                );
            }
        }

        // Check if any variable in the current POU clashes with a property in the parent POU
        for member in context.index.get_pou_members(context.qualifier.unwrap_or_default()) {
            if let Some(property) = pou_parent.get_property(&member.get_name()) {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Name conflict between property `{}` defined in `{}` and variable `{}` defined in POU `{}`",
                        property.name,
                        pou_parent.get_name(),
                        member.get_name(),
                        pou.get_name()
                    ))
                    .with_error_code("E021")
                    .with_location(&property.location)
                    .with_secondary_location(&member.source_location),
                );
            }
        }

        super_class = pou_parent.get_super_class();
    }
}

/// Validates if a derived property is redefined with a conflicting signature
fn validate_overridden_signatures<T>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    pou: &PouIndexEntry,
) where
    T: AnnotationMap,
{
    if pou.get_properties().is_none() {
        return;
    }

    let mut seen = HashSet::new();
    let mut super_class = pou.get_super_class();
    while let Some(parent_str) = super_class {
        if !seen.insert(parent_str) {
            break;
        }

        let Some(pou_parent) = context.index.find_pou(parent_str) else {
            break;
        };

        // No conflicts if one of the two has no properties
        let Some(properties_child) = pou.get_properties() else {
            break;
        };

        let Some(properties_parent) = pou_parent.get_properties() else {
            break;
        };

        for (name, property_child) in properties_child {
            if let Some(property_parent) = properties_parent.get(name) {
                let dt = {
                    let name = property_child.return_type.get_name().unwrap_or_default();
                    context.index.get_effective_type_or_void_by_name(name)
                };

                let dt_parent = {
                    let name = property_parent.return_type.get_name().unwrap_or_default();
                    context.index.get_effective_type_or_void_by_name(name)
                };

                if dt != dt_parent {
                    validator.push_diagnostic(
                        Diagnostic::new(format!(
                            "Overridden property `{}` has different signatures in POU `{}` and `{}`",
                            property_child.ident.name,
                            pou.get_name(),
                            pou_parent.get_name()
                        ))
                        .with_error_code("E112")
                        .with_location(&property_child.ident.location)
                        .with_secondary_location(&property_parent.ident.location),
                    );
                }
            }
        }

        super_class = pou_parent.get_super_class();
    }
}

pub(crate) fn validate_properties_in_interfaces<T>(
    validator: &mut Validator,
    context: &ValidationContext<'_, T>,
    interface: &Interface,
) where
    T: AnnotationMap,
{
    let interface = context.index.find_interface(&interface.identifier.name).expect("must exist");
    let derived_properties = interface
        .get_derived_interfaces_recursive(context.index)
        .iter()
        .map(|it| (it.ident.clone(), &it.properties))
        .flat_map(|(name, properties)| properties.into_iter().map(move |it| (name.clone(), it)))
        .collect::<Vec<_>>();

    for property in &interface.properties {
        for (ident_derived_intf, property_derived) in &derived_properties {
            if property.ident.name != property_derived.ident.name {
                continue;
            }

            let dt = {
                let name = property.return_type.get_name().unwrap_or_default();
                context.index.get_effective_type_or_void_by_name(name)
            };

            let dt_derived = {
                let name = property_derived.return_type.get_name().unwrap_or_default();
                context.index.get_effective_type_or_void_by_name(name)
            };

            if dt != dt_derived {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Property `{}` defined in `{}` has a different return type than in derived `{}` interface",
                        property.ident.name,
                        interface.ident.name,
                        ident_derived_intf.name
                    ))
                    .with_error_code("E048")
                    .with_location(property_derived.return_type.get_location())
                    .with_secondary_location(property.return_type.get_location())
                );
            }
        }
    }
}
