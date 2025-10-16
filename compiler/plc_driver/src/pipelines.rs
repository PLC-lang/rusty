use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::{
    cli::{self, CompileParameters, ConfigOption, GenerateLanguage, GenerateOption, SubCommands}, get_project, CompileOptions, GenerateHeaderOptions, LinkOptions, LinkerScript
};
use ast::{
    ast::{pre_process, CompilationUnit, LinkageType},
    provider::IdProvider,
};

use itertools::Itertools;
use log::debug;
use participant::{PipelineParticipant, PipelineParticipantMut};
use plc::{
    codegen::{CodegenContext, GeneratedModule},
    index::{indexer, FxIndexSet, Index},
    linker::LinkerType,
    lowering::{
        calls::AggregateTypeLowerer, polymorphism::PolymorphicCallLowerer, property::PropertyLowerer,
        vtable::VirtualTableGenerator, InitVisitor,
    },
    output::{FormatOption},
    parser::parse_file,
    resolver::{
        const_evaluator::UnresolvableConstant, AnnotationMapImpl, AstAnnotations, Dependency, StringLiterals,
        TypeAnnotator,
    },
    validation::Validator,
    ConfigFormat, ErrorFormat, OnlineChange, Target, Threads,
};
use plc_diagnostics::{
    diagnostician::Diagnostician,
    diagnostics::{Diagnostic, Severity},
};
use plc_index::GlobalContext;
use plc_lowering::inheritance::InheritanceLowerer;
use project::{
    object::Object,
    project::{LibraryInformation, Project},
};
use rayon::prelude::*;
use source_code::{source_location::SourceLocation, SourceContainer};

use serde_json;
use tempfile::NamedTempFile;
use toml;

pub mod participant;
pub mod property;

pub struct BuildPipeline<T: SourceContainer> {
    pub context: GlobalContext,
    pub project: Project<T>,
    pub diagnostician: Diagnostician,
    pub compile_parameters: Option<CompileParameters>,
    pub linker: LinkerType,
    pub mutable_participants: Vec<Box<dyn PipelineParticipantMut>>,
    pub participants: Vec<Box<dyn PipelineParticipant>>,
    pub module_name: Option<String>,
}

pub trait Pipeline {
    fn run(&mut self) -> Result<(), Diagnostic>;
    fn parse(&mut self) -> Result<ParsedProject, Diagnostic>;
    fn index(&mut self, project: ParsedProject) -> Result<IndexedProject, Diagnostic>;
    fn annotate(&mut self, project: IndexedProject) -> Result<AnnotatedProject, Diagnostic>;
    fn generate(&mut self, context: &CodegenContext, project: AnnotatedProject) -> Result<(), Diagnostic>;
    fn generate_headers(&mut self, project: AnnotatedProject) -> Result<(), Diagnostic>;
}

impl TryFrom<CompileParameters> for BuildPipeline<PathBuf> {
    type Error = anyhow::Error;

    fn try_from(compile_parameters: CompileParameters) -> Result<Self, Self::Error> {
        //Create the project that will be compiled
        let project = get_project(&compile_parameters)?;
        let location = project.get_location().map(|it| it.to_path_buf());
        if let Some(location) = &location {
            log::debug!("PROJECT_ROOT={}", location.to_string_lossy());
            env::set_var("PROJECT_ROOT", location);
        }
        let build_location = compile_parameters.get_build_location();
        if let Some(location) = &build_location {
            log::debug!("BUILD_LOCATION={}", location.to_string_lossy());
            env::set_var("BUILD_LOCATION", location);
        }
        let lib_location = compile_parameters.get_lib_location();
        if let Some(location) = &lib_location {
            log::debug!("LIB_LOCATION={}", location.to_string_lossy());
            env::set_var("LIB_LOCATION", location);
        }
        //Create diagnostics registry
        //Create a diagnostican with the specified registry
        //Use diagnostican
        let diagnostician = match compile_parameters.error_format {
            ErrorFormat::Rich => Diagnostician::default(),
            ErrorFormat::Clang => Diagnostician::clang_format_diagnostician(),
            ErrorFormat::None => Diagnostician::null_diagnostician(),
        };
        let diagnostician = if let Some(configuration) = compile_parameters.get_error_configuration()? {
            diagnostician.with_configuration(configuration)
        } else {
            diagnostician
        };

        // TODO: This can be improved quite a bit, e.g. `GlobalContext::new(project);`, to do that see the
        //       commented `project` method in the GlobalContext implementation block
        let context = GlobalContext::new()
            .with_source(project.get_sources(), compile_parameters.encoding)?
            .with_source(project.get_includes(), compile_parameters.encoding)?
            .with_source(
                project
                    .get_libraries()
                    .iter()
                    .flat_map(LibraryInformation::get_includes)
                    .collect::<Vec<_>>()
                    .as_slice(),
                None,
            )?;

        let linker = compile_parameters.linker.as_deref().into();
        Ok(BuildPipeline {
            context,
            project,
            diagnostician,
            compile_parameters: Some(compile_parameters),
            linker,
            mutable_participants: vec![],
            participants: vec![],
            module_name: None,
        })
    }
}

impl BuildPipeline<PathBuf> {
    pub fn new<T>(args: &[T]) -> anyhow::Result<Self>
    where
        T: AsRef<str> + AsRef<OsStr> + std::fmt::Debug,
    {
        let compile_parameters = CompileParameters::parse(args)?;
        BuildPipeline::try_from(compile_parameters)
    }
}

impl<T: SourceContainer> BuildPipeline<T> {
    pub fn register_mut_participant(&mut self, participant: Box<dyn PipelineParticipantMut>) {
        self.mutable_participants.push(participant)
    }

    pub fn register_participant(&mut self, participant: Box<dyn PipelineParticipant>) {
        self.participants.push(participant)
    }
    pub fn get_compile_options(&self) -> Option<CompileOptions> {
        self.compile_parameters.as_ref().map(|params| {
            let location = &self.project.get_location().map(|it| it.to_path_buf());
            let output_format = params.output_format().unwrap_or_else(|| self.project.get_output_format());
            CompileOptions {
                root: location.to_owned(),
                build_location: params.get_build_location(),
                output: self.project.get_output_name(),
                output_format,
                optimization: params.optimization,
                error_format: params.error_format,
                debug_level: params.debug_level(),
                single_module: params.single_module,
                online_change: if params.online_change {
                    OnlineChange::Enabled {
                        file_name: params.got_layout_file.clone(),
                        format: params.got_layout_format(),
                    }
                } else {
                    OnlineChange::Disabled
                },
            }
        })
    }

    pub fn get_link_options(&self) -> Option<LinkOptions> {
        self.compile_parameters.as_ref().map(|params| {
            let output_format = params.output_format().unwrap_or_else(|| self.project.get_output_format());
            let libraries = self
                .project
                .get_libraries()
                .iter()
                .map(LibraryInformation::get_link_name)
                .map(str::to_string)
                .collect();
            let mut library_paths: Vec<PathBuf> = self
                .project
                .get_libraries()
                .iter()
                .filter_map(LibraryInformation::get_path)
                .map(Path::to_path_buf)
                .collect();

            library_paths.extend_from_slice(self.project.get_library_paths());
            //Get the specified linker script or load the default linker script in a temp file
            let linker_script = if params.no_linker_script {
                LinkerScript::None
            } else {
                params.linker_script.clone().map(LinkerScript::Path).unwrap_or_default()
            };

            LinkOptions {
                libraries,
                library_paths,
                format: output_format,
                linker: self.linker.clone(),
                lib_location: params.get_lib_location(),
                build_location: params.get_build_location(),
                linker_script,
                module_name: self.get_module_name(),
            }
        })
    }

    pub fn get_generate_header_options(&self) -> Option<GenerateHeaderOptions> {
        self.compile_parameters.as_ref().map(|params| {
            let location = &self.project.get_location().map(|it| it.to_path_buf());
            let project_name = self.project.get_name();
            match params.commands.as_ref().unwrap() {
                SubCommands::Generate { option, .. } => {
                    match option {
                        GenerateOption::Headers { include_stubs, language, output, prefix, .. } => {
                            GenerateHeaderOptions {
                                include_stubs: *include_stubs,
                                language: *language,
                                output_path: if output.is_some() { PathBuf::from(output.clone().unwrap()) } else { location.clone().unwrap_or(PathBuf::from(String::new())) },
                                prefix: prefix.clone().unwrap_or_else(|| project_name.to_string()),
                            }
                        }
                    }
                },
                _ => GenerateHeaderOptions::default(),
            }
        })
    }

    fn print_config_options(&self, option: ConfigOption) -> Result<(), Diagnostic> {
        match option {
            cli::ConfigOption::Schema => {
                println!("{}", self.project.get_validation_schema().as_ref())
            }
            cli::ConfigOption::Diagnostics => {
                println!("{}", self.diagnostician.get_diagnostic_configuration())
            }
        };

        Ok(())
    }

    fn initialize_thread_pool(&self) {
        //Set the global thread count
        let thread_pool = rayon::ThreadPoolBuilder::new();
        let global_pool = if let Some(CompileParameters { threads: Some(Threads::Fix(threads)), .. }) =
            self.compile_parameters
        {
            log::info!("Using {threads} parallel threads");
            thread_pool.num_threads(threads)
        } else {
            thread_pool
        }
        .build_global();

        if let Err(err) = global_pool {
            // Ignore the error here as the global threadpool might have been initialized
            log::info!("{err}")
        }
    }
    /// Register all default participants (excluding codegen/linking)
    pub fn register_default_participants(&mut self) {
        use participant::InitParticipant;

        // XXX: should we use a static array of participants?
        let mut_participants: Vec<Box<dyn PipelineParticipantMut>> = vec![
            Box::new(VirtualTableGenerator::new(self.context.provider())),
            Box::new(PolymorphicCallLowerer::new(self.context.provider())),
            Box::new(PropertyLowerer::new(self.context.provider())),
            Box::new(InitParticipant::new(self.project.get_init_symbol_name(), self.context.provider())),
            Box::new(AggregateTypeLowerer::new(self.context.provider())),
            Box::new(InheritanceLowerer::new(self.context.provider())),
        ];

        for participant in mut_participants {
            self.register_mut_participant(participant)
        }
    }

    /// Returns an internal module name if specified
    /// Useful for tests
    fn get_module_name(&self) -> Option<String> {
        self.module_name.clone()
    }
}

impl<T: SourceContainer> Pipeline for BuildPipeline<T> {
    fn run(&mut self) -> anyhow::Result<(), Diagnostic> {
        if let Some((options, _format)) =
            self.compile_parameters.as_ref().and_then(CompileParameters::get_config_options)
        {
            return self.print_config_options(options);
        }
        if let Some(CompileParameters { build_info: true, .. }) = self.compile_parameters {
            println!("{}", option_env!("RUSTY_BUILD_INFO").unwrap_or("version information unavailable"));
            return Ok(());
        }

        if let Some(CompileParameters { commands: Some(SubCommands::Explain { error }), .. }) =
            &self.compile_parameters
        {
            //Explain the given error
            println!("{}", self.diagnostician.explain(error));
            return Ok(());
        }

        self.initialize_thread_pool();

        // 1. Parse
        let parsed_project = self.parse()?;

        if self.compile_parameters.as_ref().is_some_and(|opt| opt.print_ast) {
            println!("{:#?}", parsed_project.units);
            return Ok(());
        }

        // 2. Index
        let indexed_project = self.index(parsed_project)?;

        // 3. Resolve
        let annotated_project = self.annotate(indexed_project)?;

        if self.compile_parameters.as_ref().is_some_and(|opt| opt.print_ast_lowered) {
            println!("{:#?}", annotated_project.units);
            return Ok(());
        }

        // 4. Validate
        annotated_project.validate(&self.context, &mut self.diagnostician)?;

        //TODO: probably not needed, should be a participant anyway
        if let Some((location, format)) = self
            .compile_parameters
            .as_ref()
            .and_then(|it| it.hardware_config.as_ref())
            .zip(self.compile_parameters.as_ref().and_then(CompileParameters::config_format))
        {
            annotated_project.generate_hardware_information(format, location)?;
        }

        // Skip code-gen if it is check
        if self.compile_parameters.as_ref().is_some_and(CompileParameters::is_check) {
            return Ok(());
        }

        // Generate Header files
        if self.compile_parameters.as_ref().is_some_and(CompileParameters::is_header_generator) {
            return self.generate_headers(annotated_project);
        }

        // 5. Codegen
        self.generate(&CodegenContext::create(), annotated_project)
    }

    fn parse(&mut self) -> Result<ParsedProject, Diagnostic> {
        let project = ParsedProject::parse(&self.context, &self.project, &mut self.diagnostician)?;
        Ok(project)
    }

    fn index(&mut self, project: ParsedProject) -> Result<IndexedProject, Diagnostic> {
        self.participants.iter_mut().for_each(|p| {
            p.pre_index(&project);
        });
        let project = self.mutable_participants.iter_mut().fold(project, |project, p| p.pre_index(project));
        let indexed_project = project.index(self.context.provider());
        self.participants.iter().for_each(|p| {
            p.post_index(&indexed_project);
        });
        let project =
            self.mutable_participants.iter_mut().fold(indexed_project, |project, p| p.post_index(project));

        Ok(project)
    }

    fn annotate(&mut self, project: IndexedProject) -> Result<AnnotatedProject, Diagnostic> {
        self.participants.iter().for_each(|p| {
            p.pre_annotate(&project);
        });
        let project =
            self.mutable_participants.iter_mut().fold(project, |project, p| p.pre_annotate(project));
        let annotated_project = project.annotate(self.context.provider());
        self.participants.iter().for_each(|p| {
            p.post_annotate(&annotated_project);
        });
        let annotated_project = self
            .mutable_participants
            .iter_mut()
            .fold(annotated_project, |project, p| p.post_annotate(project));
        Ok(annotated_project)
    }

    fn generate(&mut self, _context: &CodegenContext, project: AnnotatedProject) -> Result<(), Diagnostic> {
        self.participants.iter_mut().try_fold((), |_, participant| participant.pre_generate(&project))?;
        let Some(compile_options) = self.get_compile_options() else {
            log::debug!("No compile options provided");
            return Ok(());
        };
        let got_layout = if let OnlineChange::Enabled { file_name, format } = &compile_options.online_change {
            read_got_layout(file_name, *format)?
        } else {
            HashMap::default()
        };
        let got_layout = Mutex::new(got_layout);
        let target = self.compile_parameters.as_ref().and_then(|it| it.target.as_ref());
        if compile_options.single_module || matches!(compile_options.output_format, FormatOption::Object) {
            log::info!("Using single module mode");
            let context = CodegenContext::create();
            project
                .generate_single_module(&context, &compile_options, target)?
                .map(|module| {
                    self.participants.iter_mut().try_fold((), |_, participant| participant.generate(&module))
                })
                .unwrap_or(Ok(()))?;
        } else {
            let _ = project
                .units
                .par_iter()
                .map(|AnnotatedUnit { unit, dependencies, literals }| {
                    let context = CodegenContext::create();
                    let module = project.generate_module(
                        &context,
                        &compile_options,
                        unit,
                        dependencies,
                        literals,
                        &got_layout,
                        target,
                    )?;
                    self.participants.iter().try_fold((), |_, participant| participant.generate(&module))
                })
                .collect::<Result<Vec<_>, Diagnostic>>()?;
        }
        if let OnlineChange::Enabled { file_name, format } = &compile_options.online_change {
            write_got_layout(got_layout.into_inner().unwrap(), file_name, *format)?;
        }
        self.participants
            .iter()
            .map(|participant| participant.post_generate())
            .reduce(|prev, curr| prev.and(curr))
            .unwrap_or(Ok(()))?;
        Ok(())
    }

    fn generate_headers(&mut self, _project: AnnotatedProject) -> Result<(), Diagnostic> {
        let Some(generate_header_options) = self.get_generate_header_options() else {
            log::debug!("No generate header options provided!");
            return Ok(());
        };

        // TODO: Load template file and perform content replacement to inject various items to the header
        let contents = "No content yet...";

        // TODO: Should we split the header into multiple files? If so, then this should be modified.
        let (header_path, header_dir) = match generate_header_options.language {
            GenerateLanguage::C => {
                let mut output_path = generate_header_options.output_path.clone();
                let output_dir = generate_header_options.output_path.clone();

                output_path.push(format!("{}.h", generate_header_options.prefix));

                (output_path, output_dir)
            }
            language => {
                log::debug!("{language:?} language not yet supported!");
                return Ok(());
            }
        };

        fs::create_dir_all(header_dir)?;
        fs::write(header_path, contents).map_err(|_| Diagnostic::new("Unable to generate header file..."))
    }
}

pub fn read_got_layout(location: &str, format: ConfigFormat) -> Result<HashMap<String, u64>, Diagnostic> {
    if !Path::new(location).is_file() {
        // Assume if the file doesn't exist that there is no existing GOT layout yet. write_got_layout will handle
        // creating our file when we want to.
        return Ok(HashMap::new());
    }

    let s = fs::read_to_string(location)
        .map_err(|_| Diagnostic::new("GOT layout could not be read from file"))?;
    match format {
        ConfigFormat::JSON => serde_json::from_str(&s)
            .map_err(|_| Diagnostic::new("Could not deserialize GOT layout from JSON")),
        ConfigFormat::TOML => {
            toml::de::from_str(&s).map_err(|_| Diagnostic::new("Could not deserialize GOT layout from TOML"))
        }
    }
}

fn write_got_layout(
    got_entries: HashMap<String, u64>,
    location: &str,
    format: ConfigFormat,
) -> Result<(), Diagnostic> {
    let s = match format {
        ConfigFormat::JSON => serde_json::to_string(&got_entries)
            .map_err(|_| Diagnostic::new("Could not serialize GOT layout to JSON"))?,
        ConfigFormat::TOML => toml::ser::to_string(&got_entries)
            .map_err(|_| Diagnostic::new("Could not serialize GOT layout to TOML"))?,
    };

    fs::write(location, s).map_err(|_| Diagnostic::new("GOT layout could not be written to file"))
}

///Represents a parsed project
///For this struct to be built, the project would have been parsed correctly and an AST would have
///been generated
pub struct ParsedProject {
    units: Vec<CompilationUnit>,
}

impl ParsedProject {
    /// Parses a giving project, transforming it to a `ParsedProject`
    /// Reports parsing diagnostics such as Syntax error on the fly
    pub fn parse<T: SourceContainer + Sync>(
        ctxt: &GlobalContext,
        project: &Project<T>,
        diagnostician: &mut Diagnostician,
    ) -> Result<Self, Diagnostic> {
        //TODO in parallel
        //Parse the source files
        let mut units = vec![];

        let sources = project
            .get_sources()
            .iter()
            .map(|it| {
                let source = ctxt.get(it.get_location_str()).expect("All sources should've been read");

                let parse_func = match source.get_type() {
                    source_code::SourceType::Text => parse_file,
                    source_code::SourceType::Xml => cfc::xml_parser::parse_file,
                    source_code::SourceType::Unknown => unreachable!(),
                };

                parse_func(source, LinkageType::Internal, ctxt.provider(), diagnostician)
            })
            .collect::<Vec<_>>();

        units.extend(sources);

        //Parse the includes
        let includes = project
            .get_includes()
            .iter()
            .map(|it| {
                let source = ctxt.get(it.get_location_str()).expect("All sources should've been read");
                parse_file(source, LinkageType::External, ctxt.provider(), diagnostician)
            })
            .collect::<Vec<_>>();
        units.extend(includes);

        //For each lib, parse the includes
        let lib_includes = project
            .get_libraries()
            .iter()
            .flat_map(LibraryInformation::get_includes)
            .map(|it| {
                let source = ctxt.get(it.get_location_str()).expect("All sources should've been read");
                parse_file(source, LinkageType::External, ctxt.provider(), diagnostician)
            })
            .collect::<Vec<_>>();
        units.extend(lib_includes);

        let units = units.into_iter().collect::<Result<Vec<_>, Diagnostic>>()?;

        Ok(ParsedProject { units })
    }

    /// Creates an index out of a pased project. The index could then be used to query datatypes
    pub fn index(self, id_provider: IdProvider) -> IndexedProject {
        let indexed_units = self
            .units
            .into_par_iter()
            .map(|mut unit| {
                //Preprocess
                pre_process(&mut unit, id_provider.clone());
                //import to index
                let index = indexer::index(&unit);

                (index, unit)
            })
            .collect::<Vec<_>>();

        let mut global_index = Index::default();
        let mut units = vec![];
        for (index, unit) in indexed_units {
            units.push(unit);
            global_index.import(index);
        }

        // import built-in types like INT, BOOL, etc.
        for data_type in plc::typesystem::get_builtin_types() {
            global_index.register_type(data_type);
        }

        // import builtin functions
        let builtins = plc::builtins::parse_built_ins(id_provider);
        global_index.import(indexer::index(&builtins));

        //TODO: evaluate constants should probably be a participant
        let (index, unresolvables) = plc::resolver::const_evaluator::evaluate_constants(global_index);
        IndexedProject { project: ParsedProject { units }, index, unresolvables }
    }
}

///A project that has also been indexed
/// Units inside an index project are ready be resolved and annotated
pub struct IndexedProject {
    project: ParsedProject,
    index: Index,
    unresolvables: Vec<UnresolvableConstant>,
}

impl IndexedProject {
    /// Creates annotations on the project in order to facilitate codegen and validation
    pub fn annotate(self, mut id_provider: IdProvider) -> AnnotatedProject {
        //Create and call the annotator
        let mut annotated_units = Vec::new();
        let mut all_annotations = AnnotationMapImpl::default();
        let result = self
            .project
            .units
            .into_par_iter()
            .map(|unit| {
                let (annotation, dependencies, literals) =
                    TypeAnnotator::visit_unit(&self.index, &unit, id_provider.clone());
                (unit, annotation, dependencies, literals)
            })
            .collect::<Vec<_>>();

        for (unit, annotation, dependencies, literals) in result {
            annotated_units.push(AnnotatedUnit::new(unit, dependencies, literals));
            all_annotations.import(annotation);
        }

        let mut index = self.index;
        index.import(std::mem::take(&mut all_annotations.new_index));

        let annotations = AstAnnotations::new(all_annotations, id_provider.next_id());

        AnnotatedProject { units: annotated_units, index, annotations }
    }

    /// Adds additional, internally generated units to provide functions to be called by a runtime
    /// in order to initialize pointers before first cycle.
    ///
    /// This method will consume the provided indexed project, modify the AST and re-index each unit
    pub fn extend_with_init_units(
        self,
        symbol_name: &'static str,
        id_provider: IdProvider,
    ) -> IndexedProject {
        let units = self.project.units;
        let lowered =
            InitVisitor::visit(units, self.index, self.unresolvables, id_provider.clone(), symbol_name);
        ParsedProject { units: lowered }.index(id_provider.clone())
    }
}

#[derive(Debug)]
pub struct AnnotatedUnit {
    unit: CompilationUnit,
    dependencies: FxIndexSet<Dependency>,
    literals: StringLiterals,
}

impl AnnotatedUnit {
    pub fn new(
        unit: CompilationUnit,
        dependencies: FxIndexSet<Dependency>,
        literals: StringLiterals,
    ) -> Self {
        Self { unit, dependencies, literals }
    }

    pub fn get_unit(&self) -> &CompilationUnit {
        &self.unit
    }
}

impl From<AnnotatedUnit> for CompilationUnit {
    fn from(value: AnnotatedUnit) -> Self {
        value.unit
    }
}

/// A project that has been annotated with information about different types and used units
pub struct AnnotatedProject {
    pub units: Vec<AnnotatedUnit>,
    pub index: Index,
    pub annotations: AstAnnotations,
}

impl AnnotatedProject {
    /// Validates the project, reports any new diagnostics on the fly
    pub fn validate(
        &self,
        ctxt: &GlobalContext,
        diagnostician: &mut Diagnostician,
    ) -> Result<(), Diagnostic> {
        // perform global validation
        let mut validator = Validator::new(ctxt);
        validator.perform_global_validation(&self.index);
        let diagnostics = validator.diagnostics();
        let mut severity = diagnostician.handle(&diagnostics);

        //Perform per unit validation
        self.units.iter().for_each(|AnnotatedUnit { unit, .. }| {
            // validate unit
            validator.visit_unit(&self.annotations, &self.index, unit);
            // log errors
            let diagnostics = validator.diagnostics();
            severity = severity.max(diagnostician.handle(&diagnostics));
        });
        if severity == Severity::Error {
            Err(Diagnostic::new("Compilation aborted due to critical errors"))
        } else {
            Ok(())
        }
    }

    pub fn codegen_to_string(&self, compile_options: &CompileOptions) -> Result<Vec<String>, Diagnostic> {
        let got_layout = if let OnlineChange::Enabled { file_name, format } = &compile_options.online_change {
            read_got_layout(file_name, *format)?
        } else {
            HashMap::default()
        };
        let got_layout = Mutex::new(got_layout);

        self.units
            .iter()
            .map(|AnnotatedUnit { unit, dependencies, literals }| {
                let context = CodegenContext::create();
                self.generate_module(
                    &context,
                    compile_options,
                    unit,
                    dependencies,
                    literals,
                    &got_layout,
                    None,
                )
                .map(|it| it.persist_to_string())
            })
            .collect()
    }

    pub fn generate_single_module<'ctx>(
        &self,
        context: &'ctx CodegenContext,
        compile_options: &CompileOptions,
        target: Option<&Target>,
    ) -> Result<Option<GeneratedModule<'ctx>>, Diagnostic> {
        let got_layout = if let OnlineChange::Enabled { file_name, format } = &compile_options.online_change {
            read_got_layout(file_name, *format)?
        } else {
            HashMap::default()
        };
        let got_layout = Mutex::new(got_layout);

        let Some(module) = self
            .units
            .iter()
            // TODO: this can be parallelized
            .map(|AnnotatedUnit { unit, dependencies, literals }| {
                self.generate_module(
                    context,
                    compile_options,
                    unit,
                    dependencies,
                    literals,
                    &got_layout,
                    target,
                )
            })
            .reduce(|a, b| {
                let a = a?;
                let b = b?;
                a.merge(b).map_err(Into::into)
            })
        else {
            return Ok(None);
        };
        module.map(Some)
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_module<'ctx>(
        &self,
        context: &'ctx CodegenContext,
        compile_options: &CompileOptions,
        unit: &CompilationUnit,
        dependencies: &FxIndexSet<Dependency>,
        literals: &StringLiterals,
        got_layout: &Mutex<HashMap<String, u64>>,
        target: Option<&Target>,
    ) -> Result<GeneratedModule<'ctx>, Diagnostic> {
        // Determine target from compile_options or use default
        let target = target.unwrap_or(&Target::System);

        let mut code_generator = plc::codegen::CodeGen::new(
            context,
            compile_options.root.as_deref(),
            unit.file,
            compile_options.optimization,
            compile_options.debug_level,
            //FIXME don't clone here
            compile_options.online_change.clone(),
            target,
        );
        //Create a types codegen, this contains all the type declarations
        //Associate the index type with LLVM types
        let llvm_index = code_generator.generate_llvm_index(
            context,
            &self.annotations,
            literals,
            dependencies,
            &self.index,
            got_layout,
        )?;
        code_generator.generate(context, unit, &self.annotations, &self.index, llvm_index).map_err(Into::into)
    }

    pub fn codegen_single_module<'ctx>(
        &'ctx self,
        compile_options: &CompileOptions,
        targets: &'ctx [Target],
    ) -> Result<Vec<GeneratedProject>, Diagnostic> {
        let compile_directory = compile_options.build_location.clone().unwrap_or_else(|| {
            let tempdir = tempfile::tempdir().unwrap();
            tempdir.keep()
        });
        ensure_compile_dirs(targets, &compile_directory)?;
        let context = CodegenContext::create(); //Create a build location for the generated object files
        let targets = if targets.is_empty() { &[Target::System] } else { targets };
        let modules =
            targets.iter().map(|target| self.generate_single_module(&context, compile_options, Some(target)));
        let mut result = vec![];
        for (target, module) in targets.iter().zip(modules) {
            let obj: Object = module?
                .unwrap()
                .persist(
                    Some(&compile_directory),
                    &compile_options.output,
                    compile_options.output_format,
                    target,
                    compile_options.optimization,
                )
                .map(Into::into)?;

            result.push(GeneratedProject { target: target.clone(), objects: vec![obj] });
        }

        Ok(result)
    }

    pub fn generate_modules<'ctx>(
        &self,
        context: &'ctx CodegenContext,
        compile_options: &CompileOptions,
    ) -> Result<Vec<GeneratedModule<'ctx>>, Diagnostic> {
        let got_layout = if let OnlineChange::Enabled { file_name, format } = &compile_options.online_change {
            read_got_layout(file_name, *format)?
        } else {
            HashMap::default()
        };
        let got_layout = Mutex::new(got_layout);
        self.units
            .iter()
            .map(|AnnotatedUnit { unit, dependencies, literals }| {
                self.generate_module(
                    context,
                    compile_options,
                    unit,
                    dependencies,
                    literals,
                    &got_layout,
                    None,
                )
            })
            .collect()
    }

    pub fn generate_hardware_information(
        &self,
        format: ConfigFormat,
        location: &str,
    ) -> Result<(), Diagnostic> {
        let hw_conf = plc::hardware_binding::collect_hardware_configuration(&self.index)?;
        let generated_conf = plc::hardware_binding::generate_hardware_configuration(&hw_conf, format)?;
        File::create(location).and_then(|mut it| it.write_all(generated_conf.as_bytes())).map_err(|it| {
            Diagnostic::new(it.to_string()).with_internal_error(it.into()).with_error_code("E002")
        })?;
        Ok(())
    }
}

/// Ensures the directores for the various targets have been created
fn ensure_compile_dirs(targets: &[Target], compile_directory: &Path) -> Result<(), Diagnostic> {
    for target in targets {
        if let Some(name) = target.try_get_name() {
            let dir = compile_directory.join(name);
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}

/// A project that has been transformed into a binary representation
/// Can be linked to generate a usable application
#[derive(Debug)]
pub struct GeneratedProject {
    pub target: Target,
    pub objects: Vec<Object>,
}

impl GeneratedProject {
    pub fn link(
        &self,
        objects: &[Object],
        build_location: Option<&Path>,
        lib_location: Option<&Path>,
        output: &str,
        link_options: LinkOptions,
    ) -> Result<Object, Diagnostic> {
        let output_location = build_location
            .map(|it| self.target.append_to(it))
            .map(|it| it.join(output))
            .unwrap_or_else(|| PathBuf::from(output));

        let output_location = match link_options.format {
            FormatOption::Bitcode => {
                let context = CodegenContext::create();
                let codegen = self
                    .objects
                    .iter()
                    .sorted()
                    .map(|obj| GeneratedModule::try_from_bitcode(&context, obj.get_path()))
                    .reduce(|a, b| {
                        let a = a?;
                        let b = b?;
                        a.merge(b)
                    })
                    .inspect(|it| {
                        if let Ok(it) = it.as_ref() {
                            if let Some(name) = link_options.module_name.as_ref() {
                                it.set_name(name)
                            }
                        }
                    })
                    .ok_or_else(|| {
                        Diagnostic::codegen_error("Could not create bitcode", SourceLocation::undefined())
                    })??;
                codegen.persist_to_bitcode(output_location).map_err(Diagnostic::from)
            }
            FormatOption::IR => {
                let context = CodegenContext::create();
                let codegen = self
                    .objects
                    .iter()
                    .sorted()
                    .map(|obj| GeneratedModule::try_from_ir(&context, obj.get_path()))
                    .reduce(|a, b| {
                        let a = a?;
                        let b = b?;
                        a.merge(b)
                    })
                    .inspect(|it| {
                        if let Ok(it) = it.as_ref() {
                            if let Some(name) = link_options.module_name.as_ref() {
                                it.set_name(name)
                            }
                        }
                    })
                    .ok_or_else(|| {
                        Diagnostic::codegen_error("Could not create ir", SourceLocation::undefined())
                    })??;
                codegen.persist_to_ir(output_location).map_err(Into::into)
            }
            FormatOption::Object if objects.is_empty() => {
                //Just copy over the object file, no need for a linker
                if let [obj] = &self.objects[..] {
                    if obj.get_path() != output_location {
                        // If we already generated to the target path, don't copy
                        std::fs::copy(obj.get_path(), &output_location)?;
                    }
                }
                Ok(output_location)
            }
            _ => {
                // Only initialize a linker if we need to use it
                let target_triple = self.target.get_target_triple();
                let mut linker =
                    plc::linker::Linker::new(&target_triple.as_str().to_string_lossy(), link_options.linker)?;
                for obj in &self.objects {
                    linker.add_obj(&obj.get_path().to_string_lossy());
                }
                for obj in objects {
                    linker.add_obj(&obj.get_path().to_string_lossy());
                }
                for lib_path in &link_options.library_paths {
                    linker.add_lib_path(&lib_path.to_string_lossy());
                }
                for lib in &link_options.libraries {
                    linker.add_lib(lib);
                }
                if let Some(sysroot) = self.target.get_sysroot() {
                    linker.add_sysroot(sysroot);
                }
                //Include the current directory in lib search
                linker.add_lib_path(".");
                if let Some(loc) = build_location {
                    linker.add_lib_path(&loc.to_string_lossy());
                }
                if let Some(loc) = lib_location {
                    linker.add_lib_path(&loc.to_string_lossy());
                }

                //HACK: Create a temp file that would contain the bultin linker script
                //FIXME: This has to be done regardless if the file is used or not because it has
                //to be in scope by the time we call the linker
                let mut file = NamedTempFile::new()?;
                match link_options.linker_script {
                    LinkerScript::Builtin => {
                        let target = self.target.get_target_triple().to_string();
                        //Only do this on linux systems
                        if target.contains("linux") {
                            if target.contains("x86_64") {
                                let content = include_str!("../../../scripts/linker/x86_64.script");
                                writeln!(file, "{content}")?;
                                linker.set_linker_script(file.get_location_str().to_string());
                            } else if target.contains("aarch64") {
                                let content = include_str!("../../../scripts/linker/aarch64.script");
                                writeln!(file, "{content}")?;
                                linker.set_linker_script(file.get_location_str().to_string());
                            } else {
                                debug!("No script for target : {target}");
                            }
                        } else {
                            debug!("No script for target : {target}");
                        }
                    }
                    LinkerScript::Path(script) => linker.set_linker_script(script),
                    LinkerScript::None => {}
                };

                match link_options.format {
                    FormatOption::Static => linker.build_exectuable(output_location).map_err(Into::into),
                    FormatOption::Shared | FormatOption::PIC | FormatOption::NoPIC => {
                        linker.build_shared_obj(output_location).map_err(Into::into)
                    }
                    FormatOption::Object | FormatOption::Relocatable => {
                        linker.build_relocatable(output_location).map_err(Into::into)
                    }
                    _ => unreachable!("Already handled in previous match"),
                }
            }
        }?;

        let output: Object = Object::from(output_location).with_target(&self.target);
        Ok(output)
    }
}
