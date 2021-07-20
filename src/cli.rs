// Copyright (c) 2021 Ghaith Hachem and Mathias Rieder
use std::path::Path;
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

pub type ParameterError = structopt::clap::Error;

#[derive(StructOpt, Debug)]
#[structopt(
    group = ArgGroup::with_name("format") /* .required(true) */,
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

    #[structopt(
        long,
        name = "target-triple",
        help = "A target-tripple supported by LLVM"
    )]
    pub target: Option<String>,

    #[structopt(
        name = "input-files",
        help = "Read input from <input-files>, may be a glob expression like 'src/**/*' or a sequence of files",
        required = true,
        min_values = 1
    )]
    // having a vec allows bash to resolve *.st itself
    pub input: Vec<String>,
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
        self.output_format().unwrap_or_else(|| DEFAULT_FORMAT)
    }

    /// return the output filename with the correct ending
    pub fn output_name(&self) -> Option<String> {
        if let Some(n) = &self.output {
            Some(n.to_string())
        } else {
            let ending = match self.output_format_or_default() {
                FormatOption::Bitcode => "bc",
                FormatOption::Static => "o",
                FormatOption::Shared => "so",
                FormatOption::PIC => "so",
                FormatOption::IR => "ir",
            };

            let output_name = self.input.first().unwrap();
            let basename = Path::new(output_name).file_stem()?.to_str()?;
            Some(format!("{}.{}", basename, ending))
        }
    }
}

#[cfg(test)]
mod cli_tests {
    use super::{CompileParameters, FormatOption, ParameterError};
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
            CompileParameters::parse(vec_of_strings!("examples/test/delta.st", "--static"))
                .unwrap();
        assert_eq!(parameters.output_name().unwrap(), "delta.o".to_string());

        let parameters =
            CompileParameters::parse(vec_of_strings!("examples/test/echo", "--bc")).unwrap();
        assert_eq!(parameters.output_name().unwrap(), "echo.bc".to_string());
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
}
