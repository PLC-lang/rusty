use anyhow::Result;
use plc::{ast::SourceRangeFactory, lexer::IdProvider};
use std::{
    path::Path,
    time::{Duration, Instant},
};
use xshell::{cmd, Shell};

use self::{compile::Compile, run::Run};

pub(crate) mod compile;
pub(crate) mod run;

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

pub(crate) fn get_default_tasks(work_dir: &Path, compiler: &Path) -> Result<Vec<Box<dyn Task>>> {
    //Clone the extra required code
    println!("Clone Oscat into the benchmarks");
    let sh = Shell::new()?;
    cmd!(&sh, "git clone https://github.com/plc-lang/oscat --depth 1 {work_dir}/oscat").run()?;
    let mut tasks: Vec<Box<dyn Task>> = vec![];
    //Create a default benchmark run
    //This includes oscat in 4 different opt
    for opt in &["none", "less", "default", "aggressive"] {
        let task = Compile {
            name: "oscat".into(),
            compiler: compiler.into(),
            directory: work_dir.join("oscat"),
            optimization: opt.to_string(),
        };
        tasks.push(Box::new(task));
    }

    // This includes the sieve of eratosthenes in
    // C
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
    // and ST
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

    Ok(tasks)
}
