use super::{BenchmarkReport, Reporter};
use anyhow::Result;

pub struct SysoutReporter;

impl Reporter for SysoutReporter {
    fn persist(&self, report: BenchmarkReport) -> Result<()> {
        println!("Benchmark results for commit: {}", &report.commit);
        println!("Host information:");
        println!("-----------------");
        println!("  CPU: {}", &report.host.cpu);
        println!("  Memory: {}", &report.host.mem);
        println!("  OS: {}", &report.host.os);
        println!("-----------------");
        for (name, time) in report.metrics {
            println!("Run {name} took {time} ms");
        }
        println!("-----------------");
        Ok(())
    }
}
