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
pub struct Metrics {
    /// Host information, see [`Host`].
    host: Host,

    /// Unix timestamp of when this xtask was called.
    timestamp: u64,

    /// Commit hash on which the benchmark ran.
    commit: String,

    /// Collected benchmarks, where the first tuple element describes the benchmark and the second
    /// element is its raw wall-time value in milliseconds.
    /// For example one such element might be `("oscat/aggressive", 8000)`, indicating an oscat build
    /// with the `aggressive` optimization flag took 8000ms.
    metrics: BTreeMap<String, u64>,
}

#[derive(Serialize)]
struct Host {
    os: String,
    cpu: String,
    mem: u64,
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

    /// Starts the execution of various [`Task`]s, collecting bechmark data.
    /// Additionally the data is pushed onto the `metrics` branch on rusty if the task
    /// is executed within a CI enviroment, i.e. specified by the `CI_RUN` environment flag.
    pub fn execute(&mut self, sh: &Shell) -> anyhow::Result<()> {
        // Remove and re-create the folder in case of previous dry runs
        sh.remove_path("./benchmark")?;
        sh.create_dir("./benchmark")?;

        let tasks: Vec<Box<dyn Task>> = vec![Box::new(Oscat), Box::new(Sieve)];
        for task in tasks {
            task.prepare(sh)?;
            task.execute(sh, self)?;
        }

        // Only commit and push IF we executed the task within a CI job
        if std::env::var("CI_RUN").is_ok() {
            self.finalize(sh)?;
        }

        println!("{}", serde_json::to_string_pretty(self)?);
        Ok(())
    }

    /// Appends the collected data to a JSON file, commiting and pushing it onto
    /// the `metrics` branch hosted on RuSTy. Whoever the author of the last commit
    /// on the RuSTy master branch is thereby also the author of this commit.
    pub fn finalize(&self, sh: &Shell) -> anyhow::Result<()> {
        let branch = "metrics";
        let filename = "metrics.json";
        let message = format!("'Append {}'", self.commit);
        let user_name = cmd!(sh, "git log -1 --pretty=format:'%an'").read()?;
        let user_mail = cmd!(sh, "git log -1 --pretty=format:'%ae'").read()?;

        cmd!(sh, "git pull").run()?;
        cmd!(sh, "git config user.name \"{user_name}\"").run()?;
        cmd!(sh, "git config user.email \"{user_mail}\"").run()?;
        cmd!(sh, "git checkout {branch}").run()?;

        let mut file = fs::File::options().create(true).append(true).open(filename)?;
        writeln!(file, "{}", serde_json::to_string(self)?)?;

        cmd!(sh, "git add {filename}").run()?;
        cmd!(sh, "git commit -m {message}").run()?;
        cmd!(sh, "git push origin {branch}").run()?;

        Ok(())
    }
}
