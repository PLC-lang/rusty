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
        // `anstream::eprintln!` auto-detects whether stderr supports ANSI:
        // raw escapes on TTYs that handle them, Win32 console API on legacy
        // `cmd.exe`, plain text when piped. Replaces the manual
        // `is_terminal()` + raw-escape branch this used to carry.
        let style = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into())).bold();
        anstream::eprintln!("{style}error{style:#}: {e}");
        std::process::exit(1)
    }
}
