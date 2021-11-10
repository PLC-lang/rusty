// Copyright (c) 2021 Ghaith Hachem and Mathias Rieder
use encoding_rs::Encoding;
use std::{
    ffi::{OsStr, OsString},
    path::Path,
};
use structopt::{clap::ArgGroup, StructOpt};

#[derive(PartialEq, Debug)]
pub enum FormatOption {
    Static,
    PIC,
    Shared,
    Bitcode,
    IR,
}

// => Set the default output format here:
const DEFAULT_FORMAT: FormatOption = FormatOption::Static;
const DEFAULT_OUTPUT_NAME: &str = "out";

pub type ParameterError = structopt::clap::Error;

#[derive(StructOpt, Debug)]
#[structopt(
    group = ArgGroup::with_name("format"),
    about = "IEC61131-3 Structured Text compiler powered by Rust & LLVM "
)]
pub struct CompileParameters {
    #[structopt(
        short,
        long,
        name = "output-file",
        help = "Write output to <output-file>"
    )]
    pub output: Option<String>,

    #[structopt(
        long = "ir",
        group = "format",
        help = "Emit IR (LLVM Intermediate Representation) as output"
    )]
    pub output_ir: bool,

    #[structopt(
        long = "shared",
        group = "format",
        help = "Emit a shared object as output"
    )]
    pub output_shared_obj: bool,

    #[structopt(
        long = "pic",
        group = "format",
        help = "Emit PIC (Position Independent Code) as output"
    )]
    pub output_pic_obj: bool,

    #[structopt(long = "static", group = "format", help = "Emit an object as output")]
    pub output_obj_code: bool,

    #[structopt(
        long = "bc",
        group = "format",
        help = "Emit binary IR (binary representation of LLVM-IR) as output"
    )]
    pub output_bit_code: bool,

    #[structopt(short = "c", help = "Do not link after compiling object code")]
    pub skip_linking: bool,

    #[structopt(
        long,
        name = "target-triple",
        help = "A target-tripple supported by LLVM"
    )]
    pub target: Option<String>,

    #[structopt(
        long,
        name = "encoding",
        help = "The file encoding used to read the input-files, as defined by the Encoding Standard",
        parse(try_from_str = parse_encoding),
    )]
    pub encoding: Option<&'static Encoding>,

    #[structopt(
        name = "input-files",
        help = "Read input from <input-files>, may be a glob expression like 'src/**/*' or a sequence of files",
        required = true,
        min_values = 1
    )]
    // having a vec allows bash to resolve *.st itself
    pub input: Vec<String>,

    #[structopt(
        name = "library-path",
        long,
        short = "L",
        help = "Search path for libraries, used for linking"
    )]
    pub library_pathes: Vec<String>,

    #[structopt(name = "library", long, short = "l", help = "Library name to link")]
    pub libraries: Vec<String>,

    #[structopt(long, name = "sysroot", help = "Path to system root, used for linking")]
    pub sysroot: Option<String>,
}

fn parse_encoding(encoding: &str) -> Result<&'static Encoding, String> {
    Encoding::for_label(encoding.as_bytes()).ok_or(format!("Unknown encoding {}", encoding))
}

impl CompileParameters {
    pub fn parse(args: Vec<String>) -> Result<CompileParameters, ParameterError> {
        CompileParameters::from_iter_safe(args)
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
        } else if self.output_obj_code {
            Some(FormatOption::Static)
        } else {
            None
        }
    }

    /// return the selected output format, or the default if none.
    pub fn output_format_or_default(&self) -> FormatOption {
        // structop makes sure only one or zero format flags are
        // selected. So if none are selected, the default is chosen
        self.output_format().unwrap_or(DEFAULT_FORMAT)
    }

    /// return the output filename with the correct ending
    pub fn output_name(&self) -> Option<String> {
        let out_format = self.output_format_or_default();
        if let Some(n) = &self.output {
            Some(n.to_string())
        } else {
            let ending = match out_format {
                FormatOption::Bitcode => ".bc",
                FormatOption::Static if self.skip_linking => ".o",
                FormatOption::Static => "",
                FormatOption::Shared | FormatOption::PIC => ".so",
                FormatOption::IR => ".ir",
            };

            let output_name = self.input.first().map(String::as_str);
            let basename = output_name
                .and_then(|it| Path::new(it).file_stem())
                .and_then(OsStr::to_str)
                .unwrap_or(DEFAULT_OUTPUT_NAME);
            Some(format!("{}{}", basename, ending))
        }
    }
}

#[cfg(test)]
mod cli_tests {
    use super::{CompileParameters, FormatOption, ParameterError};
    use pretty_assertions::assert_eq;
    use structopt::clap::ErrorKind;

    fn expect_argument_error(args: Vec<String>, expected_error_kind: ErrorKind) {
        let params = CompileParameters::parse(args.clone());
        match params {
            Err(ParameterError { kind, .. }) => {
                assert_eq!(kind, expected_error_kind);
            }
            Ok(p) => panic!(
                "expected error, but found none. arguments: {:?}. params: {:?}",
                args, p
            ),
        }
    }
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec!["rustyc".to_string(), $($x.to_string()),*]);
    }

    #[test]
    fn missing_parameters_results_in_error() {
        // no arguments
        expect_argument_error(vec![], ErrorKind::MissingRequiredArgument);
        // no input file
        expect_argument_error(vec_of_strings!["--ir"], ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn multiple_output_formats_results_in_error() {
        expect_argument_error(
            vec_of_strings!["input.st", "--ir", "--shared"],
            ErrorKind::ArgumentConflict,
        );
        expect_argument_error(
            vec_of_strings!["input.st", "--ir", "--shared", "--pic"],
            ErrorKind::ArgumentConflict,
        );
        expect_argument_error(
            vec_of_strings!["input.st", "--ir", "--shared", "--pic", "--bc"],
            ErrorKind::ArgumentConflict,
        );
    }

    #[test]
    fn unknown_arguments_results_in_error() {
        expect_argument_error(
            vec_of_strings!["input.st", "--unknown"],
            ErrorKind::UnknownArgument,
        );
        expect_argument_error(
            vec_of_strings!["input.st", "--ir", "--unknown"],
            ErrorKind::UnknownArgument,
        );
        expect_argument_error(
            vec_of_strings!["input.st", "--ir", "-u"],
            ErrorKind::UnknownArgument,
        );
    }

    #[test]
    fn valid_output_files() {
        //short -o
        let parameters =
            CompileParameters::parse(vec_of_strings!("input.st", "--ir", "-o", "myout.out"))
                .unwrap();
        assert_eq!(parameters.output_name().unwrap(), "myout.out".to_string());

        //long --output
        let parameters = CompileParameters::parse(vec_of_strings!(
            "input.st",
            "--ir",
            "--output",
            "myout2.out"
        ))
        .unwrap();
        assert_eq!(parameters.output_name().unwrap(), "myout2.out".to_string());
    }

    #[test]
    fn test_default_output_names() {
        let parameters = CompileParameters::parse(vec_of_strings!("alpha.st", "--ir")).unwrap();
        assert_eq!(parameters.output_name().unwrap(), "alpha.ir".to_string());

        let parameters = CompileParameters::parse(vec_of_strings!("bravo", "--shared")).unwrap();
        assert_eq!(parameters.output_name().unwrap(), "bravo.so".to_string());

        let parameters =
            CompileParameters::parse(vec_of_strings!("examples/charlie.st", "--pic")).unwrap();
        assert_eq!(parameters.output_name().unwrap(), "charlie.so".to_string());

        let parameters =
            CompileParameters::parse(vec_of_strings!("examples/test/delta.st", "--static", "-c"))
                .unwrap();
        assert_eq!(parameters.output_name().unwrap(), "delta.o".to_string());

        let parameters =
            CompileParameters::parse(vec_of_strings!("examples/test/echo", "--bc")).unwrap();
        assert_eq!(parameters.output_name().unwrap(), "echo.bc".to_string());

        let parameters =
            CompileParameters::parse(vec_of_strings!("examples/test/echo.st")).unwrap();
        assert_eq!(parameters.output_name().unwrap(), "echo".to_string());
    }

    #[test]
    fn test_target_triple() {
        let parameters =
            CompileParameters::parse(vec_of_strings!("alpha.st", "--target", "x86_64-linux-gnu"))
                .unwrap();

        assert_eq!(parameters.target, Some("x86_64-linux-gnu".to_string()));
    }

    #[test]
    fn test_default_format() {
        let parameters = CompileParameters::parse(vec_of_strings!("alpha.st", "--ir")).unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::IR);

        let parameters = CompileParameters::parse(vec_of_strings!("bravo", "--shared")).unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::Shared);

        let parameters =
            CompileParameters::parse(vec_of_strings!("examples/charlie.st", "--pic")).unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::PIC);

        let parameters =
            CompileParameters::parse(vec_of_strings!("examples/test/delta.st", "--static"))
                .unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::Static);

        let parameters =
            CompileParameters::parse(vec_of_strings!("examples/test/echo", "--bc")).unwrap();
        assert_eq!(parameters.output_format_or_default(), FormatOption::Bitcode);

        let parameters =
            CompileParameters::parse(vec_of_strings!("examples/test/echo.st")).unwrap();
        assert_eq!(parameters.output_format_or_default(), super::DEFAULT_FORMAT);
    }

    #[test]
    fn encoding_resolution() {
        let parameters =
            CompileParameters::parse(vec_of_strings!("input.st", "--ir", "--encoding", "cp1252"))
                .unwrap();
        assert_eq!(parameters.encoding, Some(encoding_rs::WINDOWS_1252));
        let parameters = CompileParameters::parse(vec_of_strings!(
            "input.st",
            "--ir",
            "--encoding",
            "windows-1252"
        ))
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
        assert_eq!(parameters.output_ir, true);
        assert_eq!(parameters.output_bit_code, false);
        assert_eq!(parameters.output_obj_code, false);
        assert_eq!(parameters.output_pic_obj, false);
        assert_eq!(parameters.output_shared_obj, false);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--bc")).unwrap();
        assert_eq!(parameters.output_ir, false);
        assert_eq!(parameters.output_bit_code, true);
        assert_eq!(parameters.output_obj_code, false);
        assert_eq!(parameters.output_pic_obj, false);
        assert_eq!(parameters.output_shared_obj, false);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--static")).unwrap();
        assert_eq!(parameters.output_ir, false);
        assert_eq!(parameters.output_bit_code, false);
        assert_eq!(parameters.output_obj_code, true);
        assert_eq!(parameters.output_pic_obj, false);
        assert_eq!(parameters.output_shared_obj, false);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--pic")).unwrap();
        assert_eq!(parameters.output_ir, false);
        assert_eq!(parameters.output_bit_code, false);
        assert_eq!(parameters.output_obj_code, false);
        assert_eq!(parameters.output_pic_obj, true);
        assert_eq!(parameters.output_shared_obj, false);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st", "--shared")).unwrap();
        assert_eq!(parameters.output_ir, false);
        assert_eq!(parameters.output_bit_code, false);
        assert_eq!(parameters.output_obj_code, false);
        assert_eq!(parameters.output_pic_obj, false);
        assert_eq!(parameters.output_shared_obj, true);

        let parameters = CompileParameters::parse(vec_of_strings!("input.st")).unwrap();
        assert_eq!(parameters.output_ir, false);
        assert_eq!(parameters.output_bit_code, false);
        assert_eq!(parameters.output_obj_code, false);
        assert_eq!(parameters.output_pic_obj, false);
        assert_eq!(parameters.output_shared_obj, false);
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
        assert_eq!(parameters.library_pathes, vec!["xxx", "test", ".", "/tmp"]);
    }

    #[test]
    fn libraries_added() {
        let parameters = CompileParameters::parse(vec_of_strings!(
            "input.st",
            "-l",
            "test",
            "-lc",
            "--library",
            "xx"
        ))
        .unwrap();
        assert_eq!(parameters.libraries, vec!["test", "c", "xx"]);
    }

    #[test]
    fn cli_supports_version() {
        match CompileParameters::parse(vec_of_strings!("input.st", "--version")) {
            Ok(_) => panic!("expected version output, but found OK"),
            Err(ParameterError { kind, .. }) => assert_eq!(kind, ErrorKind::VersionDisplayed),
        }
    }

    #[test]
    fn cli_supports_help() {
        match CompileParameters::parse(vec_of_strings!("input.st", "--help")) {
            Ok(_) => panic!("expected help output, but found OK"),
            Err(ParameterError { kind, .. }) => assert_eq!(kind, ErrorKind::HelpDisplayed),
        }
    }

    #[test]
    fn sysroot_added() {
        let parameters =
            CompileParameters::parse(vec_of_strings!("input.st", "--sysroot", "path/to/sysroot"))
                .unwrap();
        assert_eq!(parameters.sysroot, Some("path/to/sysroot".to_string()));
    }
}
