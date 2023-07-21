use crate::reporter::DurationWrapper;

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
        for (name, duration) in report.metrics {
            let (duration, duration_format) = match duration {
                DurationWrapper::Millis(duration) => (duration.as_millis(), "ms"),
                DurationWrapper::Micros(duration) => (duration.as_micros(), "us"),
            };

            println!("Run {name} took {duration} {duration_format}");
        }
        println!("-----------------");
        Ok(())
    }
}
