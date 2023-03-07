use std::str::FromStr;

use anyhow::anyhow;
use metrics::Metrics;
use xshell::Shell;

mod metrics;
mod traits;

enum Task {
    Metrics,
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "metrics" => Ok(Task::Metrics),
            _ => Err(anyhow!("Unrecognized task '{s}'")),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let Some(arg) = std::env::args().nth(1) else { anyhow::bail!("No argument specified, try `xtask <arg>`") };

    match Task::from_str(&arg)? {
        Task::Metrics => Metrics::new(&sh)?.execute(&sh)?,
    }

    Ok(())
}
