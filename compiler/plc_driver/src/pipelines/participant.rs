//!
//! Pipeline participants allow for additional steps to happen during the build.
//! Such steps can be read only using the `PipelineParticipant` such as Validators
//! or Read Write using the `PipelineParticipantMut` such as lowering operations
//!

use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, RwLock},
};

use ast::{
    ast::{
        Assignment, AstFactory, AstNode, AstStatement, CallStatement, CompilationUnit, Implementation,
        LinkageType, Pou, PouType, Property, PropertyKind, ReferenceAccess, ReferenceExpr, VariableBlock,
        VariableBlockType,
    },
    mut_visitor::AstVisitorMut,
    provider::IdProvider,
};
use plc::{
    codegen::GeneratedModule,
    index::{ArgumentType, VariableType},
    lowering::property::PropertyDesugar,
    output::FormatOption,
    resolver::{AnnotationMap, StatementAnnotation},
    ConfigFormat, OnlineChange, Target,
};
use plc_diagnostics::{
    diagnostician::{self, Diagnostician},
    diagnostics::Diagnostic,
};
use plc_index::GlobalContext;
use project::{object::Object, project::LibraryInformation};
use source_code::{source_location::SourceLocation, SourceContainer};

use super::{AnnotatedProject, GeneratedProject, IndexedProject, ParsedProject};

/// A Build particitpant for different steps in the pipeline
/// Implementors can decide parse the Ast and project information
/// to do actions like validation or logging
pub trait PipelineParticipant: Sync + Send {
    /// Implement this to access the project before it gets indexed
    /// This happens directly after parsing
    fn pre_index(&self, _parsed_project: &ParsedProject, diagnostician: &mut Diagnostician) {}
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
    fn pre_index(
        &mut self,
        parsed_project: ParsedProject,
        _diagnostician: &mut Diagnostician,
    ) -> ParsedProject {
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
    fn pre_index(
        &mut self,
        parsed_project: ParsedProject,
        diagnostician: &mut Diagnostician,
    ) -> ParsedProject {
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
    fn pre_index(
        &mut self,
        parsed_project: ParsedProject,
        diagnostician: &mut Diagnostician,
    ) -> ParsedProject {
        let ParsedProject { mut units, .. } = parsed_project;
        // desugar
        for unit in &mut units {
            self.visit_compilation_unit(unit);
        }

        PropertyDesugar::validate_units(&units);

        ParsedProject { units }
    }

    fn post_annotate(&mut self, annotated_project: AnnotatedProject) -> AnnotatedProject {
        let AnnotatedProject { mut units, index, annotations } = annotated_project;
        for unit in &mut units {
            for implementation in &mut unit.unit.implementations {
                // TODO: Find a way to not update statements when we're inside the parent POU where the property
                // has been defined
                if implementation.name == "fb.get_prop" || implementation.name == "fb.set_prop" {
                    continue;
                }

                for node in &mut implementation.statements.iter_mut().filter(|it| it.is_assignment()) {
                    let mut replace_me = &mut node.stmt;
                    let AstStatement::Assignment(Assignment { ref mut left, ref mut right, .. }) =
                        &mut replace_me
                    else {
                        unreachable!()
                    };

                    // dbg!(annotations.get(&left));
                    // dbg!(annotations.get(&right));

                    if annotations.get(&right).is_some_and(StatementAnnotation::is_property) {
                        insert_get_prefix("get_", right);

                        let mut call = AstFactory::create_call_statement(
                            right.as_ref().clone(),
                            None,
                            self.id_provider.next_id(),
                            SourceLocation::undefined(),
                        );

                        std::mem::swap(right.as_mut(), &mut call);
                    } else if annotations.get(&left).is_some_and(StatementAnnotation::is_property) {
                        insert_get_prefix("set_", left);
                        let mut call = AstFactory::create_call_statement(
                            left.as_ref().clone(),
                            Some(right.as_ref().clone()),
                            self.id_provider.next_id(),
                            SourceLocation::undefined(),
                        );

                        dbg!(&call);

                        std::mem::swap(node, &mut call);
                    }
                }
            }
        }

        let project = AnnotatedProject { units, index, annotations };
        // TODO: Re-annotate, copy from PR
        project.redo(self.id_provider.clone())
    }
}

fn insert_get_prefix(prefix: &str, node: &mut AstNode) {
    let AstStatement::ReferenceExpr(ReferenceExpr { access, .. }) = &mut node.stmt else { unreachable!() };
    let ReferenceAccess::Member(ref mut name) = access else { unreachable!() };
    let AstStatement::Identifier(name) = &mut name.stmt else { unreachable!() };

    name.insert_str(0, prefix);
}

impl PipelineParticipant for PropertyDesugar {
    fn pre_index(&self, parsed_project: &ParsedProject, diagnostician: &mut Diagnostician) {
        let ParsedProject { units } = parsed_project;

        for unit in units {
            for property in &unit.properties {
                dbg!(&property);
                if property.implementations.is_empty() {
                    let diagnostic = Diagnostic::new("test")
                        .with_location(property.name_location.clone())
                        .with_error_code("E001");

                    dbg!(&diagnostic);

                    diagnostician.handle(&[diagnostic]);
                }
            }
        }
    }
}

pub struct Validator2 {
    context: Arc<GlobalContext>,
    diagnostics: Vec<Diagnostic>,
}

impl Validator2 {
    pub fn new(context: Arc<GlobalContext>) -> Validator2 {
        Validator2 { context, diagnostics: Vec::new() }
    }
}

impl PipelineParticipantMut for Validator2 {}
