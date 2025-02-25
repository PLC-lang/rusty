use plc::lowering::validator::ParticipantValidator;

use super::{participant::PipelineParticipant, ParsedProject};

impl PipelineParticipant for ParticipantValidator {
    fn pre_index(&mut self, project: &ParsedProject) {
        for unit in &project.units {
            self.validate_properties(&unit.properties);
        }

        self.report_diagnostics();
    }
}
