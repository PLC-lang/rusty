use anyhow::Result;
use std::{path::Path, time::Duration};
use xshell::{cmd, Shell};

use crate::{reporter::DurationWrapper, task::lexer::Lexer};

use self::{compile::Compile, run::Run};

pub(crate) mod compile;
pub(crate) mod lexer;
pub(crate) mod run;
pub(crate) mod grammar;

pub(crate) trait Task {
    /// Returns the name of the task being benchmarked
    fn get_name(&self) -> String;

    /// Returns a [DurationWrapper] with its inner type being the given argument
    fn get_wrapped(&self, duration: Duration) -> DurationWrapper;

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
        self.execute()?; // Cold Run

        let mut duration = Duration::from_millis(0);
        for _ in 0..executions {
            duration += self.execute()?;
        }

        Ok(duration / executions)
    }
}

pub(crate) fn get_default_tasks(work_dir: &Path, compiler: &Path) -> Result<Vec<Box<dyn Task>>> {
    let sh = Shell::new()?;
    cmd!(&sh, "git clone https://github.com/plc-lang/oscat --depth 1 {work_dir}/oscat").run()?;

    let mut tasks: Vec<Box<dyn Task>> = vec![];
    tasks.extend(oscat(work_dir, compiler));
    tasks.extend(sieve_st(work_dir, compiler));
    tasks.extend(sieve_c(work_dir));
    tasks.push(Box::new(Lexer("combined.st")));

    Ok(tasks)
}

/// Benchmark task for `oscat`
fn oscat(work_dir: &Path, compiler: &Path) -> Vec<Box<dyn Task>> {
    let mut tasks: Vec<Box<dyn Task>> = vec![];
    for opt in &["none", "less", "default", "aggressive"] {
        let task = Compile {
            name: "oscat".into(),
            compiler: compiler.into(),
            directory: work_dir.join("oscat"),
            optimization: opt.to_string(),
        };

        tasks.push(Box::new(task));
    }

    tasks
}

/// Benchmark task for `res/sieve.c`
fn sieve_c(work_dir: &Path) -> Vec<Box<dyn Task>> {
    let mut tasks: Vec<Box<dyn Task>> = vec![];
    for opt in ["0", "1", "2", "3"] {
        let task = Run {
            name: "sieve-c".into(),
            optimization: opt.to_string(),
            compiler: "cc".into(),
            location: "xtask/res/sieve.c".into(),
            parameters: None,
            work_dir: work_dir.into(),
        };

        tasks.push(Box::new(task));
    }

    tasks
}

/// Benchmark task for `res/sieve.st`
fn sieve_st(work_dir: &Path, compiler: &Path) -> Vec<Box<dyn Task>> {
    let mut tasks: Vec<Box<dyn Task>> = vec![];
    for opt in ["none", "less", "default", "aggressive"] {
        let task = Run {
            name: "sieve-st".into(),
            optimization: opt.to_string(),
            compiler: compiler.into(),
            location: "xtask/res/sieve.st".into(),
            parameters: Some("--linker=cc".into()),
            work_dir: work_dir.into(),
        };

        tasks.push(Box::new(task));
    }

    tasks
}
