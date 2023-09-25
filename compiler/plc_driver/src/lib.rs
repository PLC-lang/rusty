//! Compiler driver for the PLC Compiler
//!
//! This crates offers the main methods to interact with the PLC Compiler
//! It can be used to verify a project or to produce:
//!  - Object files
//!  - LLVM files
//!  - LLVM Bitcode
//!  - Shared Objects
//!  - Executables

use std::{
    env,
    ffi::OsStr,
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use ast::provider::IdProvider;
use cli::{CompileParameters, ParameterError};
use pipelines::AnnotatedProject;
use plc::{
    codegen::CodegenContext, output::FormatOption, DebugLevel, ErrorFormat, OptimizationLevel, Threads,
};

use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};
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
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct LinkOptions {
    pub libraries: Vec<String>,
    pub library_pathes: Vec<PathBuf>,
    pub format: FormatOption,
    pub linker: Option<String>,
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

pub fn compile<T: AsRef<str> + AsRef<OsStr> + Debug>(args: &[T]) -> Result<(), CompileError> {
    //Parse the arguments
    let compile_parameters = CompileParameters::parse(args)?;
    let project = get_project(&compile_parameters)?;
    let output_format = compile_parameters.output_format().unwrap_or_else(|| project.get_output_format());
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
    let id_provider = IdProvider::default();
    let mut diagnostician = match compile_parameters.error_format {
        ErrorFormat::Rich => Diagnostician::default(),
        ErrorFormat::Clang => Diagnostician::clang_format_diagnostician(),
        ErrorFormat::None => Diagnostician::null_diagnostician(),
    };

    //Set the global thread count
    let thread_pool = rayon::ThreadPoolBuilder::new();
    let global_pool = if let Some(Threads::Fix(threads)) = compile_parameters.threads {
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

    // 1 : Parse
    let annotated_project = pipelines::ParsedProject::parse(
        &project,
        compile_parameters.encoding,
        id_provider.clone(),
        &mut diagnostician,
    )?
    // 2 : Index
    .index(id_provider.clone())?
    // 3 : Resolve
    .annotate(id_provider, &diagnostician)?;
    // 4 : Validate
    annotated_project.validate(&mut diagnostician)?;
    // 5 : Codegen
    if !compile_parameters.is_check() {
        let res = generate(
            location,
            compile_parameters,
            project,
            output_format,
            annotated_project,
            build_location,
            lib_location,
        )
        .map_err(|err| Diagnostic::codegen_error(err.get_message(), err.get_location()));
        if let Err(res) = res {
            diagnostician.handle(&[res]);
            return Err(Diagnostic::GeneralError {
                message: "Compilation aborted due to previous errors".into(),
                err_no: plc_diagnostics::errno::ErrNo::codegen__general,
            }
            .into());
        }
    }

    Ok(())
}

/// Parses and annotates a given project. Can be used in tests or api calls
pub fn parse_and_annotate<T: SourceContainer>(
    name: &str,
    src: Vec<T>,
) -> Result<AnnotatedProject, Diagnostic> {
    // Parse the source to ast
    let project = Project::new(name.to_string()).with_sources(src);
    let id_provider = IdProvider::default();
    let mut diagnostician = Diagnostician::default();
    pipelines::ParsedProject::parse(&project, None, id_provider.clone(), &mut diagnostician)?
        // Create an index, add builtins
        .index(id_provider.clone())?
        // Resolve
        .annotate(id_provider, &diagnostician)
}

/// Generates an IR string from a list of sources. Useful for tests or api calls
pub fn generate_to_string<T: SourceContainer>(name: &str, src: Vec<T>) -> Result<String, Diagnostic> {
    let mut diagnostician = Diagnostician::default();
    let project = parse_and_annotate(name, src)?;
    //Validate
    project.validate(&mut diagnostician)?;
    // Generate
    let context = CodegenContext::create();
    let module = project.generate_single_module(&context, &CompileOptions::default())?;

    module.map(|it| it.persist_to_string()).ok_or_else(|| Diagnostic::GeneralError {
        message: "Cannot generate module".to_string(),
        err_no: plc_diagnostics::errno::ErrNo::general__err,
    })
}

fn generate(
    location: Option<PathBuf>,
    compile_parameters: CompileParameters,
    project: Project<PathBuf>,
    output_format: FormatOption,
    annotated_project: pipelines::AnnotatedProject,
    build_location: Option<PathBuf>,
    lib_location: Option<PathBuf>,
) -> Result<(), Diagnostic> {
    let compile_options = CompileOptions {
        root: location,
        build_location: compile_parameters.get_build_location(),
        output: project.get_output_name(),
        output_format,
        optimization: compile_parameters.optimization,
        error_format: compile_parameters.error_format,
        debug_level: compile_parameters.debug_level(),
    };
    let res = if compile_parameters.single_module {
        log::info!("Using single module mode");
        annotated_project.codegen_single_module(compile_options, &compile_parameters.target)?
    } else {
        annotated_project.codegen(compile_options, &compile_parameters.target)?
    };
    let libraries =
        project.get_libraries().iter().map(LibraryInformation::get_link_name).map(str::to_string).collect();
    let library_pathes = project
        .get_libraries()
        .iter()
        .filter_map(LibraryInformation::get_path)
        .map(Path::to_path_buf)
        .collect();
    let linker_options = LinkOptions {
        libraries,
        library_pathes,
        format: output_format,
        linker: compile_parameters.linker.to_owned(),
    };
    let output_name = project.get_output_name();
    res.into_par_iter()
        .map(|res| {
            res.link(
                project.get_objects(),
                build_location.as_deref(),
                lib_location.as_deref(),
                &output_name,
                linker_options.clone(),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    if let Some((location, format)) =
        compile_parameters.hardware_config.as_ref().zip(compile_parameters.config_format())
    {
        annotated_project.generate_hardware_information(format, location)?;
    }
    if let Some(lib_location) = lib_location {
        for library in
            project.get_libraries().iter().filter(|it| it.should_copy()).map(|it| it.get_compiled_lib())
        {
            for obj in library.get_objects() {
                let path = obj.get_path();
                if let Some(name) = path.file_name() {
                    std::fs::copy(path, lib_location.join(name))?;
                }
            }
        }
    }
    //Run packaging commands
    Ok(())
}

fn get_project(compile_parameters: &CompileParameters) -> Result<Project<PathBuf>, Diagnostic> {
    let current_dir = env::current_dir()?;
    //Create a project from either the subcommand or single params
    let project = if let Some(command) = &compile_parameters.commands {
        //Build with subcommand
        let config = command
            .get_build_configuration()
            .map(PathBuf::from)
            .or_else(|| get_config(&current_dir))
            .ok_or_else(|| Diagnostic::param_error("Could not find 'plc.json'"))?;
        Project::from_config(&config)
    } else {
        //Build with parameters
        let name = compile_parameters
            .input
            .get(0)
            .and_then(|it| it.get_location())
            .and_then(|it| it.file_name())
            .and_then(|it| it.to_str())
            .unwrap_or(DEFAULT_OUTPUT_NAME);
        let project = Project::new(name.to_string())
            .with_file_pathes(compile_parameters.input.iter().map(PathBuf::from).collect())
            .with_include_pathes(compile_parameters.includes.iter().map(PathBuf::from).collect())
            .with_libraries(compile_parameters.libraries.clone());
        Ok(project)
    };

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

fn get_config(root: &Path) -> Option<PathBuf> {
    Some(root.join("plc.json"))
}
