use crate::{
    ast::{Pou, PouType},
    Diagnostic,
};

/// validates POUs
pub struct PouValidator {
    pub diagnostics: Vec<Diagnostic>,
}

impl PouValidator {
    pub fn new() -> PouValidator {
        PouValidator { diagnostics: Vec::new()}
    }

    pub fn validate_pou(&mut self, pou: &Pou) {
        if pou.pou_type == PouType::Function && pou.return_type.is_none() {
            self.diagnostics.push(Diagnostic::function_return_missing(pou.location.clone()));
        } else if pou.pou_type != PouType::Function && pou.return_type.is_some() {
            self.diagnostics.push(Diagnostic::return_type_not_supported(
                &pou.pou_type,
                pou.location.clone(),
            ));
        }
    }
}
