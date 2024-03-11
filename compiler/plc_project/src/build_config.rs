use anyhow::Result;
use jsonschema::JSONSchema;
use plc::Target;
use plc_diagnostics::diagnostics::Diagnostic;
use regex::Captures;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use source_code::BuildDescriptionSource;
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
#[serde(deny_unknown_fields)]
pub struct ProjectConfig {
    pub name: String,
    pub files: Vec<PathBuf>,
    #[serde(default)]
    pub compile_type: FormatOption,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(default)]
    pub libraries: Vec<LibraryConfig>,
    #[serde(default)]
    pub package_commands: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "format-version")]
    pub format_version: Option<String>,
}

impl ProjectConfig {
    /// Returns a project from the given json-source
    /// All environment variables (marked with `$VAR_NAME`) that can be resovled at this time are resolved before the conversion
    pub fn try_parse(source: BuildDescriptionSource) -> Result<Self> {
        let content = source.source.as_str();
        let content = resolve_environment_variables(content)?;
        let config: ProjectConfig =
            serde_json::from_str(&content).map_err(|err| Diagnostic::from_serde_error(err, &source))?;
        config.validate()?;

        Ok(config)
    }

    pub(crate) fn from_file(config: &Path) -> Result<Self> {
        //read from file
        let content = fs::read_to_string(config)?;
        let content = BuildDescriptionSource::new(content, config);

        //convert file to Object
        let project = ProjectConfig::try_parse(content)?;

        Ok(project)
    }

    fn validate(&self) -> Result<()> {
        let schema = include_str!("../schema/plc-json.schema");
        let schema_obj = serde_json::from_str(schema).expect("A valid schema");
        let compiled = JSONSchema::compile(&schema_obj).expect("A valid schema");
        let instance = json!(self);
        compiled.validate(&instance).map_err(|errors| {
            let mut message = String::from("plc.json could not be validated due to the following errors:\n");
            for err in errors {
                let prefix = match err.kind {
                    jsonschema::error::ValidationErrorKind::MinItems { .. } => {
                        err.instance_path.to_string().replace('/', "")
                    }
                    _ => "".into(),
                };
                message.push_str(&format!("{prefix}{err}\n"));
            }

            // XXX: jsonschema does not provide error messages with location info
            Diagnostic::new(message).with_error_code("E088")
        })?;
        Ok(())
    }
}

//TODO: I don't think this belongs here
fn resolve_environment_variables(to_replace: &str) -> Result<String> {
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
    use insta::assert_snapshot;
    use plc::output::FormatOption;

    use super::LibraryConfig;
    use super::{LinkageInfo, ProjectConfig};

    const SIMPLE_PROGRAM: &str = r#"
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
"#;

    const ADDITIONAL_UNKNOWN_PROPERTIES: &str = r#"
    {
        "name": "MyProject",
        "files" : [
            "file.st"
            ],
        "compile_type" : "Shared",
        "output": "proj.so",
        "additional_field" : "should give an error"
    }
"#;

    const NO_FILES_SPECIFIED: &str = r#"
    {
        "name": "MyProject",
        "files" : [],
        "compile_type" : "Shared",
        "output": "proj.so"
    }
    "#;

    const INVALID_ENUM_VARIANTS: &str = r#"
    {
        "name": "MyProject",
        "files" : [
            "simple_program.st"
        ],
        "compile_type" : "Interpreted",
        "output": "proj.so",
        "libraries" : [
            {
                "name" : "static",
                "path" : "libs/",
                "package" : "Opened",
                "include_path" : [
                    "simple_program.st"
                ]
            }
        ]
    }
"#;

    const MISSING_REQUIRED_LIBRARY_PROPERTIES: &str = r#"
    {
        "name": "MyProject",
        "files" : [
            "simple_program.st"
        ],
        "compile_type" : "Shared",
        "output": "proj.so",
        "libraries" : [
            {
                "name" : "static",
                "package" : "Static",
                "include_path" : [
                    "simple_program.st"
                ]
            }
        ]
    }
"#;

    const MISSING_REQUIRED_PROPERTIES: &str = r#"
        {
            "files" : [
                "simple_program.st"
            ]
        }
    "#;

    const OPTIONAL_PROPERTIES: &str = r#"
    {
        "version": "0.1",
        "format-version": "0.2",
        "name": "MyProject",
        "files" : [
            "file.st"
            ],
        "compile_type" : "Shared",
        "output": "proj.so"
    }
"#;

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
            version: None,
            format_version: None,
        };
        let proj = ProjectConfig::try_parse(SIMPLE_PROGRAM.into()).unwrap();

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
                ],
                "compile_type" : "Shared",
                "output": "proj.so"
            }
        "#
            .into(),
        )
        .unwrap();

        assert_eq!("test_value", &proj.name);
    }

    #[test]
    fn valid_json_validates_without_errors() {
        let cfg = ProjectConfig::try_parse(SIMPLE_PROGRAM.into());

        assert!(cfg.is_ok())
    }

    #[test]
    fn json_with_additional_fields_reports_unexpected_fields() {
        let Err(diag) = ProjectConfig::try_parse(ADDITIONAL_UNKNOWN_PROPERTIES.into()) else {
            panic!("expected errors")
        };

        assert_snapshot!(diag.to_string())
    }

    #[test]
    fn json_with_invalid_enum_variants_reports_error() {
        let Err(diag) = ProjectConfig::try_parse(INVALID_ENUM_VARIANTS.into()) else {
            panic!("expected errors")
        };

        assert_snapshot!(diag.to_string())
    }

    #[test]
    fn json_with_missing_required_properties_reports_error() {
        // missing name and compile_type
        //XXX: only the first error found is reported by both serde and jsonschema
        let Err(diag) = ProjectConfig::try_parse(MISSING_REQUIRED_PROPERTIES.into()) else {
            panic!("expected errors")
        };
        assert_snapshot!(diag.to_string());

        // missing library path
        let Err(diag) = ProjectConfig::try_parse(MISSING_REQUIRED_LIBRARY_PROPERTIES.into()) else {
            panic!("expected errors")
        };
        assert_snapshot!(diag.to_string())
    }

    #[test]
    fn json_with_empty_files_array_reports_error() {
        let Err(diag) = ProjectConfig::try_parse(NO_FILES_SPECIFIED.into()) else {
            panic!("expected errors")
        };

        assert_snapshot!(diag.to_string())
    }

    #[test]
    fn json_with_optional_properties_is_valid() {
        match ProjectConfig::try_parse(OPTIONAL_PROPERTIES.into()) {
            Ok(cfg) => assert_snapshot!(&format!("{:#?}", cfg)),
            Err(err) => panic!("expected ProjectConfig to be OK, got \n {err}"),
        };
    }
}
