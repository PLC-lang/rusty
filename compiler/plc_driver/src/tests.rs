use std::{fmt::Debug, path::PathBuf};

use ast::provider::IdProvider;
use plc::DebugLevel;
use plc_diagnostics::diagnostics::{Diagnostic, Diagnostician};
use project::project::Project;
use source_code::SourceContainer;

use crate::{pipelines, CompileOptions};

mod external_files;
mod multi_files;

pub fn compile_with_root<S, T>(
    sources: T,
    includes: T,
    root: &str,
    debug_level: DebugLevel,
) -> Result<Vec<String>, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    compile_to_string(sources, includes, Some(root), debug_level)
}

pub fn compile_to_string<S, T>(
    sources: T,
    includes: T,
    root: Option<&str>,
    debug_level: DebugLevel,
) -> Result<Vec<String>, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    let path: Option<PathBuf> = root.map(|it| it.into());
    let mut diagnostician = Diagnostician::null_diagnostician();
    //Create a project
    let project = Project::new("TestProject".into()).with_sources(sources).with_source_includes(includes);
    //Parse
    let id_provider = IdProvider::default();
    let compile_options = CompileOptions {
        root: path,
        debug_level,
        optimization: plc::OptimizationLevel::None,
        ..Default::default()
    };
    pipelines::ParsedProject::parse(&project, None, id_provider.clone(), &mut diagnostician)?
        //Index
        .index(id_provider.clone())?
        //Resolve
        .annotate(id_provider, &diagnostician)?
        //Codegen
        .codegen_to_string(&compile_options)
}
