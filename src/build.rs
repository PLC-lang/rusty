use crate::diagnostics::Diagnostic;
use crate::diagnostics::ErrNo;
use crate::ErrorFormat;
use crate::FilePath;
use crate::FormatOption;
use crate::OptimizationLevel;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct ParseLibrary {
    pub name: String,
    pub path: String,
    pub include_path: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Library {
    pub name: Vec<String>,
    pub path: Vec<String>,
    pub include_path: Vec<FilePath>,
}

#[derive(Serialize, Deserialize)]
pub struct ParseProj {
    pub files: Vec<String>,
    pub compile_type: Option<FormatOption>,
    pub optimization: Option<OptimizationLevel>,
    pub target: Option<String>,
    pub output: String,
    pub error_format: ErrorFormat,
    pub libraries: Vec<ParseLibrary>,
    pub sysroot: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Proj {
    pub files: Vec<FilePath>,
    pub compile_type: Option<FormatOption>,
    pub optimization: Option<OptimizationLevel>,
    pub target: Option<String>,
    pub output: String,
    pub error_format: ErrorFormat,
    pub libraries: Library,
    pub sysroot: Option<String>,
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
            libraries: get_libraries_from_parselibraries(p.libraries),
            sysroot: p.sysroot,
        }
    }
}

pub fn get_project_from_file(build_config: Option<String>) -> Result<Proj, Diagnostic> {
    let mut filepath: String = String::from("plc.json");

    if let Some(filename) = build_config {
        filepath = filename;
    }

    //read from file
    let content = fs::read_to_string(filepath);

    let content = match content {
        Ok(file_content) => file_content,
        Err(_e) => {
            return Err(Diagnostic::GeneralError {
                message: String::from(
                    r#"No such file or directory found, please add the path of "plc.json""#,
                ),
                err_no: ErrNo::general__io_err,
            })
        }
    };

    //convert file to Object
    let parse_project = serde_json::from_str(&content);
    let parse_project: ParseProj = match parse_project {
        Ok(pp) => pp,
        Err(_e) => {
            return Err(Diagnostic::GeneralError {
                message: String::from(r#"An error occured whilest parsing!"#),
                err_no: ErrNo::general__io_err,
            })
        }
    };

    let proj = get_path_when_empty(Proj::from(parse_project))?;
    Ok(proj)
}

fn string_to_filepath(content: Vec<String>) -> Vec<FilePath> {
    let mut filepath: Vec<FilePath> = vec![];
    for item in content {
        filepath.push(FilePath::from(item));
    }
    filepath
}

fn get_libraries_from_parselibraries(l: Vec<ParseLibrary>) -> Library {
    let string_paths: Vec<String> = l.iter().flat_map(|it| it.include_path.clone()).collect();

    Library {
        name: l.iter().map(|it| it.name.clone()).collect(),
        path: l.iter().map(|it| it.path.clone()).collect(),
        include_path: string_to_filepath(string_paths),
    }
}

fn get_path_when_empty(p: Proj) -> Result<Proj, Diagnostic> {
    for i in 0..p.libraries.name.len() {
        if p.libraries.path.get(i).is_some() && p.libraries.path.get(i) != Some(&String::from("")) {
            continue;
        } else if let Some(name) = p.libraries.name.get(i) {
            if Path::new(&format!("{}.so", name)).is_file() {
                continue;
            } else {
                return Err(Diagnostic::GeneralError {
                    message: String::from("Lib path can not be found, please add Path"),
                    err_no: ErrNo::general__io_err,
                });
            }
        }
    }
    Ok(p)
}
