use anyhow::{bail, Result};
// Copyright (c) 2021 Ghaith Hachem and Mathias Rieder
use clap::{ArgGroup, Parser, Subcommand};
use encoding_rs::Encoding;
use plc_diagnostics::diagnostics::{diagnostics_registry::DiagnosticsConfiguration, Diagnostic};
use std::{env, ffi::OsStr, num::ParseIntError, path::PathBuf};

use plc::output::FormatOption;
use plc::{ConfigFormat, DebugLevel, ErrorFormat, Target, Threads, DEFAULT_GOT_LAYOUT_FILE};

pub type ParameterError = clap::Error;

#[derive(Parser, Debug)]
#[clap(
    group = ArgGroup::new("format"),
    about = "IEC61131-3 Structured Text compiler powered by Rust & LLVM ",
)]
#[clap(subcommand_negates_reqs = true)]
#[clap(subcommand_precedence_over_arg = true)]
pub struct CompileParameters {
    #[clap(short, long, global = true, name = "output-file", help = "Write output to <output-file>")]
    pub output: Option<String>,

    #[clap(
        long = "ast",
        group = "format",
        global = true,
        help = "Emit AST (Abstract Syntax Tree) as output"
    )]
    pub print_ast: bool,

    #[clap(
        long = "ast-lowered",
        group = "format",
        global = true,
        help = "Emit lowered AST (Abstract Syntax Tree) as output"
    )]
    pub print_ast_lowered: bool,

    #[clap(long = "version", group = "format", global = true)]
    pub build_info: bool,

    #[clap(
        long = "ir",
        group = "format",
        global = true,
        help = "Emit IR (LLVM Intermediate Representation) as output"
    )]
    pub output_ir: bool,

    #[clap(long = "shared", group = "format", global = true, help = "Emit a shared object as output")]
    pub output_shared_obj: bool,

    #[clap(long = "pic", group = "format", global = true, help = "Equivalent to --shared")]
    pub output_pic_obj: bool,

    #[clap(long = "no-pic", group = "format", global = true, help = "Emit a no PIC shared object")]
    pub output_no_pic_obj: bool,

    #[clap(long = "static", group = "format", global = true, help = "Emit an object as output")]
    pub output_obj_code: bool,

    #[clap(long = "relocatable", group = "format", global = true, help = "Emit an object as output")]
    pub output_reloc_code: bool,

    #[clap(
        long = "bc",
        group = "format",
        global = true,
        help = "Emit binary IR (binary representation of LLVM-IR) as output"
    )]
    pub output_bit_code: bool,

    #[clap(short = 'c', global = true, help = "Do not link after compiling object code")]
    pub compile_only: bool,

    #[clap(long, name = "target-triple", global = true, help = "A target-triple supported by LLVM")]
    pub target: Option<Target>,

    #[clap(
        long,
        name = "encoding",
        help = "The file encoding used to read the input-files, as defined by the Encoding Standard",
        global = true,
        parse(try_from_str = parse_encoding),
    )]
    pub encoding: Option<&'static Encoding>,

    #[clap(
        name = "input-files",
        help = "Read input from <input-files>, may be a glob expression like 'src/**/*' or a sequence of files",
        // required = true,
        min_values = 1
    )]
    // having a vec allows bash to resolve *.st itself
    pub input: Vec<String>,

    #[clap(name = "library-path", long, short = 'L', help = "Search path for libraries, used for linking")]
    pub library_paths: Vec<String>,

    #[clap(name = "library", long, short = 'l', help = "Library name to link")]
    pub libraries: Vec<String>,

    #[clap(long, name = "sysroot", global = true, help = "Path to system root, used for linking")]
    pub sysroot: Option<String>,

    #[clap(name = "include", long, short = 'i', help = "Include source files for external functions")]
    pub includes: Vec<String>,

    #[clap(
        name = "script",
        long,
        global = true,
        group = "linker_script",
        help = "Specify a linker script to use"
    )]
    pub linker_script: Option<String>,

    #[clap(
        name = "no-linker-script",
        long,
        global = true,
        group = "linker_script",
        help = "Specify that no linker script should be used"
    )]
    pub no_linker_script: bool,

    #[clap(
        name = "hardware-conf",
        long,
        global = true,
        help = "Generate Hardware configuration files to the given location.
    Format is detected by extenstion.
    Supported formats : json, toml",
    parse(try_from_str = validate_config)
    ) ]
    pub hardware_config: Option<String>,

    #[clap(
        name = "got-layout-file",
        long,
        global = true,
        help = "Obtain information about the current custom GOT layout from the given file if it exists.
    Save information about the generated custom GOT layout to the given file.
    Format is detected by extension.
    Supported formats : json, toml",
        default_value = DEFAULT_GOT_LAYOUT_FILE,
        parse(try_from_str = validate_config),
        requires = "online-change"
    ) ]
    pub got_layout_file: String,

    #[clap(
        name = "optimization",
        long,
        short = 'O',
        help = "Optimization level",
        arg_enum,
        default_value = "default",
        global = true
    )]
    pub optimization: plc::OptimizationLevel,

    #[clap(
        name = "error-format",
        long,
        help = "Set format for error reporting",
        arg_enum,
        default_value = "rich",
        global = true
    )]
    pub error_format: ErrorFormat,

    #[clap(name = "linker", long, help = "Define a custom (cc compatible) linker command", global = true)]
    pub linker: Option<String>,

    #[clap(
        name = "debug",
        long,
        short = 'g',
        help = "Generate source-level debug information",
        global = true,
        group = "dbg"
    )]
    pub generate_debug: bool,

    #[clap(
        name = "debug-variables",
        long,
        help = "Generate debug information for global variables",
        global = true,
        group = "dbg"
    )]
    pub generate_varinfo: bool,

    #[clap(
        name = "gdwarf",
        long,
        help = "Generate source-level debug information with the specified dwarf version",
        value_name = "dwarf version",
        global = true,
        group = "dbg",
        conflicts_with = "debug",
        max_values = 1,
        possible_values = &["2", "3", "4", "5"],
    )]
    pub gdwarf_version: Option<usize>,

    #[clap(
        name = "gdwarf-variables",
        long,
        help = "Generate debug information for global variables with the specified dwarf version",
        value_name = "dwarf version",
        global = true,
        group = "dbg",
        conflicts_with = "debug-variables",
        max_values = 1,
        possible_values = &["2", "3", "4", "5"],
    )]
    pub gdwarf_varinfo_version: Option<usize>,

    #[clap(
        name = "threads",
        long,
        short = 'j',
        help = "Set the number of threads to use for the compilation",
        global = true,
        parse(try_from_str = get_parallel_threads)
    )]
    pub threads: Option<Threads>,

    #[clap(name = "error-config", long, help = "A diagnostics configuration in JSON format", global = true)]
    pub error_config: Option<String>,

    #[clap(
        name = "single-module",
        long,
        help = "Build the application as a single LLVM module",
        global = true
    )]
    pub single_module: bool,

    #[clap(name = "check", long, help = "Check only, do not generate any output", global = true)]
    pub check_only: bool,

    #[clap(
        long,
        help = "Emit a binary with specific compilation information, suitable for online changes when ran under a conforming runtime",
        global = true
    )]
    pub online_change: bool,

    #[clap(subcommand)]
    pub commands: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Uses build description file.
    ///
    /// build
    ///
    /// Options:
    /// --build-location <path> --lib-location <path>
    ///
    /// Supported format: json
    ///
    Build {
        #[clap(
            parse(try_from_str = validate_config)
        )]
        build_config: Option<String>,

        #[clap(name = "build-location", long)]
        build_location: Option<String>,

        #[clap(name = "lib-location", long)]
        lib_location: Option<String>,
    },

    /// Used to trigger a check, but not compile action.
    Check {
        #[clap(
            parse(try_from_str = validate_config)
        )]
        build_config: Option<String>,
    },

    /// Prints out various configuration options
    Config {
        #[clap(
            name = "config-format",
            long = "format",
            group = "config",
            default_value = "json",
            help = "Format of the configuration file, if supported"
        )]
        format: ConfigFormat,

        #[clap(subcommand)]
        option: ConfigOption,

        #[clap(
            parse(try_from_str = validate_config)
        )]
        build_config: Option<String>,
    },

    /// Explains an error code
    Explain {
        #[clap(help = "Error code to explain, for example `E001`")]
        error: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum ConfigOption {
    #[clap(help = "Prints the plc.json's schema used for validation")]
    Schema,
    #[clap(help = "Prints the configuration for the project")]
    Diagnostics,
}

impl SubCommands {
    pub fn get_build_configuration(&self) -> Option<&str> {
        let (SubCommands::Build { build_config, .. }
        | SubCommands::Check { build_config }
        | SubCommands::Config { build_config, .. }) = self
        else {
            return None;
        };
        build_config.as_deref()
    }
}

fn parse_encoding(encoding: &str) -> Result<&'static Encoding, String> {
    Encoding::for_label(encoding.as_bytes()).ok_or(format!("Unknown encoding {encoding}"))
}

fn validate_config(config_name: &str) -> Result<String, String> {
    if get_config_format(config_name).is_some() {
        Ok(config_name.to_string())
    } else {
        Err(format!(r#"Cannot identify format type for {config_name}, valid extensions : "json", "toml""#))
    }
}

fn get_parallel_threads(thread_count: &str) -> Result<Threads, ParseIntError> {
    if thread_count.is_empty() {
        Ok(Threads::Full)
    } else {
        let count = thread_count.parse::<usize>()?;
        Ok(Threads::Fix(count))
    }
}

pub fn get_config_format(name: &str) -> Option<ConfigFormat> {
    let ext = name.split('.').next_back();
    match ext {
        Some("json") => Some(ConfigFormat::JSON),
        Some("toml") => Some(ConfigFormat::TOML),
        _ => None,
    }
}

impl CompileParameters {
    pub fn parse<T: AsRef<OsStr> + AsRef<str>>(args: &[T]) -> Result<CompileParameters, ParameterError> {
        CompileParameters::try_parse_from(args)
    }

    pub fn debug_level(&self) -> DebugLevel {
        if self.generate_debug {
            return DebugLevel::Full(plc::DEFAULT_DWARF_VERSION);
        }
        if self.generate_varinfo {
            return DebugLevel::VariablesOnly(plc::DEFAULT_DWARF_VERSION);
        }
        if let Some(version) = self.gdwarf_version {
            return DebugLevel::Full(version);
        }
        if let Some(version) = self.gdwarf_varinfo_version {
            return DebugLevel::VariablesOnly(version);
        }

        DebugLevel::None
    }

    // convert the scattered bools from structopt into an enum
    pub fn output_format(&self) -> Option<FormatOption> {
        if self.output_bit_code {
            Some(FormatOption::Bitcode)
        } else if self.output_ir {
            Some(FormatOption::IR)
        } else if self.output_pic_obj {
            Some(FormatOption::PIC)
        } else if self.output_shared_obj {
            Some(FormatOption::Shared)
        } else if self.output_no_pic_obj {
            Some(FormatOption::NoPIC)
        } else if self.compile_only {
            Some(FormatOption::Object)
        } else if self.output_obj_code {
            Some(FormatOption::Static)
        } else if self.output_reloc_code {
            Some(FormatOption::Relocatable)
        } else {
            //Keep the parameter default as static
            None
        }
    }

    /// If set, no files will be generated
    pub fn is_check(&self) -> bool {
        self.check_only || matches!(self.commands, Some(SubCommands::Check { .. }))
    }

    /// return the selected output format, or the default if none.
    #[cfg(test)]
    pub fn output_format_or_default(&self) -> FormatOption {
        // structop makes sure only one or zero format flags are
        // selected. So if none are selected, the default is chosen
        self.output_format().unwrap_or_default()
    }

    pub fn config_format(&self) -> Option<ConfigFormat> {
        self.hardware_config.as_deref().and_then(get_config_format)
    }

    pub fn got_layout_format(&self) -> ConfigFormat {
        // It is safe to unwrap here, since the provided argument to `--got-online-change` has been checked with `validate_config`
        get_config_format(&self.got_layout_file).unwrap()
    }

    /// Returns the location where the build artifacts should be stored / output
    pub fn get_build_location(&self) -> Option<PathBuf> {
        match &self.commands {
            Some(SubCommands::Build { build_location, .. }) => {
                build_location.as_deref().or(Some("build")).map(PathBuf::from)
            }
            _ => None,
        }
    }

    pub fn get_lib_location(&self) -> Option<PathBuf> {
        match &self.commands {
            Some(SubCommands::Build { build_location, lib_location, .. }) => {
                lib_location.as_deref().or(build_location.as_deref()).or(Some("build")).map(PathBuf::from)
            }
            _ => None,
        }
    }

    pub fn get_config_options(&self) -> Option<(ConfigOption, ConfigFormat)> {
        let Some(SubCommands::Config { format, option, .. }) = &self.commands else { return None };
        Some((*option, *format))
    }

    pub(crate) fn get_build_configuration(&self) -> Result<Option<PathBuf>, Diagnostic> {
        if !self.has_config()? {
            return Ok(None);
        }
        let current_dir = env::current_dir()?;
        let result = self
            .commands
            .as_ref()
            .and_then(|it| it.get_build_configuration())
            .map(PathBuf::from)
            .map(|it| {
                if it.is_relative() {
                    //Make the build path absolute
                    current_dir.join(it)
                } else {
                    it
                }
            })
            .or_else(|| Some(super::get_config(&current_dir)));
        Ok(result.clone())
    }

    fn has_config(&self) -> Result<bool, Diagnostic> {
        let res = match &self.commands {
            None | Some(SubCommands::Explain { .. }) => false,
            Some(SubCommands::Build { .. }) | Some(SubCommands::Check { .. }) => true,
            Some(SubCommands::Config { build_config, .. }) => {
                let current_dir = env::current_dir()?;
                build_config.is_some() || super::get_config(&current_dir).exists()
            }
        };
        Ok(res)
    }

    pub fn get_error_configuration(&self) -> Result<Option<DiagnosticsConfiguration>> {
        let Some(config) = &self.error_config else {
            return Ok(None);
        };
        let config_path: PathBuf = config.into();
        if config_path.exists() {
            let error_config = std::fs::read_to_string(config_path)?;
            let configuration: DiagnosticsConfiguration = serde_json::de::from_str(&error_config)?;
            Ok(Some(configuration))
        } else {
            bail!("{} does not exist", config_path.to_string_lossy())
        }
    }
}

#[cfg(test)]
mod cli_tests {
    use crate::cli::ConfigOption;

    use super::{CompileParameters, SubCommands};
    use clap::ErrorKind;
    use plc::{output::FormatOption, ConfigFormat, ErrorFormat, OptimizationLevel};
    use pretty_assertions::assert_eq;
    use std::ffi::OsStr;
    use std::fmt::Debug;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        CompileParameters::command().debug_assert()
    }

    fn expect_argument_error<T>(args: &[T], expected_error_kind: ErrorKind)
    where
        T: Debug + AsRef<OsStr> + AsRef<str>,
    {
        let params = CompileParameters::parse(args);
        match params {
            Err(e) => {
                assert_eq!(e.kind(), expected_error_kind);
            }
            Ok(p) => panic!("expected error, but found none. arguments: {args:?}. params: {p:?}"),
        }
    }
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (&["plc", $($x),*]);
    }

    #[test]
    fn multiple_output_formats_results_in_error() {
        expect_argument_error(vec_of_strings!["input.st", "--ir", "--shared"], ErrorKind::ArgumentConflict);
        expect_argument_error(
            vec_of_strings!["input.st", "--ir", "--shared", "--pic"],
            ErrorKind::ArgumentConflict,
        );
        expect_argument_error(
            vec_of_strings!["input.st", "--ir", "--shared", "--pic", "--bc"],
            ErrorKind::ArgumentConflict,
        );
        expect_argument_error(
            vec_of_strings!["input.st", "--ir", "--relocatable"],
            ErrorKind::ArgumentConflict,
        );
    }

    #[test]
    fn unknown_arguments_results_in_error() {
        expect_argument_error(vec_of_strings!["input.st", "--unknown"], ErrorKind::UnknownArgument);
        expect_argument_error(vec_of_strings!["input.st", "--ir", "--unknown"], ErrorKind::UnknownArgument);
        expect_argument_error(vec_of_strings!["input.st", "--ir", "-u"], ErrorKind::UnknownArgument);
    }

    //#[test]
    //fn valid_output_files() {
    //    //short -o
    //    let parameters =
    //        CompileParameters::parse(vec_of_strings!("input.st", "--ir", "-o", "myout.out")).unwrap();
    //    assert_eq!(parameters.output_name(), "myout.out".to_string());

    //    //long --output
    //    let parameters =
    //        CompileParameters::parse(vec_of_strings!("input.st", "--ir", "--output", "myout2.out")).unwrap();
    //    assert_eq!(parameters.output_name(), "myout2.out".to_string());
    //}

    // #[test]
    // fn test_default_output_names() {
    //     let parameters = CompileParameters::parse(vec_of_strings!("alpha.st", "--ir")).unwrap();
    //     assert_eq!(parameters.output_name(), "alpha.ll".to_string());

    //     let parameters = CompileParameters::parse(vec_of_strings!("bravo", "--shared")).unwrap();
    //     assert_eq!(parameters.output_name(), "bravo.so".to_string());

    //     let parameters = CompileParameters::parse(vec_of_strings!("examples/charlie.st", "--pic")).unwrap();
    //     assert_eq!(parameters.output_name(), "charlie.so".to_string());

    //     let parameters =
    //         CompileParameters::parse(vec_of_strings!("examples/test/delta.st", "--static", "-c")).unwrap();
    //     assert_eq!(parameters.output_name(), "delta.o".to_string());

    //     let parameters = CompileParameters::parse(vec_of_strings!("examples/test/echo", "--bc")).unwrap();
    //     assert_eq!(parameters.output_name(), "echo.bc".to_string());

    //     let parameters = CompileParameters::parse(vec_of_strings!("examples/test/echo.st")).unwrap();
    //     assert_eq!(parameters.output_name(), "echo".to_string());
    // }

    #[test]
    fn test_target_triple() {
        let parameters =
            CompileParameters::parse(vec_of_strings!("alpha.st", "--target", "x86_64-linux-gnu")).unwrap();

        assert_eq!(parameters.target, Some("x86_64-linux-gnu".into()));
    }

    #[test]
    fn test_optimization_levels() {
        let parameters = CompileParameters::parse(vec_of_strings!("alpha.st")).unwrap();

        assert_eq!(parameters.optimization, OptimizationLevel::Default);
        let parameters = CompileParameters::parse(vec_of_strings!("alpha.st", "-Onone")).unwrap();

        assert_eq!(parameters.optimization, OptimizationLevel::None);
        let parameters =
            CompileParameters::parse(vec_of_strings!("alpha.st", "--optimization", "none")).unwrap();
        assert_eq!(parameters.optimization, OptimizationLevel::None);

        let parameters = CompileParameters::parse(vec_of_strings!("alpha.st", "-Oless")).unwrap();

        assert_eq!(parameters.optimization, OptimizationLevel::Less);
        let parameters =
            CompileParameters::parse(vec_of_strings!("alpha.st", "--optimization", "less")).unwrap();
        assert_eq!(parameters.optimization, OptimizationLevel::Less);
        let parameters = CompileParameters::parse(vec_of_strings!("alpha.st", "-Odefault")).unwrap();

        assert_eq!(parameters.optimization, OptimizationLevel::Default);
        let parameters =
            CompileParameters::parse(vec_of_strings!("alpha.st", "--optimization", "default")).unwrap();
        assert_eq!(parameters.optimization, OptimizationLevel::Default);
        let parameters = CompileParameters::parse(vec_of_strings!("alpha.st", "-Oaggressive")).unwrap();

        assert_eq!(parameters.optimization, OptimizationLevel::Aggressive);
        let parameters =
            CompileParameters::parse(vec_of_strings!("alpha.st", "--optimization", "aggressive")).unwrap();
        assert_eq!(parameters.optimization, OptimizationLevel::Aggressive);
    }

    #[test]
    fn test_default_format() {
        let parameters = CompileParameters::parse(vec_of_strings!("alpha.st", "--ir")).unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::IR);

        let parameters = CompileParameters::parse(vec_of_strings!("bravo", "--shared")).unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::Shared);

        let parameters = CompileParameters::parse(vec_of_strings!("charlie", "--pic")).unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::PIC);

        let parameters =
            CompileParameters::parse(vec_of_strings!("examples/test/delta.st", "--static")).unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::Static);

        let parameters = CompileParameters::parse(vec_of_strings!("examples/test/echo", "--bc")).unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::Bitcode);

        let parameters = CompileParameters::parse(vec_of_strings!("examples/test/echo.st")).unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::Static);
    }

    #[test]
    fn encoding_resolution() {
        let parameters =
            CompileParameters::parse(vec_of_strings!("input.st", "--ir", "--encoding", "cp1252")).unwrap();
        assert_eq!(parameters.encoding, Some(encoding_rs::WINDOWS_1252));
        let parameters =
            CompileParameters::parse(vec_of_strings!("input.st", "--ir", "--encoding", "windows-1252"))
                .unwrap();
        assert_eq!(parameters.encoding, Some(encoding_rs::WINDOWS_1252));
    }

    #[test]
    fn invalid_encoding_resolution() {
        expect_argument_error(
            vec_of_strings!("input.st", "--ir", "--encoding", "invalid"),
            ErrorKind::ValueValidation,
        );
    }

    #[test]
    fn valid_output_formats() {
        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--ir")).unwrap();
        assert!(parameters.output_ir);
        assert!(!parameters.print_ast);
        assert!(!parameters.print_ast_lowered);
        assert!(!parameters.output_bit_code);
        assert!(!parameters.output_obj_code);
        assert!(!parameters.output_pic_obj);
        assert!(!parameters.output_shared_obj);
        assert!(!parameters.output_reloc_code);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--ast")).unwrap();
        assert!(!parameters.output_ir);
        assert!(parameters.print_ast);
        assert!(!parameters.print_ast_lowered);
        assert!(!parameters.output_bit_code);
        assert!(!parameters.output_obj_code);
        assert!(!parameters.output_pic_obj);
        assert!(!parameters.output_shared_obj);
        assert!(!parameters.output_reloc_code);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--ast-lowered")).unwrap();
        assert!(!parameters.output_ir);
        assert!(!parameters.print_ast);
        assert!(parameters.print_ast_lowered);
        assert!(!parameters.output_bit_code);
        assert!(!parameters.output_obj_code);
        assert!(!parameters.output_pic_obj);
        assert!(!parameters.output_shared_obj);
        assert!(!parameters.output_reloc_code);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--bc")).unwrap();
        assert!(!parameters.output_ir);
        assert!(!parameters.print_ast);
        assert!(!parameters.print_ast_lowered);
        assert!(parameters.output_bit_code);
        assert!(!parameters.output_obj_code);
        assert!(!parameters.output_pic_obj);
        assert!(!parameters.output_shared_obj);
        assert!(!parameters.output_reloc_code);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--static")).unwrap();
        assert!(!parameters.output_ir);
        assert!(!parameters.print_ast);
        assert!(!parameters.print_ast_lowered);
        assert!(!parameters.output_bit_code);
        assert!(parameters.output_obj_code);
        assert!(!parameters.output_pic_obj);
        assert!(!parameters.output_shared_obj);
        assert!(!parameters.output_reloc_code);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--pic")).unwrap();
        assert!(!parameters.output_ir);
        assert!(!parameters.print_ast);
        assert!(!parameters.print_ast_lowered);
        assert!(!parameters.output_bit_code);
        assert!(!parameters.output_obj_code);
        assert!(parameters.output_pic_obj);
        assert!(!parameters.output_shared_obj);
        assert!(!parameters.output_reloc_code);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--shared")).unwrap();
        assert!(!parameters.output_ir);
        assert!(!parameters.print_ast);
        assert!(!parameters.print_ast_lowered);
        assert!(!parameters.output_bit_code);
        assert!(!parameters.output_obj_code);
        assert!(!parameters.output_pic_obj);
        assert!(parameters.output_shared_obj);
        assert!(!parameters.output_reloc_code);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--relocatable")).unwrap();
        assert!(!parameters.output_ir);
        assert!(!parameters.print_ast);
        assert!(!parameters.print_ast_lowered);
        assert!(!parameters.output_bit_code);
        assert!(!parameters.output_obj_code);
        assert!(!parameters.output_pic_obj);
        assert!(!parameters.output_shared_obj);
        assert!(parameters.output_reloc_code);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st")).unwrap();
        assert!(!parameters.output_ir);
        assert!(!parameters.print_ast);
        assert!(!parameters.print_ast_lowered);
        assert!(!parameters.output_bit_code);
        assert!(!parameters.output_obj_code);
        assert!(!parameters.output_pic_obj);
        assert!(!parameters.output_shared_obj);
        assert!(!parameters.output_reloc_code);
    }

    #[test]
    fn library_path_added() {
        let parameters = CompileParameters::parse(vec_of_strings!(
            "input.st",
            "--library-path",
            "xxx",
            "-L",
            "test",
            "-L.",
            "-L/tmp"
        ))
        .unwrap();
        assert_eq!(parameters.library_paths, vec!["xxx", "test", ".", "/tmp"]);
    }

    #[test]
    fn libraries_added() {
        let parameters =
            CompileParameters::parse(vec_of_strings!("input.st", "-l", "test", "-lc", "--library", "xx"))
                .unwrap();
        assert_eq!(parameters.libraries, vec!["test", "c", "xx"]);
    }

    #[test]
    fn cli_supports_version() {
        match CompileParameters::parse(vec_of_strings!("input.st", "--version")) {
            Ok(version) => assert!(version.build_info),
            _ => panic!("expected the build info flag to be true"),
        }
    }

    #[test]
    fn cli_supports_help() {
        match CompileParameters::parse(vec_of_strings!("input.st", "--help")) {
            Ok(_) => panic!("expected help output, but found OK"),
            Err(e) => assert_eq!(e.kind(), ErrorKind::DisplayHelp),
        }
    }

    #[test]
    fn build_subcommand() {
        let parameters = CompileParameters::parse(vec_of_strings!(
            "build",
            "src/ProjectPlc.json",
            "--build-location",
            "bin/build",
            "--lib-location",
            "bin/build/libs",
            "--sysroot",
            "sysroot1",
            "--target",
            "targettest",
            "--linker",
            "cc"
        ))
        .unwrap();
        if let Some(commands) = parameters.commands {
            match commands {
                SubCommands::Build { build_config, build_location, lib_location, .. } => {
                    assert_eq!(build_config, Some("src/ProjectPlc.json".to_string()));
                    assert_eq!(build_location, Some("bin/build".to_string()));
                    assert_eq!(lib_location, Some("bin/build/libs".to_string()));
                }
                _ => panic!("Unexpected command"),
            };
            assert_eq!(parameters.sysroot, Some("sysroot1".to_string()));
            assert_eq!(parameters.target, Some("targettest".into()));
            assert_eq!(parameters.linker, Some("cc".to_string()));
        }
    }

    #[test]
    fn check_subcommand() {
        let parameters = CompileParameters::parse(vec_of_strings!("check", "src/ProjectPlc.json")).unwrap();
        if let Some(commands) = parameters.commands {
            match commands {
                SubCommands::Check { build_config } => {
                    assert_eq!(build_config, Some("src/ProjectPlc.json".to_string()));
                }
                _ => panic!("Unexpected command"),
            };
        }
    }

    #[test]
    fn config_subcommand() {
        let parameters =
            CompileParameters::parse(vec_of_strings!("config", "src/ProjectPlc.json", "diagnostics"))
                .unwrap();
        if let Some(commands) = parameters.commands {
            match commands {
                SubCommands::Config { format, option, build_config } => {
                    assert_eq!(build_config, Some("src/ProjectPlc.json".to_string()));
                    assert_eq!(option, ConfigOption::Diagnostics);
                    assert_eq!(format, ConfigFormat::JSON)
                }
                _ => panic!("Unexpected command"),
            };
        }

        let parameters =
            CompileParameters::parse(vec_of_strings!("config", "--format=toml", "schema")).unwrap();
        if let Some(commands) = parameters.commands {
            match commands {
                SubCommands::Config { format, option, build_config } => {
                    assert_eq!(build_config, None);
                    assert_eq!(option, ConfigOption::Schema);
                    assert_eq!(format, ConfigFormat::TOML)
                }
                _ => panic!("Unexpected command"),
            };
        }
    }

    #[test]
    fn include_files_added() {
        let parameters = CompileParameters::parse(vec_of_strings!(
            "input.st",
            "-i",
            "include1",
            "-i",
            "include2",
            "--include",
            "include3"
        ))
        .unwrap();
        assert_eq!(parameters.includes, vec!["include1", "include2", "include3"]);
    }

    #[test]
    fn config_option_set() {
        let parameters =
            CompileParameters::parse(vec_of_strings!("foo", "--hardware-conf=conf.json")).unwrap();
        assert_eq!(parameters.hardware_config, Some("conf.json".to_string()));
        assert_eq!(parameters.config_format().unwrap(), ConfigFormat::JSON);
        let parameters =
            CompileParameters::parse(vec_of_strings!("foo", "--hardware-conf=conf.toml")).unwrap();
        assert_eq!(parameters.hardware_config, Some("conf.toml".to_string()));
        assert_eq!(parameters.config_format().unwrap(), ConfigFormat::TOML);

        expect_argument_error(vec_of_strings!("foo", "--hardware-conf=foo"), ErrorKind::ValueValidation);
        expect_argument_error(vec_of_strings!("foo", "--hardware-conf=conf.foo"), ErrorKind::ValueValidation);
        expect_argument_error(vec_of_strings!("foo", "--hardware-conf=conf.xml"), ErrorKind::ValueValidation);
    }

    #[test]
    fn error_format_default_set() {
        // make sure the default error format is set
        let params = CompileParameters::parse(vec_of_strings!("input.st")).unwrap();
        assert_eq!(params.error_format, ErrorFormat::Rich);
    }

    #[test]
    fn error_format_set() {
        // set clang as error format
        let params = CompileParameters::parse(vec_of_strings!("input.st", "--error-format=clang")).unwrap();
        assert_eq!(params.error_format, ErrorFormat::Clang);
        // set invalid error format
        expect_argument_error(vec_of_strings!("input.st", "--error-format=nothing"), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_gdwarf_and_debug_mutually_exclusive() {
        assert!(CompileParameters::parse(vec_of_strings!("input.st", "--debug", "--gdwarf", "2")).is_err());
        assert!(CompileParameters::parse(vec_of_strings!("input.st", "-g", "--gdwarf", "4")).is_err());
        assert!(CompileParameters::parse(vec_of_strings!(
            "input.st",
            "--debug-variables",
            "--gdwarf-variables",
            "3"
        ))
        .is_err());
    }

    #[test]
    fn test_dwarf_version_override() {
        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--gdwarf", "2")).unwrap();
        assert_eq!(parameters.gdwarf_version, Some(2));

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--gdwarf", "3")).unwrap();
        assert_eq!(parameters.gdwarf_version, Some(3));

        let parameters =
            CompileParameters::parse(vec_of_strings!("input.st", "--gdwarf-variables", "4")).unwrap();
        assert_eq!(parameters.gdwarf_varinfo_version, Some(4));
    }

    #[test]
    fn invalid_dwarf_version() {
        let error = CompileParameters::parse(vec_of_strings!("input.st", "--gdwarf", "1")).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidValue);
        let inner = &error.info;
        assert_eq!(inner[1], "1");

        let error =
            CompileParameters::parse(vec_of_strings!("input.st", "--gdwarf-variables", "99")).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidValue);
        let inner = &error.info;
        assert_eq!(inner[1], "99");

        let error = CompileParameters::parse(vec_of_strings!("input.st", "--gdwarf", "abc")).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidValue);
        let inner = &error.info;
        assert_eq!(inner[1], "abc");
    }

    #[test]
    fn dwarf_version_override_arg_requries_value() {
        let error = CompileParameters::parse(vec_of_strings!("input.st", "--gdwarf")).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::EmptyValue);

        let error = CompileParameters::parse(vec_of_strings!("input.st", "--gdwarf-variables")).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::EmptyValue);
    }
}
