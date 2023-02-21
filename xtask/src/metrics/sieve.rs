use std::time::Instant;

use xshell::cmd;

use super::{Task, ITERATIONS_PER_BENCHMARK};

pub struct Sieve;
impl Task for Sieve {
    fn prepare(&self, sh: &xshell::Shell) -> anyhow::Result<()> {
        sh.copy_file("./target/release/rustyc", "./benchmark")?;
        sh.copy_file("./xtask/res/sieve.st", "./benchmark")?;

        Ok(())
    }

    fn execute(&self, sh: &xshell::Shell, metrics: &mut super::Metrics) -> anyhow::Result<()> {
        let _path = sh.push_dir("./benchmark");

        for optimization in &["-Onone", "-Oless", "-Odefault", "-Oaggressive"] {
            let mut elapsed_sum = 0;
            cmd!(sh, "./rustyc {optimization} --linker=clang sieve.st").run()?; // TODO: move out
            for _ in 0..ITERATIONS_PER_BENCHMARK {
                let now = Instant::now();
                cmd!(sh, "./sieve").run().ok();
                let elapsed = now.elapsed();

                elapsed_sum += elapsed.as_millis() as u64;
            }

            metrics.metrics.insert(format!("sieve {optimization}"), elapsed_sum / ITERATIONS_PER_BENCHMARK);
        }

        Ok(())
    }
}