use super::ValidationContext;
use crate::{ast::Pou, Diagnostic, PouType};

/// validates POUs
pub struct PouValidator {
    pub diagnostics: Vec<Diagnostic>,
}

impl PouValidator {
    pub fn new() -> PouValidator {
        PouValidator {
            diagnostics: Vec::new(),
        }
    }

    pub fn validate_pou(&mut self, pou: &Pou, context: &ValidationContext) {
        if pou.pou_type == PouType::Function {
            self.validate_function(pou, context);
        };
    }

    pub fn validate_function(&mut self, pou: &Pou, context: &ValidationContext) {
        let return_type = context.index.find_return_type(&pou.name);
        // functions must have a return type
        if return_type.is_none() {
            self.diagnostics
                .push(Diagnostic::function_return_missing(pou.location.to_owned()));
        }
    }
}
