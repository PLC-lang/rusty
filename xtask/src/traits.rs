use std::time::Instant;

use xshell::{Cmd, Shell};

use super::Metrics;

pub trait Task {
    /// Prepares its environment to execute command(s).
    fn prepare(&self, sh: &Shell) -> anyhow::Result<()>;

    /// Executes command(s), typically benchmarking it along the way.
    fn execute(&self, sh: &Shell, metrics: &mut Metrics) -> anyhow::Result<()>;
}

/// Trait Extension for the [`xshell::Cmd`] struct.
pub trait Benchmark {
    const ITERATIONS_PER_BENCHMARK: u64 = 3;

    /// Benchmarks a command specified by the [`self`] argument measuring its wall-time,
    /// collecting and inserting the data into the [`Metrics`] struct. The `name` thereby
    /// specifies the to be benchmarked task (e.g. `rustyc`) whereas the `desc` argument
    /// describes how the task ran (e.g. with the `-Oaggressive` flag).
    fn benchmark(&self, metrics: &mut Metrics, name: &str, desc: &str) -> anyhow::Result<()>;
}

impl<'sh> Benchmark for Cmd<'sh> {
    fn benchmark(&self, metrics: &mut Metrics, name: &str, desc: &str) -> anyhow::Result<()> {
        let mut elapsed_sum = 0;
        for _ in 0..Self::ITERATIONS_PER_BENCHMARK {
            let now = Instant::now();
            self.run()?;
            let elapsed = now.elapsed();

            elapsed_sum += elapsed.as_millis() as u64;
        }

        metrics.metrics.insert(format!("{name}/{desc}"), elapsed_sum / Self::ITERATIONS_PER_BENCHMARK);
        Ok(())
    }
}
