use std::path::PathBuf;

use plc_ast::ast::CompilationUnit;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::FileMarker;

use crate::{GenerateHeaderOptions, GenerateLanguage};

pub struct FileManager {
    pub language: GenerateLanguage,
    pub output_path: String,
    pub output_dir: String,
}

impl FileManager {
    pub const fn new() -> Self {
        FileManager { language: GenerateLanguage::C, output_path: String::new(), output_dir: String::new() }
    }

    pub fn prepare_file_and_directory(
        &mut self,
        generate_header_options: &GenerateHeaderOptions,
        compilation_unit: &CompilationUnit,
    ) -> Result<bool, Diagnostic> {
        match self.language {
            GenerateLanguage::C => {
                self.prepare_file_and_directory_for_c(generate_header_options, compilation_unit)
            }
            language => Err(Diagnostic::new(format!("{language:?} language not yet supported!"))),
        }
    }

    fn prepare_file_and_directory_for_c(
        &mut self,
        generate_header_options: &GenerateHeaderOptions,
        compilation_unit: &CompilationUnit,
    ) -> Result<bool, Diagnostic> {
        let file_path = match compilation_unit.file {
            FileMarker::File(file_path) => PathBuf::from(file_path),
            _ => PathBuf::from(String::new()),
        };

        let mut output_path = if generate_header_options.output_path.as_os_str().is_empty() {
            if file_path.parent().is_some() {
                PathBuf::from(file_path.parent().unwrap())
            } else {
                PathBuf::from(String::new())
            }
        } else {
            generate_header_options.output_path.clone()
        };

        let output_dir = output_path.clone();
        let output_name = if generate_header_options.prefix.is_empty() {
            let file_name = get_file_name_from_path_buf_without_extension(file_path);
            if file_name.is_some() {
                format!("{}.h", file_name.unwrap())
            } else {
                String::new()
            }
        } else {
            format!("{}.h", generate_header_options.prefix)
        };

        if output_name.is_empty() {
            // This means this compilation unit is not associated with a file.
            // In this case we aren't interested in drilling into it.
            return Ok(false);
        }

        output_path.push(&output_name);

        self.output_path = String::from(output_path.to_str().expect("Unable to determine the output path!"));
        self.output_dir =
            String::from(output_dir.to_str().expect("Unable to determine the output directory!"));

        Ok(true)
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

fn get_file_name_from_path_buf_without_extension(file_path: PathBuf) -> Option<String> {
    if file_path.file_name().is_some() {
        let file_name = file_path.file_name().unwrap().to_str();
        file_name?;

        let file_name = file_name.unwrap().split('.').next().unwrap_or("");

        if file_name.is_empty() {
            return None;
        }

        Some(String::from(file_name))
    } else {
        None
    }
}
