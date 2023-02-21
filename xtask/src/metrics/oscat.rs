use std::time::Instant;

use xshell::{cmd, Shell};

use super::{Task, ITERATIONS_PER_BENCHMARK};

pub struct Oscat;
impl Task for Oscat {
    fn prepare(&self, sh: &Shell) -> anyhow::Result<()> {
        cmd!(sh, "git clone https://github.com/plc-lang/oscat ./benchmark/oscat").run()?;
        cmd!(sh, "git clone https://github.com/plc-lang/standardfunctions ./benchmark/oscat/sf").run()?;
        sh.copy_file("./target/release/rustyc", "./benchmark/oscat")?;

        sh.create_dir("./benchmark/oscat/lib").unwrap();
        sh.create_dir("./benchmark/oscat/include").unwrap();

        let standardfunctions = sh.push_dir("./benchmark/oscat/sf");
        cmd!(sh, "cargo b --release").run()?;
        std::mem::drop(standardfunctions);

        let oscat = sh.push_dir("./benchmark/oscat");
        sh.copy_file("sf/target/release/libiec61131std.so", "lib").unwrap();
        sh.read_dir("sf/iec61131-st/").unwrap().iter().for_each(|f| sh.copy_file(f, "include").unwrap());
        std::mem::drop(oscat);

        Ok(())
    }

    fn execute(&self, sh: &Shell, metrics: &mut super::Metrics) -> anyhow::Result<()> {
        let _oscat = sh.push_dir("./benchmark/oscat");

        for optimization in &["-Onone", "-Oless", "-Odefault", "-Oaggressive"] {
            let mut elapsed_sum = 0;
            for _ in 0..ITERATIONS_PER_BENCHMARK {
                let now = Instant::now();
                cmd!(sh, "./rustyc {optimization} build").quiet().run()?;
                let elapsed = now.elapsed();

                elapsed_sum += elapsed.as_millis() as u64;
            }

            metrics.metrics.insert(format!("oscat {optimization}"), elapsed_sum / ITERATIONS_PER_BENCHMARK);
        }

        Ok(())
    }
}
