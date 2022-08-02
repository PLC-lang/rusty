use crate::diagnostics::Diagnostic;
use crate::diagnostics::ErrNo;
use crate::ErrorFormat;
use crate::FormatOption;
use crate::OptimizationLevel;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageFormat {
    Copy,
    System,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Libraries {
    pub name: String,
    pub path: PathBuf,
    pub package: PackageFormat,
    pub include_path: Vec<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proj {
    pub files: Vec<PathBuf>,
    pub compile_type: Option<FormatOption>,
    pub optimization: Option<OptimizationLevel>,
    pub output: Option<String>,
    #[serde(default)]
    pub error_format: ErrorFormat,
    #[serde(default)]
    pub libraries: Vec<Libraries>,
    #[serde(default)]
    pub package_commands: Vec<String>,
}

pub fn get_project_from_file(build_config: &Path, root: &Path) -> Result<Proj, Diagnostic> {
    //read from file
    let content = fs::read_to_string(build_config)?;

    //convert file to Object
    let project: Proj = serde_json::from_str(&content)?;

    check_libs_exist(&project.libraries, root)?;
    Ok(project)
}

fn check_libs_exist(libraries: &[Libraries], root: &Path) -> Result<(), Diagnostic> {
    for library in libraries {
        let path = root.join(&library.path);
        let path = path.join(&format!("lib{}.so", library.name));
        if !path.is_file() {
            return Err(Diagnostic::GeneralError {
                message: format!(
                    "The library could not be found at : {}",
                    path.to_string_lossy()
                ),
                err_no: ErrNo::general__io_err,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::vec;

    use crate::ErrorFormat;
    use crate::{FormatOption, OptimizationLevel};

    use super::Libraries;
    use super::{PackageFormat, Proj};

    #[test]
    fn check_build_struct_from_file() {
        let test_project = Proj {
            files: vec![PathBuf::from("simple_program.st")],
            compile_type: Some(FormatOption::Shared),
            optimization: Some(OptimizationLevel::Default),
            output: Some(String::from("proj.so")),
            error_format: ErrorFormat::Rich,
            libraries: vec![
                Libraries {
                    name: String::from("copy"),
                    path: PathBuf::from("libs/"),
                    package: PackageFormat::Copy,
                    include_path: vec![PathBuf::from("simple_program.st")],
                },
                Libraries {
                    name: String::from("nocopy"),
                    path: PathBuf::from("libs/"),
                    package: PackageFormat::System,
                    include_path: vec![PathBuf::from("simple_program.st")],
                },
            ],
            package_commands: vec![],
        };
        let proj: Proj = serde_json::from_str(
            r#"
            {
                "files" : [
                    "simple_program.st"
                ],
                "compile_type" : "Shared",
                "optimization" : "Default",
                "output" : "proj.so",
                "error_format": "Rich",
                "libraries" : [
                    {
                        "name" : "copy",
                        "path" : "libs/",
                        "package" : "Copy",
                        "include_path" : [
                            "simple_program.st"
                        ]
                    },
                    {
                        "name" : "nocopy",
                        "path" : "libs/",
                        "package" : "System",
                        "include_path" : [
                            "simple_program.st"
                        ]
                    }
                ]
            }
        "#,
        )
        .unwrap();

        assert_eq!(test_project.files, proj.files);
        assert_eq!(test_project.compile_type, proj.compile_type);
        assert_eq!(test_project.optimization, proj.optimization);
        assert_eq!(test_project.output, proj.output);
        let proj_lib = proj.libraries;
        let testproj_lib = test_project.libraries;
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
