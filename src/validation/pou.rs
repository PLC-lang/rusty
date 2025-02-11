use plc_ast::ast::{Implementation, InterfaceIdentifier, LinkageType, Pou, PouType, VariableBlockType};
use plc_diagnostics::diagnostics::Diagnostic;

use super::{
    statement::visit_statement, variable::visit_variable_block, ValidationContext, Validator, Validators,
};
use crate::{index::PouIndexEntry, resolver::AnnotationMap};
use std::collections::HashMap;

pub fn visit_pou<T: AnnotationMap>(validator: &mut Validator, pou: &Pou, context: &ValidationContext<'_, T>) {
    if pou.linkage != LinkageType::External {
        validate_pou(validator, pou);
        validate_interface_impl(validator, context, pou);
        validate_base_class(validator, context, pou);

        for block in &pou.variable_blocks {
            visit_variable_block(validator, Some(pou), block, context);
        }
    }
}

fn validate_base_class<T: AnnotationMap>(
    validator: &mut Validator<'_>,
    context: &ValidationContext<'_, T>,
    pou: &Pou,
) {
    // Find all the base classes of the pou
    let mut classes = vec![];
    let mut current = if let Some(InterfaceIdentifier { name, location }) = &pou.super_class {
        if let Some(super_class) = context.index.find_pou(name) {
            classes.push(super_class);
            super_class.get_super_class()
        } else {
            validator.push_diagnostic(
                Diagnostic::new(format!("Base `{}` does not exist", name))
                    .with_error_code("E048")
                    .with_location(location),
            );
            return;
        }
    } else {
        None
    };
    while let Some(super_class) = current {
        match context.index.find_pou(super_class) {
            Some(super_class) => {
                classes.push(super_class);
                current = super_class.get_super_class();
            }
            None => {
                return;
            }
        }
    }

    //Find all methods on classes
    // todo!()
}

fn validate_interface_impl<T>(validator: &mut Validator, ctxt: &ValidationContext<'_, T>, pou: &Pou)
where
    T: AnnotationMap,
{
    // No interfaces declared to implement
    if pou.interfaces.is_empty() {
        return;
    }

    // Check if the interfaces are implemented on the correct POU types
    if !matches!(pou.kind, PouType::FunctionBlock | PouType::Class) {
        let location = {
            let location_first = pou.interfaces.first().unwrap();
            let location_last = pou.interfaces.last().unwrap();

            location_first.location.span(&location_last.location)
        };

        validator.push_diagnostic(
            Diagnostic::new("Interfaces can only be implemented by classes or function blocks")
                .with_error_code("E110")
                .with_location(location),
        );
    }

    // Check if the declared interfaces exist, i.e. the comma seperated identifiers after `[...] IMPLEMENTS`
    let mut interfaces = Vec::new();
    for declaration in &pou.interfaces {
        match ctxt.index.find_interface(&declaration.name) {
            Some(interface) => {
                interfaces.push(interface);
            }

            None => {
                validator.push_diagnostic(
                    Diagnostic::new(format!("Interface `{}` does not exist", declaration.name))
                        .with_error_code("E048")
                        .with_location(&declaration.location),
                );
            }
        }
    }

    // We want to check if two or more interface methods with the same name also have the same signature
    let interface_methods = interfaces.iter().flat_map(|it| it.get_methods(ctxt.index)).collect::<Vec<_>>();
    let mut occurrences: HashMap<&str, Vec<&PouIndexEntry>> = HashMap::new();
    for method in &interface_methods {
        // XXX(volsa): Not entirely happy with this `split` call, is there a better approach?
        let name = method.get_name().rsplit_once('.').map(|(_, last)| last).unwrap_or(method.get_name());
        occurrences.entry(name).and_modify(|entries| entries.push(method)).or_insert(vec![method]);
    }

    let mut abort = false;
    for (name, methods) in occurrences.iter().filter(|(_, methods)| methods.len() > 1) {
        for method in methods.windows(2) {
            let (method_ref, method_impl) = (method[0], method[1]);
            let diagnostics = validate_method_signature(ctxt, method_ref, method_impl);

            if !diagnostics.is_empty() {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Method `{}` is defined with different signatures in interfaces `{}` and `{}`",
                        name,
                        method_ref.get_parent_pou_name(),
                        method_impl.get_parent_pou_name()
                    ))
                    .with_error_code("E111")
                    .with_location(&pou.name_location)
                    .with_secondary_location(method_ref.get_location())
                    .with_secondary_location(method_impl.get_location())
                    .with_sub_diagnostics(diagnostics),
                );

                abort = true;
            }
        }
    }

    // We want to early return here otherwise we could spam the user with lots of (valid) but identical
    // diagnostics reported earlier.
    if abort {
        return;
    }

    // Check if the POUs are implementing the interface methods
    for method_interface in &interface_methods {
        // XXX(volsa): Not entirely happy with this `split` call, is there a better approach?
        let (interface_name, method_name) = method_interface.get_name().split_once('.').unwrap();

        match ctxt.index.find_method(&pou.name, method_name) {
            Some(method_pou) => {
                let diagnostics = validate_method_signature(ctxt, method_interface, method_pou);

                for diagnostic in diagnostics {
                    validator.push_diagnostic(diagnostic);
                }
            }

            None => {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Method `{}` defined in interface `{}` is missing in POU `{}`",
                        method_name, interface_name, pou.name
                    ))
                    .with_error_code("E112")
                    .with_location(&pou.name_location)
                    .with_secondary_location(method_interface.get_location()),
                );
            }
        }
    }
}

pub fn validate_method_signature<T>(
    ctxt: &ValidationContext<'_, T>,
    method_ref: &PouIndexEntry,
    method_impl: &PouIndexEntry,
) -> Vec<Diagnostic>
where
    T: AnnotationMap,
{
    let mut diagnostics = Vec::new();

    // Check if the return type matches
    let return_type_ref = ctxt.index.get_return_type_or_void(method_ref.get_name());
    let return_type_impl = ctxt.index.get_return_type_or_void(method_impl.get_name());

    if return_type_impl != return_type_ref {
        diagnostics.push(
            Diagnostic::new(format!(
                "Return type of `{}` does not match the return type of the method defined in `{}`, expected `{}` but got `{}` instead",
                method_impl.get_name(),
                method_ref.get_parent_pou_name(),
                return_type_ref.get_name(),
                return_type_impl.get_name(),
            ))
            .with_error_code("E112")
            .with_location(method_impl)
            .with_secondary_location(method_ref),
        );
    }

    // Check if the parameters match; note that the order of the parameters is important due to implicit calls
    let parameters_ref = ctxt.index.get_declared_parameters(method_ref.get_name());
    let parameters_impl = ctxt.index.get_declared_parameters(method_impl.get_name());

    for (idx, parameter_ref) in parameters_ref.iter().enumerate() {
        match parameters_impl.get(idx) {
            Some(parameter_impl) => {
                // Name
                if parameter_impl.get_name() != parameter_ref.get_name() {
                    diagnostics.push(
                        Diagnostic::new(format!(
                            "Interface implementation mismatch: expected parameter `{}` but got `{}`",
                            parameter_ref.get_name(),
                            parameter_impl.get_name()
                        ))
                        .with_error_code("E112")
                        .with_location(&parameter_ref.source_location)
                        .with_secondary_location(&parameter_impl.source_location),
                    );
                }

                // Type
                if parameter_impl.get_type_name() != parameter_ref.get_type_name() {
                    diagnostics.push(
                        Diagnostic::new(format!(
                            "Interface implementation mismatch: Expected parameter `{}` to have `{}` as its type but got `{}`",
                            parameter_ref.get_name(),
                            parameter_ref.get_type_name(),
                            parameter_impl.get_type_name(),
                        ))
                        .with_error_code("E112")
                        .with_location(method_impl)
                        .with_secondary_location(&parameter_ref.source_location),
                    );
                }

                // Declaration Type (VAR_INPUT, VAR_OUTPUT, VAR_IN_OUT)
                if parameter_impl.get_declaration_type() != parameter_ref.get_declaration_type() {
                    diagnostics.push(
                        Diagnostic::new(format!(
                            "Interface implementation mismatch: Expected parameter `{}` to have `{}` as its declaration type but got `{}`",
                            parameter_impl.get_name(),
                            parameter_ref.get_declaration_type().get_inner(),
                            parameter_impl.get_declaration_type().get_inner(),
                        ))
                        .with_error_code("E112")
                        .with_location(method_impl)
                        .with_secondary_location(&parameter_ref.source_location),
                    );
                }
            }

            // Method did not implement the parameter
            None => {
                diagnostics.push(
                    Diagnostic::new(format!(
                        "Parameter `{} : {}` missing in method `{}`",
                        parameter_ref.get_name(),
                        parameter_ref.get_type_name(),
                        method_impl.get_name()
                    ))
                    .with_error_code("E112")
                    .with_location(method_impl)
                    .with_secondary_location(&parameter_ref.source_location),
                );
            }
        }
    }

    // Exceeding parameters in the POU, which we did not catch in the for loop above because we were only
    // iterating over the interface parameters; anyhow any exceeding parameter is considered an error because
    // the function signature no longer holds
    if parameters_impl.len() > parameters_ref.len() {
        for parameter in parameters_impl.into_iter().skip(parameters_ref.len()) {
            diagnostics.push(
                Diagnostic::new(format!(
                    "Parameter count mismatch: `{}` has more parameters than the method defined in `{}`",
                    method_impl.get_name(),
                    method_ref.get_parent_pou_name(),
                ))
                .with_error_code("E112")
                .with_location(&parameter.source_location)
                .with_secondary_location(method_ref),
            );
        }
    }

    diagnostics
}

pub fn visit_implementation<T: AnnotationMap>(
    validator: &mut Validator,
    implementation: &Implementation,
    context: &ValidationContext<'_, T>,
) {
    if implementation.pou_type == PouType::Class && !implementation.statements.is_empty() {
        validator.push_diagnostic(
            Diagnostic::new("A class cannot have an implementation")
                .with_error_code("E017")
                .with_location(&implementation.location),
        );
    }
    if implementation.linkage != LinkageType::External {
        validate_action_container(validator, implementation);
        //Validate the label uniqueness

        if let Some(labels) = context.index.get_labels(&implementation.name) {
            for (_, labels) in labels.entries() {
                let mut label_iter = labels.iter();
                if let Some(first) = label_iter.next() {
                    if let Some(second) = label_iter.next() {
                        //Collect remaining
                        let mut locations: Vec<_> = label_iter.map(|it| it.location.clone()).collect();
                        locations.push(first.location.clone());
                        locations.push(second.location.clone());
                        validator.push_diagnostic(
                            Diagnostic::new(format!("{}: Duplicate label.", &first.name))
                                .with_error_code("E018")
                                .with_location(&first.location)
                                .with_secondary_locations(locations),
                        );
                    }
                }
            }
        }
        implementation.statements.iter().for_each(|s| {
            visit_statement(validator, s, &context.with_qualifier(implementation.name.as_str()))
        });
    }
}

fn validate_pou(validator: &mut Validator, pou: &Pou) {
    if pou.kind == PouType::Function {
        validate_function(validator, pou);
    };
    if pou.kind == PouType::Class {
        validate_class(validator, pou);
    };
    if pou.kind == PouType::Program {
        validate_program(validator, pou);
    }
    //If the POU is not a function or method, it cannot have a return type
    if !matches!(pou.kind, PouType::Function | PouType::Method { .. }) {
        if let Some(start_return_type) = &pou.return_type {
            validator.push_diagnostic(
                Diagnostic::new(format!("POU Type {:?} does not support a return type", pou.kind))
                    .with_error_code("E026")
                    .with_location(start_return_type.get_location()),
            )
        }
    }
}

fn validate_class(validator: &mut Validator, pou: &Pou) {
    // var in/out/inout blocks are not allowed inside of class declaration
    // TODO: This should be on each block
    if pou.variable_blocks.iter().any(|it| {
        matches!(
            it.variable_block_type,
            VariableBlockType::InOut | VariableBlockType::Input(_) | VariableBlockType::Output
        )
    }) {
        validator.push_diagnostic(
            Diagnostic::new("A class cannot contain VAR_IN VAR_IN_OUT or VAR_OUT blocks")
                .with_error_code("E019")
                .with_location(&pou.name_location),
        );
    }
}

fn validate_function(validator: &mut Validator, pou: &Pou) {
    // functions cannot use EXTENDS
    if pou.super_class.is_some() {
        validator.push_diagnostic(
            Diagnostic::new("A function cannot use `EXTEND`")
                .with_error_code("E021")
                .with_location(&pou.name_location),
        );
    }
}

fn validate_program(validator: &mut Validator, pou: &Pou) {
    // programs cannot use EXTENDS
    if pou.super_class.is_some() {
        validator.push_diagnostic(
            Diagnostic::new("A program cannot use `EXTEND`")
                .with_error_code("E021")
                .with_location(&pou.name_location),
        );
    }
}

pub fn validate_action_container(validator: &mut Validator, implementation: &Implementation) {
    if implementation.pou_type == PouType::Action && implementation.type_name == "__unknown__" {
        validator.push_diagnostic(
            Diagnostic::new("Missing Actions Container Name")
                .with_error_code("E022")
                .with_location(&implementation.location),
        );
    }
}
