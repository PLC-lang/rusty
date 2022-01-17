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

use glob::glob;
use std::path::Path;

use ast::{PouType, SourceRange};
use cli::CompileParameters;
use diagnostics::Diagnostic;
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use index::Index;
use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use lexer::IdProvider;
use resolver::AstAnnotations;
use std::{fs::File, io::Read};
use validation::Validator;

use crate::ast::CompilationUnit;
use crate::diagnostics::Diagnostician;
use crate::resolver::{AnnotationMapImpl, TypeAnnotator};
mod ast;
pub mod cli;
mod codegen;
pub mod diagnostics;
pub mod index;
mod lexer;
mod linker;
mod parser;
mod resolver;
mod test_utils;
mod typesystem;
mod validation;

#[macro_use]
#[cfg(test)]
extern crate pretty_assertions;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum FormatOption {
    Static,
    PIC,
    Shared,
    Relocatable,
    Bitcode,
    IR,
}

pub struct CompileOptions {
    pub format: FormatOption,
    pub output: String,
    pub target: Option<String>,
}

pub struct LinkOptions {
    pub libraries: Vec<String>,
    pub library_pathes: Vec<String>,
    pub sysroot: Option<String>,
}

/// SourceContainers offer source-code to be compiled via the load_source function.
/// Furthermore it offers a location-String used when reporting diagnostics.
pub trait SourceContainer {
    /// loads and returns the SourceEntry that contains the SourceCode and the path it was loaded from
    fn load_source(self, encoding: Option<&'static Encoding>) -> Result<SourceCode, String>;
    /// returns the location of this source-container. Used when reporting diagnostics.
    fn get_location(&self) -> &str;
}

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

pub fn get_target_triple(triple: Option<String>) -> TargetTriple {
    triple
        .as_ref()
        .map(|it| TargetTriple::create(it))
        .unwrap_or_else(TargetMachine::get_default_triple)
}

///
/// Compiles the given source into an object file and saves it in output
///
fn compile_to_obj<T: SourceContainer>(
    sources: Vec<T>,
    includes: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    reloc: RelocMode,
    triple: TargetTriple,
    diagnostician: Diagnostician,
) -> Result<(), Diagnostic> {
    let initialization_config = &InitializationConfig::default();
    Target::initialize_all(initialization_config);

    let target = Target::from_triple(&triple).map_err(|it| {
        Diagnostic::codegen_error(
            &format!("Invalid target-tripple '{:}' - {:?}", triple, it),
            SourceRange::undefined(),
        )
    })?;
    let machine = target
        .create_target_machine(
            &triple,
            //TODO : Add cpu features as optionals
            "generic", //TargetMachine::get_host_cpu_name().to_string().as_str(),
            "",        //TargetMachine::get_host_cpu_features().to_string().as_str(),
            //TODO Optimisation as parameter
            inkwell::OptimizationLevel::Default,
            reloc,
            CodeModel::Default,
        )
        .ok_or_else(|| {
            Diagnostic::codegen_error("Cannot create target machine.", SourceRange::undefined())
        });

    let c = Context::create();
    let code_generator = compile_module(&c, sources, includes, encoding, diagnostician)?;
    machine.and_then(|it| {
        it.write_to_file(&code_generator.module, FileType::Object, Path::new(output))
            .map_err(|it| Diagnostic::llvm_error(output, &it))
    })
}

/// Compiles a given source string to a static object and saves the output.
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `encoding` - The encoding to parse the files, None for UTF-8
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_static_obj<T: SourceContainer>(
    sources: Vec<T>,
    includes: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    target: Option<String>,
    diagnostician: Diagnostician,
) -> Result<(), Diagnostic> {
    compile_to_obj(
        sources,
        includes,
        encoding,
        output,
        RelocMode::Default,
        get_target_triple(target),
        diagnostician,
    )
}

/// Compiles a given source string to a shared position independent object and saves the output.
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `encoding` - The encoding to parse the files, None for UTF-8
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_shared_pic_object<T: SourceContainer>(
    sources: Vec<T>,
    includes: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    target: Option<String>,
    diagnostician: Diagnostician,
) -> Result<(), Diagnostic> {
    compile_to_obj(
        sources,
        includes,
        encoding,
        output,
        RelocMode::PIC,
        get_target_triple(target),
        diagnostician,
    )
}

/// Compiles a given source string to a dynamic non PIC object and saves the output.
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `encoding` - The encoding to parse the files, None for UTF-8
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_shared_object<T: SourceContainer>(
    sources: Vec<T>,
    includes: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    target: Option<String>,
    diagnostician: Diagnostician,
) -> Result<(), Diagnostic> {
    compile_to_obj(
        sources,
        includes,
        encoding,
        output,
        RelocMode::DynamicNoPic,
        get_target_triple(target),
        diagnostician,
    )
}

///
/// Compiles the given source into a bitcode file
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `output` - the location on disk to save the output
pub fn compile_to_bitcode<T: SourceContainer>(
    sources: Vec<T>,
    includes: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    diagnostician: Diagnostician,
) -> Result<(), Diagnostic> {
    let path = Path::new(output);
    let c = Context::create();
    let code_generator = compile_module(&c, sources, includes, encoding, diagnostician)?;
    code_generator.module.write_bitcode_to_path(path);
    Ok(())
}

///
/// Compiles the given source into LLVM IR and saves it to the given output location
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `encoding` - The encoding to parse the files, None for UTF-8
/// * `output`  - The location to save the generated ir file
pub fn compile_to_ir<T: SourceContainer>(
    sources: Vec<T>,
    includes: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    diagnostician: Diagnostician,
) -> Result<(), Diagnostic> {
    let ir = compile_to_string(sources, includes, encoding, diagnostician)?;
    fs::write(output, ir)
        .map_err(|err| Diagnostic::io_write_error(output, err.to_string().as_str()))
}

///
/// Compiles the given source into LLVM IR and returns it
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `encoding` - The encoding to parse the files, None for UTF-8
pub fn compile_to_string<T: SourceContainer>(
    sources: Vec<T>,
    includes: Vec<T>,
    encoding: Option<&'static Encoding>,
    diagnostician: Diagnostician,
) -> Result<String, Diagnostic> {
    let c = Context::create();
    let code_gen = compile_module(&c, sources, includes, encoding, diagnostician)?;
    Ok(code_gen.module.print_to_string().to_string())
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
    mut diagnostician: Diagnostician,
) -> Result<codegen::CodeGen<'c>, Diagnostic> {
    let mut full_index = Index::new();
    let mut id_provider = IdProvider::default();

    let mut all_units = Vec::new();

    // ### PHASE 1 ###
    // parse & index everything
    for container in sources {
        let location: String = container.get_location().into();
        let e = container
            .load_source(encoding)
            .map_err(|err| Diagnostic::io_read_error(location.as_str(), err.as_str()))?;

        let (mut parse_result, diagnostics) = parser::parse(
            lexer::lex_with_ids(e.source.as_str(), id_provider.clone()),
            ast::LinkageType::Internal,
        );

        //pre-process the ast (create inlined types)
        ast::pre_process(&mut parse_result, id_provider.clone());
        //index the pou
        full_index.import(index::visitor::visit(&parse_result, id_provider.clone()));

        //register the file with the diagnstician, so diagnostics are later able to show snippets from the code
        let file_id = diagnostician.register_file(location.clone(), e.source);
        all_units.push((file_id, diagnostics, parse_result));
    }

    // includes TODO: refactor only test for now
    // parse & index everything
    for container in includes {
        let location: String = container.get_location().into();
        let e = container
            .load_source(encoding)
            .map_err(|err| Diagnostic::io_read_error(location.as_str(), err.as_str()))?;

        let (mut parse_result, diagnostics) = parser::parse(
            lexer::lex_with_ids(e.source.as_str(), id_provider.clone()),
            ast::LinkageType::External,
        );

        //pre-process the ast (create inlined types)
        ast::pre_process(&mut parse_result, id_provider.clone());
        //index the pou
        full_index.import(index::visitor::visit(&parse_result, id_provider.clone()));

        //register the file with the diagnstician, so diagnostics are later able to show snippets from the code
        let file_id = diagnostician.register_file(location.clone(), e.source);
        all_units.push((file_id, diagnostics, parse_result));
    }

    // ### PHASE 1.1 resolve constant literal values
    let (mut full_index, _unresolvables) =
        resolver::const_evaluator::evaluate_constants(full_index);

    // ### PHASE 2 ###
    // annotation & validation everything
    let mut annotated_units: Vec<CompilationUnit> = Vec::new();
    let mut all_annotations = AnnotationMapImpl::default();
    for (file_id, syntax_errors, unit) in all_units.into_iter() {
        let annotations = TypeAnnotator::visit_unit(&full_index, &unit);

        let mut validator = Validator::new();
        validator.visit_unit(&annotations, &full_index, &unit);
        //log errors
        diagnostician.handle(syntax_errors, file_id);
        diagnostician.handle(validator.diagnostics(), file_id);

        annotated_units.push(unit);
        all_annotations.import(annotations);
    }

    //Merge the new indices with the full index
    full_index.import(std::mem::take(&mut all_annotations.new_index));

    // ### PHASE 3 ###
    // - codegen
    let code_generator = codegen::CodeGen::new(context, "main");

    let annotations = AstAnnotations::new(all_annotations, id_provider.next_id());
    //Associate the index type with LLVM types
    let llvm_index = code_generator.generate_llvm_index(&annotations, &full_index)?;
    for unit in annotated_units {
        code_generator.generate(&unit, &annotations, &full_index, &llvm_index)?;
    }
    Ok(code_generator)
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

/// The driver function for the compilation
/// Sorts files that need compilation
/// Parses, validates and generates code for the given source files
/// Links all provided object files with the compilation result
/// Links any provided libraries
/// Returns the location of the output file
pub fn build_with_params(parameters: CompileParameters) -> Result<(), Diagnostic> {
    let files = create_file_paths(&parameters.input)?;
    let includes = if parameters.includes.is_empty() {
        vec![]
    } else {
        create_file_paths(&parameters.includes)?
    };
    let output = parameters
        .output_name()
        .ok_or_else(|| Diagnostic::param_error("Missing parameter: output-name"))?;
    let out_format = parameters.output_format_or_default();
    let compile_options = CompileOptions {
        output,
        target: parameters.target,
        format: out_format,
    };
    let link_options = if !parameters.skip_linking {
        Some(LinkOptions {
            libraries: parameters.libraries,
            library_pathes: parameters.library_pathes,
            sysroot: parameters.sysroot,
        })
    } else {
        None
    };

    build(
        files,
        includes,
        compile_options,
        link_options,
        parameters.encoding,
    )
}

/// The driver function for the compilation
/// Sorts files that need compilation
/// Parses, validates and generates code for the given source files
/// Links all provided object files with the compilation result
/// Links any provided libraries
/// Returns the location of the output file
pub fn build(
    files: Vec<FilePath>,
    includes: Vec<FilePath>,
    compile_options: CompileOptions,
    link_options: Option<LinkOptions>,
    encoding: Option<&'static Encoding>,
) -> Result<(), Diagnostic> {
    let mut objects = vec![];
    let mut sources = vec![];
    files.into_iter().for_each(|it| {
        if it.is_object() {
            objects.push(it);
        } else {
            sources.push(it);
        }
    });

    if !sources.is_empty() {
        compile(
            &compile_options.output,
            compile_options.format,
            sources,
            includes,
            encoding,
            compile_options.target.clone(),
        )?;
        objects.push(compile_options.output.as_str().into());
    }

    if let Some(link_options) = link_options {
        link(
            &compile_options.output,
            compile_options.format,
            objects,
            link_options.library_pathes,
            link_options.libraries,
            compile_options.target,
            link_options.sysroot,
        )?;
    }

    Ok(())
}

pub fn compile(
    output: &str,
    out_format: FormatOption,
    sources: Vec<FilePath>,
    includes: Vec<FilePath>,
    encoding: Option<&'static Encoding>,
    target: Option<String>,
) -> Result<(), Diagnostic> {
    let diagnostician = Diagnostician::default();
    match out_format {
        FormatOption::Static | FormatOption::Relocatable => {
            compile_to_static_obj(sources, includes, encoding, output, target, diagnostician)
        }
        FormatOption::Shared => {
            compile_to_shared_object(sources, includes, encoding, output, target, diagnostician)
        }
        FormatOption::PIC => {
            compile_to_shared_pic_object(sources, includes, encoding, output, target, diagnostician)
        }
        FormatOption::Bitcode => {
            compile_to_bitcode(sources, includes, encoding, output, diagnostician)
        }
        FormatOption::IR => compile_to_ir(sources, includes, encoding, output, diagnostician),
    }?;
    Ok(())
}

pub fn link(
    output: &str,
    out_format: FormatOption,
    objects: Vec<FilePath>,
    library_pathes: Vec<String>,
    libraries: Vec<String>,
    target: Option<String>,
    sysroot: Option<String>,
) -> Result<(), Diagnostic> {
    let linkable_formats = vec![
        FormatOption::Static,
        FormatOption::Relocatable,
        FormatOption::Shared,
        FormatOption::PIC,
    ];
    if linkable_formats.contains(&out_format) {
        let triple = get_target_triple(target);
        let mut linker = triple
            .as_str()
            .to_str()
            .map_err(|e| Diagnostic::param_error(&e.to_string()))
            .and_then(|triple| linker::Linker::new(triple).map_err(|e| e.into()))?;
        linker.add_lib_path(".");

        for path in &objects {
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

        let triple = get_target_triple(Some("x86_64-pc-linux-gnu".into()));
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
