use serde::Serialize;
use std::{fs, io::Write};
use xshell::{cmd, Shell};

use super::Reporter;

#[derive(Serialize)]
pub struct GitReporter;

impl Reporter for GitReporter {
    fn persist(&self, report: super::BenchmarkReport) -> anyhow::Result<()> {
        let sh = Shell::new()?;
        let branch = "metrics-data";
        let filename = "metrics.json";
        let message = format!("Update {}", report.commit);
        let user_name = cmd!(sh, "git log -1 --pretty=format:'%an'").read()?;
        let user_mail = cmd!(sh, "git log -1 --pretty=format:'%ae'").read()?;

        cmd!(sh, "git pull").run()?;
        cmd!(sh, "git config user.name \"{user_name}\"").run()?;
        cmd!(sh, "git config user.email \"{user_mail}\"").run()?;
        cmd!(sh, "git checkout {branch}").run()?;

        let mut file = fs::File::options().create(true).append(true).open(filename)?;
        writeln!(file, "{}", serde_json::to_string(&report)?)?;

        cmd!(sh, "git add {filename}").run()?;
        cmd!(sh, "git commit -m {message}").run()?;
        cmd!(sh, "git push origin {branch}").run()?;

        Ok(())
    }
}
