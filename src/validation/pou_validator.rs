use super::{ValidationContext, Validators};
use crate::{ast::Pou, Diagnostic, PouType};

/// validates POUs
#[derive(Default)]
pub struct PouValidator {
    diagnostics: Vec<Diagnostic>,
}

impl Validators for PouValidator {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
}

impl PouValidator {
    pub fn new() -> PouValidator {
        PouValidator { diagnostics: Vec::new() }
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
            self.push_diagnostic(Diagnostic::function_return_missing(pou.name_location.to_owned()));
        }
    }
}
