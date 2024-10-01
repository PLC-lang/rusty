use std::{fmt::Debug, path::PathBuf};

use plc::DebugLevel;
use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};
use plc_index::GlobalContext;
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
    let ctxt = GlobalContext::new()
        .with_source(project.get_sources(), None)?
        .with_source(project.get_includes(), None)?;
    //Parse
    let compile_options = CompileOptions {
        root: path,
        debug_level,
        optimization: plc::OptimizationLevel::None,
        ..Default::default()
    };
    pipelines::ParsedProject::parse(&ctxt, project, &mut diagnostician)?
        //Index
        .index(ctxt.provider())
        //Resolve
        .annotate(ctxt.provider())
        .lower(ctxt.provider())
        //Codegen
        .codegen_to_string(&compile_options)
}
