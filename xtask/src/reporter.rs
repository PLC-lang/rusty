use clap::ValueEnum;
use std::{
    collections::BTreeMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use sysinfo::{CpuExt, System, SystemExt};

use anyhow::Result;
use serde::Serialize;
use xshell::{cmd, Shell};

pub(crate) mod git;
pub mod sql;
pub(crate) mod sysout;

use sql::SqlReporter;

use self::{git::GitReporter, sysout::SysoutReporter};

pub trait Reporter {
    /// Persists the benchmark data into a database
    fn persist(&self, report: BenchmarkReport) -> Result<()>;
}

#[derive(Default, ValueEnum, Clone, Copy)]
pub enum ReporterType {
    Sql,
    Git,
    #[default]
    Sysout,
}

#[derive(Serialize)]
pub struct BenchmarkReport {
    #[serde(skip_serializing)]
    pub id: u64,
    /// Host information, see [`Host`].
    pub host: Host,

    /// Unix timestamp of when this xtask was called.
    pub timestamp: u64,

    /// Commit hash on which the benchmark ran.
    pub commit: String,

    /// Collected benchmarks, where the first tuple element describes the benchmark and the second
    /// element is its raw wall-time value in milliseconds.
    /// For example one such element might be `("oscat/aggressive", 8000)`, indicating an oscat build
    /// with the `aggressive` optimization flag took 8000ms.
    pub(crate) metrics: BTreeMap<String, u64>,
}

#[derive(Serialize, Debug)]
pub struct Host {
    #[serde(skip_serializing)]
    pub id: u64,
    pub os: String,
    pub cpu: String,
    pub mem: u64,
}

impl Host {
    fn new() -> Self {
        let sys = System::new_all();

        let os = sys.long_os_version().unwrap_or("n/a".to_string());
        let cpu = sys.global_cpu_info().brand().to_owned();
        let mem = sys.total_memory() / 1024;

        Self { id: 0, os, cpu, mem }
    }
}

impl BenchmarkReport {
    pub fn new(data: Vec<(String, Duration)>) -> Result<Self> {
        let mut metrics = BTreeMap::new();
        for (name, duration) in data {
            metrics.insert(name, duration.as_millis() as u64);
        }
        let sh = Shell::new()?;
        let commit = cmd!(sh, "git rev-parse HEAD").read()?;
        Ok(BenchmarkReport {
            id: 0,
            host: Host::new(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            commit,
            metrics,
        })
    }
}

pub fn from_type(r_type: ReporterType) -> Box<dyn Reporter> {
    match r_type {
        ReporterType::Sysout => Box::new(SysoutReporter),
        ReporterType::Sql => Box::new(SqlReporter),
        ReporterType::Git => Box::new(GitReporter),
    }
}
