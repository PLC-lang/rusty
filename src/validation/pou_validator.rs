use crate::{Diagnostic, ast::{Pou, PouType}};

use super::SemanticDiagnosticAcceptor;

/// validates POUs
pub struct PouValidator {}

impl PouValidator {
    pub fn new() -> PouValidator {
        PouValidator {}
    }

    pub fn validate_pou(&self, pou: &Pou, da: &mut dyn SemanticDiagnosticAcceptor) {
        if pou.pou_type == PouType::Function && pou.return_type.is_none() {
            da.report(Diagnostic::function_return_missing(pou.location.clone()));
        } else if pou.pou_type != PouType::Function && pou.return_type.is_some() {
            da.report(Diagnostic::return_type_not_supported(&pou.pou_type, pou.location.clone()));
        }
    }
}
