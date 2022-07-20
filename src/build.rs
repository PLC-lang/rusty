use crate::ErrorFormat;
use crate::FilePath;
use crate::FormatOption;
use crate::OptimizationLevel;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct ParseProj {
    pub files: Vec<String>,
    pub compile_type: Option<FormatOption>,
    pub optimization: OptimizationLevel,
    pub target: Option<String>,
    pub output: String,
    pub error_format: ErrorFormat,
    pub includes: Vec<String>,
    pub libraries: Vec<String>,
    pub library_paths: Vec<String>,
    pub sysroot: Option<String>,
    pub skip_linking: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Proj {
    pub files: Vec<FilePath>,
    pub compile_type: Option<FormatOption>,
    pub optimization: OptimizationLevel,
    pub target: Option<String>,
    pub output: String,
    pub error_format: ErrorFormat,
    pub includes: Vec<FilePath>,
    pub libraries: Vec<String>,
    pub library_paths: Vec<String>,
    pub sysroot: Option<String>,
    pub skip_linking: bool,
}

impl From<ParseProj> for Proj {
    fn from(p: ParseProj) -> Proj {
        Proj {
            files: string_to_filepath(p.files),
            compile_type: p.compile_type,
            optimization: p.optimization,
            target: p.target,
            output: p.output,
            error_format: p.error_format,
            includes: string_to_filepath(p.includes),
            libraries: p.libraries,
            library_paths: p.library_paths,
            skip_linking: p.skip_linking,
            sysroot: p.sysroot,
        }
    }
}

pub fn get_project_from_file(filename: String) -> Proj {
    //read from file
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    //convert file to Object
    let parse_project: ParseProj =
        serde_json::from_str(&content).expect("converting was a problem");

    Proj::from(parse_project)
}

fn string_to_filepath(content: Vec<String>) -> Vec<FilePath> {
    let mut filepath: Vec<FilePath> = vec![];
    for item in content {
        filepath.push(FilePath::from(item));
    }
    filepath
}
