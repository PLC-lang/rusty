use crate::reporter::DurationFormat;

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
        for (name, (duration, duration_format)) in report.metrics {
            let (duration, duration_format) = match duration_format {
                DurationFormat::Micros => (duration.as_micros(), "us"),
                DurationFormat::Millis => (duration.as_millis(), "ms"),
            };

            println!("Run {name} took {duration} {duration_format}");
        }
        println!("-----------------");
        Ok(())
    }
}
