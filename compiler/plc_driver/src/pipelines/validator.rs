use plc::lowering::validator::ParticipantValidator;

use super::{participant::PipelineParticipant, ParsedProject};

impl PipelineParticipant for ParticipantValidator {
    fn pre_index(&mut self, project: &ParsedProject) {
        for unit in &project.units {
            for pou in &unit.units {
                self.validate_properties(pou);
            }
        }

        self.report_diagnostics();
    }
}
