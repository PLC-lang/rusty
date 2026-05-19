use std::path::PathBuf;
use std::process::ExitCode;

use plc_lsp::Settings;

fn main() -> ExitCode {
    env_logger::init();
    let settings = match parse_args() {
        Ok(s) => s,
        Err(msg) => {
            eprintln!("{msg}");
            return ExitCode::from(2);
        }
    };
    match plc_lsp::run(settings) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e:?}");
            ExitCode::FAILURE
        }
    }
}

fn parse_args() -> Result<Settings, String> {
    let mut settings = Settings::default();
    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--config" => {
                let path = iter.next().ok_or_else(|| "--config requires a path argument".to_string())?;
                settings.config_override = Some(PathBuf::from(path));
            }
            "--help" | "-h" => {
                println!("plc-lsp [--config <plc.json>]");
                std::process::exit(0);
            }
            "--version" | "-V" => {
                println!("plc-lsp {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            other => {
                return Err(format!("Unknown argument: {other}"));
            }
        }
    }
    Ok(settings)
}
