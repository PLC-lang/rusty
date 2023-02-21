use metrics::Metrics;
use xshell::Shell;

mod metrics;

fn main() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    match std::env::args().nth(1) {
        Some(arg) => match arg.as_ref() {
            "metrics" => Metrics::new(&sh)?.execute(&sh)?,

            _ => anyhow::bail!("Unrecognized task, try: metrics"),
        },

        None => anyhow::bail!("No argument specified, try `cargo xtask <task-id>"),
    };

    Ok(())
}