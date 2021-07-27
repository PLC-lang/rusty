use crate::{Diagnostic, ast::{Pou, PouType}};

use super::{ValidationContext};

/// validates POUs
pub struct PouValidator {}

impl PouValidator {
    pub fn new() -> PouValidator {
        PouValidator {}
    }

    pub fn validate_pou(&self, pou: &Pou, ctx: &mut ValidationContext) {
        if pou.pou_type == PouType::Function && pou.return_type.is_none() {
            ctx.report(Diagnostic::function_return_missing(pou.location.clone()));
        } else if pou.pou_type != PouType::Function && pou.return_type.is_some() {
            ctx.report(Diagnostic::return_type_not_supported(&pou.pou_type, pou.location.clone()));
        }
    }
}
