// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
//! A Structured Text LLVM Frontent
//!
//! RuSTy is an [`ST`] Compiler using LLVM
//!
//! # Features
//! ## Standard language support
//! Most of the [`IEC61131-3`] standard for ST and general programing is supported.
//! ## Native compilation
//! A (currently) single ST files into object code using LLVM.
//! A compiled object can be linked statically or dynamically linked
//!     with other programs using standard compiler linkers (ld, clang, gcc)
//! ## IR Output
//! An [`IR`] file can be generated from any given ST file in order to examin the generated LLVM IR code.
//! For a usage guide refer to the [User Documentation](../../)
//!
//! [`ST`]: https://en.wikipedia.org/wiki/Structured_text
//! [`IEC61131-3`]: https://en.wikipedia.org/wiki/IEC_61131-3
//! [`IR`]: https://llvm.org/docs/LangRef.html
use std::fs;
use std::io::Write;
use std::str::FromStr;

use build::{get_project_from_file, string_to_filepath};
use clap::ArgEnum;
use codegen::CodeGen;
use glob::glob;
use inkwell::passes::PassBuilderOptions;
use serde::{Deserialize, Serialize};
use std::path::Path;

use ast::{LinkageType, PouType, SourceRange};
use cli::{CompileParameters, SubCommands};
use diagnostics::Diagnostic;
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use index::Index;
use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use lexer::IdProvider;
use resolver::{AstAnnotations, StringLiterals};
use std::{fs::File, io::Read};
use validation::Validator;

use crate::ast::CompilationUnit;
use crate::diagnostics::Diagnostician;
use crate::resolver::{AnnotationMapImpl, TypeAnnotator};
mod ast;
pub mod build;
mod builtins;
pub mod cli;
mod codegen;
pub mod diagnostics;
pub mod expression_path;
mod hardware_binding;
pub mod index;
mod lexer;
mod linker;
mod parser;
mod resolver;
mod test_utils;

pub mod runner;
mod typesystem;
mod validation;

#[macro_use]
#[cfg(test)]
extern crate pretty_assertions;

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FormatOption {
    Static,
    PIC,
    Shared,
    Relocatable,
    Bitcode,
    IR,
}

#[derive(PartialEq, Debug, Clone, Copy, ArgEnum)]
pub enum ConfigFormat {
    JSON,
    TOML,
}

impl FromStr for ConfigFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(ConfigFormat::JSON),
            "toml" => Ok(ConfigFormat::TOML),
            _ => Err(format!("Invalid option {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CompileOptions {
    pub format: Option<FormatOption>,
    pub output: String,
    pub target: Option<String>,
    pub optimization: OptimizationLevel,
}

pub struct LinkOptions {
    pub libraries: Vec<String>,
    pub library_pathes: Vec<String>,
    pub sysroot: Option<String>,
    pub format: FormatOption,
}

struct ConfigurationOptions {
    format: ConfigFormat,
    output: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ArgEnum, Serialize, Deserialize)]
pub enum ErrorFormat {
    Rich,
    Clang,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Serialize, Deserialize)]
pub enum OptimizationLevel {
    None,
    Less,
    Default,
    Aggressive,
}

impl From<OptimizationLevel> for inkwell::OptimizationLevel {
    fn from(val: OptimizationLevel) -> Self {
        match val {
            OptimizationLevel::None => inkwell::OptimizationLevel::None,
            OptimizationLevel::Less => inkwell::OptimizationLevel::Less,
            OptimizationLevel::Default => inkwell::OptimizationLevel::Default,
            OptimizationLevel::Aggressive => inkwell::OptimizationLevel::Aggressive,
        }
    }
}

impl OptimizationLevel {
    fn opt_params(&self) -> &str {
        match self {
            OptimizationLevel::None => "default<O0>",
            OptimizationLevel::Less => "default<O1>",
            OptimizationLevel::Default => "default<O2>",
            OptimizationLevel::Aggressive => "default<O3>",
        }
    }
}

/// A struct representing the result of a compilation
#[derive(Default)]
pub struct CompileResult {
    pub index: Index,
    pub objects: Vec<FilePath>,
}

/// SourceContainers offer source-code to be compiled via the load_source function.
/// Furthermore it offers a location-String used when reporting diagnostics.
pub trait SourceContainer {
    /// loads and returns the SourceEntry that contains the SourceCode and the path it was loaded from
    fn load_source(self, encoding: Option<&'static Encoding>) -> Result<SourceCode, String>;
    /// returns the location of this source-container. Used when reporting diagnostics.
    fn get_location(&self) -> &str;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FilePath {
    pub path: String,
}

impl From<String> for FilePath {
    fn from(it: String) -> Self {
        FilePath { path: it }
    }
}

impl From<&str> for FilePath {
    fn from(it: &str) -> Self {
        FilePath { path: it.into() }
    }
}

impl FilePath {
    fn get_extension(&self) -> &str {
        self.path.split('.').last().unwrap_or("")
    }

    fn is_object(&self) -> bool {
        self.get_extension() == "o"
    }
}

impl SourceContainer for FilePath {
    fn load_source(self, encoding: Option<&'static Encoding>) -> Result<SourceCode, String> {
        if self.is_object() {
            Err(format!("{} is not a source file", &self.path))
        } else {
            let mut file = File::open(&self.path).map_err(|err| err.to_string())?;
            let source = create_source_code(&mut file, encoding)?;

            Ok(SourceCode {
                source,
                path: self.path,
            })
        }
    }

    fn get_location(&self) -> &str {
        &self.path
    }
}

/// The SourceCode unit is the smallest unit of compilation that can be passed to the compiler
#[derive(Clone)]
pub struct SourceCode {
    /// the source code to be compiled
    pub source: String,
    /// the location this code was loaded from
    pub path: String,
}

/// tests can provide a SourceCode directly
impl SourceContainer for SourceCode {
    fn load_source(self, _: Option<&'static Encoding>) -> Result<SourceCode, String> {
        Ok(self)
    }

    fn get_location(&self) -> &str {
        &self.path
    }
}

impl From<&str> for SourceCode {
    fn from(src: &str) -> Self {
        SourceCode {
            source: src.into(),
            path: "external_file.st".into(),
        }
    }
}

impl From<String> for SourceCode {
    fn from(source: String) -> Self {
        SourceCode {
            source,
            path: "external_file.st".into(),
        }
    }
}

fn create_source_code<T: Read>(
    reader: &mut T,
    encoding: Option<&'static Encoding>,
) -> Result<String, String> {
    let mut buffer = String::new();
    let mut decoder = DecodeReaderBytesBuilder::new()
        .encoding(encoding)
        .build(reader);
    decoder
        .read_to_string(&mut buffer)
        .map_err(|err| format!("{:}", err))?;
    Ok(buffer)
}

pub fn get_target_triple(triple: Option<&str>) -> TargetTriple {
    triple
        .map(TargetTriple::create)
        .unwrap_or_else(TargetMachine::get_default_triple)
}

///
/// Compiles the given source into an object file and saves it in output
///
fn persist_to_obj(
    codegen: CodeGen,
    output: &str,
    reloc: RelocMode,
    triple: &TargetTriple,
    optimization: OptimizationLevel,
) -> Result<(), Diagnostic> {
    let initialization_config = &InitializationConfig::default();
    Target::initialize_all(initialization_config);

    let target = Target::from_triple(triple).map_err(|it| {
        Diagnostic::codegen_error(
            &format!("Invalid target-tripple '{:}' - {:?}", triple, it),
            SourceRange::undefined(),
        )
    })?;
    let machine = target
        .create_target_machine(
            triple,
            //TODO : Add cpu features as optionals
            "generic", //TargetMachine::get_host_cpu_name().to_string().as_str(),
            "",        //TargetMachine::get_host_cpu_features().to_string().as_str(),
            optimization.into(),
            reloc,
            CodeModel::Default,
        )
        .ok_or_else(|| {
            Diagnostic::codegen_error("Cannot create target machine.", SourceRange::undefined())
        });

    ////Run the passes
    machine.and_then(|it| {
        codegen
            .module
            .run_passes(optimization.opt_params(), &it, PassBuilderOptions::create())
            .map_err(|it| Diagnostic::llvm_error(output, &it))
            .and_then(|_| {
                it.write_to_file(&codegen.module, FileType::Object, Path::new(output))
                    .map_err(|it| Diagnostic::llvm_error(output, &it))
            })
    })
}

/// Persists a given LLVM module to a static object and saves the output.
///
/// # Arguments
///
/// * `codegen` - The generated LLVM module to persist
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn persist_as_static_obj(
    codegen: CodeGen,
    output: &str,
    target: &TargetTriple,
    optimization: OptimizationLevel,
) -> Result<(), Diagnostic> {
    persist_to_obj(codegen, output, RelocMode::Default, target, optimization)
}

/// Persists a given LLVM module to a shared postiion indepedent object and saves the output.
///
/// # Arguments
///
/// * `codegen` - The generated LLVM module to persist
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn persist_to_shared_pic_object(
    codegen: CodeGen,
    output: &str,
    target: &TargetTriple,
    optimization: OptimizationLevel,
) -> Result<(), Diagnostic> {
    persist_to_obj(codegen, output, RelocMode::PIC, target, optimization)
}

/// Persists the given LLVM module to a dynamic non PIC object and saves the output.
///
/// # Arguments
///
/// * `codegen` - The generated LLVM module to persits
/// * `output` - the location on disk to save the output
/// * `target` - llvm target triple
pub fn persist_to_shared_object(
    codegen: CodeGen,
    output: &str,
    target: &TargetTriple,
    optimization: OptimizationLevel,
) -> Result<(), Diagnostic> {
    persist_to_obj(
        codegen,
        output,
        RelocMode::DynamicNoPic,
        target,
        optimization,
    )
}

///
/// Persists the given LLVM module into a bitcode file
///
/// # Arguments
///
/// * `codegen` - the genated LLVM module to persist
/// * `output` - the location on disk to save the output
pub fn persist_to_bitcode(codegen: CodeGen, output: &str) -> Result<(), Diagnostic> {
    let path = Path::new(output);
    if codegen.module.write_bitcode_to_path(path) {
        Ok(())
    } else {
        Err(Diagnostic::codegen_error(
            "Could not write bitcode to file",
            SourceRange::undefined(),
        ))
    }
}

///
/// Persits the given LLVM module into LLVM IR and saves it to the given output location
///
/// # Arguments
///
/// * `codegen` - The generated LLVM module to be persisted
/// * `output`  - The location to save the generated ir file
pub fn persist_to_ir(codegen: CodeGen, output: &str) -> Result<(), Diagnostic> {
    let ir = codegen.module.print_to_string().to_string();
    fs::write(output, ir)
        .map_err(|err| Diagnostic::io_write_error(output, err.to_string().as_str()))
}

struct IndexComponents {
    id_provider: IdProvider,
    all_annotations: AnnotationMapImpl,
    all_literals: StringLiterals,
    annotated_units: Vec<CompilationUnit>,
}

fn index_module<T: SourceContainer>(
    sources: Vec<T>,
    includes: Vec<T>,
    encoding: Option<&'static Encoding>,
    mut diagnostician: Diagnostician,
) -> Result<(Index, IndexComponents), Diagnostic> {
    let mut full_index = Index::default();
    let id_provider = IdProvider::default();

    let mut all_units = Vec::new();

    // ### PHASE 1 ###
    // parse & index everything
    let (index, mut units) = parse_and_index(
        sources,
        encoding,
        &id_provider,
        &mut diagnostician,
        LinkageType::Internal,
    )?;
    full_index.import(index);
    all_units.append(&mut units);

    let (includes_index, mut includes_units) = parse_and_index(
        includes,
        encoding,
        &id_provider,
        &mut diagnostician,
        LinkageType::External,
    )?;
    full_index.import(includes_index);
    all_units.append(&mut includes_units);

    // ### PHASE 1.1 resolve constant literal values
    let (mut full_index, _unresolvables) =
        resolver::const_evaluator::evaluate_constants(full_index);

    // ### PHASE 2 ###
    // annotation & validation everything
    let mut annotated_units: Vec<CompilationUnit> = Vec::new();
    let mut all_annotations = AnnotationMapImpl::default();
    let mut all_literals = StringLiterals::default();
    for (file_id, syntax_errors, unit) in all_units.into_iter() {
        let (annotations, string_literals) = TypeAnnotator::visit_unit(&full_index, &unit);

        let mut validator = Validator::new();
        validator.visit_unit(&annotations, &full_index, &unit);
        //log errors
        diagnostician.handle(syntax_errors, file_id);
        diagnostician.handle(validator.diagnostics(), file_id);

        annotated_units.push(unit);
        all_annotations.import(annotations);
        all_literals.import(string_literals);
    }

    //Merge the new indices with the full index
    full_index.import(std::mem::take(&mut all_annotations.new_index));

    Ok((
        full_index,
        IndexComponents {
            id_provider,
            all_annotations,
            all_literals,
            annotated_units,
        },
    ))
}

///
/// Compiles the given source into a `codegen::CodeGen` using the provided context
///
/// # Arguments
///
/// * `context` - the LLVM Context to be used for the compilation
/// * `sources` - the source to be compiled
/// * `encoding` - The encoding to parse the files, None for UTF-8
pub fn compile_module<'c, T: SourceContainer>(
    context: &'c Context,
    sources: Vec<T>,
    includes: Vec<T>,
    encoding: Option<&'static Encoding>,
    diagnostician: Diagnostician,
) -> Result<(Index, CodeGen<'c>), Diagnostic> {
    let (full_index, mut index) = index_module(sources, includes, encoding, diagnostician)?;

    // ### PHASE 3 ###
    // - codegen
    let code_generator = codegen::CodeGen::new(context, "main");

    let annotations = AstAnnotations::new(index.all_annotations, index.id_provider.next_id());
    //Associate the index type with LLVM types
    let llvm_index =
        code_generator.generate_llvm_index(&annotations, index.all_literals, &full_index)?;
    for unit in index.annotated_units {
        code_generator.generate(&unit, &annotations, &full_index, &llvm_index)?;
    }

    Ok((full_index, code_generator))
}

type Units = Vec<(usize, Vec<Diagnostic>, CompilationUnit)>;
fn parse_and_index<T: SourceContainer>(
    source: Vec<T>,
    encoding: Option<&'static Encoding>,
    id_provider: &IdProvider,
    diagnostician: &mut Diagnostician,
    linkage: LinkageType,
) -> Result<(Index, Units), Diagnostic> {
    let mut index = Index::default();

    let mut units = Vec::new();

    //parse the builtins into the index
    let builtins = builtins::parse_built_ins(id_provider.clone());
    index.import(index::visitor::visit(&builtins, id_provider.clone()));

    for container in source {
        let location: String = container.get_location().into();
        let e = container
            .load_source(encoding)
            .map_err(|err| Diagnostic::io_read_error(location.as_str(), err.as_str()))?;

        let (mut parse_result, diagnostics) = parser::parse(
            lexer::lex_with_ids(e.source.as_str(), id_provider.clone()),
            linkage,
        );

        //pre-process the ast (create inlined types)
        ast::pre_process(&mut parse_result, id_provider.clone());
        //index the pou
        index.import(index::visitor::visit(&parse_result, id_provider.clone()));

        //register the file with the diagnstician, so diagnostics are later able to show snippets from the code
        let file_id = diagnostician.register_file(location.clone(), e.source);
        units.push((file_id, diagnostics, parse_result));
    }
    Ok((index, units))
}

fn create_file_paths(inputs: &[String]) -> Result<Vec<FilePath>, Diagnostic> {
    let mut sources = Vec::new();
    for input in inputs {
        let paths = glob(input).map_err(|e| {
            Diagnostic::param_error(&format!("Failed to read glob pattern: {}, ({})", input, e))
        })?;

        for p in paths {
            let path =
                p.map_err(|err| Diagnostic::param_error(&format!("Illegal path: {:}", err)))?;
            sources.push(FilePath {
                path: path.to_string_lossy().to_string(),
            });
        }
    }
    if sources.is_empty() {
        return Err(Diagnostic::param_error(&format!(
            "No such file(s): {}",
            inputs.join(",")
        )));
    }
    Ok(sources)
}

pub fn build_with_subcommand(parameters: CompileParameters) -> Result<(), Diagnostic> {
    let config_options = parameters
        .hardware_config
        .as_ref()
        .map(|config| ConfigurationOptions {
            format: parameters
                .config_format()
                .expect("Never none for valid parameters"),
            output: config.to_owned(),
        });

    match parameters.commands.unwrap() {
        SubCommands::Build {
            build_config,
            sysroot,
            target,
        } => {
            let project = get_project_from_file(build_config)?;
            let files = project.files;
            let compile_options = CompileOptions {
                output: project.output,
                target,
                format: project.compile_type,
                optimization: if project.optimization.is_some() {
                    project.optimization.unwrap()
                } else {
                    parameters.optimization
                },
            };

            let includes = if project.libraries.is_some() {
                string_to_filepath(
                    project
                        .libraries
                        .as_ref()
                        .unwrap()
                        .iter()
                        .flat_map(|it| it.include_path.clone())
                        .collect(),
                )
            } else {
                vec![]
            };

            let link_options = if let Some(format) = project.compile_type {
                Some(LinkOptions {
                    libraries: if project.libraries.is_some() {
                        project
                            .libraries
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|it| it.name.clone())
                            .collect()
                    } else {
                        vec![]
                    },
                    library_pathes: if project.libraries.is_some() {
                        project
                            .libraries
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|it| it.path.clone())
                            .collect()
                    } else {
                        vec![]
                    },
                    sysroot,
                    format,
                })
            } else {
                None
            };

            let target = get_target_triple(compile_options.target.as_deref());

            let compile_result = build(
                string_to_filepath(files),
                includes,
                &compile_options,
                parameters.encoding,
                &project.error_format,
                &target,
            )?;

            link_and_create(
                link_options,
                &compile_result,
                &compile_options,
                &target,
                config_options,
            )?;
        }
    }
    Ok(())
}

fn link_and_create(
    link_options: Option<LinkOptions>,
    compile_result: &CompileResult,
    compile_options: &CompileOptions,
    target: &TargetTriple,
    config_options: Option<ConfigurationOptions>,
) -> Result<(), Diagnostic> {
    if let Some(link_options) = link_options {
        link(
            &compile_options.output,
            link_options.format,
            &compile_result.objects,
            link_options.library_pathes,
            link_options.libraries,
            target,
            link_options.sysroot,
        )?;
    }

    if let Some(config) = config_options {
        let hw_config = hardware_binding::collect_hardware_configuration(&compile_result.index)?;
        let generated_conf =
            hardware_binding::generate_hardware_configuration(&hw_config, config.format)?;

        File::create(config.output)
            .and_then(|mut it| it.write_all(generated_conf.as_bytes()))
            .map_err(|it| Diagnostic::GeneralError {
                err_no: diagnostics::ErrNo::general__io_err,
                message: it.to_string(),
            })?;
    }
    Ok(())
}

/// The driver function for the compilation
/// Sorts files that need compilation
/// Parses, validates and generates code for the given source files
/// Links all provided object files with the compilation result
/// Links any provided libraries
/// Returns the location of the output file
pub fn build_with_params(parameters: CompileParameters) -> Result<(), Diagnostic> {
    let includes = if parameters.includes.is_empty() {
        vec![]
    } else {
        create_file_paths(&parameters.includes)?
    };
    let output = parameters
        .output_name()
        .ok_or_else(|| Diagnostic::param_error("Missing parameter: output-name"))?;
    let out_format = parameters.output_format_or_default();

    let config_options = parameters
        .hardware_config
        .as_ref()
        .map(|config| ConfigurationOptions {
            format: parameters
                .config_format()
                .expect("Never none for valid parameters"),
            output: config.to_owned(),
        });

    let compile_options = CompileOptions {
        output,
        target: parameters.target,
        format: out_format,
        optimization: parameters.optimization,
    };

    let error_format = parameters.error_format;

    let files = create_file_paths(&parameters.input)?;

    let link_options = if let Some(format) = out_format {
        if !parameters.skip_linking {
            Some(LinkOptions {
                libraries: parameters.libraries,
                library_pathes: parameters.library_paths,
                sysroot: parameters.sysroot,
                format,
            })
        } else {
            None
        }
    } else {
        None
    };

    let target = get_target_triple(compile_options.target.as_deref());

    let compile_result = build(
        files,
        includes,
        &compile_options,
        parameters.encoding,
        &error_format,
        &target,
    )?;

    link_and_create(
        link_options,
        &compile_result,
        &compile_options,
        &target,
        config_options,
    )?;

    Ok(())
}

/// The builder function for the compilation
/// Sorts files that need compilation
/// Parses, validates and generates code for the given source files
/// Persists the generated code to output location
/// Returns a compilation result with the index, and a list of object files
pub fn build(
    files: Vec<FilePath>,
    includes: Vec<FilePath>,
    compile_options: &CompileOptions,
    encoding: Option<&'static Encoding>,
    error_format: &ErrorFormat,
    target: &TargetTriple,
) -> Result<CompileResult, Diagnostic> {
    let mut objects = vec![];
    let mut sources = vec![];
    files.into_iter().for_each(|it| {
        if it.is_object() {
            objects.push(it);
        } else {
            sources.push(it);
        }
    });

    let context = Context::create();
    let diagnostician = match error_format {
        ErrorFormat::Rich => Diagnostician::default(),
        ErrorFormat::Clang => Diagnostician::clang_format_diagnostician(),
    };

    let index = match compile_options.format {
        None => index_module(sources, includes, encoding, diagnostician)?.0,
        Some(out_format) => {
            let (index, codegen) =
                compile_module(&context, sources, includes, encoding, diagnostician)?;

            objects.push(persist(
                codegen,
                &compile_options.output,
                out_format,
                target,
                compile_options.optimization,
            )?);

            index
        }
    };

    Ok(CompileResult { index, objects })
}

pub fn persist(
    input: codegen::CodeGen,
    output: &str,
    out_format: FormatOption,
    target: &TargetTriple,
    optimization: OptimizationLevel,
) -> Result<FilePath, Diagnostic> {
    match out_format {
        FormatOption::Static | FormatOption::Relocatable => {
            persist_as_static_obj(input, output, target, optimization)
        }
        FormatOption::Shared => persist_to_shared_object(input, output, target, optimization),
        FormatOption::PIC => persist_to_shared_pic_object(input, output, target, optimization),
        FormatOption::Bitcode => persist_to_bitcode(input, output),
        FormatOption::IR => persist_to_ir(input, output),
    }?;

    Ok(output.into())
}

pub fn link(
    output: &str,
    out_format: FormatOption,
    objects: &[FilePath],
    library_pathes: Vec<String>,
    libraries: Vec<String>,
    target: &TargetTriple,
    sysroot: Option<String>,
) -> Result<(), Diagnostic> {
    let linkable_formats = vec![
        FormatOption::Static,
        FormatOption::Relocatable,
        FormatOption::Shared,
        FormatOption::PIC,
    ];
    if linkable_formats.contains(&out_format) {
        let mut linker = target
            .as_str()
            .to_str()
            .map_err(|e| Diagnostic::param_error(&e.to_string()))
            .and_then(|triple| linker::Linker::new(triple).map_err(|e| e.into()))?;
        linker.add_lib_path(".");

        for path in objects {
            linker.add_obj(&path.path);
        }

        for path in &library_pathes {
            linker.add_lib_path(path);
        }
        for library in &libraries {
            linker.add_lib(library);
        }

        if let Some(sysroot) = &sysroot {
            linker.add_sysroot(sysroot);
        }

        match out_format {
            FormatOption::Static => linker.build_exectuable(Path::new(&output))?,
            FormatOption::Relocatable => linker.build_relocatable(Path::new(&output))?,
            _ => linker.build_shared_obj(Path::new(&output))?,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    mod adr;
    mod external_files;
    mod multi_files;

    use inkwell::targets::TargetMachine;

    use crate::{create_source_code, get_target_triple};

    #[test]
    fn test_get_target_triple() {
        let triple = get_target_triple(None);
        assert_eq!(
            triple.as_str().to_str().unwrap(),
            TargetMachine::get_default_triple()
                .as_str()
                .to_str()
                .unwrap()
        );

        let triple = get_target_triple(Some("x86_64-pc-linux-gnu"));
        assert_eq!(triple.as_str().to_str().unwrap(), "x86_64-pc-linux-gnu");
    }

    #[test]
    fn windows_encoded_file_content_read() {
        let expected = r"PROGRAM ä
(* Cöment *)
END_PROGRAM
";
        let mut source = &b"\x50\x52\x4f\x47\x52\x41\x4d\x20\xe4\x0a\x28\x2a\x20\x43\xf6\x6d\x65\x6e\x74\x20\x2a\x29\x0a\x45\x4e\x44\x5f\x50\x52\x4f\x47\x52\x41\x4d\x0a"[..];
        // let read = std::io::Read()
        let source = create_source_code(&mut source, Some(encoding_rs::WINDOWS_1252)).unwrap();

        assert_eq!(expected, &source);
    }

    #[test]
    fn utf_16_encoded_file_content_read() {
        let expected = r"PROGRAM ä
(* Cömment *)
END_PROGRAM
";

        let mut source = &b"\xff\xfe\x50\x00\x52\x00\x4f\x00\x47\x00\x52\x00\x41\x00\x4d\x00\x20\x00\xe4\x00\x0a\x00\x28\x00\x2a\x00\x20\x00\x43\x00\xf6\x00\x6d\x00\x6d\x00\x65\x00\x6e\x00\x74\x00\x20\x00\x2a\x00\x29\x00\x0a\x00\x45\x00\x4e\x00\x44\x00\x5f\x00\x50\x00\x52\x00\x4f\x00\x47\x00\x52\x00\x41\x00\x4d\x00\x0a\x00" [..];

        let source = create_source_code(&mut source, None).unwrap();
        assert_eq!(expected, &source);
    }

    #[test]
    fn utf_8_encoded_file_content_read() {
        let expected = r"PROGRAM ä
(* Cöment *)
END_PROGRAM
";

        let mut source = &b"\x50\x52\x4f\x47\x52\x41\x4d\x20\xc3\xa4\x0a\x28\x2a\x20\x43\xc3\xb6\x6d\x65\x6e\x74\x20\x2a\x29\x0a\x45\x4e\x44\x5f\x50\x52\x4f\x47\x52\x41\x4d\x0a" [..];
        let source = create_source_code(&mut source, None).unwrap();
        assert_eq!(expected, &source);
    }
}
