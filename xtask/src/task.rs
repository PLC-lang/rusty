use std::time::Duration;

use anyhow::Result;

pub(crate) trait Task {
    /// Returns the name of the task being benchmarked
    fn get_name(&self) -> String;
    /// Executes any actions required before time measurement starts
    /// By default we make sure we have a release build ready
    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }
    /// Executes the task to be measured and returns the time it took
    fn execute(&self) -> Result<Duration>;

    /// Benchmarks the current task and returns the avarage execution time
    fn benchmark(&mut self, executions: u32) -> Result<Duration> {
        self.prepare()?;
        //Cold run
        self.execute()?;
        let mut duration = Duration::from_millis(0);
        for _ in 0..executions {
            duration += self.execute()?;
        }

        Ok(duration / executions)
    }
}
