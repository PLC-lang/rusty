use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{CompileOptions, LinkOptions};
use ast::{CompilationUnit, SourceRange};
use diagnostics::{Diagnostic, Diagnostician};
use encoding_rs::Encoding;
use indexmap::IndexSet;
use plc::{
    codegen::{CodegenContext, GeneratedModule},
    index::Index,
    lexer::IdProvider,
    output::FormatOption,
    parser::parse_file,
    resolver::{AnnotationMapImpl, AstAnnotations, Dependency, StringLiterals, TypeAnnotator},
    validation::Validator,
    ConfigFormat, Target,
};
use project::{
    object::Object,
    project::{LibraryInformation, Project},
};
use rayon::prelude::*;
use source_code::SourceContainer;

///Represents a parsed project
///For this struct to be built, the project would have been parsed correctly and an AST would have
///been generated
pub struct ParsedProject(Vec<CompilationUnit>);

impl ParsedProject {
    /// Parses a giving project, transforming it to a `ParsedProject`
    /// Reprots parsing diagnostics such as Syntax error on the fly
    pub fn parse<T: SourceContainer>(
        project: &Project<T>,
        encoding: Option<&'static Encoding>,
        id_provider: IdProvider,
        diagnostician: &mut Diagnostician,
    ) -> Result<Self, Diagnostic> {
        //TODO in parallel
        //Parse the source files
        let mut units = vec![];

        let sources = project
            .get_sources()
            .iter()
            .map(|it| {
                let loaded_source = it.load_source(encoding).map_err(|err| {
                    Diagnostic::io_read_error(
                        &it.get_location().expect("Location should not be empty").to_string_lossy(),
                        &err,
                    )
                })?;

                let parse_func = match loaded_source.get_type() {
                    source_code::SourceType::Text => parse_file,
                    source_code::SourceType::Xml => cfc::xml_parser::parse_file,
                    source_code::SourceType::Unknown => unreachable!(),
                };
                Ok(parse_func(
                    &loaded_source.source,
                    loaded_source.get_location_str(),
                    ast::LinkageType::Internal,
                    id_provider.clone(),
                    diagnostician,
                ))
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;
        units.extend(sources);
        //Parse the includes
        let includes = project
            .get_includes()
            .iter()
            .map(|it| {
                let loaded_source = it.load_source(encoding).map_err(|err| {
                    Diagnostic::io_read_error(
                        &it.get_location().expect("Location should not be empty").to_string_lossy(),
                        &err,
                    )
                })?;
                Ok(parse_file(
                    &loaded_source.source,
                    loaded_source.get_location_str(),
                    ast::LinkageType::External,
                    id_provider.clone(),
                    diagnostician,
                ))
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;
        units.extend(includes);
        //For each lib, parse the includes
        let lib_includes = project
            .get_libraries()
            .iter()
            .flat_map(LibraryInformation::get_includes)
            .map(|it| {
                let loaded_source = it.load_source(encoding).map_err(|err| {
                    Diagnostic::io_read_error(
                        &it.get_location().expect("Location should not be empty").to_string_lossy(),
                        &err,
                    )
                })?;
                Ok(parse_file(
                    &loaded_source.source,
                    loaded_source.get_location_str(),
                    ast::LinkageType::External,
                    id_provider.clone(),
                    diagnostician,
                ))
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;
        units.extend(lib_includes);

        Ok(ParsedProject(units))
    }

    /// Creates an index out of a pased project. The index could then be used to query datatypes
    pub fn index(self, id_provider: IdProvider) -> Result<IndexedProject, Diagnostic> {
        let indexed_units = self
            .0
            .into_par_iter()
            .map(|mut unit| {
                //Preprocess
                ast::pre_process(&mut unit, id_provider.clone());
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

        Ok(IndexedProject { units, index: global_index })
    }
}

///A project that has also been indexed
/// Units inside an index project could be resolved and annotated
pub struct IndexedProject {
    units: Vec<CompilationUnit>,
    index: Index,
}

impl IndexedProject {
    /// Creates annotations on the project in order to facilitate codegen and validation
    pub fn annotate(
        self,
        mut id_provider: IdProvider,
        _diagnostician: &Diagnostician,
    ) -> Result<AnnotatedProject, Diagnostic> {
        //Resolve constants
        //TODO: Not sure what we are currently doing with unresolvables
        let (mut full_index, _unresolvables) = plc::resolver::const_evaluator::evaluate_constants(self.index);
        //Create and call the annotator
        let mut annotated_units = Vec::new();
        let mut all_annotations = AnnotationMapImpl::default();

        let result = self
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

        Ok(AnnotatedProject { units: annotated_units, index: full_index, annotations })
    }
}

/// A project that has been annotated with information about different types and used units
pub struct AnnotatedProject {
    units: Vec<(CompilationUnit, IndexSet<Dependency>, StringLiterals)>,
    index: Index,
    annotations: AstAnnotations,
}

impl AnnotatedProject {
    /// Validates the project, reports any new diagnostics on the fly
    pub fn validate(&self, diagnostician: &Diagnostician) -> Result<(), Diagnostic> {
        // perform global validation
        let mut validator = Validator::new();
        validator.perform_global_validation(&self.index);
        diagnostician.handle(validator.diagnostics());

        //Perform per unit validation
        self.units.iter().for_each(|(unit, _, _)| {
            // validate unit
            validator.visit_unit(&self.annotations, &self.index, unit);
            // log errors
            diagnostician.handle(validator.diagnostics());
        });
        Ok(())
    }

    pub fn codegen_to_string(&self, compile_options: &CompileOptions) -> Result<Vec<String>, Diagnostic> {
        self.units
            .iter()
            .map(|(unit, dependencies, literals)| {
                let context = CodegenContext::create();
                self.generate_module(&context, compile_options, unit, dependencies, literals)
                    .map(|it| it.persist_to_string())
            })
            .collect()
    }

    pub fn generate_single_module<'ctx>(
        &self,
        context: &'ctx CodegenContext,
        compile_options: &CompileOptions,
    ) -> Result<Option<GeneratedModule<'ctx>>, Diagnostic> {
        let Some(module) = self.units.iter().map(|(unit, dependencies, literals)| {
            self.generate_module(context, compile_options, unit, dependencies, literals)
        }).reduce(|a,b| {
            let a = a?;
            let b = b?;
            a.merge(b)
        }) else {
            return Ok(None)
        };
        module.map(Some)
    }

    fn generate_module<'ctx>(
        &self,
        context: &'ctx CodegenContext,
        compile_options: &CompileOptions,
        unit: &CompilationUnit,
        dependencies: &IndexSet<Dependency>,
        literals: &StringLiterals,
    ) -> Result<GeneratedModule<'ctx>, Diagnostic> {
        let mut code_generator = plc::codegen::CodeGen::new(
            context,
            compile_options.root.as_deref(),
            &unit.file_name,
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
        )?;
        code_generator.generate(context, unit, &self.annotations, &self.index, &llvm_index)
    }

    pub fn codegen_single_module<'ctx>(
        &'ctx self,
        compile_options: CompileOptions,
        targets: &'ctx [Target],
    ) -> Result<Vec<GeneratedProject>, Diagnostic> {
        let compile_directory = compile_options.build_location.clone().unwrap_or_else(|| {
            let tempdir = tempfile::tempdir().unwrap();
            tempdir.into_path()
        });
        ensure_compile_dirs(targets, &compile_directory)?;
        let context = CodegenContext::create(); //Create a build location for the generated object files
        let targets = if targets.is_empty() { &[Target::System] } else { targets };
        let module = self.generate_single_module(&context, &compile_options)?.unwrap();
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

            result.push(GeneratedProject { target, objects: vec![obj] });
        }

        Ok(result)
    }

    pub fn codegen<'ctx>(
        &'ctx self,
        compile_options: CompileOptions,
        targets: &'ctx [Target],
    ) -> Result<Vec<GeneratedProject>, Diagnostic> {
        let compile_directory = compile_options.build_location.clone().unwrap_or_else(|| {
            let tempdir = tempfile::tempdir().unwrap();
            tempdir.into_path()
        });
        ensure_compile_dirs(targets, &compile_directory)?;
        let targets = if targets.is_empty() { &[Target::System] } else { targets };
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
                        let unit_location = std::fs::canonicalize(unit_location)?;
                        let output_name = if unit_location.starts_with(current_dir) {
                            unit_location.strip_prefix(current_dir)?
                        } else if unit_location.has_root() {
                            let root = Path::new("/").canonicalize()?;
                            unit_location.strip_prefix(root).expect("Name has root")
                        } else {
                            unit_location.as_path()
                        };

                        let output_name = match compile_options.output_format {
                            FormatOption::IR => output_name.with_extension("ll"),
                            FormatOption::Bitcode => output_name.with_extension("bc"),
                            _ => output_name.with_extension("o"),
                        };

                        let context = CodegenContext::create(); //Create a build location for the generated object files
                        let module =
                            self.generate_module(&context, &compile_options, unit, dependencies, literals)?;
                        module
                            .persist(
                                Some(&compile_directory),
                                &output_name.to_string_lossy(),
                                compile_options.output_format,
                                target,
                                compile_options.optimization,
                            )
                            .map(Into::into)
                            // Not needed here but might be a good idea for consistency
                            .map(|it: Object| it.with_target(target))
                    })
                    .collect::<Result<Vec<_>, Diagnostic>>()?;

                Ok(GeneratedProject { target, objects })
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;

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
            Diagnostic::GeneralError { err_no: diagnostics::ErrNo::general__io_err, message: it.to_string() }
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
pub struct GeneratedProject<'ctx> {
    target: &'ctx Target,
    objects: Vec<Object>,
}

impl GeneratedProject<'_> {
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
                        Diagnostic::codegen_error("Could not create bitcode", SourceRange::undefined())
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
                        Diagnostic::codegen_error("Could not create ir", SourceRange::undefined())
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
                let mut linker = plc::linker::Linker::new(
                    target_triple.as_str().to_str()?,
                    link_options.linker.as_deref(),
                )?;
                for obj in &self.objects {
                    linker.add_obj(&obj.get_path().to_string_lossy());
                }
                for obj in objects {
                    linker.add_obj(&obj.get_path().to_string_lossy());
                }
                for lib_path in &link_options.library_pathes {
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
                    FormatOption::Shared | FormatOption::PIC => {
                        linker.build_shared_obj(output_location).map_err(Into::into)
                    }
                    FormatOption::Object | FormatOption::Relocatable => {
                        linker.build_relocatable(output_location).map_err(Into::into)
                    }
                    _ => unreachable!("Already handled in previous match"),
                }
            }
        }?;

        let output: Object = Object::from(output_location).with_target(&self.target.clone());
        Ok(output)
    }
}
