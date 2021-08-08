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
use std::path::Path;

use ast::{PouType, SourceRange};
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
use parser::ParsedAst;
use resolver::TypeAnnotator;
use std::{fs::File, io::Read};

use crate::ast::CompilationUnit;
mod ast;
pub mod cli;
mod codegen;
pub mod compile_error;
pub mod index;
mod lexer;
mod parser;
mod resolver;
mod typesystem;

#[macro_use]
extern crate pretty_assertions;

#[derive(PartialEq, Debug, Clone)]
pub enum Diagnostic {
    SyntaxError { message: String, range: SourceRange },
    ImprovementSuggestion { message: String, range: SourceRange },
}

impl Diagnostic {
    pub fn syntax_error(message: String, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError { message, range }
    }

    pub fn unexpected_token_found(
        expected: String,
        found: String,
        range: SourceRange,
    ) -> Diagnostic {
        Diagnostic::syntax_error(
            format!(
                "Unexpected token: expected {} but found {}",
                expected, found
            ),
            range,
        )
    }

    pub fn return_type_not_supported(pou_type: &PouType, range: SourceRange) -> Diagnostic {
        Diagnostic::syntax_error(
            format!(
                "POU Type {:?} does not support a return type. Did you mean Function?",
                pou_type
            ),
            range,
        )
    }

    pub fn missing_token(epxected_token: String, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Missing expected Token {}", epxected_token),
            range,
        }
    }

    pub fn missing_action_container(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Missing Actions Container Name".to_string(),
            range,
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

///
/// Compiles the given source into an object file and saves it in output
///
fn compile_to_obj<T: SourceContainer>(
    sources: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    reloc: RelocMode,
    triple: Option<String>,
) -> Result<(), CompileError> {
    let initialization_config = &InitializationConfig::default();
    Target::initialize_all(initialization_config);

    let triple = triple
        .map(|it| TargetTriple::create(it.as_str()))
        .or_else(|| Some(TargetMachine::get_default_triple()))
        .unwrap();
    let target = Target::from_triple(&triple).unwrap();
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
        .unwrap();

    let c = Context::create();
    let code_generator = compile_module(&c, sources, encoding)?;
    machine
        .write_to_file(&code_generator.module, FileType::Object, Path::new(output))
        .unwrap();

    Ok(())
}

/// Compiles a given source string to a static object and saves the output.
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_static_obj<T: SourceContainer>(
    sources: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(sources, encoding, output, RelocMode::Default, target)
}

/// Compiles a given source string to a shared position independent object and saves the output.
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_shared_pic_object<T: SourceContainer>(
    sources: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(sources, encoding, output, RelocMode::PIC, target)
}

/// Compiles a given source string to a dynamic non PIC object and saves the output.
///
/// # Arguments
///
/// * `sources` - the source to be compiled
/// * `output` - the location on disk to save the output
/// * `target` - an optional llvm target triple
///     If not provided, the machine's triple will be used.
pub fn compile_to_shared_object<T: SourceContainer>(
    sources: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
    target: Option<String>,
) -> Result<(), CompileError> {
    compile_to_obj(sources, encoding, output, RelocMode::DynamicNoPic, target)
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
/// Compiles the given source into LLVM IR and returns it
///
/// # Arguments
///
/// * `sources` - the source to be compiled
pub fn compile_to_ir<T: SourceContainer>(
    sources: Vec<T>,
    encoding: Option<&'static Encoding>,
    output: &str,
) -> Result<(), CompileError> {
    let c = Context::create();
    let code_gen = compile_module(&c, sources, encoding)?;
    let ir = code_gen.module.print_to_string().to_string();
    fs::write(output, ir)
        .map_err(|err| CompileError::io_write_error(output.into(), err.to_string()))
}

///
/// Compiles the given source into a `codegen::CodeGen` using the provided context
///
/// # Arguments
///
/// * `context` - the LLVM Context to be used for the compilation
/// * `sources` - the source to be compiled
pub fn compile_module<'c, T: SourceContainer>(
    context: &'c Context,
    sources: Vec<T>,
    encoding: Option<&'static Encoding>,
) -> Result<codegen::CodeGen<'c>, CompileError> {
    let mut full_index = Index::new();
    let mut unit = CompilationUnit::default();
    // let mut diagnostics : Vec<Diagnostic> = vec![];
    let mut files: SimpleFiles<String, String> = SimpleFiles::new();
    let mut all_units = Vec::new();
    for container in sources {
        let location: String = container.get_location().into();
        let e = container
            .load_source(encoding)
            .map_err(|err| CompileError::io_read_error(err, location.clone()))?;

        let (mut parse_result, diagnostics) = parse(e.source.as_str());
        //pre-process the ast (create inlined types)
        ast::pre_process(&mut parse_result);
        //index the pou
        full_index.import(index::visitor::visit(&parse_result));
        all_units.push(parse_result);

        //log errors
        let file_id = files.add(location, e.source.clone());
        for error in diagnostics {
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

            term::emit(&mut writer.lock(), &config, &files, &diag).map_err(|err| {
                CompileError::codegen_error(
                    format!("Cannot print errors {:#?}", err),
                    SourceRange::undefined(),
                )
            })?;
        }
    }

    //annotate the ASTs
    for u in all_units {
        let _type_map = TypeAnnotator::visit_unit(&full_index, &u);
        //TODO validate and find solution for type_map
        unit.import(u); //TODO this needs to be changed so we have unique AstIds
    }

    //and finally codegen
    let code_generator = codegen::CodeGen::new(context, "main");
    code_generator.generate(unit, &full_index)?;
    Ok(code_generator)
}

fn parse(source: &str) -> ParsedAst {
    let lexer = lexer::lex(source);
    parser::parse(lexer)
}

#[cfg(test)]
mod tests {
    use crate::create_source_code;

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
