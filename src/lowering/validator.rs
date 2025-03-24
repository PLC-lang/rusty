use plc_diagnostics::diagnostics::Diagnostic;
use plc_index::GlobalContext;

use crate::ErrorFormat;

#[derive(Default)]
pub struct ParticipantValidator {
    pub diagnostics: Vec<Diagnostic>,
    context: GlobalContext,
    error_fmt: ErrorFormat,
}

// TODO: Remove this module; generally revert the POC?
impl ParticipantValidator {
    pub fn new(context: &GlobalContext, error_fmt: ErrorFormat) -> ParticipantValidator {
        ParticipantValidator { diagnostics: Vec::new(), context: context.clone(), error_fmt }
    }

    pub fn report_diagnostics(&mut self) {
        self.context.with_error_fmt(self.error_fmt.into());

        for diagnostic in &self.diagnostics {
            self.context.handle(diagnostic);
        }
    }
}
