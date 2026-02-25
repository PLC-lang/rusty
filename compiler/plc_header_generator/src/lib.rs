use clap::ArgEnum;
use std::path::PathBuf;

pub mod header_generator;

#[derive(PartialEq, Eq, Debug, Clone, Copy, ArgEnum, Default)]
pub enum GenerateLanguage {
    #[default]
    C,
    Rust,
}

#[derive(Debug)]
pub struct GenerateHeaderOptions {
    /// Whether or not to include generated code stubs for the library.
    pub include_stubs: bool,

    /// The language used to generate the header file.
    pub language: GenerateLanguage,

    /// The output folder where generated headers and stubs will be placed. Will default by convention.
    pub output_path: PathBuf,

    /// The prefix for the generated header file(s). Will default to the project name if not supplied.
    pub prefix: String,
}

impl Default for GenerateHeaderOptions {
    fn default() -> Self {
        GenerateHeaderOptions {
            include_stubs: false,
            language: GenerateLanguage::C,
            output_path: PathBuf::from(String::new()),
            prefix: String::new(),
        }
    }
}
