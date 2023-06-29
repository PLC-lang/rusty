use std::time::Instant;

use xshell::Shell;

use crate::task::Task;
use xshell::cmd;

pub(crate) struct Run {
    pub name: String,
    pub optimization: String,
    // Call the appropriate compiler with the file name. Output and optimization are not specifed here
    pub compiler: String,
    pub location: String,
    pub parameters: Option<String>,
}

impl Task for Run {
    fn execute(&self) -> anyhow::Result<std::time::Duration> {
        //Run the application
        let shell = Shell::new()?;
        let task = std::env::current_dir()?.join("benchmarks").join(&self.name).with_extension("out");
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
        let output = std::env::current_dir()?.join("benchmarks").join(&self.name).with_extension("out");
        let file = &self.location;
        let cmd = cmd!(&shell, "{command} {file} -O{opt} -o {output}");
        let cmd = if let Some(parameters) = &self.parameters { cmd.arg(parameters) } else { cmd };

        cmd.run()?;

        if !output.exists() {
            anyhow::bail!("Output does not exist");
        }
        Ok(())
    }
}
