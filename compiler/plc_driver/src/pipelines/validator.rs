use plc::lowering::validator::ParticipantValidator;

use super::{participant::PipelineParticipant, ParsedProject};

impl PipelineParticipant for ParticipantValidator {
    fn pre_index(&mut self, _: &ParsedProject) {
        // TODO: Remove this file
    }
}
