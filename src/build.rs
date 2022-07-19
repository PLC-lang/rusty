use std::fs;
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

pub fn get_project_from_file(filename: String) -> Proj {
    //read from file
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    //convert file to Object
    let parse_project: ParseProj =
        serde_json::from_str(&content).expect("converting was a problem");

    Proj {
        files: string_to_filepath(parse_project.files),
        compile_type: parse_project.compile_type,
        output: parse_project.output,
    }
}

fn string_to_filepath(content: Vec<String>) -> Vec<FilePath> {
    let mut filepath: Vec<FilePath> = vec![];
    for item in content {
        filepath.push(FilePath::from(item));
    }
    filepath
}
