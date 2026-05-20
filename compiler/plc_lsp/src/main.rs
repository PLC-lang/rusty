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
                println!("plc-lsp [--config <plc.json>] [--stdio]");
                std::process::exit(0);
            }
            "--version" | "-V" => {
                println!("plc-lsp {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            // Transport selection flags that LSP clients (notably
            // `vscode-languageclient`) pass by convention. We only support
            // stdio; accept --stdio silently, reject the others clearly.
            "--stdio" => {
                log::debug!("--stdio accepted (only transport supported)");
            }
            "--node-ipc" | "--socket" | "--pipe" => {
                return Err(format!("transport {arg} not supported by plc-lsp; only stdio is implemented",));
            }
            _ => {
                // `--clientProcessId=<pid>` and other vendor-specific
                // flags clients sometimes add: warn but keep going so a
                // new client flag doesn't crash the server.
                log::warn!("ignoring unknown argument: {arg}");
            }
        }
    }
    Ok(settings)
}
