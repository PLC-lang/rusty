use clap::ValueEnum;
use std::{
    collections::BTreeMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use sysinfo::{CpuExt, System, SystemExt};

use anyhow::Result;
use serde::{Serialize, Serializer};
use xshell::{cmd, Shell};

pub(crate) mod git;
#[cfg(feature = "sql")]
pub mod sql;
pub(crate) mod sysout;

#[cfg(feature = "sql")]
use sql::SqlReporter;

use self::{git::GitReporter, sysout::SysoutReporter};

pub trait Reporter {
    /// Persists the benchmark data into a database
    fn persist(&self, report: BenchmarkReport) -> Result<()>;
}

#[derive(Default, ValueEnum, Clone, Copy)]
pub enum ReporterType {
    #[cfg(feature = "sql")]
    Sql,
    Git,
    #[default]
    Sysout,
}

#[derive(Serialize)]
pub struct BenchmarkReport {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub id: u64,
    /// Host information, see [`Host`].
    pub host: Host,

    /// Unix timestamp of when this xtask was called.
    pub timestamp: u64,

    /// Commit hash on which the benchmark ran.
    pub commit: String,

    /// Collected benchmarks, where the first tuple element describes the benchmark and the second element
    /// is its raw wall-time value in milli- or microseconds, however it is defined in [`DurationFormat`].
    /// For example one such element might be `("oscat/aggressive", (8000, DurationFormat::Millis))`
    /// indicating that compiling oscat with the `aggressive` optimization flag took 8000 milliseconds.
    pub metrics: BTreeMap<String, DurationWrapper>,
}

impl Serialize for DurationWrapper {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DurationWrapper::Millis(value) => serializer.serialize_u128(value.as_millis()),
            DurationWrapper::Micros(value) => serializer.serialize_u128(value.as_micros()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Host {
    #[allow(dead_code)]
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

pub enum DurationWrapper {
    Millis(Duration),
    Micros(Duration),
}

impl BenchmarkReport {
    pub fn new(data: Vec<(String, DurationWrapper)>) -> Result<Self> {
        let mut metrics = BTreeMap::new();
        for (name, duration) in data {
            metrics.insert(name, duration);
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
        #[cfg(feature = "sql")]
        ReporterType::Sql => Box::new(SqlReporter),
        ReporterType::Git => Box::new(GitReporter),
    }
}
