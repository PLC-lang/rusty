/// Copyright (c) 2021 Ghaith Hachem and Mathias Rieder
use structopt::{clap::ArgGroup, StructOpt};

pub type ParameterError = structopt::clap::Error;

pub fn parse_parameters(args: Vec<String>) -> Result<CompileParameters, ParameterError> {
    CompileParameters::from_iter_safe(args)
}

#[derive(StructOpt)]
#[structopt(
        group = ArgGroup::with_name("format").required(true),
        about = "IEC61131-3 Structured Text compiler powered by Rust & LLVM "
    )]
pub struct CompileParameters {
    #[structopt(name = "input-file", help = "Read input from <input-file>")]
    pub input: String,

    #[structopt(
        short,
        long,
        name = "output-file",
        default_value = "a.out",
        help = "Write output to <output-file>"
    )]
    pub output: String,

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
}

#[cfg(test)]
mod cli_tests {
    use super::parse_parameters;
    use super::ParameterError;
    use structopt::clap::ErrorKind;

    fn expect_argument_error(args: Vec<String>, expected_error_kind: ErrorKind) {
        let params = parse_parameters(args.clone());
        match params {
            Err(ParameterError { kind, .. }) => {
                assert_eq!(kind, expected_error_kind);
            }
            _ => panic!("expected error, but found none. arguments: {:?}", args),
        }
    }
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec!["rustyc".to_string(), $($x.to_string()),*]);
    }

    #[test]
    fn missing_parameters_results_in_error() {
        // no arguments
        expect_argument_error(vec![], ErrorKind::MissingRequiredArgument);
        // only input file, missing format
        expect_argument_error(
            vec_of_strings!["input.st"],
            ErrorKind::MissingRequiredArgument,
        );
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
            parse_parameters(vec_of_strings!("input.st", "--ir", "-o", "myout.out")).unwrap();
        assert_eq!(parameters.output, "myout.out".to_string());
        //long --output
        let parameters = parse_parameters(vec_of_strings!(
            "input.st",
            "--ir",
            "--output",
            "myout2.out"
        ))
        .unwrap();
        assert_eq!(parameters.output, "myout2.out".to_string());
    }

    #[test]
    fn valid_output_formats() {
        let parameters = parse_parameters(vec_of_strings!("input.st", "--ir")).unwrap();
        assert_eq!(parameters.output_ir, true);
        assert_eq!(parameters.output_bit_code, false);
        assert_eq!(parameters.output_obj_code, false);
        assert_eq!(parameters.output_pic_obj, false);
        assert_eq!(parameters.output_shared_obj, false);

        let parameters = parse_parameters(vec_of_strings!("input.st", "--bc")).unwrap();
        assert_eq!(parameters.output_ir, false);
        assert_eq!(parameters.output_bit_code, true);
        assert_eq!(parameters.output_obj_code, false);
        assert_eq!(parameters.output_pic_obj, false);
        assert_eq!(parameters.output_shared_obj, false);

        let parameters = parse_parameters(vec_of_strings!("input.st", "--static")).unwrap();
        assert_eq!(parameters.output_ir, false);
        assert_eq!(parameters.output_bit_code, false);
        assert_eq!(parameters.output_obj_code, true);
        assert_eq!(parameters.output_pic_obj, false);
        assert_eq!(parameters.output_shared_obj, false);

        let parameters = parse_parameters(vec_of_strings!("input.st", "--pic")).unwrap();
        assert_eq!(parameters.output_ir, false);
        assert_eq!(parameters.output_bit_code, false);
        assert_eq!(parameters.output_obj_code, false);
        assert_eq!(parameters.output_pic_obj, true);
        assert_eq!(parameters.output_shared_obj, false);

        let parameters = parse_parameters(vec_of_strings!("input.st", "--shared")).unwrap();
        assert_eq!(parameters.output_ir, false);
        assert_eq!(parameters.output_bit_code, false);
        assert_eq!(parameters.output_obj_code, false);
        assert_eq!(parameters.output_pic_obj, false);
        assert_eq!(parameters.output_shared_obj, true);
    }

    #[test]
    fn cli_supports_version() {
        match parse_parameters(vec_of_strings!("input.st", "--version")) {
            Ok(_) => panic!("expected version output, but found OK"),
            Err(ParameterError { kind, .. }) => assert_eq!(kind, ErrorKind::VersionDisplayed),
        }
    }

    #[test]
    fn cli_supports_help() {
        match parse_parameters(vec_of_strings!("input.st", "--help")) {
            Ok(_) => panic!("expected help output, but found OK"),
            Err(ParameterError { kind, .. }) => assert_eq!(kind, ErrorKind::HelpDisplayed),
        }
    }
}
