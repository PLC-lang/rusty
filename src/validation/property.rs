//! Module validating properties
//!
//! Properties are lowered into methods, so most validation logic is already handled by existing code.
//! However, some of these validations produce generic error messages. This module provides more specific
//! validations to improve the error reporting experience for users with regards to properties.

use std::collections::HashSet;

use itertools::Itertools;
use plc_ast::ast::{Identifier, Interface, PropertyBlock, PropertyKind, VariableBlockType};
use plc_diagnostics::diagnostics::Diagnostic;
use rustc_hash::FxHashMap;

use crate::{index::PouIndexEntry, resolver::AnnotationMap};

use super::{ValidationContext, Validator, Validators};

pub fn visit_property<T>(validator: &mut Validator, context: &ValidationContext<T>)
where
    T: AnnotationMap,
{
    let Some(pou) = context.index.find_pou(context.qualifier.unwrap_or_default()) else {
        return;
    };

    validate_definition(validator, pou);
    validate_name_clashes(validator, context, pou);
    validate_overridden_signatures(validator, context, pou);
}

fn validate_definition(validator: &mut Validator, pou: &PouIndexEntry) {
    for property in pou.get_properties_vec() {
        let mut count_get = 0;
        let mut count_set = 0;

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
            if let Some(variable) = context.index.find_local_member(parent_str, &property.ident.name) {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Name conflict between property and variable `{}` defined in POU `{}` and `{}`",
                        property.ident.name,
                        pou_parent.get_name(),
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
            if let Some(property) = pou_parent.get_property(member.get_name()) {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Name conflict between property and variable `{}` defined in POU `{}` and `{}`",
                        property.name,
                        pou_parent.get_name(),
                        pou.get_name(),
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
            super_class = pou_parent.get_super_class();
            continue;
        };

        let Some(properties_parent) = pou_parent.get_properties() else {
            super_class = pou_parent.get_super_class();
            continue;
        };

        for (name, property_child) in properties_child {
            if let Some(property_parent) = properties_parent.get(name) {
                let dt = {
                    let name = property_child.datatype.get_name().unwrap_or_default();
                    context.index.get_effective_type_or_void_by_name(name)
                };

                let dt_parent = {
                    let name = property_parent.datatype.get_name().unwrap_or_default();
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
    let interface = context.index.find_interface(&interface.ident.name).expect("must exist");

    // Retrieve all properties an interface inherits directly or indirectly and map them into tuples of
    // (<interface name>, <property defined in that interface>)
    let derived_properties = interface
        .get_derived_interfaces_recursive(context.index)
        .iter()
        .map(|it| (&it.ident, &it.properties))
        .flat_map(|(name, properties)| properties.iter().map(|property| (name.clone(), property)))
        .collect_vec();

    // Group all these properties by their name
    let mut clusters: FxHashMap<String, Vec<(Identifier, &PropertyBlock)>> = FxHashMap::default();
    for (intf_ident, property) in derived_properties {
        clusters.entry(property.ident.name.clone()).or_default().push((intf_ident, property));
    }

    // Check if properties in these clusters have the same type, otherwise we can't implement them in e.g. a FB
    for ((left_intf_ident, left_property), (right_intf_ident, right_property)) in
        clusters.values().filter(|properties| properties.len() > 1).flatten().tuple_windows()
    {
        let dt_left = {
            let name = left_property.datatype.get_name().unwrap_or_default();
            context.index.get_effective_type_or_void_by_name(name)
        };

        let dt_right = {
            let name = right_property.datatype.get_name().unwrap_or_default();
            context.index.get_effective_type_or_void_by_name(name)
        };

        if dt_left != dt_right {
            validator.push_diagnostic(
                Diagnostic::new(format!(
                    "Property `{}` defined in interface `{}` and `{}` have different datatypes",
                    left_property.ident.name, left_intf_ident.name, right_intf_ident.name
                ))
                .with_error_code("E112")
                .with_location(&interface.ident.location)
                .with_secondary_locations(vec![
                    left_property.datatype.get_location(),
                    right_property.datatype.get_location(),
                ]),
            );
        }
    }
}
