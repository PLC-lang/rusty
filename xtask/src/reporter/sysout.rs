use super::{BenchmarkReport, Reporter};
use anyhow::Result;

pub struct SysoutReporter;

impl Reporter for SysoutReporter {
    fn persist(&self, report: BenchmarkReport) -> Result<()> {
        println!("{}", serde_json::to_string_pretty(&report)?);
        Ok(())
    }
}
