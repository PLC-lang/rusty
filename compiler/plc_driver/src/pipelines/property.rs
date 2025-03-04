//! Module implementing the participant methods for the [`plc::lowering::property::PropertyLowerer`]

use plc::lowering::property::PropertyLowerer;

use super::{
    participant::PipelineParticipantMut, AnnotatedProject, AnnotatedUnit, IndexedProject, ParsedProject,
};

impl PipelineParticipantMut for PropertyLowerer {
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        let ParsedProject { mut units } = parsed_project;

        for unit in &mut units {
            self.lower_properties_to_pous(unit);
        }

        ParsedProject { units }
    }

    fn post_index(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        let IndexedProject { project, index, .. } = indexed_project;
        self.index = Some(index);

        let mut units = project.units;
        units.iter_mut().for_each(|unit| {
            self.dedup_inherited_backing_fields(unit);
        });

        units.iter_mut().for_each(|unit| {
            self.dedup_redeclared_prop_methods(unit);
        });

        ParsedProject { units }.index(self.id_provider.clone())
    }

    fn post_annotate(&mut self, project: AnnotatedProject) -> AnnotatedProject {
        let AnnotatedProject { mut units, index, annotations } = project;
        self.annotations = Some(annotations);

        for AnnotatedUnit { unit, .. } in &mut units.iter_mut() {
            self.lower_references_to_calls(unit);
        }

        let project = IndexedProject {
            project: ParsedProject { units: units.into_iter().map(|annotated| annotated.unit).collect() },
            index,
            unresolvables: vec![],
        };

        project.annotate(self.id_provider.clone())
    }
}
