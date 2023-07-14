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
        for (name, (time, time_format)) in report.metrics {
            let (time, format) = match time_format {
                DurationFormat::Micros => (time.as_micros(), "us"),
                DurationFormat::Millis => (time.as_millis(), "ms"),
            };

            println!("Run {name} took {time} {format}");
        }
        println!("-----------------");
        Ok(())
    }
}
