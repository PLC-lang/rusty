use crate::diagnostics::Diagnostic;
use crate::diagnostics::ErrNo;
use crate::ErrorFormat;
use crate::FilePath;
use crate::FormatOption;
use crate::OptimizationLevel;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageFormat {
    Copy,
    System,
}

#[derive(Serialize, Deserialize)]
pub struct Libraries {
    pub name: String,
    pub path: String,
    pub package: PackageFormat,
    pub include_path: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Proj {
    pub files: Vec<String>,
    pub compile_type: Option<FormatOption>,
    pub optimization: Option<OptimizationLevel>,
    pub output: String,
    pub error_format: ErrorFormat,
    pub libraries: Option<Vec<Libraries>>,
}

pub fn get_project_from_file(build_config: Option<String>) -> Result<Proj, Diagnostic> {
    let filepath = build_config.unwrap_or_else(|| String::from("plc.json"));

    //read from file
    let content = fs::read_to_string(filepath);

    let content = match content {
        Ok(file_content) => file_content,
        Err(e) => {
            return Err(Diagnostic::GeneralError {
                message: e.to_string(),
                err_no: ErrNo::general__io_err,
            })
        }
    };

    //convert file to Object
    let project = serde_json::from_str(&content);
    let project: Proj = match project {
        Ok(project) => project,
        Err(_e) => {
            return Err(Diagnostic::GeneralError {
                message: String::from(r#"An error occured whilest parsing!"#),
                err_no: ErrNo::general__io_err,
            })
        }
    };

    let project = get_path_when_empty(project)?;
    Ok(project)
}

pub fn string_to_filepath(content: Vec<String>) -> Vec<FilePath> {
    let mut filepath: Vec<FilePath> = vec![];
    for item in content {
        filepath.push(FilePath::from(item));
    }
    filepath
}

fn get_path_when_empty(p: Proj) -> Result<Proj, Diagnostic> {
    if let Some(ref libraries) = p.libraries {
        for library in libraries {
            let path = if library.path.is_empty() {
                None
            } else {
                Some(&library.path)
            };
            let path = path
                .map_or(Path::new("."), Path::new)
                .join(&format!("lib{}.so", library.name));
            if !Path::new(&path).is_file() {
                return Err(Diagnostic::GeneralError {
                    message: format!(
                        "The library could not be found at : {}",
                        path.to_string_lossy()
                    ),
                    err_no: ErrNo::general__io_err,
                });
            }
        }
    }
    Ok(p)
}
