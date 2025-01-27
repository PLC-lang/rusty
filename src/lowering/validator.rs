use std::{ops::Deref, sync::RwLock};

use plc_ast::ast::{Property, PropertyImplementation, PropertyKind};
use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};

pub struct ParticipantValidator {
    pub diagnostics: Vec<Diagnostic>,
}

impl ParticipantValidator {
    // TODO: Temporary solution with that diagnostician, ideally the diagnostician lazy reads source files and
    // doesn't rely on register_file
    pub fn new() -> ParticipantValidator {
        ParticipantValidator { diagnostics: Vec::new() }
    }

    pub fn validate_properties(&mut self, properties: &Vec<Property>) {
        // TODO: [x] validate number of set blocks
        //       [x] validate number of get blocks
        //       [x] at least one setter/getter is required
        //       [x] pou has to be FUNCTION_BLOCK or PROGRAM
        //       [x] only VAR blocks are allowed by user
        for property in properties {
            let mut get_blocks = vec![];
            let mut set_blocks = vec![];
            if !property.kind_parent.is_stateful_pou() {
                self.diagnostics.push(
                    Diagnostic::new("Only FUNCTION_BLOCK or PROGRAM are allowed as parent for properties")
                        .with_location(property.name_parent_location.clone())
                        .with_error_code("E001"),
                );
            }
            for implementation in &property.implementations {
                // implementation.variable_block.variable_block_type
                for variable in &implementation.variables {
                    match variable.variable_block_type {
                        plc_ast::ast::VariableBlockType::Local => {}
                        _ => {
                            self.diagnostics.push(
                                Diagnostic::new("Only VAR blocks are allowed for properties")
                                    .with_secondary_location(variable.location.clone())
                                    .with_location(property.name_location.clone())
                                    .with_error_code("E001"),
                            );
                        }
                    }
                }
                if implementation.kind == PropertyKind::Get {}
                match implementation.kind {
                    PropertyKind::Get => {
                        get_blocks.push(implementation.location.clone());
                    }
                    PropertyKind::Set => {
                        set_blocks.push(implementation.location.clone());
                    }
                }
            }
            if set_blocks.len() + get_blocks.len() == 0 {
                // one block is required
                self.diagnostics.push(
                    Diagnostic::new("Property has no GET or SET block")
                        .with_location(property.name_location.clone())
                        .with_error_code("E001"),
                );
                continue;
            }
            // TODO: check why END_GET is part of error message in property_with_more_than_one_get_block()
            if get_blocks.len() > 1 {
                self.diagnostics.push(
                    Diagnostic::new("Property has more than one GET block")
                        .with_location(property.name_location.clone())
                        .with_secondary_locations(get_blocks)
                        .with_error_code("E001"),
                );
            }
            if set_blocks.len() > 1 {
                self.diagnostics.push(
                    Diagnostic::new("Property has more than one SET block")
                        .with_location(property.name_location.clone())
                        .with_secondary_locations(set_blocks)
                        .with_error_code("E001"),
                );
            }
        }
        // todo!()
    }
}
// -------

pub struct ParticipantDiagnostician {
    inner: RwLock<Diagnostician>,
}

unsafe impl Sync for ParticipantDiagnostician {}
unsafe impl Send for ParticipantDiagnostician {}

impl ParticipantDiagnostician {
    pub fn new() -> ParticipantDiagnostician {
        ParticipantDiagnostician { inner: RwLock::new(Diagnostician::default()) }
    }
}

impl Deref for ParticipantDiagnostician {
    type Target = RwLock<Diagnostician>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
