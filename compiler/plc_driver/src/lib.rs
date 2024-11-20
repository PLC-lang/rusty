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
use pipelines::{AnnotatedProject, BuildPipeline, Pipeline};
use std::{
    env,
    ffi::OsStr,
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use cli::{CompileParameters, ParameterError, SubCommands};
use plc::{
    codegen::CodegenContext, linker::LinkerType, output::FormatOption, DebugLevel, ErrorFormat, OnlineChange,
    OptimizationLevel, Target, Threads,
};

use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};
use plc_index::GlobalContext;
use project::project::{LibraryInformation, Project};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
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
    let mut pipeline = BuildPipeline::new(args)?;
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
pub fn parse_and_annotate<T: SourceContainer>(
    name: &str,
    src: Vec<T>,
) -> Result<(GlobalContext, AnnotatedProject), Diagnostic> {
    // Parse the source to ast
    let project = Project::new(name.to_string()).with_sources(src);
    let context = GlobalContext::new().with_source(project.get_sources(), None)?;
    let mut diagnostician = Diagnostician::default();
    let pipeline = BuildPipeline {
        context,
        project,
        diagnostician,
        compile_parameters: None,
    };
    let parsed = pipelines::ParsedProject::parse(&ctxt, project, &mut diagnostician)?;

    // Create an index, add builtins then resolve
    let provider = ctxt.provider();
    Ok((ctxt, parsed.index(provider.clone()).annotate(provider.clone())))
}

/// Generates an IR string from a list of sources. Useful for tests or api calls
//todo: test code
pub fn generate_to_string<T: SourceContainer>(name: &'static str, src: Vec<T>) -> Result<String, Diagnostic> {
    generate_to_string_internal(name, src, false)
}

/// Generates an IR string from a list of sources with debug information enabled. Useful for tests or api calls
//todo: test code
pub fn generate_to_string_debug<T: SourceContainer>(name: &str, src: Vec<T>) -> Result<String, Diagnostic> {
    generate_to_string_internal(name, src, true)
}

//todo: test code
fn generate_to_string_internal<T: SourceContainer>(
    name: &str,
    src: Vec<T>,
    debug: bool,
) -> Result<String, Diagnostic> {
    let mut diagnostician = Diagnostician::default();
    let (ctxt, project) = parse_and_annotate(name, src)?;

    // Validate
    project.validate(&ctxt, &mut diagnostician)?;

    // Generate
    let context = CodegenContext::create();
    let mut options = CompileOptions::default();
    if debug {
        options.debug_level = DebugLevel::Full(plc::DEFAULT_DWARF_VERSION);
    }
    let module = project.generate_single_module(&context, &options)?;

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
