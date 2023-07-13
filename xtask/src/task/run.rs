use std::time::Instant;

use std::path::PathBuf;
use xshell::Shell;

use crate::reporter::DurationFormat;
use crate::task::Task;
use xshell::cmd;

pub(crate) struct Run {
    pub name: String,
    pub optimization: String,
    // Call the appropriate compiler with the file name. Output and optimization are not specifed here
    pub compiler: PathBuf,
    pub location: PathBuf,
    pub parameters: Option<String>,
    pub work_dir: PathBuf,
}

impl Task for Run {
    fn execute(&self) -> anyhow::Result<std::time::Duration> {
        //Run the application
        let shell = Shell::new()?;
        let task = self.work_dir.join(&self.name).with_extension("out");
        let start = Instant::now();
        cmd!(&shell, "{task}").run()?;

        Ok(start.elapsed())
    }

    fn get_name(&self) -> String {
        format!("{}/{}", self.name, self.optimization)
    }

    fn prepare(&mut self) -> anyhow::Result<()> {
        let shell = Shell::new()?;
        //Compile the application with the correct optimization flag
        let command = &self.compiler;
        let opt = &self.optimization;
        let output = self.work_dir.join(&self.name).with_extension("out");
        let file = &self.location;
        let cmd = cmd!(&shell, "{command} {file} -O{opt} -o {output}");
        let cmd = if let Some(parameters) = &self.parameters { cmd.arg(parameters) } else { cmd };

        cmd.run()?;

        if !output.exists() {
            anyhow::bail!("Output does not exist");
        }
        Ok(())
    }

    fn get_time_format(&self) -> crate::reporter::DurationFormat {
        DurationFormat::Millis
    }
}
