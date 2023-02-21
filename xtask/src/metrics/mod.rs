use crate::metrics::{oscat::Oscat, sieve::Sieve};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    env, fs,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};
use sysinfo::{CpuExt, System, SystemExt};
use xshell::{cmd, Shell};

mod oscat;
mod sieve;

const ITERATIONS_PER_BENCHMARK: u64 = 3;

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

trait Task {
    /// Prepares its environment to execute a benchmark
    fn prepare(&self, sh: &Shell) -> anyhow::Result<()>;

    /// Executes a benchmark
    fn execute(&self, sh: &Shell, metrics: &mut Metrics) -> anyhow::Result<()>;
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
        // self.push(sh)?;

        Ok(())
    }

    fn push(&mut self, sh: &Shell) -> anyhow::Result<()> {
        let token = env::var("GITHUB_TOKEN")?;
        cmd!(sh, "git clone https://{token}@github.com/plc-lang/metrics").run()?;

        let mut file = fs::File::options().append(true).open("metrics/metrics.json")?;
        writeln!(file, "{}", serde_json::to_string(self)?)?;

        let _dir = sh.push_dir("metrics");
        cmd!(sh, "git add . ").run()?;
        cmd!(sh, "git -c user.name=bot -c user.email=doesnt@really.matter commit --message Bump").run()?;
        cmd!(sh, "git push origin main").run()?;

        Ok(())
    }
}