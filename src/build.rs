use std::fs;

use crate::build;
use crate::cli::CompileParameters;
use crate::create_file_paths;
use crate::diagnostics::Diagnostic;
use crate::get_target_triple;
use crate::CompileOptions;
use crate::ErrorFormat;
use crate::FilePath;
use crate::FormatOption;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ParseProj {
    pub files: Vec<String>,
    pub compile_type: Option<FormatOption>,
    pub output: String,
}

#[derive(Serialize, Deserialize)]
pub struct Proj {
    pub files: Vec<FilePath>,
    pub compile_type: Option<FormatOption>,
    pub output: String,
}

//shouldn't use compile parameters as this should be called from parse_with_parameters
//Therefore, it is an additional opption if the flag "--build-config" is set to build from the file
//otherwise it should continue as it was. Therefore, everything in here should be called from
//lib.rs withing build_with_parameters

//this is only optional instead adding all the flaggs to add a file

pub fn get_project_from_file(filename: String) -> Proj {
    //read from file
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    //convert file to Object
    let parse_project: ParseProj =
        serde_json::from_str(&content).expect("converting was a problem");

    let project = Proj {
        files: string_to_filepath(parse_project.files),
        compile_type: parse_project.compile_type,
        output: parse_project.output,
    };

    project
}

fn string_to_filepath(content: Vec<String>) -> Vec<FilePath> {
    let mut filepath: Vec<FilePath> = vec![];
    for item in content {
        filepath.push(FilePath::from(item));
    }
    filepath
}
