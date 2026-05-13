//!
//! Pipeline participants allow for additional steps to happen during the build.
//! Such steps can be read only using the `PipelineParticipant` such as Validators
//! or Read Write using the `PipelineParticipantMut` such as lowering operations
//!

use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use ast::provider::IdProvider;
use plc::{
    codegen::GeneratedModule,
    index::{Index, PouIndexEntry},
    lowering::{calls::AggregateTypeLowerer, polymorphism::PolymorphismLowerer},
    output::FormatOption,
    ConfigFormat, OnlineChange, Target,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_lowering::{
    array_lowering, control_statement::ControlStatementParticipant, inheritance::InheritanceLowerer,
    initializer::Initializer, loops::LoopDesugarer, reference_to_return::ReferenceToReturnParticipant,
    retain::RetainParticipant,
};
use project::{object::Object, project::LibraryInformation};
use source_code::SourceContainer;

use super::{AnnotatedProject, AnnotatedUnit, GeneratedProject, IndexedProject, ParsedProject};

/// A Build particitpant for different steps in the pipeline
/// Implementors can decide parse the Ast and project information
/// to do actions like validation or logging
pub trait PipelineParticipant: Sync + Send {
    /// Short label for this participant, used by the phase-timing
    /// instrumentation. Default returns the implementing type's name.
    fn name(&self) -> &'static str {
        super::timing::short_type_name(std::any::type_name::<Self>())
    }
    /// Implement this to access the project before it gets indexed
    /// This happens directly after parsing
    fn pre_index(&mut self, _parsed_project: &ParsedProject) {}
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
    fn pre_generate(&mut self, _annotated_project: &AnnotatedProject) -> Result<(), Diagnostic> {
        Ok(())
    }
    /// Implement this to get access to the module generation section of the codegen
    /// This is useful if generating multiple modules to hook into single module generation
    fn generate(&self, _generated_module: &GeneratedModule) -> Result<(), Diagnostic> {
        Ok(())
    }
    /// Implement this to access the project after it got generated
    /// This happens after codegen
    fn post_generate(&self) -> Result<(), Diagnostic> {
        Ok(())
    }
}

/// A Mutating Build particitpant for different steps in the pipeline
/// Implementors can decide to modify the AST, project and generated code,
/// for example for de-sugaring/lowering/pre-processing the AST
/// If a previous step is being modified, such as the AST or index,
/// the caller is responsible for calling the previous steps
pub trait PipelineParticipantMut {
    /// Short label for this participant, used by the phase-timing
    /// instrumentation. Default returns the implementing type's name.
    fn name(&self) -> &'static str {
        super::timing::short_type_name(std::any::type_name::<Self>())
    }
    /// Implement this to access the project before it gets indexed
    /// This happens directly after parsing
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        parsed_project
    }
    /// Implement this to access the project after it got indexed
    /// This happens directly after the index returns
    fn post_index(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        indexed_project
    }
    /// Implement this to access the project before it gets annotated
    /// This happens directly after the constants are evaluated
    fn pre_annotate(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        indexed_project
    }
    /// Implement this to access the project after it got annotated
    /// This happens directly after annotations
    fn post_annotate(&mut self, annotated_project: AnnotatedProject) -> AnnotatedProject {
        annotated_project
    }

    /// Returns any diagnostics accumulated during this participant's pipeline stages.
    /// The default implementation returns an empty vec.
    fn diagnostics(&mut self) -> Vec<Diagnostic> {
        Vec::new()
    }
}

pub struct CodegenParticipant<T: SourceContainer> {
    pub compile_options: crate::CompileOptions,
    pub link_options: crate::LinkOptions,
    pub target: Target,
    pub got_layout: Mutex<HashMap<String, u64>>,
    pub compile_dirs: HashMap<Target, PathBuf>,
    pub objects: Arc<RwLock<GeneratedProject>>,
    pub libraries: Vec<LibraryInformation<T>>,
}

impl<T: SourceContainer> CodegenParticipant<T> {
    /// Ensures the directores for the various targets have been created
    fn ensure_compile_dirs(&mut self) -> Result<(), Diagnostic> {
        let compile_directory = self.compile_options.build_location.clone().unwrap_or_else(|| {
            let tempdir = tempfile::tempdir().expect("Could not create tempdir");
            tempdir.keep()
        });
        if let Some(name) = self.target.try_get_name() {
            let dir = compile_directory.join(name);
            fs::create_dir_all(&dir)?;
            self.compile_dirs.insert(self.target.clone(), dir);
        } else {
            self.compile_dirs.insert(self.target.clone(), compile_directory);
        }
        Ok(())
    }
    pub fn read_got_layout(location: &str, format: ConfigFormat) -> Result<HashMap<String, u64>, Diagnostic> {
        let path = Path::new(location);
        if !path.is_file() {
            // Assume if the file doesn't exist that there is no existing GOT layout yet. write_got_layout will handle
            // creating our file when we want to.
            return Ok(HashMap::new());
        }

        let s = fs::read_to_string(location)
            .map_err(|_| Diagnostic::new("GOT layout could not be read from file"))?;
        match format {
            ConfigFormat::JSON => serde_json::from_str(&s)
                .map_err(|_| Diagnostic::new("Could not deserialize GOT layout from JSON")),
            ConfigFormat::TOML => toml::de::from_str(&s)
                .map_err(|_| Diagnostic::new("Could not deserialize GOT layout from TOML")),
        }
    }
}

impl<T: SourceContainer + Send> PipelineParticipant for CodegenParticipant<T> {
    fn pre_generate(&mut self, _annotated_project: &AnnotatedProject) -> Result<(), Diagnostic> {
        self.ensure_compile_dirs()?;

        let got_layout =
            if let OnlineChange::Enabled { file_name, format } = &self.compile_options.online_change {
                Self::read_got_layout(file_name, *format)?
            } else {
                HashMap::default()
            };
        self.got_layout = Mutex::new(got_layout);
        Ok(())
    }

    fn generate(&self, module: &GeneratedModule) -> Result<(), Diagnostic> {
        let current_dir = env::current_dir()?;
        let current_dir = self.compile_options.root.as_deref().unwrap_or(&current_dir);
        let unit_location = module.get_unit_location();
        let unit_location =
            if unit_location.exists() { fs::canonicalize(unit_location)? } else { unit_location.into() };
        let output_name = if unit_location.starts_with(current_dir) {
            unit_location.strip_prefix(current_dir).map_err(|it| {
                Diagnostic::new(format!("Could not strip prefix for {}", current_dir.to_string_lossy()))
                    .with_internal_error(it.into())
            })?
        } else if unit_location.has_root() {
            let root = unit_location.ancestors().last().expect("Should exist?");
            unit_location.strip_prefix(root).expect("The root directory should exist")
        } else {
            unit_location.as_path()
        };

        let output_name = match self.compile_options.output_format {
            FormatOption::IR => format!("{}.ll", output_name.to_string_lossy()),
            FormatOption::Bitcode => format!("{}.bc", output_name.to_string_lossy()),
            _ => format!("{}.o", output_name.to_string_lossy()),
        };

        let target = &self.target;
        let compile_directory = self.compile_dirs.get(target).expect("Required dir");
        let object = module
            .persist(
                Some(compile_directory),
                &output_name,
                self.compile_options.output_format,
                self.compile_options.relocation_preference,
                target,
                self.compile_options.optimization,
            )
            .map(Into::into)
            .map(|it: Object| it.with_target(target))?;
        self.objects.write().expect("Failed to aquire read write lock").objects.push(object);
        Ok(())
    }

    fn post_generate(&self) -> Result<(), Diagnostic> {
        let output_name = &self.compile_options.output;

        let _objects = self.objects.read().expect("Failed to aquire read lock for objects").link(
            &[], //Original project objects embedded in participant
            self.link_options.build_location.as_deref(),
            self.link_options.lib_location.as_deref(),
            output_name,
            self.link_options.clone(),
        )?;
        if let Some(lib_location) = &self.link_options.lib_location {
            for library in self.libraries.iter().filter(|it| it.should_copy()).map(|it| it.get_compiled_lib())
            {
                for obj in library.get_objects() {
                    let path = obj.get_path();
                    if let Some(name) = path.file_name() {
                        std::fs::copy(path, lib_location.join(name))?;
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct InitParticipant {
    id_provider: IdProvider,
    generate_externals: bool,
}

impl InitParticipant {
    pub fn new(id_provider: IdProvider, generate_externals: bool) -> Self {
        Self { id_provider, generate_externals }
    }
}

impl PipelineParticipantMut for InitParticipant {
    fn pre_annotate(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        // Create a new init lowerer
        let IndexedProject { project: ParsedProject { units }, index, .. } = indexed_project;
        let mut resulting_units = vec![];
        let index = Rc::new(index);
        for unit in units {
            let initializer = Initializer::new(self.id_provider.clone(), self.generate_externals);
            let unit = initializer.apply_initialization(unit, index.clone());
            resulting_units.push(unit);
        }
        // Append new units and constructor to the ast and re-index
        let project = ParsedProject { units: resulting_units };
        project.index(self.id_provider.clone())
    }
}

pub struct ArrayLowerer {
    id_provider: IdProvider,
}

impl ArrayLowerer {
    pub fn new(id_provider: IdProvider) -> Self {
        Self { id_provider }
    }

    /// Lowers literal-array assignments in a single compilation unit.
    /// Returns `true` if the unit was modified (a literal array was
    /// rewritten into indexed assignments and/or a counter loop). Used
    /// by per-unit adapters such as the `UnitLowerer` framework.
    pub fn lower_one_unit(
        &mut self,
        unit: &mut ast::ast::CompilationUnit,
        index: &plc::index::Index,
    ) -> bool {
        array_lowering::lower_literal_arrays(unit, index, &mut self.id_provider)
    }
}

/// `UnitLowerer` wrapper for [`ArrayLowerer`]. Walks each unit
/// independently — literal-array lowering is naturally per-unit (counter
/// variables are added to the same POU's `VAR_TEMP`). Registered as
/// `AutoLowerer::new(ArrayUnitLowerer::new(...), LoweringStage::PreAnnotate, ids)`.
pub struct ArrayUnitLowerer {
    inner: ArrayLowerer,
}

impl ArrayUnitLowerer {
    pub fn new(ids: ast::provider::IdProvider) -> Self {
        Self { inner: ArrayLowerer::new(ids) }
    }
}

impl super::unit_lowerer::UnitLowerer for ArrayUnitLowerer {
    fn name(&self) -> &'static str {
        "ArrayLowerer"
    }

    fn lower_unit(
        &mut self,
        unit: &mut ast::ast::CompilationUnit,
        ctx: &super::unit_lowerer::LoweringContext,
    ) -> super::unit_lowerer::UnitChange {
        let mutated = self.inner.lower_one_unit(unit, ctx.index);
        if mutated {
            super::unit_lowerer::UnitChange::mutated()
        } else {
            super::unit_lowerer::UnitChange::none()
        }
    }
}

impl PipelineParticipantMut for InheritanceLowerer {
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        let ParsedProject { mut units } = parsed_project;
        units.iter_mut().for_each(|unit| self.visit_unit(unit));
        ParsedProject { units }
    }

    fn post_annotate(&mut self, annotated_project: AnnotatedProject) -> AnnotatedProject {
        // Skip if the project declares no inheritance or interfaces — the
        // visit would be a no-op and the implicit re-annotate would only
        // recompute the same annotations.
        if !project_uses_inheritance(&annotated_project.index) {
            return annotated_project;
        }

        let AnnotatedProject { mut units, index, annotations, diagnostics } = annotated_project;
        self.annotations = Some(Box::new(annotations));
        self.index = Some(index);
        units.iter_mut().for_each(|unit| self.visit_unit(&mut unit.unit));
        let index = self.index.take().expect("Index should be present");
        // re-resolve
        let mut project = IndexedProject {
            project: ParsedProject {
                units: units.into_iter().map(|AnnotatedUnit { unit, .. }| unit).collect(),
            },
            index,
            _unresolvables: vec![],
        }
        .annotate(self.provider());
        project.diagnostics = diagnostics;
        project
    }
}

impl PipelineParticipantMut for AggregateTypeLowerer {
    fn post_annotate(&mut self, annotated_project: AnnotatedProject) -> AnnotatedProject {
        // Skip if no POU has an aggregate return type — nothing to do and
        // no invalidation needed.
        if !project_has_aggregate_returns(&annotated_project.index) {
            return annotated_project;
        }

        // Capture the reverse-dep graph before mutation; the closure asks
        // "who used the symbol before its signature changed?".
        let reverse_deps = annotated_project.reverse_dependencies();

        let AnnotatedProject { mut units, index, annotations, diagnostics } = annotated_project;
        self.index = Some(index);
        self.annotation = Some(annotations);

        // Walk each unit; bookkeeper records which were mutated and which
        // POU names had their signature rewritten (aggregate return →
        // VAR_IN_OUT). Constants enter the index because the new
        // VAR_IN_OUT parameter's type can be e.g. `STRING[K+1]`.
        let mut book = super::bookkeeping::LoweringBookkeeper::new();
        for (idx, annotated_unit) in units.iter_mut().enumerate() {
            if self.visit_unit_tracked(&mut annotated_unit.unit) {
                book.mark_unit(idx);
                for pou in &annotated_unit.unit.pous {
                    book.signature_changed(&pou.name);
                }
            }
        }
        if !book.is_empty() {
            book.mark_const_introduced();
        }

        // Reclaim the index and annotations from `self` (the visitor
        // borrowed them for the walk).
        let index = self.index.take().expect("index returned by visit");
        let annotations = self.annotation.take().expect("annotations returned by visit");

        let project = AnnotatedProject { units, index, annotations, diagnostics };
        book.apply_to_annotated(project, &reverse_deps, self.id_provider.clone())
    }
}

impl PipelineParticipantMut for PolymorphismLowerer {
    fn post_index(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        let IndexedProject { mut project, mut index, _unresolvables } = indexed_project;
        let mutated = self.table(&index, &mut project.units);
        if mutated.is_empty() {
            return IndexedProject { project, index, _unresolvables };
        }
        // Re-index only the units the table generator actually touched.
        // The pipeline previously re-indexed the whole project here.
        for unit_idx in &mutated {
            let unit_id = plc::index::UnitId::source(*unit_idx);
            index.reindex_unit(unit_id, &mut project.units[*unit_idx], self.ids.clone());
        }
        IndexedProject { project, index, _unresolvables }
    }

    fn post_annotate(&mut self, annotated_project: AnnotatedProject) -> AnnotatedProject {
        // Dispatch lowering only has work to do if the project has any
        // polymorphic constructs (classes, function blocks, methods, or
        // interfaces). The precheck is an exact predicate: if none of those
        // are in the index, the dispatch visitor wouldn't rewrite anything
        // and the implicit re-index + re-annotate would reproduce the
        // existing state. Threading a `changed` flag through the dispatch
        // visitors is more invasive; the index precheck gives the same
        // answer for cheaper.
        if !project_uses_polymorphism(&annotated_project.index) {
            return annotated_project;
        }

        let AnnotatedProject { units, index, annotations, diagnostics } = annotated_project;
        let mut units: Vec<_> = units.into_iter().map(|AnnotatedUnit { unit, .. }| unit).collect();

        let new_diagnostics = self.dispatch(index, annotations.annotation_map, &mut units);
        self.stash_diagnostics(new_diagnostics);
        let project = ParsedProject { units };

        // Dispatch lowering may inject new types (e.g. `__FATPOINTER` and itables for interface
        // dispatch) into the compilation units' `user_types`. Re-indexing from the units ensures
        // these types are present in the index for the subsequent re-annotation.
        let mut project = project.index(self.ids.clone()).annotate(self.ids.clone());
        project.diagnostics = diagnostics;
        project
    }

    fn diagnostics(&mut self) -> Vec<Diagnostic> {
        self.take_diagnostics()
    }
}

/// Shared handle to a [`PolymorphismLowerer`] used by both the table
/// pass (per-unit, via [`PolymorphismTableUnitLowerer`]) and the
/// dispatch pass (project-wide, via [`PolymorphismDispatchAdapter`]).
///
/// The two passes have to see the same `PolymorphismLowerer` state —
/// in particular the same `ids: IdProvider` and the same
/// `generate_external_constructors` flag — so the vtable type names
/// the table pass emits line up with what dispatch later rewrites
/// calls to dereference through. Constructing a fresh
/// `PolymorphismLowerer` for each pass diverges that state and breaks
/// `InitParticipant`'s constructor synthesis (it stops emitting
/// `__<container>___vtable__ctor(self.__vtable)` for FB/class
/// members).
pub type SharedPolymorphismLowerer = std::rc::Rc<std::cell::RefCell<PolymorphismLowerer>>;

pub fn shared_polymorphism_lowerer(
    ids: ast::provider::IdProvider,
    generate_external_constructors: bool,
) -> SharedPolymorphismLowerer {
    std::rc::Rc::new(std::cell::RefCell::new(PolymorphismLowerer::new(ids, generate_external_constructors)))
}

/// `UnitLowerer` wrapper for the polymorphism table pass. Holds a
/// shared handle to the same [`PolymorphismLowerer`] the dispatch
/// adapter uses. Registered at [`LoweringStage::PostIndex`].
pub struct PolymorphismTableUnitLowerer {
    inner: SharedPolymorphismLowerer,
}

impl PolymorphismTableUnitLowerer {
    pub fn from_shared(inner: SharedPolymorphismLowerer) -> Self {
        Self { inner }
    }
}

impl super::unit_lowerer::UnitLowerer for PolymorphismTableUnitLowerer {
    fn name(&self) -> &'static str {
        "PolymorphismLowerer (table)"
    }

    fn lower_unit(
        &mut self,
        unit: &mut ast::ast::CompilationUnit,
        ctx: &super::unit_lowerer::LoweringContext,
    ) -> super::unit_lowerer::UnitChange {
        let mutated = self.inner.borrow().table_one_unit(ctx.index, unit);
        if mutated {
            super::unit_lowerer::UnitChange::mutated()
        } else {
            super::unit_lowerer::UnitChange::none()
        }
    }
}

/// Project-wide `post_annotate` adapter that forwards to the shared
/// [`PolymorphismLowerer`]'s dispatch pass. Paired with
/// [`PolymorphismTableUnitLowerer`] via a common
/// [`SharedPolymorphismLowerer`] handle.
pub struct PolymorphismDispatchAdapter {
    inner: SharedPolymorphismLowerer,
}

impl PolymorphismDispatchAdapter {
    pub fn from_shared(inner: SharedPolymorphismLowerer) -> Self {
        Self { inner }
    }
}

impl PipelineParticipantMut for PolymorphismDispatchAdapter {
    fn name(&self) -> &'static str {
        "PolymorphismLowerer (dispatch)"
    }

    fn post_annotate(&mut self, annotated_project: AnnotatedProject) -> AnnotatedProject {
        if !project_uses_polymorphism(&annotated_project.index) {
            return annotated_project;
        }

        let AnnotatedProject { units, index, annotations, diagnostics } = annotated_project;
        let mut units: Vec<_> = units.into_iter().map(|AnnotatedUnit { unit, .. }| unit).collect();

        let mut inner = self.inner.borrow_mut();
        let new_diagnostics = inner.dispatch(index, annotations.annotation_map, &mut units);
        inner.stash_diagnostics(new_diagnostics);
        drop(inner);

        let project = ParsedProject { units };
        let mut project =
            project.index(self.inner.borrow().ids.clone()).annotate(self.inner.borrow().ids.clone());
        project.diagnostics = diagnostics;
        project
    }

    fn diagnostics(&mut self) -> Vec<Diagnostic> {
        self.inner.borrow_mut().take_diagnostics()
    }
}

/// `UnitLowerer` wrapper for [`RetainParticipant`]. The retain rewrite
/// is naturally per-unit (each unit's `VAR RETAIN` blocks are hoisted to
/// the same unit's global vars), so this is the first lowerer ported to
/// the new adapter framework. Registered as
/// `AutoLowerer::new(RetainUnitLowerer::new(...), LoweringStage::PostIndex, ids)`.
pub struct RetainUnitLowerer {
    inner: RetainParticipant,
}

impl RetainUnitLowerer {
    pub fn new(ids: ast::provider::IdProvider) -> Self {
        Self { inner: RetainParticipant::new(ids) }
    }
}

impl super::unit_lowerer::UnitLowerer for RetainUnitLowerer {
    fn name(&self) -> &'static str {
        "RetainParticipant"
    }

    fn lower_unit(
        &mut self,
        unit: &mut ast::ast::CompilationUnit,
        ctx: &super::unit_lowerer::LoweringContext,
    ) -> super::unit_lowerer::UnitChange {
        let mutated = self.inner.lower_one_unit(unit, ctx.index);
        if mutated {
            super::unit_lowerer::UnitChange::mutated()
        } else {
            super::unit_lowerer::UnitChange::none()
        }
    }
}

impl PipelineParticipantMut for ControlStatementParticipant {
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        let ParsedProject { mut units } = parsed_project;
        self.lower_control_statements(&mut units);

        ParsedProject { units }
    }
}

impl PipelineParticipantMut for LoopDesugarer {
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        let ParsedProject { mut units } = parsed_project;
        self.desugar(&mut units);

        ParsedProject { units }
    }
}

impl PipelineParticipantMut for ReferenceToReturnParticipant {
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        let ParsedProject { mut units } = parsed_project;
        self.lower_reference_to_return(&mut units);
        ParsedProject { units }
    }
}

// ─── Precheck helpers ──────────────────────────────────────────────────────
//
// Several lowering participants used to unconditionally re-run a whole-project
// index or annotate after their hook fired, even when the lowerer had nothing
// to do on this project. The helpers below answer "is there any work for me?"
// from the already-built index (or the units themselves) so the participant
// can skip both the work and the implicit re-pass when the answer is no.

/// True if the project contains any object-oriented constructs that the
/// [`PolymorphismLowerer`] would rewrite: interfaces, classes, or function
/// blocks. Inheritance-only constructs (super-class chains on functions) are
/// caught by `project_uses_inheritance` instead.
fn project_uses_polymorphism(index: &Index) -> bool {
    if !index.get_interfaces().is_empty() {
        return true;
    }
    index.get_pous().values().any(|p| {
        matches!(
            p,
            PouIndexEntry::Class { .. } | PouIndexEntry::FunctionBlock { .. } | PouIndexEntry::Method { .. }
        )
    })
}

/// True if any POU's return type is aggregate (array, struct, or string), in
/// which case [`AggregateTypeLowerer`] needs to rewrite that POU's signature.
fn project_has_aggregate_returns(index: &Index) -> bool {
    for pou in index.get_pous().values() {
        let return_type = match pou {
            PouIndexEntry::Function { return_type, .. } | PouIndexEntry::Method { return_type, .. } => {
                return_type.as_str()
            }
            _ => continue,
        };
        if return_type.is_empty() {
            continue;
        }
        if index.get_effective_type_or_void_by_name(return_type).is_aggregate_type() {
            return true;
        }
    }
    false
}

/// True if any POU declares a super-class or any interfaces, in which case
/// [`InheritanceLowerer`] needs to rewrite calls and walk inheritance chains.
fn project_uses_inheritance(index: &Index) -> bool {
    index.get_pous().values().any(|p| match p {
        PouIndexEntry::FunctionBlock { super_class, interfaces, .. }
        | PouIndexEntry::Class { super_class, interfaces, .. } => {
            super_class.is_some() || !interfaces.is_empty()
        }
        _ => false,
    })
}
