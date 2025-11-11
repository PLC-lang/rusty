use std::path::PathBuf;

use plc_ast::ast::CompilationUnit;
use plc_source::source_location::FileMarker;

use crate::GenerateHeaderOptions;

mod file_helper_c;

pub trait FileHelper {
    fn determine_header_file_information(
        &mut self,
        generate_header_options: &GenerateHeaderOptions,
        compilation_unit: &CompilationUnit,
    ) -> bool;
}

fn get_header_file_information(
    generate_header_options: &GenerateHeaderOptions,
    compilation_unit: &CompilationUnit,
    file_extension: &str,
) -> (HeaderFileInformation, bool) {
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
            format!("{}.{}", file_name.unwrap(), file_extension)
        } else {
            String::new()
        }
    } else {
        format!("{}.{}", generate_header_options.prefix, file_extension)
    };

    if output_name.is_empty() {
        // This means this compilation unit is not associated with a file.
        // In this case we aren't interested in drilling into it.
        return (HeaderFileInformation::default(), false);
    }

    output_path.push(&output_name);

    (
        HeaderFileInformation {
            directory: String::from(output_dir.to_str().expect("Unable to determine the output directory!")),
            path: String::from(output_path.to_str().expect("Unable to determine the output path!")),
        },
        true,
    )
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

pub struct HeaderFileInformation {
    pub directory: String,
    pub path: String,
}

impl Default for HeaderFileInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl HeaderFileInformation {
    pub const fn new() -> Self {
        HeaderFileInformation { directory: String::new(), path: String::new() }
    }
}
