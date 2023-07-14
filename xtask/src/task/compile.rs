use anyhow::Result;
use std::time::Instant;
use std::{path::PathBuf, time::Duration};
use xshell::Shell;

use crate::reporter::DurationFormat;
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

        // Run compile
        let start = Instant::now();
        sh.cmd(&self.compiler).args(&["build", "-O", &self.optimization]).run()?;
        Ok(start.elapsed())
    }

    fn get_duration_format(&self) -> DurationFormat {
        DurationFormat::Millis
    }
}
