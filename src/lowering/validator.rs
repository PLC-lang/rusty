use plc_ast::ast::{Property, PropertyKind, VariableBlockType};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_index::GlobalContext;

use crate::ErrorFormat;

#[derive(Default)]
pub struct ParticipantValidator {
    pub diagnostics: Vec<Diagnostic>,
    context: GlobalContext,
    error_fmt: ErrorFormat,
}

impl ParticipantValidator {
    pub fn new(context: &GlobalContext, error_fmt: ErrorFormat) -> ParticipantValidator {
        ParticipantValidator { diagnostics: Vec::new(), context: context.clone(), error_fmt }
    }

    pub fn validate_properties(&mut self, properties: &Vec<Property>) {
        for property in properties {
            let mut get_blocks = vec![];
            let mut set_blocks = vec![];

            if !property.parent_kind.is_stateful() {
                self.diagnostics.push(
                    Diagnostic::new(format!(
                        "Property `{name}` must be defined in a stateful POU type (PROGRAM, CLASS or FUNCTION_BLOCK)",
                        name = property.name
                    ))
                    .with_location(property.parent_name_location.clone())
                    .with_error_code("E115"),
                );
            }

            for implementation in &property.implementations {
                for variable in &implementation.variable_blocks {
                    match variable.variable_block_type {
                        VariableBlockType::Local => {}
                        _ => {
                            self.diagnostics.push(
                                Diagnostic::new("Properties only allow variable blocks of type VAR")
                                    .with_secondary_location(variable.location.clone())
                                    .with_location(property.name_location.clone())
                                    .with_error_code("E116"),
                            );
                        }
                    }
                }

                match implementation.kind {
                    PropertyKind::Get => get_blocks.push(implementation.location.clone()),
                    PropertyKind::Set => set_blocks.push(implementation.location.clone()),
                }
            }

            if set_blocks.len() + get_blocks.len() == 0 {
                // one block is required
                self.diagnostics.push(
                    Diagnostic::new("Property has no GET or SET block")
                        .with_location(property.name_location.clone())
                        .with_error_code("E117"),
                );
                continue;
            }

            if get_blocks.len() > 1 {
                self.diagnostics.push(
                    Diagnostic::new("Property has more than one GET block")
                        .with_location(property.name_location.clone())
                        .with_secondary_locations(get_blocks)
                        .with_error_code("E117"),
                );
            }

            if set_blocks.len() > 1 {
                self.diagnostics.push(
                    Diagnostic::new("Property has more than one SET block")
                        .with_location(property.name_location.clone())
                        .with_secondary_locations(set_blocks)
                        .with_error_code("E117"),
                );
            }
        }
    }

    pub fn report_diagnostics(&mut self) {
        self.context.with_error_fmt(self.error_fmt.into());

        for diagnostic in &self.diagnostics {
            self.context.handle(diagnostic);
        }
    }
}
