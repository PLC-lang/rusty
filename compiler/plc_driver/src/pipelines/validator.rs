use plc::lowering::validator::ParticipantValidator;
use plc_diagnostics::diagnostician::Diagnostician;

use super::{participant::PipelineParticipantMut, ParsedProject};

impl PipelineParticipantMut for ParticipantValidator {
    fn pre_index(&mut self, project: ParsedProject) -> ParsedProject {
        let ParsedProject { units } = project;

        for unit in &units {
            self.validate_properties(&unit.properties);
        }

        let mut diagnostician = Diagnostician::clang_format_diagnostician();
        diagnostician.handle(&self.diagnostics);

        ParsedProject { units }
    }
}
