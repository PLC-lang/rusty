//!
//! Pipeline participants allow for additional steps to happen during the build.
//! Such steps can be read only using the `PipelineParticipant` such as Validators
//! or Read Write using the `PipelineParticipantMut` such as lowering operations
//!

use std::{
    cell::OnceCell,
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use plc::{
    codegen::{self, GeneratedModule},
    output::FormatOption,
    ConfigFormat, OnlineChange, Target,
};
use plc_diagnostics::diagnostics::Diagnostic;
use project::object::Object;

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
    fn pre_codegen(&mut self, _annotated_project: &AnnotatedProject) -> Result<(), Diagnostic> {
        Ok(())
    }
    /// Implement this to get access to the module generation section of the codegen
    /// This is useful if generating multiple modules to hook into single module generation
    fn codegen(&self, _generated_module: &GeneratedModule) -> Result<(), Diagnostic> {
        Ok(())
    }
    /// Implement this to access the project after it got generated
    /// This happens after codegen
    fn post_codegen(&self) {}
}

/// A Mutating Build particitpant for different steps in the pipeline
/// Implementors can decide to modify the AST, project and generated code,
/// for example for de-sugaring/lowering/pre-processing the AST
/// If a previous step is being modified, such as the AST or index,
/// the caller is responsible for calling the previous steps
pub trait PipelineParticipantMut {
    /// Implement this to access the project before it gets indexed
    /// This happens directly after parsing
    fn pre_index(&self, _parsed_project: &mut ParsedProject) -> bool {
        false
    }
    /// Implement this to access the project after it got indexed
    /// This happens directly after the index returns
    fn post_index(&self, _indexed_project: &mut IndexedProject) -> bool {
        false
    }
    /// Implement this to access the project before it gets annotated
    /// This happens after indexing
    fn pre_annotate(&self, _indexed_project: &mut IndexedProject) -> bool {
        false
    }
    /// Implement this to access the project after it got annotated
    /// This happens directly after annotations
    fn post_annotate(&self, _annotated_project: &mut AnnotatedProject) -> bool {
        false
    }
    /// Implement this to access the project before it gets generated
    /// This happens after annotation
    fn pre_codegen(&self, _annotated_project: &mut AnnotatedProject) {}
}

struct CodegenParticipant<'tgt> {
    compile_options: crate::CompileOptions,
    targets: &'tgt [Target],
    got_layout: Mutex<HashMap<String, u64>>,
    compile_dirs: HashMap<&'tgt Target, PathBuf>,
    objects : HashMap<&'tgt Target, Vec<Object>>
}

impl<'tgt> CodegenParticipant<'tgt> {
    /// Ensures the directores for the various targets have been created
    fn ensure_compile_dirs(&mut self) -> Result<(), Diagnostic> {
        let targets = if self.targets.is_empty() { &[Target::System] } else { self.targets };
        let compile_directory = self.compile_options.build_location.clone().unwrap_or_else(|| {
            let tempdir = tempfile::tempdir().unwrap();
            tempdir.into_path()
        });
        for target in targets {
            if let Some(name) = target.try_get_name() {
                let dir = compile_directory.join(name);
                fs::create_dir_all(&dir)?;
                self.compile_dirs.insert(target, dir);
            }
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

impl<'tgt> PipelineParticipant for CodegenParticipant<'tgt> {
    fn pre_codegen(&mut self, _annotated_project: &AnnotatedProject) -> Result<(), Diagnostic>{
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

    fn codegen(&self, module: &GeneratedModule) -> Result<(), Diagnostic> {
        let current_dir = env::current_dir()?;
        let current_dir = self.compile_options.root.as_deref().unwrap_or(&current_dir);
        let unit_location = module.get_unit_location();
        let unit_location =
            if unit_location.exists() { fs::canonicalize(unit_location)? } else { unit_location.into() };
        let output_name = if unit_location.starts_with(current_dir) {
            unit_location
                .strip_prefix(current_dir)
                .map_err(|it| {
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

        for target in self.targets {
            let compile_directory = self.compile_dirs.get(target).expect("Required dir");
            let object = module
                .persist(
                    Some(&compile_directory),
                    &output_name,
                    self.compile_options.output_format,
                    target,
                    self.compile_options.optimization,
                )
                .map(Into::into)
                .map(|it: Object| it.with_target(target))?;
            self.objects.entry(target).or_insert_with(Vec::default).push(object);
        }
        Ok(())

    }

    fn post_codegen(&self) {
        //TODO: link generated modules
    }
}
