use diagnostics::Diagnostic;
use plc::Target;
use regex::Captures;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use plc::output::FormatOption;

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryConfig {
    pub name: String,
    pub path: PathBuf,
    pub package: LinkageInfo,
    pub include_path: Vec<PathBuf>,
    #[serde(default = "default_targets")]
    pub architectures: Vec<Target>,
}

/// Targets to use if no other targets have been defined
fn default_targets() -> Vec<Target> {
    vec!["x86_64-linux-gnu".into(), "aarch64-linux-gnu".into()]
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum LinkageInfo {
    Copy,
    Local,
    System,
    Static,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub files: Vec<PathBuf>,
    #[serde(default)]
    pub compile_type: FormatOption,
    #[serde(default)]
    pub output: Option<String>,
    #[serde(default)]
    pub libraries: Vec<LibraryConfig>,
    #[serde(default)]
    pub package_commands: Vec<String>,
}

impl ProjectConfig {
    /// Retuns a project from the given string (in json format)
    /// All environment variables (marked with `$VAR_NAME`) that can be resovled at this time are resolved before the conversion
    pub fn try_parse(content: &str) -> Result<Self, Diagnostic> {
        let content = resolve_environment_variables(content)?;
        serde_json::from_str(&content).map_err(Into::into)
    }

    pub(crate) fn from_file(config: &Path) -> Result<Self, Diagnostic> {
        //read from file
        let content = fs::read_to_string(config)?;

        //convert file to Object
        let project = ProjectConfig::try_parse(&content)?;

        Ok(project)
    }
}

//TODO: I don't think this belongs here
fn resolve_environment_variables(to_replace: &str) -> Result<String, Diagnostic> {
    let pattern = Regex::new(r"\$(\w+)")?;
    let result = pattern.replace_all(to_replace, |it: &Captures| {
        let original = it.get(0).map(|it| it.as_str().to_string()).unwrap();
        if let Some(var) = it.get(1).map(|it| it.as_str()) {
            env::var(var).unwrap_or(original)
        } else {
            original
        }
    });
    Ok(result.replace('\\', r"\\"))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{env, vec};

    use crate::build_config::default_targets;
    use plc::output::FormatOption;

    use super::LibraryConfig;
    use super::{LinkageInfo, ProjectConfig};

    #[test]
    fn check_build_struct_from_file() {
        let test_project = ProjectConfig {
            name: "MyProject".to_string(),
            files: vec![PathBuf::from("simple_program.st")],
            compile_type: FormatOption::Shared,
            output: Some(String::from("proj.so")),
            libraries: vec![
                LibraryConfig {
                    name: String::from("copy"),
                    path: PathBuf::from("libs/"),
                    package: LinkageInfo::Copy,
                    include_path: vec![PathBuf::from("simple_program.st")],
                    architectures: default_targets(),
                },
                LibraryConfig {
                    name: String::from("nocopy"),
                    path: PathBuf::from("libs/"),
                    package: LinkageInfo::System,
                    include_path: vec![PathBuf::from("simple_program.st")],
                    architectures: default_targets(),
                },
                LibraryConfig {
                    name: String::from("static"),
                    path: PathBuf::from("libs/"),
                    package: LinkageInfo::Static,
                    include_path: vec![PathBuf::from("simple_program.st")],
                    architectures: default_targets(),
                },
                LibraryConfig {
                    name: String::from("withTargets"),
                    path: PathBuf::from("libs/"),
                    package: LinkageInfo::Static,
                    include_path: vec![PathBuf::from("simple_program.st")],
                    architectures: vec!["myArch".into(), "myArch2".into()],
                },
            ],
            package_commands: vec![],
        };
        let proj = ProjectConfig::try_parse(
            r#"
            {
                "name": "MyProject",
                "files" : [
                    "simple_program.st"
                ],
                "compile_type" : "Shared",
                "output": "proj.so",
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
                    },
                    {
                        "name" : "static",
                        "path" : "libs/",
                        "package" : "Static",
                        "include_path" : [
                            "simple_program.st"
                        ]
                    },
                    {
                        "name" : "withTargets",
                        "path" : "libs/",
                        "package" : "Static",
                        "include_path" : [
                            "simple_program.st"
                        ],
                        "architectures": ["myArch", "myArch2"]
                    }
                ]
            }
        "#,
        )
        .unwrap();

        assert_eq!(test_project.name, proj.name);
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
        let proj = ProjectConfig::try_parse(
            r#"
            {
                "name" : "$test_var",
                "files" : [
                    "simple_program.st"
                ]
            }
        "#,
        )
        .unwrap();

        assert_eq!("test_value", &proj.name);
    }
}
