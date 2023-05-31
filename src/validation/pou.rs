use plc_ast::ast::{Implementation, LinkageType, Pou, PouType, VariableBlockType};
use plc_diagnostics::diagnostics::Diagnostic;

use super::{
    statement::visit_statement, variable::visit_variable_block, ValidationContext, Validator, Validators,
};
use crate::resolver::AnnotationMap;

pub fn visit_pou<T: AnnotationMap>(validator: &mut Validator, pou: &Pou, context: &ValidationContext<'_, T>) {
    if pou.linkage != LinkageType::External {
        validate_pou(validator, pou, context);

        for block in &pou.variable_blocks {
            visit_variable_block(validator, Some(pou), block, context);
        }
    }
}

pub fn visit_implementation<T: AnnotationMap>(
    validator: &mut Validator,
    implementation: &Implementation,
    context: &ValidationContext<'_, T>,
) {
    if implementation.pou_type == PouType::Class && !implementation.statements.is_empty() {
        validator.push_diagnostic(Diagnostic::syntax_error(
            "A class cannot have an implementation",
            implementation.location.to_owned(),
        ));
    }
    if implementation.linkage != LinkageType::External {
        validate_action_container(validator, implementation);
        //Validate the label uniquiness
        if let Some(labels) = context.index.get_labels(&implementation.name) {
            for (_, labels) in labels.entries() {
                let mut label_iter = labels.iter();
                if let Some(first) = label_iter.next() {
                    if let Some(second) = label_iter.next() {
                        //Collect remaining
                        let mut locations: Vec<_> = label_iter.map(|it| it.location.clone()).collect();
                        locations.push(first.location.clone());
                        locations.push(second.location.clone());
                        validator.push_diagnostic(Diagnostic::duplicate_label(
                            &first.name,
                            first.location.clone(),
                            locations,
                        ));
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
        validate_function(validator, pou, context);
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
    if pou.variable_blocks.iter().any(|it| {
        matches!(
            it.variable_block_type,
            VariableBlockType::InOut | VariableBlockType::Input(_) | VariableBlockType::Output
        )
    }) {
        validator.push_diagnostic(Diagnostic::syntax_error(
            "A class cannot have a var in/out/inout blocks",
            pou.name_location.to_owned(),
        ));
    }

    // classes cannot have a return type
    if context.index.find_return_type(&pou.name).is_some() {
        validator.push_diagnostic(Diagnostic::syntax_error(
            "A class cannot have a return type",
            pou.name_location.to_owned(),
        ));
    }
}

fn validate_function<T: AnnotationMap>(validator: &mut Validator, pou: &Pou, context: &ValidationContext<T>) {
    // functions cannot use EXTENDS
    if pou.super_class.is_some() {
        validator.push_diagnostic(Diagnostic::syntax_error(
            "A function cannot use EXTEND",
            pou.name_location.to_owned(),
        ));
    }

    let return_type = context.index.find_return_type(&pou.name);
    // functions must have a return type
    if return_type.is_none() {
        validator.push_diagnostic(Diagnostic::function_return_missing(pou.name_location.to_owned()));
    }
}

fn validate_program(validator: &mut Validator, pou: &Pou) {
    // programs cannot use EXTENDS
    if pou.super_class.is_some() {
        validator.push_diagnostic(Diagnostic::syntax_error(
            "A program cannot use EXTEND",
            pou.name_location.to_owned(),
        ));
    }
}

pub fn validate_action_container(validator: &mut Validator, implementation: &Implementation) {
    if implementation.pou_type == PouType::Action && implementation.type_name == "__unknown__" {
        validator.push_diagnostic(Diagnostic::missing_action_container(implementation.location.clone()));
    }
}
