use std::env;

use plc_driver::cli::CompileParameters;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse CLI first to extract log level, then initialize the logger.
    // If parsing fails, initialize default logger so the error prints cleanly.
    let log_filter = CompileParameters::parse(&args).ok().and_then(|p| p.log_level_filter());

    let mut builder = env_logger::Builder::from_default_env();
    if let Some(filter) = log_filter {
        builder.filter_level(filter);
    }
    builder.init();

    if let Err(e) = plc_driver::compile(&args) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
