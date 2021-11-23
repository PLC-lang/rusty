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
use std::ops::Range;
use std::path::Path;

use ast::{DataTypeDeclaration, PouType, SourceRange};
use codespan_reporting::diagnostic::{self, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{self, Chars, Styles};
use compile_error::CompileError;
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use index::Index;
use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use lexer::IdProvider;
use std::{fs::File, io::Read};
use validation::Validator;

use crate::ast::CompilationUnit;
use crate::resolver::{AnnotationMap, TypeAnnotator};
mod ast;
pub mod cli;
mod codegen;
pub mod compile_error;
pub mod index;
mod lexer;
mod parser;
mod resolver;
mod test_utils;
mod typesystem;
mod validation;

#[macro_use]
#[cfg(test)]
extern crate pretty_assertions;

#[derive(PartialEq, Debug, Clone)]
pub enum Diagnostic {
    SyntaxError {
        message: String,
        range: SourceRange,
        err_no: ErrNo,
    },
    ImprovementSuggestion {
        message: String,
        range: SourceRange,
    },
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Clone)]
pub enum ErrNo {
    undefined,

    //syntax
    syntax__generic_error,
    syntax__missing_token,
    syntax__unexpected_token,

    //semantic
    // pou related
    pou__missing_return_type,
    pou__unexpected_return_type,
    pou__unsupported_return_type,
    pou__empty_variable_block,

    //variable related
    var__unresolved_constant,
    var__invalid_constant_block,
    var__invalid_constant,
    var__cannot_assign_to_const,
    var__invalid_assignment,

    //reference related
    reference__unresolved,

    //type related
    type__literal_out_of_range,
    type__incompatible_literal_cast,
    type__incompatible_directaccess,
    type__incompatible_directaccess_variable,
    type__incompatible_directaccess_range,
    type__expected_literal,
}

impl Diagnostic {
    pub fn syntax_error(message: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: message.to_string(),
            range,
            err_no: ErrNo::syntax__generic_error,
        }
    }

    pub fn unexpected_token_found(expected: &str, found: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Unexpected token: expected {} but found {}",
                expected, found
            ),
            range,
            err_no: ErrNo::syntax__unexpected_token,
        }
    }

    pub fn unexpected_initializer_on_function_return(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Return types cannot have a default value".into(),
            range,
            err_no: ErrNo::syntax__unexpected_token,
        }
    }

    pub fn return_type_not_supported(pou_type: &PouType, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "POU Type {:?} does not support a return type. Did you mean Function?",
                pou_type
            ),
            range,
            err_no: ErrNo::pou__unexpected_return_type,
        }
    }

    pub fn function_unsupported_return_type(data_type: &DataTypeDeclaration) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Data Type {:?} not supported as a function return type!",
                data_type
            ),
            range: data_type.get_location(),
            err_no: ErrNo::pou__unsupported_return_type,
        }
    }

    pub fn function_return_missing(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Function Return type missing".into(),
            range,
            err_no: ErrNo::pou__missing_return_type,
        }
    }

    pub fn missing_token(epxected_token: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Missing expected Token {}", epxected_token),
            range,
            err_no: ErrNo::syntax__missing_token,
        }
    }

    pub fn missing_action_container(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Missing Actions Container Name".to_string(),
            range,
            err_no: ErrNo::undefined,
        }
    }

    pub fn unrseolved_reference(reference: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Could not resolve reference to {:}", reference),
            range: location,
            err_no: ErrNo::reference__unresolved,
        }
    }

    pub fn incompatible_directaccess(
        access_type: &str,
        access_size: u64,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "{}-Wise access requires a Numerical type larger than {} bits",
                access_type, access_size
            ),
            range: location,
            err_no: ErrNo::type__incompatible_directaccess,
        }
    }

    pub fn incompatible_directaccess_range(
        access_type: &str,
        target_type: &str,
        access_range: Range<u64>,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "{}-Wise access for type {} must be in the range {}..{}",
                access_type, target_type, access_range.start, access_range.end
            ),
            range: location,
            err_no: ErrNo::type__incompatible_directaccess_range,
        }
    }

    pub fn incompatible_directaccess_variable(
        access_type: &str,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid type {} for direct variable access. Only variables of Integer types are allowed",
                access_type
            ),
            range: location,
            err_no: ErrNo::type__incompatible_directaccess_variable,
        }
    }

    pub fn incompatible_literal_cast(
        cast_type: &str,
        literal_type: &str,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Literal {:} is not campatible to {:}",
                literal_type, cast_type
            ),
            range: location,
            err_no: ErrNo::type__incompatible_literal_cast,
        }
    }

    pub fn literal_expected(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Expected literal".into(),
            range: location,
            err_no: ErrNo::type__expected_literal,
        }
    }

    pub fn literal_out_of_range(
        literal: &str,
        range_hint: &str,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Literal {:} out of range ({})", literal, range_hint),
            range: location,
            err_no: ErrNo::type__literal_out_of_range,
        }
    }

    pub fn empty_variable_block(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Variable block is empty".into(),
            range: location,
            err_no: ErrNo::pou__empty_variable_block,
        }
    }

    pub fn unresolved_constant(
        constant_name: &str,
        reason: Option<&str>,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Unresolved constant '{:}' variable{:}",
                constant_name,
                reason
                    .map(|it| format!(": {:}", it))
                    .unwrap_or_else(|| "".into()),
            ),
            range: location,
            err_no: ErrNo::pou__empty_variable_block,
        }
    }

    pub fn invalid_constant_block(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "This variable block does not support the CONSTANT modifier".to_string(),
            range: location,
            err_no: ErrNo::var__invalid_constant_block,
        }
    }

    pub fn invalid_constant(constant_name: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Invalid constant {:} - Functionblock- and Class-instances cannot be delcared constant", constant_name),
            range: location,
            err_no: ErrNo::var__invalid_constant,
        }
    }

    pub fn cannot_assign_to_constant(qualified_name: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Cannot assign to CONSTANT '{:}'", qualified_name),
            range: location,
            err_no: ErrNo::var__cannot_assign_to_const,
        }
    }

    pub fn invalid_assignment(
        right_type: &str,
        left_type: &str,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid assignment: cannot assign '{:}' to '{:}'",
                right_type, left_type
            ),
            range: location,
            err_no: ErrNo::var__invalid_assignment,
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            Diagnostic::SyntaxError { message, .. } => message.as_str(),
            Diagnostic::ImprovementSuggestion { message, .. } => message.as_str(),
        }
    }

    pub fn get_location(&self) -> SourceRange {
        match self {
            Diagnostic::SyntaxError { range, .. } => range.clone(),
            Diagnostic::ImprovementSuggestion { range, .. } => range.clone(),
        }
    }
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

impl SourceContainer for FilePath {
    fn load_source(self, encoding: Option<&'static Encoding>) -> Result<SourceCode, String> {
        let mut file = File::open(&self.path).map_err(|err| err.to_string())?;
        let source = create_source_code(&mut file, encoding)?;

        Ok(SourceCode {
            source,
            path: self.path,
        })
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
        SourceCode { source: src.into(), path: "<undefined>".into() }
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
        .map(|it| TargetTriple::create(it.as_str()))
        .unwrap_or_else(TargetMachine::get_default_triple)
}

///
/// Compiles the given source into an object file and saves it in output
///
fn compile_to_obj<T: SourceContainer>(
    sources: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    reloc: RelocMode,
    triple: TargetTriple,
) -> Result<(), CompileError> {
    let initialization_config = &InitializationConfig::default();
    Target::initialize_all(initialization_config);

    let target = Target::from_triple(&triple).map_err(|it| {
        CompileError::codegen_error(
            format!("Invalid target-tripple '{:}' - {:?}", triple, it),
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
            CompileError::codegen_error(
                "Cannot create target machine.".into(),
                SourceRange::undefined(),
            )
        });

    let c = Context::create();
    let code_generator = compile_module(&c, sources, encoding)?;
    machine.and_then(|it| {
        it.write_to_file(&code_generator.module, FileType::Object, Path::new(output))
            .map_err(|it| it.into())
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
    encoding: Option<&'static Encoding>,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(
        sources,
        encoding,
        output,
        RelocMode::Default,
        get_target_triple(target),
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
    encoding: Option<&'static Encoding>,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(
        sources,
        encoding,
        output,
        RelocMode::PIC,
        get_target_triple(target),
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
    encoding: Option<&'static Encoding>,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(
        sources,
        encoding,
        output,
        RelocMode::DynamicNoPic,
        get_target_triple(target),
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
    encoding: Option<&'static Encoding>,
    output: &str,
) -> Result<(), CompileError> {
    let path = Path::new(output);
    let c = Context::create();
    let code_generator = compile_module(&c, sources, encoding)?;
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
    encoding: Option<&'static Encoding>,
    output: &str,
) -> Result<(), CompileError> {
    let ir = compile_to_string(sources, encoding)?; 
    fs::write(output, ir)
        .map_err(|err| CompileError::io_write_error(output.into(), err.to_string()))
}

///
/// Compiles the given source into LLVM IR and returns it
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `encoding` - The encoding to parse the files, None for UTF-8
pub fn compile_to_string<T: SourceContainer> (
    sources: Vec<T>,
    encoding: Option<&'static Encoding>,
) -> Result<String, CompileError> {
    let c = Context::create();
    let code_gen = compile_module(&c, sources, encoding)?;
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
    encoding: Option<&'static Encoding>,
) -> Result<codegen::CodeGen<'c>, CompileError> {
    let mut full_index = Index::new();
    let id_provider = IdProvider::default();
    let mut files: SimpleFiles<String, String> = SimpleFiles::new();

    let mut all_units = Vec::new();

    // ### PHASE 1 ###
    // parse & index everything
    for container in sources {
        let location: String = container.get_location().into();
        let e = container
            .load_source(encoding)
            .map_err(|err| CompileError::io_read_error(err, location.clone()))?;
        let file_id = files.add(location.clone(), e.source.clone());

        let (mut parse_result, diagnostics) =
            parser::parse(lexer::lex_with_ids(e.source.as_str(), id_provider.clone()));

        //pre-process the ast (create inlined types)
        ast::pre_process(&mut parse_result);
        //index the pou
        full_index.import(index::visitor::visit(&parse_result, id_provider.clone()));
        all_units.push((file_id, diagnostics, parse_result));
    }

    // ### PHASE 1.1 resolve constant literal values
    let (full_index, _unresolvables) = resolver::const_evaluator::evaluate_constants(full_index);

    // ### PHASE 2 ###
    // annotation & validation everything
    type AnnotatedAst<'a> = (&'a CompilationUnit, AnnotationMap);
    let mut annotated_units: Vec<AnnotatedAst> = Vec::new();
    for (file_id, syntax_errors, unit) in all_units.iter() {
        let annotations = TypeAnnotator::visit_unit(&full_index, unit);

        let mut validator = Validator::new();
        validator.visit_unit(&annotations, &full_index, unit);
        //log errors
        report_diagnostics(*file_id, syntax_errors.iter(), &files)?;
        report_diagnostics(*file_id, validator.diagnostics().iter(), &files)?;

        annotated_units.push((unit, annotations));
    }

    // ### PHASE 3 ###
    // - codegen
    let code_generator = codegen::CodeGen::new(context, "main");

    for (unit, annotations) in annotated_units {
        code_generator.generate(unit, &annotations, &full_index)?;
    }
    Ok(code_generator)
}

fn report_diagnostics(
    file_id: usize,
    semantic_diagnostics: std::slice::Iter<Diagnostic>,
    files: &SimpleFiles<String, String>,
) -> Result<(), CompileError> {
    for error in semantic_diagnostics {
        let diag = diagnostic::Diagnostic::error()
            .with_message(error.get_message())
            .with_labels(vec![Label::primary(
                file_id,
                error.get_location().get_start()..error.get_location().get_end(),
            )]);
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config {
            display_style: term::DisplayStyle::Rich,
            tab_width: 2,
            styles: Styles::default(),
            chars: Chars::default(),
            start_context_lines: 5,
            end_context_lines: 3,
        };

        term::emit(&mut writer.lock(), &config, files, &diag).map_err(|err| {
            CompileError::codegen_error(
                format!("Cannot print errors {:#?}", err),
                SourceRange::undefined(),
            )
        })?;
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
