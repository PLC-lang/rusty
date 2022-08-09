use crate::diagnostics::Diagnostic;
use crate::resolve_environment_variables;
use crate::FormatOption;
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
pub struct Project {
    pub files: Vec<PathBuf>,
    pub compile_type: Option<FormatOption>,
    pub output: Option<String>,
    #[serde(default)]
    pub libraries: Vec<Libraries>,
    #[serde(default)]
    pub package_commands: Vec<String>,
}

impl Project {
    /// Converts all pathes to absolute
    pub fn to_resolved(self, root: &Path) -> Self {
        Project {
            files: self
                .files
                .into_iter()
                .map(|it| if it.is_absolute() { it } else { root.join(it) })
                .collect(),
            libraries: self
                .libraries
                .into_iter()
                .map(|it| Libraries {
                    path: if it.path.is_absolute() {
                        it.path
                    } else {
                        root.join(it.path)
                    },
                    include_path: it
                        .include_path
                        .into_iter()
                        .map(|it| if it.is_absolute() { it } else { root.join(it) })
                        .collect(),
                    ..it
                })
                .collect(),
            ..self
        }
    }

    /// Retuns a project from the given string (in json format)
    /// All environment variables (marked with `$VAR_NAME`) that can be resovled at this time are resolved before the conversion
    pub fn try_parse(content: &str) -> Result<Self, Diagnostic> {
        let content = resolve_environment_variables(content)?;
        serde_json::from_str(&content).map_err(Into::into)
    }
}

pub fn get_project_from_file(build_config: &Path) -> Result<Project, Diagnostic> {
    //read from file
    let content = fs::read_to_string(build_config)?;

    //convert file to Object
    let project = Project::try_parse(&content)?;

    Ok(project)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{env, vec};

    use crate::FormatOption;

    use super::Libraries;
    use super::{PackageFormat, Project};

    #[test]
    fn check_build_struct_from_file() {
        let test_project = Project {
            files: vec![PathBuf::from("simple_program.st")],
            compile_type: Some(FormatOption::Shared),
            output: Some(String::from("proj.so")),
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
        let proj = Project::try_parse(
            r#"
            {
                "files" : [
                    "simple_program.st"
                ],
                "compile_type" : "Shared",
                "output" : "proj.so",
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

    #[test]
    fn project_creation_resolves_environment_vars() {
        //Add env
        env::set_var("test_var", "test_value");
        let proj = Project::try_parse(
            r#"
            {
                "files" : [
                    "simple_program.st"
                ],
                "output" : "$test_var.so"
            }
        "#,
        )
        .unwrap();

        assert_eq!("test_value.so", &proj.output.unwrap());
    }

    // #[test]
    // fn project_resolve_makes_pathes_absolute() {
    //     let root = PathBuf::from("root");
    //     //Add env
    //     let proj = Project::try_parse(
    //         r#"
    //         {
    //             "files" : [
    //                 "simple_program.st"
    //             ]
    //         }
    //     "#,
    //     )
    //     .unwrap()
    //     .to_resolved(&root);

    //     assert_eq!(root.join("simple_program.st"), proj.files[0]);
    // }
}
