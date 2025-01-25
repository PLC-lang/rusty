//! Module implementing the participant methods for the [`plc::lowering::property::PropertyLowerer`]

use plc::lowering::property::PropertyLowerer;

use super::{participant::PipelineParticipantMut, ParsedProject};

impl PipelineParticipantMut for PropertyLowerer {
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        let ParsedProject { mut units } = parsed_project;

        for unit in &mut units {
            self.lower_properties_to_methods(unit);
        }

        ParsedProject { units }
    }
}
