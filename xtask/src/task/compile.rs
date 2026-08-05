use anyhow::Result;
use std::time::Instant;
use std::{path::PathBuf, time::Duration};
use xshell::Shell;

use crate::reporter::DurationWrapper;
use crate::task::Task;

pub(crate) struct Compile {
    pub name: String,
    pub compiler: PathBuf,
    pub directory: PathBuf,
    pub optimization: String,
}

impl Task for Compile {
    fn get_name(&self) -> String {
        format!("{}/{}", &self.name, &self.optimization)
    }

    fn execute(&self) -> Result<Duration> {
        let sh = Shell::new()?;
        //Navigate to directory
        sh.change_dir(&self.directory);

        // Run compile. Benchmark corpora such as oscat are libraries that reference stdlib runtime
        // symbols they do not define (resolved at load time), so allow undefined symbols — otherwise
        // the default `--no-undefined` for shared-object links would reject the build.
        let start = Instant::now();
        sh.cmd(&self.compiler)
            .args(&["build", "-O", &self.optimization, "--allow-undefined-symbols"])
            .run()?;
        Ok(start.elapsed())
    }

    fn get_wrapped(&self, duration: Duration) -> DurationWrapper {
        DurationWrapper::Millis(duration)
    }
}
