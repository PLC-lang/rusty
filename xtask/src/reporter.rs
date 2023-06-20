use std::{
    collections::BTreeMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use sysinfo::{CpuExt, System, SystemExt};

use anyhow::Result;
use serde::Serialize;
use xshell::{cmd, Shell};

use crate::ReporterType;

mod git_reporter;

#[derive(Serialize)]
pub struct BenchmarkReport {
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
    pub(crate) metrics: BTreeMap<String, u128>,
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

impl BenchmarkReport {
    pub fn new(data: Vec<(String, Duration)>) -> Result<Self> {
        let mut metrics = BTreeMap::new();
        for (name, duration) in data {
            metrics.insert(name, duration.as_millis());
        }
        let sh = Shell::new()?;
        let commit = cmd!(sh, "git rev-parse HEAD").read()?;
        Ok(BenchmarkReport {
            host: Host::new(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            commit,
            metrics,
        })
    }
}

pub trait Reporter {
    /// Persists the benchmark data into a database
    fn persist(&self, report: BenchmarkReport) -> Result<()>;
}

pub fn from_type(r_type: ReporterType) -> Box<dyn Reporter> {
    match r_type {
        ReporterType::Sysout => Box::new(SysoutReporter),
        _ => todo!(),
    }
}

pub struct SysoutReporter;

impl Reporter for SysoutReporter {
    fn persist(&self, report: BenchmarkReport) -> Result<()> {
        println!("{}", serde_json::to_string_pretty(&report)?);
        Ok(())
    }
}
