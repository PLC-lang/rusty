//!
//! Pipeline participants allow for additional steps to happen during the build.
//! Such steps can be read only using the `PipelineParticipant` such as Validators
//! or Read Write using the `PipelineParticipantMut` such as lowering operations
//!

use project::object::Object;

use super::{AnnotatedProject, GeneratedProject, IndexedProject, ParsedProject};

/// A Build particitpant for different steps in the pipeline
/// Implementors can decide parse the Ast and project information
/// to do actions like validation or logging
pub trait PipelineParticipant {
    /// Implement this to access the project before it gets indexed
    /// This happens directly after parsing
    fn pre_index(&self, _parsed_project: &ParsedProject) {}
    /// Implement this to access the project after it got indexed
    /// This happens directly after the index returns
    fn post_index(&self, _indexed_project: &IndexedProject) {}
    /// Implement this to access the project before it gets annotated
    /// This happens after indexing
    fn pre_annotate(&self, _indexed_project: &IndexedProject) {}
    /// Implement this to access the project after it got annotated
    /// This happens directly after annotations
    fn post_annotate(&self, _annotated_project: &AnnotatedProject) {}
    /// Implement this to access the project before it gets generated
    /// This happens after annotation
    fn pre_codegen(&self, _annotated_project: &AnnotatedProject) {}
    /// Implement this to access the project after it got generated
    /// This happens after codegen
    fn post_codegen(&self, _generated_project: &GeneratedProject) {}
    /// Implement this to access the project before it gets linked
    /// This happens after codegen
    fn pre_link(&self, _generated_project: &GeneratedProject) {}
    /// Implement this to access the genarated / linked object
    /// This happens after linking
    fn post_link(&self, _linked_object: &Object) {}

}

/// A Mutating Build particitpant for different steps in the pipeline
/// Implementors can decide to modify the AST, project and generated code,
/// for example for de-sugaring/lowering/pre-processing the AST
pub trait PipelineParticipantMut {
    /// Implement this to access the project before it gets indexed
    /// This happens directly after parsing
    fn pre_index(&self, _parsed_project: &mut ParsedProject) -> bool { false }
    /// Implement this to access the project after it got indexed
    /// This happens directly after the index returns
    fn post_index(&self, _indexed_project: &mut IndexedProject) -> bool { false }
    /// Implement this to access the project before it gets annotated
    /// This happens after indexing
    fn pre_annotate(&self, _indexed_project: &mut IndexedProject) -> bool { false }
    /// Implement this to access the project after it got annotated
    /// This happens directly after annotations
    fn post_annotate(&self, _annotated_project: &mut AnnotatedProject) -> bool { false }
    /// Implement this to access the project before it gets generated
    /// This happens after annotation
    fn pre_codegen(&self, _annotated_project: &mut AnnotatedProject)  {}
    /// Implement this to access the project after it got generated
    /// This happens after codegen
    fn post_codegen(&self, _generated_project: &mut GeneratedProject) {}
    /// Implement this to access the project before it gets linked
    /// This happens after codegen
    fn pre_link(&self, _generated_project: &mut GeneratedProject) {}

}

