use plc_ast::ast::{Implementation, LinkageType, Pou, PouType, VariableBlockType};
use plc_diagnostics::diagnostics::Diagnostic;

use super::{
    statement::visit_statement, variable::visit_variable_block, ValidationContext, Validator, Validators,
};
use crate::{index::PouIndexEntry, resolver::AnnotationMap};

pub fn visit_pou<T: AnnotationMap>(validator: &mut Validator, pou: &Pou, context: &ValidationContext<'_, T>) {
    if pou.linkage != LinkageType::External {
        validate_pou(validator, pou, context);
        validate_interface_impl(validator, context, pou);

        for block in &pou.variable_blocks {
            visit_variable_block(validator, Some(pou), block, context);
        }
    }
}

// TODO: a method or property can be defined by multiple implemented interfaces, as long as the signature is the same
//       -> an error is shown when a method or property is declared in multiple interfaces of a function block, when the signature does not match
// TODO: Check that the interface is only implemented once
// TODO: Go over all new diagnostics and make sure they have correct (new?) error codes with a nice to read markdown file
fn validate_interface_impl<T>(validator: &mut Validator, ctxt: &ValidationContext<'_, T>, pou: &Pou)
where
    T: AnnotationMap,
{
    if pou.interfaces.is_empty() {
        return;
    }

    // Check if the interfaces are implemented on the correct POU types
    if !matches!(pou.pou_type, PouType::FunctionBlock | PouType::Class) {
        let location = {
            let location_first = pou.interfaces.first().unwrap();
            let location_last = pou.interfaces.last().unwrap();

            location_first.location.span(&location_last.location)
        };

        validator.push_diagnostic(
            Diagnostic::new("Interfaces can only be implemented by either classes or function blocks")
                .with_error_code("E001")
                .with_location(location),
        );
    }

    // Check if the declared interfaces exist, i.e. the comma seperated interfaces after `[...] IMPLEMENTS`
    let mut interfaces = Vec::new();
    for declaration in &pou.interfaces {
        match ctxt.index.find_interface(&declaration.name) {
            Some(interface) => interfaces.push(interface),

            None => {
                validator.push_diagnostic(
                    Diagnostic::new(format!("Interface `{}` does not exist", declaration.name))
                        .with_error_code("E001")
                        .with_location(&declaration.location),
                );
            }
        }
    }

    // Check if the POUs are implementing interfaces methods
    let methods_interface = interfaces.iter().flat_map(|it| it.get_methods(ctxt.index)).collect::<Vec<_>>();

    for method_interface in &methods_interface {
        // TODO(volsa): Find a better approach for this. Maybe refactor Pou and PouIndexEntry to have both
        //              a name and qualified_name field?
        let (_, method_name) = method_interface.get_name().split_once('.').unwrap();

        match ctxt.index.find_method(&pou.name, method_name) {
            Some(method_pou) => {
                validate_method_signature(validator, ctxt, method_pou, method_interface);
            }
            None => {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Method implementation of `{}` missing in POU `{}`",
                        method_name, pou.name
                    ))
                    .with_error_code("E002")
                    .with_location(&pou.name_location)
                    .with_secondary_location(method_interface.get_location()),
                );
            }
        }
    }
}

pub fn validate_method_signature<T>(
    validator: &mut Validator,
    ctxt: &ValidationContext<'_, T>,
    method_pou: &PouIndexEntry,
    method_interface: &PouIndexEntry,
) where
    T: AnnotationMap,
{
    // Check if the return type matches
    let return_type_pou = ctxt.index.get_return_type_or_void(method_pou.get_name());
    let return_type_interface = ctxt.index.get_return_type_or_void(method_interface.get_name());

    if return_type_pou != return_type_interface {
        validator.push_diagnostic(
            Diagnostic::new(format!(
                "Return type of method `{}` does not match the return type of the interface method, expected `{}` but got `{}` instead",
                method_pou.get_name(), return_type_interface.get_name(), return_type_pou.get_name()
            ))
            .with_error_code("E001")
            .with_location(method_pou.get_location())
            .with_secondary_location(method_interface.get_location()),
        );
    }

    // Check if the parameters match; note that the order of the parameters is important due to implicit calls
    let parameters_pou = ctxt.index.get_declared_parameters(method_pou.get_name());
    let parameters_interface = ctxt.index.get_declared_parameters(method_interface.get_name());

    dbg!(&parameters_pou, &parameters_interface);

    for (idx, parameter_interface) in parameters_interface.iter().enumerate() {
        match parameters_pou.get(idx) {
            Some(parameter_pou) => {
                // Name
                if parameter_pou.get_name() != parameter_interface.get_name() {
                    validator.push_diagnostic(
                        // TODO: be more explicit in error message as to why the order is important (implicit calls)
                        Diagnostic::new(format!(
                            "Expected parameter `{}` but got `{}`",
                            parameter_interface.get_name(),
                            parameter_pou.get_name()
                        ))
                        .with_error_code("E001")
                        .with_location(&parameter_interface.source_location)
                        .with_secondary_location(&parameter_pou.source_location),
                    );
                }

                // Type
                if parameter_pou.get_type_name() != parameter_interface.get_type_name() {
                    validator.push_diagnostic(
                        Diagnostic::new(format!(
                            "Expected parameter `{}` to have type `{}` but got `{}` instead",
                            parameter_pou.get_name(),
                            parameter_pou.get_type_name(),
                            parameter_interface.get_type_name(),
                        ))
                        .with_error_code("E001")
                        .with_location(method_pou.get_location())
                        .with_secondary_location(&parameter_interface.source_location),
                    );
                }

                // Declaration Type (VAR_INPUT, VAR_OUTPUT, VAR_IN_OUT)
                if parameter_pou.get_declaration_type() != parameter_interface.get_declaration_type() {
                    validator.push_diagnostic(
                        Diagnostic::new(format!(
                            "Expected parameter `{}` to have declaration type `{}` but got `{}` instead",
                            parameter_pou.get_name(),
                            parameter_interface.get_declaration_type().get_inner(),
                            parameter_pou.get_declaration_type().get_inner(),
                        ))
                        .with_error_code("E001")
                        .with_location(method_pou.get_location())
                        .with_secondary_location(&parameter_interface.source_location),
                    );
                }
            }

            // Method did not implement the parameter
            None => {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Parameter `{}` missing in method `{}`",
                        parameter_interface.get_name(),
                        method_pou.get_name()
                    ))
                    .with_error_code("E001")
                    .with_location(method_pou.get_location())
                    .with_secondary_location(&parameter_interface.source_location),
                );
            }
        }
    }

    // Exceeding parameters in the POU, which we did not catch in the for loop above because we were only
    // iterating over the interface parameters; anyhow any exceeding parameter is considered an error because
    // the function signature no longer holds
    if parameters_pou.len() > parameters_interface.len() {
        for parameter in parameters_pou.into_iter().skip(parameters_interface.len()) {
            validator.push_diagnostic(
                Diagnostic::new(format!(
                    "Parameter `{}` is not defined in the interface method",
                    parameter.get_name()
                ))
                .with_error_code("E001")
                .with_location(&parameter.source_location)
                .with_secondary_location(method_interface.get_location()),
            );
        }
    }
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

fn validate_pou<T: AnnotationMap>(validator: &mut Validator, pou: &Pou, context: &ValidationContext<'_, T>) {
    if pou.pou_type == PouType::Function {
        validate_function(validator, pou);
    };
    if pou.pou_type == PouType::Class {
        validate_class(validator, pou, context);
    };
    if pou.pou_type == PouType::Program {
        validate_program(validator, pou);
    }
}

fn validate_class<T: AnnotationMap>(validator: &mut Validator, pou: &Pou, context: &ValidationContext<T>) {
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

    // classes cannot have a return type
    if context.index.find_return_type(&pou.name).is_some() {
        validator.push_diagnostic(
            Diagnostic::new("A class cannot have a return type")
                .with_error_code("E020")
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
