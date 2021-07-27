// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use thiserror::Error;

use crate::ast::SourceRange;
use crate::Diagnostic;

#[derive(Error, Debug, PartialEq)]
pub enum CompileError {
    #[error("Unknown reference '{reference}' at {location:?}")]
    InvalidReference {
        reference: String,
        location: SourceRange,
    },

    #[error("Unknown type '{type_name}' at {location:?}")]
    UnknownType {
        type_name: String,
        location: SourceRange,
    },

    #[error("{message}")]
    CodeGenError {
        message: String,
        location: SourceRange,
    },

    #[error("Cannot generate code outside of function context at {location:?}")]
    MissingFunctionError { location: SourceRange },

    #[error("Cannot cast from {type_name:} to {target_type:} at {location:?}")]
    CastError {
        type_name: String,
        target_type: String,
        location: SourceRange,
    },

    #[error("Cannot read File {path:}: {reason:}")]
    IoReadError { path: String, reason: String },

    #[error("Cannot write File {path:}: {reason:}")]
    IoWriteError { path: String, reason: String },
}

impl From<Diagnostic> for CompileError {
    fn from(diag: Diagnostic) -> Self {
        CompileError::codegen_error(diag.get_message().into(), diag.get_location())
    }
}

impl CompileError {
    pub fn missing_function(location: SourceRange) -> CompileError {
        CompileError::MissingFunctionError { location }
    }

    pub fn casting_error(
        type_name: &str,
        target_type: &str,
        location: SourceRange,
    ) -> CompileError {
        CompileError::CastError {
            type_name: type_name.to_string(),
            target_type: target_type.to_string(),
            location,
        }
    }

    pub fn invalid_reference(reference: &str, location: SourceRange) -> CompileError {
        CompileError::InvalidReference {
            reference: reference.to_string(),
            location,
        }
    }

    pub fn unknown_type(type_name: &str, location: SourceRange) -> CompileError {
        CompileError::UnknownType {
            type_name: type_name.to_string(),
            location,
        }
    }

    pub fn codegen_error(message: String, location: SourceRange) -> CompileError {
        CompileError::CodeGenError { message, location }
    }

    pub fn io_read_error(path: String, reason: String) -> CompileError {
        CompileError::IoReadError { path, reason }
    }

    pub fn io_write_error(path: String, reason: String) -> CompileError {
        CompileError::IoWriteError { path, reason }
    }

    pub fn no_type_associated(type_name: &str, location: SourceRange) -> CompileError {
        CompileError::CodeGenError {
            message: format!("No type associated to {:}", type_name),
            location,
        }
    }
}
