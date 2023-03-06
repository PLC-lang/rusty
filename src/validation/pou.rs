use super::{variable::visit_variable_block, ValidationContext, Validator, Validators};
use crate::{
    ast::{Implementation, Pou, PouType},
    Diagnostic,
};

pub fn visit_pou(validator: &mut Validator, pou: &Pou, context: &ValidationContext) {
    validate_pou(validator, pou, context);

    for block in &pou.variable_blocks {
        visit_variable_block(validator, block, context);
    }
}

fn validate_pou(validator: &mut Validator, pou: &Pou, context: &ValidationContext) {
    if pou.pou_type == PouType::Function {
        validate_function(validator, pou, context);
    };
}

fn validate_function(validator: &mut Validator, pou: &Pou, context: &ValidationContext) {
    let return_type = context.index.find_return_type(&pou.name);
    // functions must have a return type
    if return_type.is_none() {
        validator.push_diagnostic(Diagnostic::function_return_missing(pou.name_location.to_owned()));
    }
}

pub fn validate_action_container(validator: &mut Validator, implementation: &Implementation) {
    if implementation.pou_type == PouType::Action && implementation.type_name == "__unknown__" {
        validator.push_diagnostic(Diagnostic::missing_action_container(implementation.location.clone()));
    }
}
