use anyhow::Result;
use std::time::Instant;
use std::{path::PathBuf, time::Duration};
use xshell::cmd;
use xshell::Shell;

use crate::task::Task;

pub(crate) struct Compile {
    pub name: String,
    pub directory: PathBuf,
    pub optimization: String,
}

impl Task for Compile {
    fn get_name(&self) -> String {
        format!("{}/{}", &self.name, &self.optimization)
    }

    fn execute(&self) -> Result<Duration> {
        let sh = Shell::new()?;
        let compiler = sh.var("COMPILER")?;
        //Navitage to directory
        sh.change_dir(&self.directory);
        let opt = &self.optimization;

        // Run compile
        let start = Instant::now();
        cmd!(&sh, "{compiler} build -O{opt}").run()?;
        let end = start.elapsed();

        Ok(end)
    }
}
