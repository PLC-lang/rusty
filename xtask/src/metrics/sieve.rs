use xshell::cmd;

use super::traits::{Benchmark, Task};

pub struct Sieve;
impl Task for Sieve {
    fn prepare(&self, sh: &xshell::Shell) -> anyhow::Result<()> {
        cmd!(sh, "cargo b --release").run()?;
        sh.copy_file("./target/release/rustyc", "./benchmark")?;
        sh.copy_file("./xtask/res/sieve.st", "./benchmark")?;
        sh.copy_file("./xtask/res/sieve.c", "./benchmark")?;

        Ok(())
    }

    fn execute(&self, sh: &xshell::Shell, metrics: &mut super::Metrics) -> anyhow::Result<()> {
        let _path = sh.push_dir("./benchmark");

        for flag in ["none", "less", "default", "aggressive"] {
            cmd!(sh, "./rustyc --linker=clang -O{flag} sieve.st").run()?;
            cmd!(sh, "./sieve").ignore_status().benchmark(metrics, "sieve-st", flag)?;
        }

        for flag in ["0", "1", "2", "3"] {
            cmd!(sh, "gcc -O{flag} sieve.c").run()?;
            cmd!(sh, "./a.out").ignore_status().benchmark(metrics, "sieve-c", flag)?;
        }

        Ok(())
    }
}
