use super::ValidationContext;
use crate::{ast::Pou, typesystem::DataTypeInformation, Diagnostic, PouType};

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
        if let Some(data_type) = return_type {
            let type_info = data_type.get_type_information();
            let location = pou.return_type.to_owned().unwrap().get_location();
            match type_info {
                DataTypeInformation::Enum { .. } => self
                    .diagnostics
                    .push(Diagnostic::unsupported_return_type(type_info, location)),
                DataTypeInformation::Struct { .. } => self
                    .diagnostics
                    .push(Diagnostic::unsupported_return_type(type_info, location)),
                _ => {}
            }
        } else {
            self.diagnostics
                .push(Diagnostic::function_return_missing(pou.location.to_owned()));
        }
    }
}
