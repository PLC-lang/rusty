use crate::metrics::{oscat::Oscat, sieve::Sieve, traits::Task};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};
use sysinfo::{CpuExt, System, SystemExt};
use xshell::{cmd, Shell};

mod oscat;
mod sieve;
mod traits;

#[derive(Serialize)]
struct Host {
    os: String,
    cpu: String,
    mem: u64,
}

#[derive(Serialize)]
pub struct Metrics {
    host: Host,
    timestamp: u64,
    commit: String,
    metrics: BTreeMap<String, u64>,
}

impl Host {
    fn new() -> Self {
        let sys = System::new_all();

        let os = sys.long_os_version().unwrap_or("n/a".to_string());
        let cpu = sys.global_cpu_info().brand().to_owned();
        let mem = sys.total_memory() / 1024;

        Self { os, cpu, mem }
    }
}

impl Metrics {
    pub fn new(sh: &Shell) -> anyhow::Result<Self> {
        // Needed because of "fatal: detected deubious ownership in repository at '/build'" error
        cmd!(sh, "git config --global --add safe.directory /build").run()?;

        let host = Host::new();
        let commit = cmd!(sh, "git rev-parse HEAD").read()?;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let metrics = BTreeMap::new();

        Ok(Self { host, timestamp, commit, metrics })
    }

    pub fn execute(&mut self, sh: &Shell) -> anyhow::Result<()> {
        // Remove and re-create the folder in case of previous dry runs
        sh.remove_path("./benchmark")?;
        sh.create_dir("./benchmark")?;

        cmd!(sh, "cargo b --release").run()?;

        let tasks: Vec<Box<dyn Task>> = vec![Box::new(Oscat), Box::new(Sieve)];
        for task in tasks {
            task.prepare(sh)?;
            task.execute(sh, self)?;
        }

        println!("{}", serde_json::to_string_pretty(self)?);

        // Only commit and push IF we executed the task within a CI job
        if std::env::var("CI_RUN").is_ok() {
            self.finalize(sh)?;
        }

        Ok(())
    }

    pub fn finalize(&self, sh: &Shell) -> anyhow::Result<()> {
        let branch = "metrics-data";
        let filename = "metrics.json";
        let user_name = cmd!(sh, "git log -1 --pretty=format:'%an'").read()?;
        let user_mail = cmd!(sh, "git log -1 --pretty=format:'%ae'").read()?;

        cmd!(sh, "git pull").run()?;
        cmd!(sh, "git config user.name \"{user_name}\"").run()?;
        cmd!(sh, "git config user.email \"{user_mail}\"").run()?;
        cmd!(sh, "git checkout {branch}").run()?;

        let mut file = fs::File::options().create(true).append(true).open(filename)?;
        writeln!(file, "{}", serde_json::to_string(self)?)?;

        cmd!(sh, "git add {filename}").run()?;
        cmd!(sh, "git commit -m 'Update data'").run()?;
        cmd!(sh, "git push origin {branch}").run()?;

        Ok(())
    }
}