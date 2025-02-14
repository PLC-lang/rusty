use plc::lowering::validator::ParticipantValidator;

use super::{participant::PipelineParticipantMut, ParsedProject};

impl PipelineParticipantMut for ParticipantValidator {
    fn pre_index(&mut self, project: ParsedProject) -> ParsedProject {
        let ParsedProject { units } = project;

        for unit in &units {
            self.validate_properties(&unit.properties);
        }

        self.report_diagnostics();

        ParsedProject { units }
    }
}
