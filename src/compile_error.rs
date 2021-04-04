/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::ops::Range;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CompileError {
    #[error("Unknown reference '{reference}' at {location:?}")]
    InvalidReference {
        reference: String,
        location: core::ops::Range<usize>,
    },

    #[error("Unknown type '{type_name}' at {location:?}")]
    UnknownType {
        type_name: String,
        location: core::ops::Range<usize>,
    },

    #[error("{message}")]
    CodeGenError {
        message: String,
        location: core::ops::Range<usize>,
    },

    #[error("Cannot generate code outside of function context at {location:?}")]
    MissingFunctionError { location: core::ops::Range<usize> },

    #[error("Cannot cast from {type_name:} to {target_type:} at {location:?}")]
    CastError {
        type_name: String,
        target_type: String,
        location: core::ops::Range<usize>,
    },
}

impl CompileError {
    pub fn missing_function(location: Range<usize>) -> CompileError {
        CompileError::MissingFunctionError {
            location: location.clone(),
        }
    }

    pub fn casting_error(
        type_name: &str,
        target_type: &str,
        location: Range<usize>,
    ) -> CompileError {
        CompileError::CastError {
            type_name: type_name.to_string(),
            target_type: target_type.to_string(),
            location,
        }
    }

    pub fn invalid_reference(reference: &str, location: Range<usize>) -> CompileError {
        CompileError::InvalidReference {
            reference: reference.to_string(),
            location,
        }
    }

    pub fn unknown_type(type_name: &str, location: Range<usize>) -> CompileError {
        CompileError::UnknownType {
            type_name: type_name.to_string(),
            location,
        }
    }

    pub fn codegen_error(message: String, location: Range<usize>) -> CompileError {
        CompileError::CodeGenError { message, location }
    }

    pub fn no_type_associated(type_name: &str, location: Range<usize>) -> CompileError {
        CompileError::CodeGenError {
            message: format!("No type associated to {:}", type_name),
            location,
        }
    }
}
