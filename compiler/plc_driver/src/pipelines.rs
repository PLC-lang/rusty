use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::{CompileOptions, LinkOptions};
use ast::{
    ast::{pre_process, CompilationUnit, LinkageType},
    provider::IdProvider,
};

use plc::index::FxIndexSet;
use plc::{
    codegen::{CodegenContext, GeneratedModule},
    index::Index,
    output::FormatOption,
    parser::parse_file,
    resolver::{AnnotationMapImpl, AstAnnotations, Dependency, StringLiterals, TypeAnnotator},
    validation::Validator,
    ConfigFormat, Target,
};
use plc_diagnostics::{
    diagnostician::Diagnostician,
    diagnostics::{Diagnostic, Severity},
};
use plc_index::GlobalContext;
use project::{
    object::Object,
    project::{LibraryInformation, Project},
};
use rayon::prelude::*;
use source_code::{source_location::SourceLocation, SourceContainer};

use serde_json;
use toml;

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
pub struct ParsedProject<T: SourceContainer + Sync> {
    project: Project<T>,
    units: Vec<CompilationUnit>,
}

impl<T: SourceContainer + Sync> ParsedProject<T> {
    /// Parses a giving project, transforming it to a `ParsedProject`
    /// Reports parsing diagnostics such as Syntax error on the fly
    pub fn parse(
        ctxt: &GlobalContext,
        project: Project<T>,
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

        Ok(ParsedProject { project, units })
    }

    /// Creates an index out of a pased project. The index could then be used to query datatypes
    pub fn index(self, id_provider: IdProvider) -> IndexedProject<T> {
        let indexed_units = self
            .units
            .into_par_iter()
            .map(|mut unit| {
                //Preprocess
                pre_process(&mut unit, id_provider.clone());
                //import to index
                let index = plc::index::visitor::visit(&unit);

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
        global_index.import(plc::index::visitor::visit(&builtins));

        IndexedProject { project: ParsedProject { project: self.project, units }, index: global_index }
    }

    pub fn get_project(&self) -> &Project<T> {
        &self.project
    }
}

///A project that has also been indexed
/// Units inside an index project are ready be resolved and annotated
pub struct IndexedProject<T: SourceContainer + Sync> {
    project: ParsedProject<T>,
    index: Index,
}

impl<T: SourceContainer + Sync> IndexedProject<T> {
    /// Creates annotations on the project in order to facilitate codegen and validation
    pub fn annotate(self, mut id_provider: IdProvider) -> AnnotatedProject<T> {
        //Resolve constants
        //TODO: Not sure what we are currently doing with unresolvables
        let (mut full_index, _unresolvables) = plc::resolver::const_evaluator::evaluate_constants(self.index);
        //Create and call the annotator
        let mut annotated_units = Vec::new();
        let mut all_annotations = AnnotationMapImpl::default();

        let result = self
            .project
            .units
            .into_par_iter()
            .map(|unit| {
                let (annotation, dependencies, literals) =
                    TypeAnnotator::visit_unit(&full_index, &unit, id_provider.clone());
                (unit, annotation, dependencies, literals)
            })
            .collect::<Vec<_>>();

        for (unit, annotation, dependencies, literals) in result {
            annotated_units.push((unit, dependencies, literals));
            all_annotations.import(annotation);
        }

        full_index.import(std::mem::take(&mut all_annotations.new_index));

        let annotations = AstAnnotations::new(all_annotations, id_provider.next_id());

        AnnotatedProject {
            project: self.project.project,
            units: annotated_units,
            index: full_index,
            annotations,
        }
    }

    fn get_parsed_project(&self) -> &ParsedProject<T> {
        &self.project
    }

    pub fn get_project(&self) -> &Project<T> {
        self.get_parsed_project().get_project()
    }
}

/// A project that has been annotated with information about different types and used units
pub struct AnnotatedProject<T: SourceContainer + Sync> {
    pub project: Project<T>,
    pub units: Vec<(CompilationUnit, FxIndexSet<Dependency>, StringLiterals)>,
    pub index: Index,
    pub annotations: AstAnnotations,
}

impl<T: SourceContainer + Sync> AnnotatedProject<T> {
    pub fn get_project(&self) -> &Project<T> {
        &self.project
    }
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
        self.units.iter().for_each(|(unit, _, _)| {
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
        self.units
            .iter()
            .map(|(unit, dependencies, literals)| {
                let context = CodegenContext::create();
                self.generate_module(
                    &context,
                    compile_options,
                    unit,
                    dependencies,
                    literals,
                    todo!("GOT layout for codegen_to_string?"),
                )
                .map(|it| it.persist_to_string())
            })
            .collect()
    }

    pub fn generate_single_module<'ctx>(
        &self,
        context: &'ctx CodegenContext,
        compile_options: &CompileOptions,
    ) -> Result<Option<GeneratedModule<'ctx>>, Diagnostic> {
        let Some(module) = self
            .units
            .iter()
            .map(|(unit, dependencies, literals)| {
                self.generate_module(
                    context,
                    compile_options,
                    unit,
                    dependencies,
                    literals,
                    todo!("give GOT layout for single modules?"),
                )
            })
            .reduce(|a, b| {
                let a = a?;
                let b = b?;
                a.merge(b)
            })
        else {
            return Ok(None);
        };
        module.map(Some)
    }

    fn generate_module<'ctx>(
        &self,
        context: &'ctx CodegenContext,
        compile_options: &CompileOptions,
        unit: &CompilationUnit,
        dependencies: &FxIndexSet<Dependency>,
        literals: &StringLiterals,
        got_layout: &Mutex<Option<HashMap<String, u64>>>,
    ) -> Result<GeneratedModule<'ctx>, Diagnostic> {
        let mut code_generator = plc::codegen::CodeGen::new(
            context,
            compile_options.root.as_deref(),
            &unit.file_name,
            compile_options.got_layout_file.clone().zip(compile_options.got_layout_format),
            compile_options.optimization,
            compile_options.debug_level,
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
        code_generator.generate(context, unit, &self.annotations, &self.index, llvm_index)
    }

    pub fn codegen_single_module<'ctx>(
        &'ctx self,
        compile_options: &CompileOptions,
        targets: &'ctx [Target],
    ) -> Result<Vec<GeneratedProject>, Diagnostic> {
        let compile_directory = compile_options.build_location.clone().unwrap_or_else(|| {
            let tempdir = tempfile::tempdir().unwrap();
            tempdir.into_path()
        });
        ensure_compile_dirs(targets, &compile_directory)?;
        let context = CodegenContext::create(); //Create a build location for the generated object files
        let targets = if targets.is_empty() { &[Target::System] } else { targets };
        let module = self.generate_single_module(&context, compile_options)?.unwrap();
        let mut result = vec![];
        for target in targets {
            let obj: Object = module
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

    pub fn codegen<'ctx>(
        &'ctx self,
        compile_options: &CompileOptions,
        targets: &'ctx [Target],
    ) -> Result<Vec<GeneratedProject>, Diagnostic> {
        let compile_directory = compile_options.build_location.clone().unwrap_or_else(|| {
            let tempdir = tempfile::tempdir().unwrap();
            tempdir.into_path()
        });
        ensure_compile_dirs(targets, &compile_directory)?;
        let targets = if targets.is_empty() { &[Target::System] } else { targets };

        let got_layout = compile_options
            .got_layout_file
            .as_ref()
            .map(|path| read_got_layout(path, ConfigFormat::JSON))
            .transpose()?;

        let got_layout = Mutex::new(got_layout);

        let res = targets
            .par_iter()
            .map(|target| {
                let objects = self
                    .units
                    .par_iter()
                    .map(|(unit, dependencies, literals)| {
                        let current_dir = env::current_dir()?;
                        let current_dir = compile_options.root.as_deref().unwrap_or(&current_dir);
                        let unit_location = PathBuf::from(&unit.file_name);
                        let unit_location = fs::canonicalize(unit_location)?;
                        let output_name = if unit_location.starts_with(current_dir) {
                            unit_location.strip_prefix(current_dir).map_err(|it| {
                                Diagnostic::new(format!(
                                    "Could not strip prefix for {}",
                                    current_dir.to_string_lossy()
                                ))
                                .with_internal_error(it.into())
                            })?
                        } else if unit_location.has_root() {
                            let root = unit_location.ancestors().last().expect("Should exist?");
                            unit_location.strip_prefix(root).expect("The root directory should exist")
                        } else {
                            unit_location.as_path()
                        };

                        let output_name = match compile_options.output_format {
                            FormatOption::IR => format!("{}.ll", output_name.to_string_lossy()),
                            FormatOption::Bitcode => format!("{}.bc", output_name.to_string_lossy()),
                            _ => format!("{}.o", output_name.to_string_lossy()),
                        };

                        let context = CodegenContext::create(); //Create a build location for the generated object files
                        let module = self.generate_module(
                            &context,
                            compile_options,
                            unit,
                            dependencies,
                            literals,
                            &got_layout,
                        )?;

                        module
                            .persist(
                                Some(&compile_directory),
                                &output_name,
                                compile_options.output_format,
                                target,
                                compile_options.optimization,
                            )
                            .map(Into::into)
                            // Not needed here but might be a good idea for consistency
                            .map(|it: Object| it.with_target(target))
                    })
                    .collect::<Result<Vec<_>, Diagnostic>>()?;

                Ok(GeneratedProject { target: target.clone(), objects })
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;

        compile_options.got_layout_file.as_ref().map(|path| {
            write_got_layout(got_layout.into_inner().unwrap().unwrap(), path, ConfigFormat::JSON)
        });

        Ok(res)
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
    target: Target,
    objects: Vec<Object>,
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
                    .map(|obj| GeneratedModule::try_from_bitcode(&context, obj.get_path()))
                    .reduce(|a, b| {
                        let a = a?;
                        let b = b?;
                        a.merge(b)
                    })
                    .ok_or_else(|| {
                        Diagnostic::codegen_error("Could not create bitcode", SourceLocation::undefined())
                    })??;
                codegen.persist_to_bitcode(output_location)
            }
            FormatOption::IR => {
                let context = CodegenContext::create();
                let codegen = self
                    .objects
                    .iter()
                    .map(|obj| GeneratedModule::try_from_ir(&context, obj.get_path()))
                    .reduce(|a, b| {
                        let a = a?;
                        let b = b?;
                        a.merge(b)
                    })
                    .ok_or_else(|| {
                        Diagnostic::codegen_error("Could not create ir", SourceLocation::undefined())
                    })??;
                codegen.persist_to_ir(output_location)
            }
            FormatOption::Object if self.objects.len() == 1 && objects.is_empty() => {
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
