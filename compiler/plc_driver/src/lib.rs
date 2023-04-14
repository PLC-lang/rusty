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
    fmt::Debug,
    path::{Path, PathBuf},
};

use cli::CompileParameters;
use diagnostics::{Diagnostic, Diagnostician};
use plc::{lexer::IdProvider, output::FormatOption};
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

#[derive(Clone, Default, Debug)]
pub struct LinkOptions {
    pub libraries: Vec<String>,
    pub library_pathes: Vec<PathBuf>,
    pub format: FormatOption,
    pub linker: Option<String>,
}

pub fn compile<T: AsRef<str> + AsRef<OsStr> + Debug>(args: &[T]) -> Result<(), Diagnostic> {
    //Parse the arguments
    let compile_parameters = CompileParameters::parse(args)?;
    let project = get_project(&compile_parameters)?;
    let output_format = compile_parameters.output_format().unwrap_or_else(|| project.get_output_format());
    let location = project.get_location().map(|it| it.to_path_buf());
    if let Some(location) = &location {
        env::set_var("PROJECT_ROOT", location);
    }
    let build_location = compile_parameters.get_build_location();
    if let Some(location) = &build_location {
        env::set_var("BUILD_LOCATION", location);
    }
    let lib_location = compile_parameters.get_lib_location();
    if let Some(location) = &lib_location {
        env::set_var("LIB_LOCATION", location);
    }
    let id_provider = IdProvider::default();
    let mut diagnostician = Diagnostician::default(); //TODO
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
    // 4 : Validate and Codegen (parallel)
    annotated_project.validate(&diagnostician)?;
    let res = annotated_project.codegen(
        location.as_deref(),
        compile_parameters.get_build_location().as_deref(),
        compile_parameters.optimization,
        compile_parameters.debug_level(),
        output_format,
        &compile_parameters.target,
    )?;
    // 5 : Link
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
            res.link(project.get_objects(), build_location.as_deref(), &output_name, linker_options.clone())
        })
        .collect::<Result<Vec<_>, _>>()?;
    //Generate hardware configuration
    if let Some((location, format)) =
        compile_parameters.hardware_config.as_ref().zip(compile_parameters.config_format())
    {
        annotated_project.generate_hardware_information(format, &location)?;
    }

    // Copy libraries
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
