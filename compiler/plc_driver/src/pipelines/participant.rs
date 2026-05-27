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
    typesystem::DataTypeInformation,
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

use super::timing::PhaseTimer;
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

        let _objects = {
            let _t = PhaseTimer::new("link");
            self.objects.read().expect("Failed to aquire read lock for objects").link(
                &[], //Original project objects embedded in participant
                self.link_options.build_location.as_deref(),
                self.link_options.lib_location.as_deref(),
                output_name,
                self.link_options.clone(),
            )?
        };
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
}

impl PipelineParticipantMut for ArrayLowerer {
    fn pre_annotate(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        let IndexedProject { project: ParsedProject { mut units }, index, .. } = indexed_project;
        for unit in &mut units {
            array_lowering::lower_literal_arrays(unit, &index, &mut self.id_provider);
        }
        // Re-index since we modified the AST (new statements, possible new alloca variables)
        let project = ParsedProject { units };
        project.index(self.id_provider.clone())
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
        // Skip if no POU has an aggregate return type — the visit would walk
        // every unit and rewrite nothing, and the implicit re-index +
        // re-annotate would reproduce the existing state.
        if !project_has_aggregate_returns(&annotated_project.index) {
            return annotated_project;
        }

        let AnnotatedProject { units, index, annotations, diagnostics } = annotated_project;
        self.index = Some(index);
        self.annotation = Some(Box::new(annotations));

        let units = units
            .into_iter()
            .map(|AnnotatedUnit { mut unit, .. }| {
                self.visit_unit(&mut unit);
                unit
            })
            .collect();

        // Re-index from modified units so the index reflects POU signature
        // changes (e.g. aggregate returns converted to VAR_IN_OUT parameters).
        let project = ParsedProject { units };
        let mut project = project.index(self.id_provider.clone()).annotate(self.id_provider.clone());
        project.diagnostics = diagnostics;
        project
    }
}

impl PipelineParticipantMut for PolymorphismLowerer {
    fn post_index(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        // The table pass emits a `__vtable` type and instance member for
        // every FB / class — even methodless ones. The slot is part of the
        // FB ABI: a downstream library consumer that extends one of these
        // FBs must see a layout-compatible base. Skip only when the
        // project has no FBs, classes, methods, or interfaces at all.
        if !project_needs_vtables(&indexed_project.index) {
            return indexed_project;
        }

        let IndexedProject { mut project, index, .. } = indexed_project;

        self.table(&index, &mut project.units);

        project.index(self.ids.clone())
    }

    fn post_annotate(&mut self, annotated_project: AnnotatedProject) -> AnnotatedProject {
        // Dispatch lowering rewrites call sites that need vtable
        // indirection: method calls / body invocations through
        // `POINTER TO <FB>`, calls through interface variables, and
        // `SUPER^`. None of those can exist without methods or interfaces
        // declared somewhere in the project — pre-OOP libraries (FBs
        // with no methods, no `EXTENDS`, no interfaces) have nothing to
        // rewrite even when their FBs ship with vtable slots for
        // downstream extenders.
        if !project_uses_polymorphic_dispatch(&annotated_project.index) {
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

impl PipelineParticipantMut for RetainParticipant {
    fn post_index(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        let IndexedProject { mut project, index, .. } = indexed_project;
        self.lower_retain(&mut project.units, index);

        // Re-index
        project.index(self.ids.clone())
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
// from the already-built index so the participant can skip both the walk and
// the implicit re-pass when the answer is no. Each helper is an exact
// predicate: if it returns `false`, the lowerer would produce a project
// identical to its input.

/// True if the project has any FB, class, method, or interface — i.e. any
/// type that needs a vtable slot emitted. The vtable layout is part of the
/// FB ABI: even a methodless FB ships with a `__vtable` member so that a
/// downstream library consumer extending that FB sees a layout-compatible
/// base. Used by `PolymorphismLowerer::post_index` (table pass).
fn project_needs_vtables(index: &Index) -> bool {
    if index.get_interfaces().keys().next().is_some() {
        return true;
    }
    index.get_pous().values().any(|p| {
        matches!(
            p,
            PouIndexEntry::Class { .. } | PouIndexEntry::FunctionBlock { .. } | PouIndexEntry::Method { .. }
        )
    })
}

/// True if the project has any construct whose call sites the dispatch pass
/// might rewrite into vtable-indirected form:
///
/// - methods or interfaces (method-call dispatch through the vtable);
/// - any FB or class with `EXTENDS` (body-call dispatch and `SUPER^`);
/// - any `POINTER TO <FB-or-Class>` type. A pre-OOP library may declare a
///   pointer-to-FB member and call it via `ptr^()` with no `EXTENDS` or
///   methods in its own compilation. The lib has no way to know whether
///   a downstream consumer will retarget that pointer to a derived
///   instance, so the call site must be vtable-indirected — otherwise
///   the library binary bakes in a static call to the base body and the
///   derived body never runs. `REFERENCE TO X` is encoded as a
///   `Pointer { auto_deref: Some(_) }` in the type system, so it is
///   covered by the same check.
///
/// Used by `PolymorphismLowerer::post_annotate`.
fn project_uses_polymorphic_dispatch(index: &Index) -> bool {
    if index.get_interfaces().keys().next().is_some() {
        return true;
    }
    let pou_match = index.get_pous().values().any(|p| match p {
        PouIndexEntry::Method { .. } => true,
        PouIndexEntry::FunctionBlock { super_class, .. } | PouIndexEntry::Class { super_class, .. } => {
            super_class.is_some()
        }
        _ => false,
    });
    if pou_match {
        return true;
    }
    let is_pou_type = |type_name: &str| {
        index
            .find_effective_type_by_name(type_name)
            .map(|t| {
                let info = t.get_type_information();
                info.is_function_block() || info.is_class()
            })
            .unwrap_or(false)
    };
    index.get_types().values().any(|dt| match &dt.information {
        DataTypeInformation::Pointer { inner_type_name, is_function, .. } => {
            // Exclude two kinds of compiler-synthesized pointers:
            //  * function pointers (`is_function: true`) — vtable body
            //    slots, not user-declared pointers-to-FB;
            //  * internal `__auto_pointer_to_X` types emitted alongside
            //    every FB / class.
            // Both of these exist in oscat-like libraries that have no
            // user-declared `POINTER TO FB` at all; counting them would
            // cause the dispatch pass to fire unnecessarily.
            !is_function && !dt.is_internal() && is_pou_type(inner_type_name)
        }
        _ => false,
    })
}

/// True if any POU's return type is aggregate (array, struct, or string), in
/// which case `AggregateTypeLowerer` needs to rewrite that POU's signature.
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
/// `InheritanceLowerer` needs to rewrite calls and walk inheritance chains.
fn project_uses_inheritance(index: &Index) -> bool {
    index.get_pous().values().any(|p| match p {
        PouIndexEntry::FunctionBlock { super_class, interfaces, .. }
        | PouIndexEntry::Class { super_class, interfaces, .. } => {
            super_class.is_some() || !interfaces.is_empty()
        }
        _ => false,
    })
}
