use crate::diagnostics::Diagnostic;
use crate::diagnostics::ErrNo;
use crate::ErrorFormat;
use crate::FilePath;
use crate::FormatOption;
use crate::OptimizationLevel;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub package_commands: Option<Vec<String>>,
}

pub fn get_project_from_file(build_config: Option<String>) -> Result<Proj, Diagnostic> {
    let filepath = build_config.unwrap_or_else(|| String::from("plc.json"));

    //read from file
    let content = fs::read_to_string(filepath)?;

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

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::build::get_project_from_file;
    use crate::ErrorFormat;
    use crate::{FormatOption, OptimizationLevel};

    use super::Libraries;
    use super::{PackageFormat, Proj};

    #[test]
    fn check_build_struct_from_file() {
        let test_project = Proj {
            files: vec![String::from(
                "tests/integration/data/json/simple_program.st",
            )],
            compile_type: Some(FormatOption::Shared),
            optimization: Some(OptimizationLevel::Default),
            output: String::from("proj.so"),
            error_format: ErrorFormat::Rich,
            libraries: Some(vec![
                Libraries {
                    name: String::from("copy"),
                    path: String::from("tests/integration/data/json/libs/"),
                    package: PackageFormat::Copy,
                    include_path: vec![String::from(
                        "tests/integration/data/json/simple_program.st",
                    )],
                },
                Libraries {
                    name: String::from("nocopy"),
                    path: String::from("tests/integration/data/json/libs/"),
                    package: PackageFormat::System,
                    include_path: vec![String::from(
                        "tests/integration/data/json/simple_program.st",
                    )],
                },
            ]),
            package_commands: Some(vec![]),
        };
        let proj = get_project_from_file(Some(String::from(
            "tests/integration/data/json/build_description_file.json",
        )))
        .unwrap();

        assert_eq!(test_project.files, proj.files);
        assert_eq!(test_project.compile_type, proj.compile_type);
        assert_eq!(test_project.optimization, proj.optimization);
        assert_eq!(test_project.output, proj.output);
        if let Some(proj_lib) = proj.libraries {
            if let Some(testproj_lib) = test_project.libraries {
                assert_eq!(testproj_lib[0].name, proj_lib[0].name);
                assert_eq!(testproj_lib[0].path, proj_lib[0].path);
                assert_eq!(testproj_lib[0].package, proj_lib[0].package);
                assert_eq!(testproj_lib[0].include_path, proj_lib[0].include_path);
                assert_eq!(testproj_lib[1].name, proj_lib[1].name);
                assert_eq!(testproj_lib[1].path, proj_lib[1].path);
                assert_eq!(testproj_lib[1].package, proj_lib[1].package);
                assert_eq!(testproj_lib[1].include_path, proj_lib[1].include_path);
            }
        }
    }
}
