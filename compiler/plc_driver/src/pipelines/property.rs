//! Module implementing the participant methods for the [`plc::lowering::property::PropertyLowerer`]

use plc::lowering::property::PropertyLowerer;

use super::{participant::PipelineParticipantMut, AnnotatedProject, AnnotatedUnit, ParsedProject};

impl PipelineParticipantMut for PropertyLowerer {
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        let ParsedProject { mut units } = parsed_project;

        for unit in &mut units {
            self.lower_properties_to_pous(unit);
        }

        ParsedProject { units }
    }

    fn post_annotate(&mut self, project: AnnotatedProject) -> AnnotatedProject {
        let AnnotatedProject { mut units, index, annotations } = project;
        self.annotations = Some(annotations);

        for AnnotatedUnit { unit, .. } in &mut units.iter_mut() {
            self.lower_references_to_calls(unit);
        }

        let project = AnnotatedProject { units, index, annotations: self.annotations.take().unwrap() };
        project.re_annotate(self.id_provider.clone())
    }
}
