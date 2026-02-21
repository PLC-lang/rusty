use std::path::PathBuf;

use plc_ast::ast::CompilationUnit;
use plc_source::source_location::FileMarker;

use regex::Regex;

use crate::GenerateHeaderOptions;

mod file_helper_c;

use once_cell::sync::Lazy;

static WHITE_SPACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s").unwrap());
static UNDERSCORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\_{2,}").unwrap());
static CHARACTER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]*[a-z]*[0-9]*\_*").unwrap());

pub trait FileHelper {
    /// Returns the directory the header should be written to
    ///
    /// ---
    ///
    /// This should return the directory for writing the header to (without the header file).
    /// It can return an empty string if the header is being written to the same directory as the interface itself.
    fn get_directory(&self) -> &str;

    /// Sets the directory the header should be written to
    fn set_directory(&mut self, directory: &str);

    /// Returns the file path that the header file should be written to
    fn get_path(&self) -> &str;

    /// Sets the file path that the header file should be written to
    fn set_path(&mut self, path: &str);

    // Returns the file name for the header file
    fn get_file_name(&self) -> &str;

    /// Sets the file name for the header file
    fn set_file_name(&mut self, file_name: &str);

    // Returns the formatted path for the header file
    fn get_formatted_path(&self) -> &str;

    /// Sets the formatted path for the header file
    fn set_formatted_path(&mut self, formatted_path: &str);

    /// Determines file information for the header file and returns whether or not the determination was successful
    ///
    /// ---
    ///
    /// The succesful result of this method must be that the "directory" (accessible via the "get_directory" method)
    /// and the "path" (accessible via the "get_path" method) are both populated with valid results.
    fn determine_header_file_information(
        &mut self,
        generate_header_options: &GenerateHeaderOptions,
        compilation_unit: &CompilationUnit,
    ) -> bool;
}

/// Given a GenerateHeaderOptions, CompilationUnit and a file extension (string)
/// this will return a struct containing header file information,
/// and a boolean defining whether or not the process was successful.
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
        if let Some(parent) = file_path.parent() {
            PathBuf::from(parent)
        } else {
            PathBuf::from(String::new())
        }
    } else {
        generate_header_options.output_path.clone()
    };

    let output_dir = output_path.clone();
    let (output_name, file_name) = if generate_header_options.prefix.is_empty() {
        let option_file_name = get_file_name_from_path_buf_without_extension(file_path);
        if let Some(file_name) = option_file_name {
            (format!("{}.{}", file_name, file_extension), format_file_name(&file_name))
        } else {
            (String::new(), String::new())
        }
    } else {
        (
            format!("{}.{}", generate_header_options.prefix, file_extension),
            format_file_name(&generate_header_options.prefix),
        )
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
            name: file_name.to_string(),
            formatted_path: format_path(output_path.to_str().expect("Unable to determine the output path!")),
        },
        true,
    )
}

/// Format the path
pub fn format_path(output_path: &str) -> String {
    let path = PathBuf::from(output_path);
    let relative_path = std::env::current_dir()
        .ok()
        .and_then(|cwd| path.strip_prefix(cwd).ok())
        .map(|p| p.to_path_buf())
        .unwrap_or(path);

    let relative_path = relative_path.to_str().unwrap_or(output_path);

    format!("{}_", format_file_name(&relative_path.replace("\\", "_").replace("/", "_").replace(".", "_")))
}

/// Returns the file name from a path buffer without the extension
///
/// ---
///
/// Will return [None] if no file name is found or if the file name has no extension.
fn get_file_name_from_path_buf_without_extension(file_path: PathBuf) -> Option<String> {
    if let Some(os_file_name) = file_path.file_name() {
        if let Some(file_name) = os_file_name.to_str() {
            let file_name = file_name.split('.').next().unwrap_or("");

            if !file_name.is_empty() {
                return Some(String::from(file_name));
            }
        }
    }

    None
}

/// Formats and returns a file name that is safe for definition usage
///
/// ---
///
/// Example:
/// ```ignore
/// "I a!m  a   v@3#r$y     s%t^r&a*n(g)e      f`i~l[e_n]4{m}e t\\h/a:t s;h'o\"u<l>d b,e f.i?x-ed"
/// ```
/// ... should be formatted to ...
/// ```ignore
/// "I_AM_A_V3RY_STRANGE_FILE_N4ME_THAT_SHOULD_BE_FIXED"
/// ```
fn format_file_name(file_name: &str) -> String {
    let white_space_formatted = WHITE_SPACE_REGEX.replace_all(file_name, "_").to_string();
    let underscore_formatted = UNDERSCORE_REGEX.replace_all(&white_space_formatted, "_").to_string();

    let mut formatted_file_name = String::new();
    for caps in CHARACTER_REGEX.captures_iter(&underscore_formatted) {
        formatted_file_name += caps.get(0).unwrap().as_str();
    }

    formatted_file_name.to_uppercase()
}

pub struct HeaderFileInformation {
    pub directory: String,
    pub path: String,
    pub name: String,
    pub formatted_path: String,
}

impl Default for HeaderFileInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl HeaderFileInformation {
    pub const fn new() -> Self {
        HeaderFileInformation {
            directory: String::new(),
            path: String::new(),
            name: String::new(),
            formatted_path: String::new(),
        }
    }
}

#[cfg(test)]
mod file_helper_tests {
    use crate::header_generator::file_helper::format_file_name;

    #[test]
    fn test_format_file_name_weird_file_characters() {
        let valid_file_name =
            "I a!m  a   v@3#r$y     s%t^r&a*n(g)e      f`i~l[e_n]4{m}e t\\h/a:t s;h'o\"u<l>d b,e f.i?x-ed";
        assert_eq!(format_file_name(valid_file_name), "I_AM_A_V3RY_STRANGE_FILE_N4ME_THAT_SHOULD_BE_FIXED");
    }
}
