//! Compiler driver for the PLC Compiler
//!
//! This crates offers the main methods to interact with the PLC Compiler
//! It can be used to verify a project or to produce:
//!  - Object files
//!  - LLVM files
//!  - LLVM Bitcode
//!  - Shared Objects
//!  - Executables

use anyhow::{anyhow, Result};
use pipelines::{
    participant::CodegenParticipant, AnnotatedProject, BuildPipeline, GeneratedProject, Pipeline,
};
use std::{
    ffi::OsStr,
    fmt::{Debug, Display},
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use cli::{CompileParameters, ParameterError};
use plc::{
    codegen::CodegenContext, linker::LinkerType, output::FormatOption, DebugLevel, ErrorFormat, OnlineChange,
    OptimizationLevel,
};

use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic, reporter::DiagnosticReporter};
use plc_index::GlobalContext;
use project::project::Project;
use source_code::SourceContainer;

pub mod cli;
pub mod pipelines;

#[cfg(test)]
mod tests;
//Not a [test] because it is used in external integration tests
pub mod runner;

pub(crate) const DEFAULT_OUTPUT_NAME: &str = "out";

#[derive(Debug)]
pub struct CompileOptions {
    /// Default project location (where the plc.json is defined, or where we are currently
    /// compiling)
    pub root: Option<PathBuf>,
    /// The location where the build would happen. This is None if the build subcommand was not
    /// used
    pub build_location: Option<PathBuf>,
    /// The name of the resulting compiled file
    pub output: String,
    pub output_format: FormatOption,
    pub optimization: OptimizationLevel,
    pub error_format: ErrorFormat,
    pub debug_level: DebugLevel,
    pub single_module: bool,
    pub online_change: OnlineChange,
}

impl Default for CompileOptions {
    fn default() -> Self {
        CompileOptions {
            root: None,
            build_location: None,
            output: String::new(),
            output_format: Default::default(),
            optimization: OptimizationLevel::None,
            error_format: ErrorFormat::None,
            debug_level: DebugLevel::None,
            single_module: false,
            online_change: OnlineChange::Disabled,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct LinkOptions {
    pub libraries: Vec<String>,
    pub library_paths: Vec<PathBuf>,
    pub format: FormatOption,
    pub linker: LinkerType,
    pub lib_location: Option<PathBuf>,
    pub build_location: Option<PathBuf>,
    pub linker_script: LinkerScript,
    pub module_name: Option<String>,
}

#[derive(Clone, Default, Debug)]
pub enum LinkerScript {
    #[default]
    Builtin,
    Path(String),
    None,
}

#[derive(Debug)]
pub enum CompileError {
    Diagnostic(Diagnostic),
    Parameter(ParameterError),
}

impl From<Diagnostic> for CompileError {
    fn from(value: Diagnostic) -> Self {
        Self::Diagnostic(value)
    }
}
impl From<ParameterError> for CompileError {
    fn from(value: ParameterError) -> Self {
        Self::Parameter(value)
    }
}

impl CompileError {
    pub fn exit(&self) {
        match self {
            CompileError::Diagnostic(err) => {
                println!("{err}");
                std::process::exit(1)
            }
            CompileError::Parameter(err) => err.exit(),
        }
    }

    pub fn into_diagnostic(self) -> Option<Diagnostic> {
        let CompileError::Diagnostic(res) = self else { return None };
        Some(res)
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Diagnostic(err) => Display::fmt(err, f),
            CompileError::Parameter(err) => Display::fmt(err, f),
        }
    }
}

pub struct CompilationContext {
    pub compile_parameters: CompileParameters,
    pub project: Project<PathBuf>,
    pub diagnostician: Diagnostician,
    pub compile_options: CompileOptions,
    pub link_options: LinkOptions,
}

pub fn compile<T: AsRef<str> + AsRef<OsStr> + Debug>(args: &[T]) -> Result<()> {
    //Parse the arguments
    let pipeline = BuildPipeline::new(args)?;
    compile_with_pipeline(pipeline)
}

pub fn compile_with_pipeline<T: SourceContainer + Clone + 'static>(
    mut pipeline: BuildPipeline<T>,
) -> Result<()> {
    //register participants
    pipeline.register_default_participants();
    let target = pipeline.compile_parameters.as_ref().and_then(|it| it.target.clone()).unwrap_or_default();
    let codegen_participant = CodegenParticipant {
        compile_options: pipeline.get_compile_options().unwrap(),
        link_options: pipeline.get_link_options().unwrap(),
        target: target.clone(),
        objects: Arc::new(RwLock::new(GeneratedProject {
            target,
            objects: pipeline.project.get_objects().to_vec(),
        })),
        got_layout: Default::default(),
        compile_dirs: Default::default(),
        libraries: pipeline.project.get_libraries().to_vec(),
    };
    pipeline.register_participant(Box::new(codegen_participant));

    let format = pipeline.compile_parameters.as_ref().map(|it| it.error_format).unwrap_or_default();

    pipeline.run().map_err(|err| {
        //Only report the hint if we are using rich error reporting
        if matches!(format, ErrorFormat::Rich) {
            anyhow!(
                "{err}.
Hint: You can use `plc explain <ErrorCode>` for more information"
            )
        } else {
            err.into()
        }
    })
}

/// Parses and annotates a given project. Can be used in tests or api calls
pub fn parse_and_annotate<T: SourceContainer + Clone>(
    name: &str,
    src: Vec<T>,
) -> Result<(GlobalContext, AnnotatedProject), Diagnostic> {
    let (pipeline, project) = parse_and_annotate_with_diagnostics(name, src, Diagnostician::buffered())
        .map_err(|it| Diagnostic::new(it.buffer().unwrap_or_default()))?;
    Ok((pipeline.context, project))
}

pub fn parse_and_annotate_with_diagnostics<T: SourceContainer + Clone>(
    name: &str,
    src: Vec<T>,
    diagnostician: Diagnostician,
) -> Result<(BuildPipeline<T>, AnnotatedProject), Diagnostician> {
    // Parse the source to ast
    let project = Project::new(name.to_string()).with_sources(src);
    let Ok(context) = GlobalContext::new().with_source(project.get_sources(), None) else {
        return Err(diagnostician);
    };
    let mut pipeline = BuildPipeline {
        context,
        project,
        diagnostician,
        compile_parameters: None,
        linker: LinkerType::Internal,
        mutable_participants: Vec::default(),
        participants: Vec::default(),
        module_name: Some("<internal>".to_string()),
    };
    pipeline.register_default_participants();
    let Ok(project) = pipeline.parse() else { return Err(pipeline.diagnostician) };
    let Ok(project) = pipeline.index(project) else { return Err(pipeline.diagnostician) };
    let Ok(project) = pipeline.annotate(project) else { return Err(pipeline.diagnostician) };
    Ok((pipeline, project))
}

pub fn parse_and_validate<T: SourceContainer + Clone>(name: &str, src: Vec<T>) -> String {
    match parse_and_annotate_with_diagnostics(name, src, Diagnostician::buffered()) {
        Ok((mut pipeline, project)) => {
            let _ = project.validate(&pipeline.context, &mut pipeline.diagnostician);
            pipeline.diagnostician.buffer().unwrap()
        }
        Err(diagnostician) => diagnostician.buffer().unwrap(),
    }
}

/// Generates an IR string from a list of sources. Useful for tests or api calls
pub fn generate_to_string<T: SourceContainer>(name: &'static str, src: Vec<T>) -> Result<String, Diagnostic> {
    generate_to_string_internal(name, src, false)
}

/// Generates an IR string from a list of sources with debug information enabled. Useful for tests or api calls
pub fn generate_to_string_debug<T: SourceContainer>(name: &str, src: Vec<T>) -> Result<String, Diagnostic> {
    generate_to_string_internal(name, src, true)
}

fn generate_to_string_internal<T: SourceContainer>(
    name: &str,
    src: Vec<T>,
    debug: bool,
) -> Result<String, Diagnostic> {
    // plc src --ir --single-module
    let project = Project::new(name.to_string()).with_sources(src);
    let context = GlobalContext::new().with_source(project.get_sources(), None)?;
    let diagnostician = Diagnostician::default();
    let mut params = cli::CompileParameters::parse(&["plc", "--ir", "--single-module", "-O", "none"])
        .map_err(|e| Diagnostic::new(e.to_string()))?;
    params.generate_debug = debug;
    let mut pipeline = BuildPipeline {
        context,
        project,
        diagnostician,
        compile_parameters: Some(params),
        linker: LinkerType::Internal,
        mutable_participants: Vec::default(),
        participants: Vec::default(),
        module_name: Some("<internal>".to_string()),
    };
    pipeline.register_default_participants();
    let project = pipeline.parse()?;
    let project = pipeline.index(project)?;
    let project = pipeline.annotate(project)?;
    // Validate
    // TODO: move validation to participants, maybe refactor codegen to stop at generated modules and persist in dedicated step?
    project.validate(&pipeline.context, &mut pipeline.diagnostician)?;
    let context = CodegenContext::create();
    let module =
        project.generate_single_module(&context, pipeline.get_compile_options().as_ref().unwrap(), None)?;

    // Generate
    module.map(|it| it.persist_to_string()).ok_or_else(|| Diagnostic::new("Cannot generate module"))
}

fn get_project(compile_parameters: &CompileParameters) -> Result<Project<PathBuf>> {
    //Create a project from either the subcommand or single params
    let project = compile_parameters
        .get_build_configuration()?
        .map(|it| Project::from_config(&it))
        .unwrap_or_else(|| {
            //Build with parameters
            let name = compile_parameters
                .input
                .first()
                .and_then(|it| it.get_location())
                .and_then(|it| it.file_name())
                .and_then(|it| it.to_str())
                .unwrap_or(DEFAULT_OUTPUT_NAME);
            let project = Project::new(name.to_string())
                .with_file_paths(compile_parameters.input.iter().map(PathBuf::from).collect())
                .with_include_paths(compile_parameters.includes.iter().map(PathBuf::from).collect())
                .with_library_paths(compile_parameters.library_paths.iter().map(PathBuf::from).collect())
                .with_libraries(compile_parameters.libraries.clone());
            Ok(project)
        });
    //Override default settings with compile options
    project
        .map(|proj| {
            if let Some(format) = compile_parameters.output_format() {
                proj.with_format(format)
            } else {
                proj
            }
        })
        .map(|proj| proj.with_output_name(compile_parameters.output.clone()))
}

fn get_config(root: &Path) -> PathBuf {
    root.join("plc.json")
}
