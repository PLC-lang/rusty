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
use std::{
    env,
    ffi::OsStr,
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use cli::{CompileParameters, ParameterError, SubCommands};
use pipelines::AnnotatedProject;
use plc::{
    codegen::CodegenContext, linker::LinkerType, output::FormatOption, ConfigFormat, DebugLevel,
    ErrorFormat, OptimizationLevel, Target, Threads,
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
    pub got_layout_file: Option<String>,
    pub got_layout_format: Option<ConfigFormat>,
    pub optimization: OptimizationLevel,
    pub error_format: ErrorFormat,
    pub debug_level: DebugLevel,
    pub single_module: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        CompileOptions {
            root: None,
            build_location: None,
            output: String::new(),
            output_format: Default::default(),
            got_layout_file: None,
            got_layout_format: None,
            optimization: OptimizationLevel::None,
            error_format: ErrorFormat::None,
            debug_level: DebugLevel::None,
            single_module: false,
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

pub fn get_compilation_context<T: AsRef<str> + AsRef<OsStr> + Debug>(
    args: &[T],
) -> Result<CompilationContext> {
    let compile_parameters = CompileParameters::parse(args)?;
    //Create the project that will be compiled
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

    let compile_options = CompileOptions {
        root: location,
        build_location: compile_parameters.get_build_location(),
        output: project.get_output_name(),
        output_format,
        got_layout_file: compile_parameters.got_layout_file.clone(),
        got_layout_format: compile_parameters.got_layout_format(),
        optimization: compile_parameters.optimization,
        error_format: compile_parameters.error_format,
        debug_level: compile_parameters.debug_level(),
        single_module: compile_parameters.single_module,
    };

    let libraries =
        project.get_libraries().iter().map(LibraryInformation::get_link_name).map(str::to_string).collect();
    let mut library_paths: Vec<PathBuf> = project
        .get_libraries()
        .iter()
        .filter_map(LibraryInformation::get_path)
        .map(Path::to_path_buf)
        .collect();

    library_paths.extend_from_slice(project.get_library_paths());

    let link_options = LinkOptions {
        libraries,
        library_paths,
        format: output_format,
        linker: compile_parameters.linker.as_deref().into(),
        lib_location,
    };

    Ok(CompilationContext { compile_parameters, project, diagnostician, compile_options, link_options })
}

pub fn compile_with_options(compile_options: CompilationContext) -> Result<()> {
    let CompilationContext { compile_parameters, project, mut diagnostician, compile_options, link_options } =
        compile_options;
    if let Some((options, _format)) = compile_parameters.get_config_options() {
        return print_config_options(&project, &diagnostician, options);
    }

    if let Some(SubCommands::Explain { error }) = &compile_parameters.commands {
        //Explain the given error
        println!("{}", diagnostician.explain(error));
        return Ok(());
    }

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

    // TODO: This can be improved quite a bit, e.g. `GlobalContext::new(project);`, to do that see the
    //       commented `project` method in the GlobalContext implementation block
    let ctxt = GlobalContext::new()
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

    // 1 : Parse, 2. Index and 3. Resolve / Annotate
    let annotated_project = pipelines::ParsedProject::parse(&ctxt, project, &mut diagnostician)?
        .index(ctxt.provider())
        .annotate(ctxt.provider());

    // 4 : Validate
    annotated_project.validate(&ctxt, &mut diagnostician)?;

    if let Some((location, format)) =
        compile_parameters.hardware_config.as_ref().zip(compile_parameters.config_format())
    {
        annotated_project.generate_hardware_information(format, location)?;
    }

    // 5 : Codegen
    if !compile_parameters.is_check() {
        let res = generate(&compile_options, &link_options, compile_parameters.target, annotated_project)
            .map_err(|err| Diagnostic::codegen_error(err.get_message(), err.get_location()));
        if let Err(res) = res {
            diagnostician.handle(&[res]);
            return Err(Diagnostic::new("Compilation aborted due to previous errors")
                .with_error_code("E071")
                .into());
        }
    }
    Ok(())
}

pub fn compile<T: AsRef<str> + AsRef<OsStr> + Debug>(args: &[T]) -> Result<()> {
    //Parse the arguments
    let compile_context = get_compilation_context(args)?;
    let format = compile_context.compile_parameters.error_format;
    compile_with_options(compile_context).map_err(|err| {
        //Only report the hint if we are using rich error reporting
        if matches!(format, ErrorFormat::Rich) {
            anyhow!(
                "{err}.
Hint: You can use `plc explain <ErrorCode>` for more information"
            )
        } else {
            err
        }
    })
}

fn print_config_options<T: AsRef<Path> + Sync>(
    project: &Project<T>,
    diagnostician: &Diagnostician,
    option: cli::ConfigOption,
) -> std::result::Result<(), anyhow::Error> {
    match option {
        cli::ConfigOption::Schema => {
            println!("{}", project.get_validation_schema().as_ref())
        }
        cli::ConfigOption::Diagnostics => {
            println!("{}", diagnostician.get_diagnostic_configuration())
        }
    };

    Ok(())
}

/// Parses and annotates a given project. Can be used in tests or api calls
pub fn parse_and_annotate<T: SourceContainer>(
    name: &str,
    src: Vec<T>,
) -> Result<(GlobalContext, AnnotatedProject<T>), Diagnostic> {
    // Parse the source to ast
    let project = Project::new(name.to_string()).with_sources(src);
    let ctxt = GlobalContext::new().with_source(project.get_sources(), None)?;
    let mut diagnostician = Diagnostician::default();
    let parsed = pipelines::ParsedProject::parse(&ctxt, project, &mut diagnostician)?;

    // Create an index, add builtins then resolve
    let provider = ctxt.provider();
    Ok((ctxt, parsed.index(provider.clone()).annotate(provider)))
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

fn generate(
    compile_options: &CompileOptions,
    linker_options: &LinkOptions,
    targets: Vec<Target>,
    annotated_project: AnnotatedProject<PathBuf>,
) -> Result<(), Diagnostic> {
    let res = if compile_options.single_module {
        log::info!("Using single module mode");
        annotated_project.codegen_single_module(compile_options, &targets)?
    } else {
        annotated_project.codegen(compile_options, &targets)?
    };
    let project = annotated_project.get_project();
    let output_name = project.get_output_name();
    res.into_par_iter()
        .map(|res| {
            res.link(
                project.get_objects(),
                compile_options.build_location.as_deref(),
                linker_options.lib_location.as_deref(),
                &output_name,
                linker_options.clone(),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    if let Some(lib_location) = &linker_options.lib_location {
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
