use crate::{ast::Pou, Diagnostic};

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

    pub fn validate_pou(&mut self, _pou: &Pou) {}
}
