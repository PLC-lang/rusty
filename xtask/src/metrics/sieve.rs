use xshell::cmd;

use super::traits::{Benchmark, Task};

pub struct Sieve;
impl Task for Sieve {
    fn prepare(&self, sh: &xshell::Shell) -> anyhow::Result<()> {
        sh.copy_file("./target/release/rustyc", "./benchmark")?;
        sh.copy_file("./xtask/res/sieve.st", "./benchmark")?;

        Ok(())
    }

    fn execute(&self, sh: &xshell::Shell, metrics: &mut super::Metrics) -> anyhow::Result<()> {
        let _path = sh.push_dir("./benchmark");

        // Compile with optimization flag, then benchmark and collect data
        for flag in ["none", "less", "default", "aggressive"] {
            cmd!(sh, "./rustyc --linker=clang -O{flag} sieve.st").run()?;
            cmd!(sh, "./sieve").ignore_status().benchmark(metrics, "sieve", flag)?;
        }

        Ok(())
    }
}
