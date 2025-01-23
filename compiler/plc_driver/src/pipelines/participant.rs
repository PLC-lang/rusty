//!
//! Pipeline participants allow for additional steps to happen during the build.
//! Such steps can be read only using the `PipelineParticipant` such as Validators
//! or Read Write using the `PipelineParticipantMut` such as lowering operations
//!

use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, RwLock},
};

use ast::{
    ast::{AstFactory, AstNode, AstStatement, PouType, ReferenceAccess, ReferenceExpr},
    mut_visitor::AstVisitorMut,
    provider::IdProvider,
};
use plc::{
    codegen::GeneratedModule,
    lowering::property::PropertyDesugar,
    output::FormatOption,
    resolver::{AnnotationMap, AstAnnotations},
    ConfigFormat, OnlineChange, Target,
};
use plc_diagnostics::diagnostics::Diagnostic;
use project::{object::Object, project::LibraryInformation};
use source_code::SourceContainer;

use super::{AnnotatedProject, GeneratedProject, IndexedProject, ParsedProject};

/// A Build particitpant for different steps in the pipeline
/// Implementors can decide parse the Ast and project information
/// to do actions like validation or logging
pub trait PipelineParticipant: Sync + Send {
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
    /// Implement this to access the project before it gets indexed
    /// This happens directly after parsing
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        parsed_project
    }

    fn pre_index_validation(&mut self, _project: &ParsedProject) -> Vec<Diagnostic> {
        Vec::new()
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
            tempdir.into_path()
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

pub struct LoweringParticipant;

impl PipelineParticipantMut for LoweringParticipant {
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        parsed_project
    }

    fn post_index(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        indexed_project
    }

    fn post_annotate(&mut self, annotated_project: AnnotatedProject) -> AnnotatedProject {
        annotated_project
    }
}

pub struct InitParticipant {
    symbol_name: String,
    id_provider: IdProvider,
}

impl InitParticipant {
    pub fn new(symbol_name: &str, id_provider: IdProvider) -> Self {
        Self { symbol_name: symbol_name.into(), id_provider }
    }
}

impl PipelineParticipantMut for InitParticipant {
    fn pre_annotate(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        indexed_project.extend_with_init_units(&self.symbol_name, self.id_provider.clone())
    }
}

impl PipelineParticipantMut for PropertyDesugar {
    fn pre_index(&mut self, parsed_project: ParsedProject) -> ParsedProject {
        let ParsedProject { mut units, .. } = parsed_project;
        PropertyDesugar::validate_units(&units);

        // desugar
        for unit in &mut units {
            self.visit_compilation_unit(unit);
        }

        ParsedProject { units }
    }

    fn post_annotate(&mut self, annotated_project: AnnotatedProject) -> AnnotatedProject {
        let AnnotatedProject { mut units, index, annotations } = annotated_project;

        for unit in units.iter_mut().map(|annotated| &mut annotated.unit) {
            for implementation in unit.implementations.iter_mut() {
                // for implementation in &mut unit.implementations {
                for node in implementation.statements.iter_mut() {
                    try_inplace_setter(&mut self.id_provider, &annotations, node, &implementation.pou_type);
                    try_inplace_getter(&mut self.id_provider, &annotations, node, &implementation.pou_type);
                }
            }
        }

        let project = AnnotatedProject { units, index, annotations };
        // TODO: Re-annotate, copy from PR
        project.redo(self.id_provider.clone())
    }
}

fn try_inplace_setter(
    ids: &mut IdProvider,
    annotations: &AstAnnotations,
    node: &mut AstNode,
    impl_type: &PouType,
) {
    let AstStatement::Assignment(inner) = &mut node.stmt else { return };
    let Some(annotation) = annotations.get(&inner.left) else { return };

    if !annotation.is_property() {
        return;
    }

    // If we're inside a POU where the property has been defined, then do not create getter and setter calls
    // because we have direct access to the underlying property variable
    if let PouType::Method { parent } = impl_type {
        if annotation.get_qualified_name().is_some_and(|name| &name[0..parent.len()] == parent) {
            // TODO: This is an assumption, but basically since we're creating a an internal variable named after the
            // property, any implementation with the same POU can access that variable directly without a getter or
            // setter call. Is this OK? Or should the property be only directly accessible within the actual property
            // block. Specifically this is currently ok:
            // ```
            // FUNCTION_BLOCK FB
            // ...
            // PROPERTY foo : DINT;
            // END_PROPERTY
            // ...
            // foo := 5; // Should this be illegal since the property foo has been accessed outside of a get/set block?
            // ...
            // END_FUNCTION_BLOCK
            // ```
            return;
        }
    }

    update_name("set_", &mut inner.left);

    let call = AstFactory::create_call_statement(
        inner.left.as_ref().clone(),
        Some(inner.right.as_ref().clone()),
        ids.next_id(),
        node.location.clone(),
    );

    let _ = std::mem::replace(node, call);
}

fn try_inplace_getter(
    ids: &mut IdProvider,
    annotations: &AstAnnotations,
    node: &mut AstNode,
    impl_type: &PouType,
) {
    let AstStatement::Assignment(inner) = &mut node.stmt else { return };
    let Some(annotation) = annotations.get(&inner.right) else { return };

    if !annotation.is_property() {
        return;
    }

    // TODO: We have to make sure the property variable isn't directly accesible other than the PROPERTY block
    // If we're inside a POU where the property has been defined, then do not create getter and setter calls
    // because we have direct access to the underlying property variable
    if let PouType::Method { parent } = impl_type {
        if annotation.get_qualified_name().is_some_and(|name| &name[0..parent.len()] == parent) {
            return;
        }
    }

    update_name("get_", &mut inner.right);

    let call = AstFactory::create_call_statement(
        inner.right.as_ref().clone(),
        None,
        ids.next_id(),
        inner.right.location.clone(),
    );

    let _ = std::mem::replace(node, call);
}

fn update_name(prefix: &'static str, node: &mut AstNode) {
    match &mut node.stmt {
        AstStatement::Identifier(ref mut name) => name.insert_str(0, prefix),
        AstStatement::ReferenceExpr(ReferenceExpr { ref mut access, .. }) => match access {
            ReferenceAccess::Member(ref mut node) => update_name(prefix, node),
            _ => todo!(),
        },

        _ => (),
    };
}
